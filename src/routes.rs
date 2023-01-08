use std::io;
use std::sync::{Arc, Mutex};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};

use crate::templates;
use crate::api;
use crate::AppState;


pub async fn greet(Path(name): Path<String>) -> impl IntoResponse {
    let template = templates::HelloTemplate { name };
    templates::HtmlTemplate(template)
}

pub async fn main_handler(Path(value_path): Path<String>, State(state): State<Arc<Mutex<AppState>>>) -> impl IntoResponse {
    let template = templates::HelloTemplate { name: state.lock().unwrap().storage.root_path.clone() };

    state.lock().unwrap().storage.set_root_path(&value_path);
    // state.storage.root_path = value_path;
    templates::HtmlTemplate(template)
}

pub async fn service_handle_error(_err: io::Error) -> impl IntoResponse {
    (StatusCode::INTERNAL_SERVER_ERROR, "Error while loading static files...")
}
