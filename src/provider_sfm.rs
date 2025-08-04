use crate::types::*;

use rust_socketio::{client::Client, ClientBuilder, Event, Payload};
use ureq::json;
use std::collections::HashMap;
use std::sync::{LazyLock, Mutex};
use rocket::tokio::{task, sync::mpsc};

static SOCKETS: LazyLock<Mutex<HashMap<u8, Client>>> = LazyLock::new(|| Mutex::new(HashMap::new()));
static TRIPS: LazyLock<Mutex<HashMap<u8, Vec<Trip>>>> = LazyLock::new(|| Mutex::new(HashMap::new()));

fn panel_callback(code: u8, payload: Payload) {
    match payload {
        Payload::Text(values) => match values[0]["info"].as_array() {
            Some(array) => process_socket_response(code, array),
            None => panic!("Expected array")
        },
        _ => panic!("Expected Payload::Text")
    }
}

fn process_socket_response(code: u8, trains: &Vec<rocket::serde::json::Value>) {
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

        task::spawn_blocking(move || {
            let socket = listen_socket(code, tx);
            SOCKETS.lock().unwrap().insert(code, socket);
        });

        rx.recv().await; // Wait until we have received data at least once
    }

    TRIPS.lock().unwrap().get(&code).unwrap().to_vec()
}
