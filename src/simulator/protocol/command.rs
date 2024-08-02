use super::{Channel, CommandId, RawHeader};

pub struct Command {
    pub command_id: CommandId,
    pub channel: Channel,
    pub payload: Vec<u8>,
}

impl Command {
    pub fn encode_header(&self) -> [u8; 8] {
        RawHeader {
            command: self.command_id.into(),
            channel: self.channel.into(),
            payload_size: self.payload.len() as u32,
        }
        .to_bytes()
    }
}
