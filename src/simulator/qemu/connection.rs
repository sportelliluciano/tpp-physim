use std::future::Future;

use tokio::io::{AsyncReadExt, AsyncWriteExt, BufReader};

use crate::simulator::protocol::{Channel, Command, CommandId, RawHeader};

pub struct QemuReader<R: AsyncReadExt> {
    connection: BufReader<R>,
}

impl<R: AsyncReadExt + Unpin> QemuReader<R> {
    pub fn new(connection: R) -> Self {
        Self {
            connection: BufReader::new(connection),
        }
    }
}

impl<R: AsyncReadExt + Unpin + Send> CommandReader for QemuReader<R> {
    async fn read_command(&mut self) -> Option<Command> {
        let mut header_buffer = [0; 8];
        self.connection.read_exact(&mut header_buffer).await.ok()?;

        let RawHeader {
            command,
            channel,
            payload_size,
        } = RawHeader::from(header_buffer);

        let payload = if payload_size > 0 {
            let mut data_buffer = vec![0; payload_size as usize];
            self.connection.read_exact(&mut data_buffer).await.ok()?;

            data_buffer
        } else {
            Vec::new()
        };

        Some(Command {
            command_id: CommandId::from(command),
            channel: Channel::from(channel),
            payload,
        })
    }
}

pub struct QemuWriter<W: AsyncWriteExt> {
    connection: W,
}

impl<W: AsyncWriteExt + Unpin> QemuWriter<W> {
    pub fn new(connection: W) -> Self {
        Self { connection }
    }
}

impl<W: AsyncWriteExt + Unpin + Send> CommandWriter for QemuWriter<W> {
    async fn write_command(&mut self, cmd: &Command) {
        self.connection
            .write_all(&cmd.encode_header())
            .await
            .unwrap();
        self.connection.write_all(&cmd.payload).await.unwrap();
        self.connection.flush().await.unwrap()
    }
}

pub trait CommandReader {
    fn read_command(&mut self) -> impl Future<Output = Option<Command>> + Send;
}

pub trait CommandWriter {
    fn write_command(&mut self, cmd: &Command) -> impl Future<Output = ()> + Send;
}
