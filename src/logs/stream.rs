use tokio::sync::mpsc::UnboundedReceiver;

pub struct LogStream {
    pub rx: UnboundedReceiver<String>,
}
