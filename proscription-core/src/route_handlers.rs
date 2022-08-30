#![deny(warnings)]
use futures_util::{FutureExt, StreamExt};
use serde_derive::{Deserialize, Serialize};
use warp::Reply;

#[derive(Deserialize, Serialize)]
pub struct Room {
    name: String,
    room_code: String,
}

pub fn stream(ws: warp::ws::Ws) -> impl Reply {
    ws.on_upgrade(|websocket| {
        eprintln!("new websocket connection");
        // Just echo all messages back...
        let (tx, rx) = websocket.split();
        rx.forward(tx).map(|result| {
            if let Err(e) = result {
                eprintln!("websocket error: {:?}", e);
            }
        })
    })
}

pub fn join(room: Room) -> warp::reply::Json {
    eprintln!("POST | Join");
    let room_str = serde_json::to_string(&room).unwrap();
    eprintln!("Room: {}", room_str);
    warp::reply::json(&room)
}

pub fn create() -> impl Reply {
    eprintln!("GET | Create");
    "Create the world!"
}
