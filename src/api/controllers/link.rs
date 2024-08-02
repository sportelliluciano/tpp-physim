use axum::{extract::State, Json};

use crate::api::state::AppState;

pub async fn create(State(state): State<AppState>) -> Json<u16> {
    Json(
        state
            .simulation
            .lock()
            .await
            .as_mut()
            .unwrap()
            .create_link(),
    )
}
