#[macro_use] extern crate rocket;

use rocket::serde::json::Json;
use rocket::fairing::{Fairing, Info, Kind};

mod provider_sfm;

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
fn info_sfm(code: u8) -> Json<Vec<provider_sfm::Train>> {
    Json(provider_sfm::retrieve(code))
}

#[launch]
async fn server() -> _ {
    rocket::build()
        .attach(Cors {})
        .mount("/", routes![info_sfm])
}
