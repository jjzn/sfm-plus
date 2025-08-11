use crate::types::*;

use rust_socketio::{client::Client, ClientBuilder, Event, Payload};
use std::collections::HashMap;
use std::sync::{LazyLock, Mutex};
use rocket::tokio::sync::mpsc;
use rocket::serde::{Deserialize, json::serde_json::{json, self}};
use chrono::TimeZone;
use chrono_tz::Europe::Madrid; // SFM uses their local time zone

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
struct SFMLinesStations {
    #[serde(rename = "linea")]
    lines: Vec<SFMLine>,

    #[serde(rename = "ubicacion")]
    stations: Vec<SFMStation>
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
struct SFMLine {
    #[serde(rename = "cod_linea")]
    code: i64,

    #[serde(rename = "simbolo")]
    name: String
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
struct SFMStation {
    #[serde(rename = "cod_ubicacion")]
    code: i64,

    #[serde(rename = "descripcion")]
    name: String
}

static SOCKETS: LazyLock<Mutex<HashMap<u8, Client>>> = LazyLock::new(|| Mutex::new(HashMap::new()));
static TRIPS: LazyLock<Mutex<HashMap<u8, Vec<Trip>>>> = LazyLock::new(|| Mutex::new(HashMap::new()));

static LINES_STATIONS: LazyLock<SFMLinesStations> = LazyLock::new(|| {
    let raw = include_str!("../sfm_lines_stations.json");
    serde_json::from_str(raw).unwrap()
});

fn panel_callback(code: u8, payload: Payload) {
    match payload {
        Payload::Text(values) => match values[0]["info"].as_array() {
            Some(array) => process_socket_response(code, array),
            None => panic!("Expected array")
        },
        _ => panic!("Expected Payload::Text")
    }
}

// TODO process the remarks field
fn process_socket_response(code: u8, trains: &Vec<serde_json::Value>) {
    let mut trips = vec![];

    for train in trains {
        let destination_code = train["cod_destino"].as_i64().unwrap();
        let headsign = LINES_STATIONS.stations
            .iter()
            .find(|station| station.code == destination_code)
            .unwrap()
            .name
            .to_string();

        let millis = train["hora"].as_i64().unwrap();
        let time = Madrid.timestamp_millis_opt(millis).unwrap().time().into();

        let track = train["via"].as_i64().map(|x| x as u8);

        let line_code = train["linea"].as_i64();
        let line = match line_code {
            Some(code) => LINES_STATIONS.lines
                .iter()
                .find(|line| line.code == code)
                .map(|line| line.name.to_string()),
            None => None
        };

        trips.push(Trip { headsign, time, track, line });
    }

    let mut map = TRIPS.lock().unwrap();
    map.insert(code, trips);
}

pub fn listen_socket(code: u8, tx: mpsc::Sender<()>) -> Client {
    ClientBuilder::new("https://info.trensfm.com/")
        .on("panel", move |payload, _| {
            panel_callback(code, payload);
            let _ = tx.clone().try_send(());
        })
        .on(Event::Connect, move |_, sock| {
            let data = vec![json!("panel"), json!({"estacion": code, "clase": "LCD"}), json!(null)];
            sock.emit("tipo", data).unwrap();
        })
        .connect()
        .unwrap()
}

pub async fn retrieve(code: u8) -> Vec<Trip> {
    let socket_exists = {
        let sockets_map = SOCKETS.lock().unwrap();
        sockets_map.contains_key(&code)
    };

    if !socket_exists {
        let (tx, mut rx) = mpsc::channel(1); // TODO

        std::thread::spawn(move || {
            let socket = listen_socket(code, tx);
            SOCKETS.lock().unwrap().insert(code, socket);
        });

        rx.recv().await; // Wait until we have received data at least once
    }

    TRIPS.lock().unwrap().get(&code).unwrap().to_vec()
}
