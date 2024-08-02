use axum::routing::get;
use axum::{extract::DefaultBodyLimit, routing::post, Router};

mod controllers;
mod state;

use self::controllers::*;
use self::state::AppState;

pub async fn run(address: &str) {
    let app = Router::new()
        .route(
            "/upload-flash-image",
            post(simulator::load_flash_image).layer(DefaultBodyLimit::max(10 * 1024 * 1024)),
        )
        .route("/link/new", post(link::create))
        .route("/qemu/new", post(device::create))
        .route("/qemu/:device/config", post(device::set_config_word))
        .route("/qemu/:device/connect", post(device::connect_link_output))
        .route("/logs", get(logs::index))
        .route("/logs/:device", get(logs::get_logs))
        .route("/launch", post(simulator::launch))
        .with_state(AppState::default());

    let listener = tokio::net::TcpListener::bind(address).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
