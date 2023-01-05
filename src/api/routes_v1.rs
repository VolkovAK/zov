use axum::{
    Router,
    routing::get,
    http::StatusCode,
    response::{IntoResponse, Json},
};
use serde::Serialize;

pub fn create_v1_api_routes() -> Router {
    Router::new()
        .route("/get_all", get(get_all_configs))
}

#[derive(Serialize)]
struct Configs {
    cfgs: Vec<String>,
}


pub async fn get_all_configs() -> impl IntoResponse {
    let configs = Configs {
        cfgs: vec!["kek".to_string(), "lol".to_string()],
    };

    (StatusCode::OK, Json(configs))
}
