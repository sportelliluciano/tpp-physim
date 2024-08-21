use std::{process::Stdio, sync::Arc, time::Duration};

use tokio::{
    io::{AsyncBufReadExt as _, BufReader},
    net::TcpStream,
    sync::mpsc::Sender,
};

use crate::{
    logs::service::LogsService,
    simulator::{device::Device, mixer::Mixer, protocol::Command, qemu::connection::QemuWriter},
};

use super::connection::{CommandReader, CommandWriter, QemuReader};

pub struct QemuInstance {
    device: Device,
    logs: Arc<LogsService>,
}

impl QemuInstance {
    pub fn new(device: Device, logs: Arc<LogsService>) -> Self {
        Self { device, logs }
    }

    pub async fn run(mut self, mixer: Arc<Mixer>, flash_image_path: String) {
        let process = tokio::spawn(Self::qemu_process(
            self.device.get_id(),
            flash_image_path,
            self.logs.clone(),
        ));

        let (rx, tx) = self.connect_to_instance().await;

        let network = tokio::spawn(Self::network_task(rx, self.device.get_handle()));

        self.device.run(&mixer, tx).await;

        process.await.unwrap();
        network.await.unwrap();
        println!("[HANDLER {}] Finished.", self.device.get_id());
    }

    async fn connect_to_instance(&self) -> (impl CommandReader, impl CommandWriter) {
        for countdown in (0..5).rev() {
            println!(
                "[HANDLER {}] Starting instance in {countdown} secs...",
                self.device.get_id()
            );
            tokio::time::sleep(Duration::from_secs(1)).await;
        }
        println!("[HANDLER {}] Starting...", self.device.get_id());

        // QEMU will hold the instance until we connect to it.
        let conn = TcpStream::connect(format!("127.0.0.1:{}", 9000 + self.device.get_id()))
            .await
            .unwrap();

        let (rx, tx) = conn.into_split();
        (QemuReader::new(rx), QemuWriter::new(tx))
    }

    async fn qemu_process(id: u32, flash_image_path: String, logs: Arc<LogsService>) {
        let mut child = tokio::process::Command::new("qemu-system-xtensa")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .args([
                "-nographic",
                "-machine",
                "esp32",
                "-drive",
                &format!("file={},if=mtd,format=raw", flash_image_path),
                "-serial",
                "stdio",
                "-serial",
                &format!("tcp::{},server", 9000 + id),
                "-monitor",
                "none",
            ])
            .spawn()
            .unwrap();

        println!("[QEMU {id}] launched");

        let mut output = BufReader::new(child.stdout.take().unwrap());
        let mut buffer = String::new();
        while let Ok(s) = output.read_line(&mut buffer).await {
            if s == 0 {
                break;
            }

            print!("[QEMU {id}] {}", buffer);
            logs.log(id, buffer).await;

            buffer = String::new();
        }

        let ret = child.wait().await.unwrap();
        println!("[QEMU {id}] exited with code {ret:?}");
    }

    async fn network_task(mut conn: impl CommandReader + Send, handle: Sender<Command>) {
        while let Some(cmd) = conn.read_command().await {
            if handle.send(cmd).await.is_err() {
                break;
            }
        }
    }
}
