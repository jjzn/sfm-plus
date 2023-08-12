use rusty_tesseract::Image;
use image::GenericImageView;
use opencv::imgproc::*;
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

fn split_region(path: &str, idx: u32) -> (Image, Image) {
    let response = ureq::get(path).call().unwrap();

    let len: usize = response
        .header("Content-Length")
        .map(|s| s.parse().unwrap())
        .unwrap_or(MAX_IMAGE_BYTES);

    let mut bytes = Vec::with_capacity(len);
    response
        .into_reader()
        .take(MAX_IMAGE_BYTES as u64)
        .read_to_end(&mut bytes);

    let mut img = image::load_from_memory(&bytes).unwrap();

    let name_img = image::imageops::crop(
            &mut img,
            0, IMAGE_ELEMENT_OFFSET + idx * IMAGE_ELEMENT_HEIGHT,
            342, IMAGE_ELEMENT_HEIGHT)
        .to_image()
        .as_raw();

    name_img
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
