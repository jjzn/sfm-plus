use rusty_tesseract::Image;
use opencv::imgproc::*;
use opencv::core::{Mat, Point, Size, VecN, BORDER_CONSTANT, copy_make_border};
use opencv::prelude::{MatTraitConst, MatTraitConstManual};
use std::io::Read;
use std::convert::TryInto;

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

    // TODO: invert image if dark background is present

    let _ = copy_make_border(
        &img.clone(), img, 12, 12, 12, 12, BORDER_CONSTANT, 255.into());
}

fn crop_image(img: &mut image::DynamicImage, x: u32, y: u32, w: u32, h: u32) -> Mat {
    let imbuf = image::imageops::crop(img, x, y, w, h).to_image();
    let (cols, rows) = imbuf.dimensions();

    let pixels: Vec<VecN<u8, 4>> = imbuf
        .into_raw()
        .chunks(4)
        .map(|x| x.try_into().unwrap())
        .map(|x: [u8; 4]| x.into())
        .collect();

    Mat::from_slice_rows_cols(
            &pixels,
            rows as usize,
            cols as usize)
        .unwrap()
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

    let mut name_img = crop_image(
        &mut img,
        0, IMAGE_ELEMENT_OFFSET + idx * IMAGE_ELEMENT_HEIGHT,
        427, IMAGE_ELEMENT_HEIGHT);

    let mut rest_img = crop_image(
        &mut img,
        342, IMAGE_ELEMENT_OFFSET + idx * IMAGE_ELEMENT_HEIGHT,
        427, IMAGE_ELEMENT_HEIGHT);

    transform_image(&mut name_img);
    transform_image(&mut rest_img);

    (mat_to_image(name_img), mat_to_image(rest_img))
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
