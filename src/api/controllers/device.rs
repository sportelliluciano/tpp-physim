use axum::{
    extract::{Path, State},
    Json,
};
use serde::Deserialize;

use crate::api::state::AppState;

pub async fn create(State(state): State<AppState>) -> Json<u32> {
    Json(
        state
            .simulation
            .lock()
            .await
            .as_mut()
            .unwrap()
            .create_device(),
    )
}

#[derive(Deserialize)]
pub struct SetConfigWordRequest {
    word_id: u32,
    value: u32,
}

pub async fn set_config_word(
    State(state): State<AppState>,
    Path(device): Path<u32>,
    Json(req): Json<SetConfigWordRequest>,
) -> Json<()> {
    state
        .simulation
        .lock()
        .await
        .as_mut()
        .unwrap()
        .set_device_config_word(device, req.word_id, req.value);

    Json(())
}

#[derive(Deserialize)]
pub struct ConnectLinkOutputRequest {
    link_id: u16,
}
pub async fn connect_link_output(
    State(state): State<AppState>,
    Path(device): Path<u32>,
    Json(req): Json<ConnectLinkOutputRequest>,
) -> Json<()> {
    state
        .simulation
        .lock()
        .await
        .as_mut()
        .unwrap()
        .connect_link_output(req.link_id, device);

    Json(())
}
