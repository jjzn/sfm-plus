use rusty_tesseract::Image;
use opencv::imgproc::*;
use opencv::core::*;
use opencv::prelude::{MatTraitConst, MatTraitConstManual};
use rayon::prelude::*;
use regex::Regex;
use std::io::Read;
use std::convert::TryInto;

use image::GenericImageView;

use crate::types::*;

const MAX_IMAGE_BYTES: usize = 10_000_000; // 10 MB
const IMAGE_ELEMENT_OFFSET: u32 = 155;
const IMAGE_ELEMENT_HEIGHT: u32 = 80;
const IMAGE_BASE_URL: &str = "https://info.trensfm.com/sapi/ivi_imagen?ubicacion=";
const N_IMAGE_REGIONS: u8 = 7; // 7 regions per image => max 7 trains per image

const HEADSIGNS: phf::Map<&str, &str> = phf::phf_map! {
    "us" => "UIB",
    "ub" => "UIB",
    "uib" => "UIB",
    "inca" => "Inca",
    "man" => "Manacor",
    "sapo" => "Sa Pobla",
    "obla" => "Sa Pobla",
    "palma" => "Palma"
};

fn mat_to_image(mat: Mat) -> Image {
    let vec = mat.data_typed().unwrap().to_vec();
    let size = mat.size().unwrap();
    let img: image::GrayImage = image::ImageBuffer::from_raw(
            size.width as u32, size.height as u32, vec).unwrap();

    Image::from_dynamic_image(&img.into()).unwrap()
}

fn structuring_rect(width: i32, height: i32) -> Mat {
    get_structuring_element(
            MORPH_RECT,
            Size { width, height },
            Point::new(-1, -1))
        .unwrap()
}

fn transform_image(img: &mut Mat) {
    let _ = cvt_color(&img.clone(), img, COLOR_RGB2GRAY, 0);

    let _ = dilate(
        &img.clone(), img, &structuring_rect(3, 3),
        Point::new(-1, -1), 1,
        BORDER_CONSTANT, morphology_default_border_value().unwrap());

    let _ = median_blur(&img.clone(), img, 7);

    let _ = erode(
        &img.clone(), img, &structuring_rect(5, 5),
        Point::new(-1, -1), 1,
        BORDER_CONSTANT, morphology_default_border_value().unwrap());

    let _ = threshold(&img.clone(), img, 0., 255., THRESH_OTSU);

    let (n_black, n_white) = {
        let vals: &[u8] = img.data_typed().unwrap();
        let count = |n| vals.iter().filter(|&&x| x == n).count();

        (count(0), count(255))
    };

    if n_black > n_white {
        let _ = bitwise_not(&img.clone(), img, &no_array());
    }

    let _ = copy_make_border(
        &img.clone(), img, 12, 12, 12, 12, BORDER_CONSTANT, 255.into());
}

fn crop_image(img: &mut Mat, x: u32, y: u32, w: u32, h: u32) -> Mat {
    Mat::roi(img, Rect {
        x: x as i32,
        y: y as i32,
        width: w as i32,
        height: h as i32
    }).unwrap()
}

fn split_region(mut img: Mat, idx: u32) -> (Image, Image) {
    let mut name_img = crop_image(
        &mut img,
        0, IMAGE_ELEMENT_OFFSET + idx * IMAGE_ELEMENT_HEIGHT,
        342, IMAGE_ELEMENT_HEIGHT);

    let mut rest_img = crop_image(
        &mut img,
        342, IMAGE_ELEMENT_OFFSET + idx * IMAGE_ELEMENT_HEIGHT,
        426, IMAGE_ELEMENT_HEIGHT);

    transform_image(&mut name_img);
    transform_image(&mut rest_img);

    (mat_to_image(name_img), mat_to_image(rest_img))
}

fn process_region(img_orig: image::DynamicImage, idx: u32) -> Trip {
    let img = {
        let (rows, cols) = img_orig.dimensions();

        let pixels: Vec<VecN<u8, 4>> = img_orig
            .into_rgba8()
            .chunks(4)
            .map(|x| x.try_into().unwrap())
            .map(|x: [u8; 4]| x.into())
            .collect();

        Mat::from_slice_rows_cols(
                &pixels,
                rows as usize,
                cols as usize)
            .unwrap()
    };

    let (name_img, rest_img) = split_region(img, idx);
    let tess_args = rusty_tesseract::Args {
        config_variables: std::collections::HashMap::from([
            ("tessedit_do_invert".into(), "0".into())
        ]),
        lang: "eng".into(),
        dpi: Some(300),
        psm: Some(11),
        oem: None
    };

    let name = rusty_tesseract::image_to_string(&name_img, &tess_args)
        .unwrap().trim().to_lowercase().replace(" ", "");
    let rest = rusty_tesseract::image_to_string(&rest_img, &tess_args)
        .unwrap().trim().to_lowercase().replace(" ", "");

    let re = Regex::new(r"(?ms)(\d\d?[:°\.]?\d\d).+(\d+)$").unwrap();
    let Some(rest_match) = re.captures(&rest) else {
        return Default::default(); };

    println!(">>> {}", name);

    let track = &rest_match[2];
    let time = {
        let mut aux = String::from(&rest_match[1])
            .replace("°", ":")
            .replace(".", ":");

        if !aux.contains(':') {
            aux.insert(aux.len() - 2, ':')
        }

        aux
    };

    let headsign = {
        let Some(found) = HEADSIGNS.keys().find(|&key| name.contains(key)) else {
            return Default::default(); };
        HEADSIGNS.get(found).unwrap()
    };

    println!("{} {} {}", headsign, time, track);
    Trip {
        headsign: headsign.to_string(),
        time: time.try_into().unwrap(),
        track: track.parse().unwrap()
    }
}

fn retrieve_from_bytes(bytes: &[u8]) -> Vec<Trip> {
    let img = image::load_from_memory(bytes).unwrap();

    let ns: Vec<_> = (0..N_IMAGE_REGIONS).collect();

    let mut res: Vec<_> = ns.par_iter()
        .map(|&i| process_region(img.clone(), i as u32))
        .filter(|x| *x != Default::default())
        .collect();

    res.sort_unstable_by_key(|x| x.time.minutes());

    res
}

pub fn retrieve(code: u8) -> Vec<Trip> {
    let bytes = {
        let path = format!("{}{}", IMAGE_BASE_URL, code);
        let response = ureq::get(&path).call().unwrap();

        let len: usize = response
            .header("Content-Length")
            .map(|s| s.parse().unwrap())
            .unwrap_or(MAX_IMAGE_BYTES);

        let mut bytes = Vec::with_capacity(len);
        let _ = response
            .into_reader()
            .take(MAX_IMAGE_BYTES as u64)
            .read_to_end(&mut bytes);

        bytes
    };

    retrieve_from_bytes(&bytes)
}

#[cfg(test)]
mod tests {
    use crate::train_info::*;
    use rocket::serde::json;
    use std::path::PathBuf;

    use rstest::rstest;
    use pretty_assertions::assert_eq;

    #[rstest]
    fn compare_to_file(#[files("test/*.json")] path: PathBuf) {
        let stem = path.file_stem().unwrap().to_str().unwrap();

        let expected: Vec<Trip> = {
            let raw = std::fs::read(format!("test/{}.json", stem)).unwrap();
            json::from_slice(&raw).unwrap()
        };

        let got = {
            let raw = std::fs::read(format!("test/{}.jpg", stem)).unwrap();
            retrieve_from_bytes(&raw)
        };

        assert_eq!(expected, got);
    }
}
