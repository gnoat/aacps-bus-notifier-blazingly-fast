use axum::{
    routing::{get},
    http::StatusCode,
    response::{IntoResponse, Html},
    Json, Router,
    extract::Query,
};
use std::net::SocketAddr;
use serde::Deserialize;
use api::route_users::route_users;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/hello", get(|| async { Html("<b>Hello bish!</b>") }))
        .route("/route_users", get(route_users));

    let addr = SocketAddr::from(([127, 0 , 0, 1], 3000));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
