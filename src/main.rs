mod api;
mod logs;
mod simulator;

#[tokio::main]
async fn main() {
    api::run("0.0.0.0:13013").await;
}
