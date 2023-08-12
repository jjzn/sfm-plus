#[macro_use] extern crate rocket;

mod train_info;

const SFM_IMAGE_BASE_URL: &str = "https://info.trensfm.com/sapi/ivi_imagen";

#[get("/<code>")]
fn info(code: u8) -> String {
    let url = format!("{}?ubicacion={}", SFM_IMAGE_BASE_URL, code);
    let trains = train_info::retrieve(&url);

    format!("Found {} trains at station {}", trains.len(), code)
}

#[launch]
async fn server() -> _ {
    rocket::build().mount("/", routes![info])
}
