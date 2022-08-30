#![deny(warnings)]
use warp::Filter;

mod constants;
mod route_handlers;

use constants::PATHS;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    let cors = warp::cors()
        .allow_any_origin()
        .allow_methods(vec!["GET", "POST", "PUT", "DELETE"])
        .allow_headers(vec!["Content-Type", "Authorization"]);

    // Websocket entry point at /stream
    let stream_route = warp::path(PATHS.stream)
        // The `ws()` filter will prepare the Websocket handshake.
        .and(warp::ws())
        // The handler for when the handshake succeeds.
        .map(route_handlers::stream);

    let join_route = warp::post()
        .and(warp::path(PATHS.join))
        .and(warp::body::json())
        .map(route_handlers::join)
        .with(cors);
    let create_route = warp::path(PATHS.create)
        .map(route_handlers::create)
        .with(warp::cors().allow_any_origin());

    let routes = stream_route.or(join_route).or(create_route);

    warp::serve(routes).run(([127, 42, 0, 69], 3030)).await;
}
