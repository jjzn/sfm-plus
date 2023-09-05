#[macro_use] extern crate rocket;

use rocket::serde::json::Json;
use rocket::fairing::{Fairing, Info, Kind};

mod train_info;

const SFM_IMAGE_BASE_URL: &str = "https://info.trensfm.com/sapi/ivi_imagen";

struct Cors();

#[rocket::async_trait]
impl Fairing for Cors {
    fn info(&self) -> Info {
        Info {
            name: "CORS headers",
            kind: Kind::Response
        }
    }

    async fn on_response<'r>(&self, _: &'r rocket::Request<'_>, res: &mut rocket::Response<'r>) {
        res.adjoin_raw_header("Access-Control-Allow-Origin", "*");
    }
}

#[get("/<code>")]
fn info(code: u8) -> Json<Vec<train_info::Train>> {
    let url = format!("{}?ubicacion={}", SFM_IMAGE_BASE_URL, code);

    Json(train_info::retrieve(&url))
}

#[launch]
async fn server() -> _ {
    rocket::build()
        .attach(Cors {})
        .mount("/", routes![info])
}
