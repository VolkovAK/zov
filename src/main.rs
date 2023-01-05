use std::net::SocketAddr;

use axum::routing::get_service;
use axum::{
    routing::get,
    Router,
};
use tower_http::services::ServeDir;

mod templates;
mod routes;
mod api;


#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();
    tracing::info!("start!");

    // build our application with some routes
    let app = Router::new()
        .nest("/v1", api::routes_v1::create_v1_api_routes())
        .route("/greet/:name", get(routes::greet))
        .nest_service("/assets", get_service(ServeDir::new("assets")).handle_error(routes::service_handle_error));

    // run it
    let addr = SocketAddr::from(([127, 0, 0, 1], 3120));
    tracing::info!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();

}

