#[macro_use] extern crate rocket;

use rocket::serde::json::Json;

mod train_info;

const SFM_IMAGE_BASE_URL: &str = "https://info.trensfm.com/sapi/ivi_imagen";

#[get("/<code>")]
fn info(code: u8) -> Json<Vec<train_info::Train>> {
    let url = format!("{}?ubicacion={}", SFM_IMAGE_BASE_URL, code);

    Json(train_info::retrieve(&url))
}

#[launch]
async fn server() -> _ {
    rocket::build().mount("/", routes![info])
}
