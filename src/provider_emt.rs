use chrono::{prelude::Local, Duration};
use regex::Regex;
use rocket::serde::{Deserialize, Serialize};

use crate::types::*;

const API_BASE_URL: &str = "https://api.mobipalma.mobi/1.2/paradas/";
const API_TOKEN_URL: &str = "https://www.emtpalma.cat/ca/inici";

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
struct EmtApiItem {
    destino: String,
    seconds: u32,
    llegando: bool,

    #[serde(skip)]
    line: String,
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
            track: None,
            line: Some(val.line),
        }
    }
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
struct EmtApiRoute {
    line: String,
    vh_first: EmtApiItem,
    vh_second: Option<EmtApiItem>,
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
struct EmtApiResponse {
    estimaciones: Vec<EmtApiRoute>,
}

#[derive(Debug, Serialize)]
#[serde(crate = "rocket::serde")]
pub enum Error {
    RemoteError(u16),
    NetworkError,
    MissingToken,
    IOError,
}

impl std::error::Error for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::RemoteError(code) => write!(f, "Remote server error, status code {}", code),
            Self::NetworkError => write!(f, "Network error"),
            Self::MissingToken => write!(f, "No API token could be found"),
            Self::IOError => write!(f, "Generic I/O error"),
        }
    }
}

impl From<ureq::Error> for Error {
    fn from(err: ureq::Error) -> Self {
        match err {
            ureq::Error::Status(code, _) => Self::RemoteError(code),
            ureq::Error::Transport(_) => Self::NetworkError,
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(_: std::io::Error) -> Self {
        Self::IOError
    }
}

pub fn retrieve(code: u32) -> Result<Vec<Trip>, Error> {
    let page_text = ureq::get(API_TOKEN_URL).call()?.into_string()?;

    let token_cap = Regex::new(r#"token:"([^"]+)""#)
        .unwrap()
        .captures(&page_text)
        .ok_or(Error::MissingToken)?;

    let token = &token_cap[1];

    let mut res = vec![];

    let api_res: EmtApiResponse = {
        let path = format!("{}{}", API_BASE_URL, code);
        let response = ureq::get(&path)
            .set("Authorization", &format!("Bearer {}", token))
            .call()?;

        response.into_json()?
    };

    for mut route in api_res.estimaciones {
        route.vh_first.line = route.line.clone();
        res.push(route.vh_first.into());

        if let Some(mut item) = route.vh_second {
            item.line = route.line;
            res.push(item.into())
        }
    }

    res.sort_unstable_by_key(|x: &Trip| x.time);
    Ok(res)
}
