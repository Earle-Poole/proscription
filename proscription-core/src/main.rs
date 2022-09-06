#![deny(warnings)]
use route_handlers::JoinRoomRequest;
use std::{
    collections::HashMap,
    sync::{atomic::AtomicUsize, Arc},
};
use tokio::sync::{mpsc, RwLock};
use warp::{ws::Message, Filter};

mod constants;
mod route_handlers;

use constants::PATHS;

static NEXT_USER_ID: AtomicUsize = AtomicUsize::new(1);

/// Our state of currently connected users.
///
/// - Key is their id
/// - Value is a sender of `warp::ws::Message`
type Users = Arc<RwLock<HashMap<usize, mpsc::UnboundedSender<Message>>>>;
type Rooms = Arc<RwLock<HashMap<String, Vec<usize>>>>;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    // Keep track of all connected users, key is usize, value
    // is a websocket sender.
    let users = Users::default();
    let rooms = Rooms::default();
    // Turn our "state" into a new Filter...
    let users = warp::any().map(move || users.clone());
    let rooms = warp::any().map(move || rooms.clone());

    let cors = warp::cors()
        .allow_any_origin()
        .allow_methods(vec!["GET", "POST", "PUT", "DELETE"])
        .allow_headers(vec!["Content-Type", "Authorization"]);

    let stream_route =
        warp::path(PATHS.stream)
            .and(warp::ws())
            .and(users)
            .map(|ws: warp::ws::Ws, users| {
                // This will call our function if the handshake succeeds.
                ws.on_upgrade(move |socket| route_handlers::stream(socket, users))
            });

    let join_route = warp::post()
        .and(warp::path(PATHS.join))
        .and(warp::body::json())
        .and(rooms)
        .map(|join_room_req: JoinRoomRequest, rooms: Rooms| {
            route_handlers::join(join_room_req, rooms)
        })
        .with(&cors);

    let create_route = warp::post()
        .and(warp::path(PATHS.create))
        .and(warp::body::json())
        .map(route_handlers::create)
        .with(cors);

    let routes = stream_route.or(join_route).or(create_route);

    warp::serve(routes).run(([127, 42, 0, 69], 3030)).await;
}
