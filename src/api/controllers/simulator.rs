use std::{io::Write as _, sync::Arc};

use axum::{
    extract::{Multipart, State},
    Json,
};

use crate::{api::state::AppState, logs::service::LogsService};

pub async fn load_flash_image(State(state): State<AppState>, mut multipart: Multipart) -> Json<()> {
    if let Some(file) = multipart.next_field().await.unwrap() {
        let path = dump_qemu_image(&file.bytes().await.unwrap());
        state
            .simulation
            .lock()
            .await
            .as_mut()
            .unwrap()
            .set_flash_image_path(path);
    } else {
        println!("simulator load flash image -- multipart empty");
    }

    Json(())
}

pub async fn launch(State(state): State<AppState>) -> Json<()> {
    println!("simulator launch");
    let mut s = state.simulation.lock().await;
    let simulation = s.take();
    if let Some(simulation) = simulation {
        let logs = Arc::new(LogsService::new(simulation.device_count()));
        *state.logs.lock().await = Some(logs.clone());
        tokio::spawn(simulation.run(logs));
    }
    Json(())
}

fn dump_qemu_image(data: &[u8]) -> String {
    let mut path = std::env::temp_dir();
    path.push("qemu-image.bin");
    let mut file = std::fs::File::create(&path).unwrap();
    file.write_all(data).unwrap();
    path.to_string_lossy().into()
}
