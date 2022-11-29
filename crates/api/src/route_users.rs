use axum::{
    response::Html,
    extract::Query,
};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Route {
    bus: u32,
    school: String,
}

pub async fn route_users(Query(route): Query<Route>) -> Html<String> {
    let user_query = Html(format!("<b>SELECT * FROM users WHERE bus='{}' AND school='{}'</b>", route.bus, route.school));

    user_query
}

