use std::net::SocketAddr;
use std::sync::{Arc, Mutex};

use axum::routing::get_service;
use axum::{
    routing::get,
    Router,
};
use config::Config;
use serde::Deserialize;
use tower_http::services::ServeDir;

mod api;
mod routes;
mod storage;
mod templates;


#[derive(Default, Deserialize)]
struct AppConfig {
    storage: String,
}



#[derive(Clone, Default)]
pub struct AppState {
    pub storage: storage::ZovStorage,
    pub some_status: String,
}


#[tokio::main]
async fn main() {
    let settings = Config::builder()
        // Add in settings from the environment (with a prefix of APP)
        // Eg.. `APP_DEBUG=1 ./target/app` would set the `debug` key
        .add_source(config::Environment::with_prefix("ZOV"))
        .build()
        .unwrap();
    let settings: AppConfig = settings.try_deserialize().unwrap();
    

    // initialize tracing
    let subscriber = tracing_subscriber::FmtSubscriber::builder()
        .with_max_level(tracing::Level::TRACE)
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("Tracing setting failed");
    tracing::info!("start!");

    // TODO FIXME: add correct error handling for paths
    let mut zov = storage::ZovStorage::new(settings.storage);
    zov.rebuild_storage_tree();
    for k in zov.root_dir.dirs.keys() {
        tracing::info!("dirs: {}", k);
    }
    for n in zov.root_dir.nodes.keys() {
        tracing::info!("node: {}", n);
    }
    let shared_state = Arc::new(Mutex::new(AppState { storage: zov, some_status: "a".to_string() }));

    // build our application with some routes
    let app = Router::new()
        .nest("/api", api::create_api_routes(shared_state.clone()))
        .route("/zov/*value_path", get(routes::main_handler).with_state(shared_state.clone()))
        .route("/greet/:name", get(routes::greet))
        .nest_service("/assets", get_service(ServeDir::new("assets")).handle_error(routes::service_handle_error));

    // run it
    let addr = SocketAddr::from(([127, 0, 0, 1], 3120));
    tracing::info!("listening on http://{}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();

}

