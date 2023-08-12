use rusty_tesseract::Image;
use opencv::imgproc::*;
use opencv::core::{Mat, Point, Size, Vector, BORDER_CONSTANT, copy_make_border};
use std::io::Read;

const MAX_IMAGE_BYTES: usize = 10_000_000; // 10 MB
const IMAGE_ELEMENT_OFFSET: u32 = 155;
const IMAGE_ELEMENT_HEIGHT: u32 = 80;

pub struct TrainTime {
    hour: u8,
    minute: u8
}

pub struct Train {
    headsign: String,
    time: TrainTime,
    track: u8
}

fn vector_to_image(vector: Vector<u8>) -> rusty_tesseract::Image {
    let img = image::load_from_memory(&vector.to_vec()).unwrap();

    rusty_tesseract::Image::from_dynamic_image(&img).unwrap()
}

fn structuring_rect(width: i32, height: i32) -> Mat {
    get_structuring_element(
            MORPH_RECT,
            Size { width, height },
            Point::new(-1, -1))
        .unwrap()
}

fn transform_image(img: &mut Vector<u8>) {
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

    // TODO: invert image if dark background is present

    let _ = copy_make_border(
        &img.clone(), img, 12, 12, 12, 12, BORDER_CONSTANT, 255.into());
}

fn split_region(path: &str, idx: u32) -> (Image, Image) {
    let response = ureq::get(path).call().unwrap();

    let len: usize = response
        .header("Content-Length")
        .map(|s| s.parse().unwrap())
        .unwrap_or(MAX_IMAGE_BYTES);

    let mut bytes = Vec::with_capacity(len);
    let _ = response
        .into_reader()
        .take(MAX_IMAGE_BYTES as u64)
        .read_to_end(&mut bytes);

    let mut img = image::load_from_memory(&bytes).unwrap();

    let mut name_img: Vector<_> = image::imageops::crop(
            &mut img,
            0, IMAGE_ELEMENT_OFFSET + idx * IMAGE_ELEMENT_HEIGHT,
            342, IMAGE_ELEMENT_HEIGHT)
        .to_image()
        .into_raw()
        .into();

    let mut rest_img: Vector<_> = image::imageops::crop(
            &mut img,
            342, IMAGE_ELEMENT_OFFSET + idx * IMAGE_ELEMENT_HEIGHT,
            427, IMAGE_ELEMENT_HEIGHT)
        .to_image()
        .into_raw()
        .into();

    transform_image(&mut name_img);
    transform_image(&mut rest_img);

    (vector_to_image(name_img), vector_to_image(rest_img))
}

pub fn retrieve(path: &str) -> Vec<Train> {
    for i in 0..7 {
        let (name_img, rest_img) = split_region(path, i);
        let tess_args = rusty_tesseract::Args {
            config_variables: std::collections::HashMap::new(),
            lang: "eng".into(),
            dpi: Some(300),
            psm: Some(11),
            oem: None
        };

        let name = rusty_tesseract::image_to_string(&name_img, &tess_args)
            .unwrap().trim();
        let rest = rusty_tesseract::image_to_string(&rest_img, &tess_args)
            .unwrap().trim();
    }

    vec![]
}
