#[macro_use] extern crate rocket;

mod provider_sfm;
mod provider_emt;
mod types;

use rocket::serde::json::Json;
use rocket::fairing::{Fairing, Info, Kind};

use crate::types::*;

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

#[get("/sfm/<code>")]
fn info_sfm(code: u8) -> Json<Vec<Trip>> {
    Json(provider_sfm::retrieve(code))
}

#[get("/emt/<code>")]
fn info_emt(code: u32) -> Json<Result<Vec<Trip>, provider_emt::Error>> {
    Json(provider_emt::retrieve(code))
}

#[launch]
async fn server() -> _ {
    rocket::build()
        .attach(Cors {})
        .mount("/", routes![info_sfm, info_emt])
}
