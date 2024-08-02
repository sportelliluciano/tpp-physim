use crate::simulator::protocol::Command;

pub struct LinkTransmit {
    pub link_id: u16,
    pub payload: Vec<u8>,
}

impl From<Command> for LinkTransmit {
    fn from(value: Command) -> Self {
        LinkTransmit {
            link_id: value.channel.other(),
            payload: value.payload,
        }
    }
}
