use std::sync::Arc;

use tokio::sync::Mutex;

use crate::{logs::service::LogsService, simulator::Simulation};

#[derive(Clone)]
pub struct AppState {
    pub simulation: Arc<Mutex<Option<Simulation>>>,
    pub logs: Arc<Mutex<Option<Arc<LogsService>>>>,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            simulation: Arc::new(Mutex::new(Some(Simulation::default()))),
            logs: Arc::new(Mutex::new(None)),
        }
    }
}
