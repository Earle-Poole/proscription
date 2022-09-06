#![deny(warnings)]
use std::sync::atomic::Ordering;

use futures_util::StreamExt;
use serde_derive::{Deserialize, Serialize};
use tokio::sync::mpsc;
use tokio_stream::wrappers::UnboundedReceiverStream;
use warp::{ws::WebSocket, Reply};

use futures_util::{SinkExt, TryFutureExt};

use crate::{Rooms, Users, NEXT_USER_ID};

#[derive(Deserialize, Serialize)]
pub struct JoinRoomRequest {
    name: String,
    room_code: String,
}

pub async fn stream(ws: WebSocket, users: Users) {
    // Use a counter to assign a new unique ID for this user.
    let my_id = NEXT_USER_ID.fetch_add(1, Ordering::Relaxed);

    eprintln!("new chat user: {}", my_id);

    // Split the socket into a sender and receive of messages.
    let (mut user_ws_tx, mut user_ws_rx) = ws.split();

    // Use an unbounded channel to handle buffering and flushing of messages
    // to the websocket...
    let (tx, rx) = mpsc::unbounded_channel();
    let mut rx = UnboundedReceiverStream::new(rx);

    tokio::task::spawn(async move {
        while let Some(message) = rx.next().await {
            user_ws_tx
                .send(message)
                .unwrap_or_else(|e| {
                    eprintln!("websocket send error: {}", e);
                })
                .await;
        }
    });

    // Save the sender in our list of connected users.
    users.write().await.insert(my_id, tx);

    while let Some(result) = user_ws_rx.next().await {
        let msg = match result {
            Ok(msg) => msg,
            Err(e) => {
                eprintln!("websocket error(uid={}): {}", my_id, e);
                break;
            }
        };

        println!(
            "broadcasting message from uid={}: {}",
            my_id,
            msg.to_str().unwrap()
        );
    }
}

pub fn join(join_room_req: JoinRoomRequest, rooms: Rooms) -> warp::reply::Json {
    eprintln!("POST | Join");
    let room_str = serde_json::to_string(&join_room_req).unwrap();
    eprintln!("Room: {}", room_str);

    check_room_exists(&join_room_req, rooms);

    warp::reply::json(&join_room_req)
}

pub fn create(join_room_req: JoinRoomRequest) -> impl Reply {
    eprintln!("POST | Create");
    let room_str = serde_json::to_string(&join_room_req).unwrap();
    eprintln!("Room: {}", room_str);
    warp::reply::json(&join_room_req)
}

fn check_room_exists(join_room_req: &JoinRoomRequest, rooms: Rooms) -> bool {
    rooms.blocking_read().contains_key(&join_room_req.room_code)
}
