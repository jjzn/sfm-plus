#[macro_use] extern crate rocket;

#[get("/<code>")]
fn info(code: u8) -> String {
    format!("Not implemented. Code {}", code)
}

#[launch]
async fn server() -> _ {
    rocket::build().mount("/", routes![info])
}
