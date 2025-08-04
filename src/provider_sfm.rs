use crate::types::*;

use rust_socketio::{ClientBuilder, Event, Payload, RawClient};
use ureq::json;
use std::sync::Mutex;
use std::collections::HashMap;
use std::sync::LazyLock;

static TRIPS: LazyLock<Mutex<HashMap<u8, Vec<Trip>>>> = LazyLock::new(|| Mutex::new(HashMap::new()));

fn panel_callback(payload: Payload, _: RawClient) {
    match payload {
        Payload::Text(values) => match values[0]["info"].as_array() {
            Some(array) => process(1, array),
            None => panic!("Expected array")
        },
        _ => panic!("Expected Payload::Text")
    }
}

fn process(code: u8, trains: &Vec<rocket::serde::json::Value>) {
    let mut trips = vec![];

    for train in trains {
        let headsign = train["cod_destino"].as_i64().unwrap().to_string();

        let millis = train["hora"].as_i64().unwrap();
        let time = chrono::DateTime::from_timestamp_millis(millis).unwrap().time().into();

        let track = train["via"].as_i64().map(|x| x as u8);
        let line = train["linea"].as_i64().map(|x| x.to_string());

        trips.push(Trip { headsign, time, track, line });
    }

    let mut map = TRIPS.lock().unwrap();
    map.insert(code, trips);
}

pub fn listen_socket() {
    let socket = ClientBuilder::new("https://info.trensfm.com/")
        .transport_type(rust_socketio::TransportType::Polling)
        .on("panel", panel_callback)
        .on(Event::Connect, |_, sock| {
            let data = vec![json!("panel"), json!({"estacion": 1, "clase": "LCD"}), json!(null)];
            sock.emit("tipo", data).unwrap();
        })
        .connect()
        .unwrap();
}

pub fn retrieve(code: u8) -> Vec<Trip> {
    TRIPS.lock().unwrap().get(&1).unwrap().to_vec()
}
