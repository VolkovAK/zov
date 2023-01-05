use std::io;
use axum::{
    extract::Path,
    http::StatusCode,
    response::IntoResponse,
};

use crate::templates;


pub async fn greet(Path(name): Path<String>) -> impl IntoResponse {
    let template = templates::HelloTemplate { name };
    return templates::HtmlTemplate(template);
}


pub async fn service_handle_error(_err: io::Error) -> impl IntoResponse {
    (StatusCode::INTERNAL_SERVER_ERROR, "Error while loading static files...")
}
