use std::sync::{Arc, Mutex};
use axum::{
    Router,
    extract::{Path, State, Query},
    routing::get,
    http::StatusCode,
    response::{IntoResponse, Json},
};
use serde::{Deserialize, Serialize};

use crate::{storage::{self, DirectoryRepresentation}, AppState};

pub fn create_api_routes(state: Arc<Mutex<AppState>>) -> Router {
    Router::new()
        .route("/get_all", get(get_all_configs))
        .route("/get_directory", get(get_directory).with_state(state.clone()))
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

#[derive(Debug, Deserialize)]
pub struct GetDirectoryParams {
    pub path: Option<String>,
    pub descending_date: Option<bool>,
}

pub async fn get_directory(Query(params): Query<GetDirectoryParams>, State(state): State<Arc<Mutex<AppState>>>) -> impl IntoResponse {
    let res = state.lock().unwrap().storage.get_directory(&params).unwrap();
    (StatusCode::OK, Json(res))
}
