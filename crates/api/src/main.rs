use axum::{
    routing::{get},
    http::StatusCode,
    response::{IntoResponse, Html},
    Json, Router,
    extract::Query,
};
use std::net::SocketAddr;
use serde::Deserialize;

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

#[derive(Deserialize)]
struct Route {
    bus: u32,
    school: String,
}

async fn route_users(Query(route): Query<Route>) -> String {
    let user_query = format!("SELECT * FROM users WHERE bus='{}' AND school={}", route.bus, route.school);

    user_query
}

