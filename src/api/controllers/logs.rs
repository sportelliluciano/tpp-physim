use axum::{
    extract::{
        ws::{Message, WebSocket},
        Path, State, WebSocketUpgrade,
    },
    http::StatusCode,
    response::{Html, IntoResponse},
};

use crate::{api::state::AppState, logs::stream::LogStream};

pub async fn index() -> Html<&'static str> {
    Html(include_str!("../assets/logs.html"))
}

pub async fn get_logs(
    ws: WebSocketUpgrade,
    State(state): State<AppState>,
    Path(device): Path<u32>,
) -> impl IntoResponse {
    if let Some(logs) = state.logs.lock().await.as_ref() {
        let stream = logs.log_stream(device).await;
        ws.on_upgrade(move |s| stream_logs(stream, s))
            .into_response()
    } else {
        (
            StatusCode::NOT_FOUND,
            "Simulation not running or invalid device ID",
        )
            .into_response()
    }
}

async fn stream_logs(mut stream: LogStream, mut s: WebSocket) {
    while let Some(log) = stream.rx.recv().await {
        if s.send(Message::Text(log)).await.is_err() {
            break;
        }
    }
}
