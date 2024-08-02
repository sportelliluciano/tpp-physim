use tokio::sync::{
    mpsc::{self, UnboundedSender},
    Mutex,
};

use super::stream::LogStream;

const RETAIN_LOGS_THRESHOLD: usize = 4000;
const MAX_LOGS_PER_DEVICE: usize = 5000;

pub struct LogsService {
    #[allow(clippy::type_complexity)]
    logs: Vec<Mutex<(Vec<String>, Option<UnboundedSender<String>>)>>,
}

impl LogsService {
    pub fn new(n_devices: usize) -> Self {
        Self {
            logs: (0..n_devices)
                .map(|_| Mutex::new((Vec::new(), None)))
                .collect(),
        }
    }

    pub async fn log(&self, device_id: u32, message: String) {
        let mut logs = self.logs[device_id as usize].lock().await;
        if logs.0.len() > MAX_LOGS_PER_DEVICE {
            logs.0.drain(0..RETAIN_LOGS_THRESHOLD);
        }
        logs.0.push(message.clone());

        let stream = &mut logs.1;
        let still_valid = if let Some(s) = stream.as_ref() {
            s.send(message).is_ok()
        } else {
            true // irrelevant
        };

        if !still_valid {
            stream.take();
        }
    }

    pub async fn log_stream(&self, device_id: u32) -> LogStream {
        let mut logs = self.logs[device_id as usize].lock().await;

        let (tx, rx) = mpsc::unbounded_channel();
        for log in logs.0.iter() {
            tx.send(log.clone()).unwrap();
        }
        logs.1 = Some(tx);
        LogStream { rx }
    }
}
