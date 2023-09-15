use rocket::serde::Deserialize;
use chrono::{Duration, prelude::Local};
use regex::Regex;

use crate::types::*;

const API_BASE_URL: &str = "https://api.mobipalma.mobi/1.2/paradas/";
const API_TOKEN_URL: &str = "https://www.emtpalma.cat/ca/inici";

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
    vh_second: Option<EmtApiItem>
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
struct EmtApiResponse {
    estimaciones: Vec<EmtApiRoute>
}

pub fn retrieve(code: u32) -> Vec<Trip> {
    let page_text = ureq::get(API_TOKEN_URL)
        .call().unwrap()
        .into_string().unwrap();

    let token_cap = Regex::new(r#"token:"([^"]+)""#)
        .unwrap()
        .captures(&page_text)
        .unwrap();

    let token = &token_cap[1];

    let mut res = vec![];

    let api_res: EmtApiResponse = {
        let path = format!("{}{}", API_BASE_URL, code);
        let response = ureq::get(&path)
            .set("Authorization", &format!("Bearer {}", token))
            .call().unwrap();

        response.into_json().unwrap()
    };

    for route in api_res.estimaciones {
        res.push(route.vh_first.into());

        if let Some(item) = route.vh_second {
            res.push(item.into())
        }
    }

    res
}
