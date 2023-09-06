use rocket::serde::Deserialize;
use chrono::{Duration, prelude::Local};

use crate::types::*;

const API_BASE_URL: &str = "https://api.mobipalma.mobi/1.2/paradas/";

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
struct EmtApiItem {
    destino: String,
    seconds: u32,
    llegando: bool
}

impl From<EmtApiItem> for Trip {
    fn from(val: EmtApiItem) -> Self {
        // TODO: account for time zones!
        let time = {
            // assume "arriving" means 0 seconds left
            let secs = if val.llegando { 0 } else { val.seconds };
            Local::now().time() + Duration::seconds(secs as i64)
        };

        Self {
            headsign: val.destino,
            time: time.into(),
            track: 0
        }
    }
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
struct EmtApiRoute {
    vh_first: EmtApiItem,
    vh_second: EmtApiItem
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
struct EmtApiResponse {
    estimaciones: Vec<EmtApiRoute>
}

pub fn retrieve(code: u32) -> Vec<Trip> {
    let mut res = vec![];

    let api_res: EmtApiResponse = {
        let path = format!("{}{}", API_BASE_URL, code);
        let response = ureq::get(&path).call().unwrap();

        response.into_json().unwrap()
    };

    for route in api_res.estimaciones {
        res.push(route.vh_first.into());
        res.push(route.vh_second.into());
    }

    res
}
