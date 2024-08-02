#[derive(Clone, Copy, Debug)]
pub enum Channel {
    Control,
    Other(u16),
}

impl Channel {
    pub fn other(&self) -> u16 {
        match self {
            Channel::Other(e) => *e,
            other => panic!("Invalid channel value `{other:?}`"),
        }
    }
}

impl From<Channel> for u16 {
    fn from(value: Channel) -> Self {
        match value {
            Channel::Control => 0xFFFF,
            Channel::Other(v) => v,
        }
    }
}

impl From<u16> for Channel {
    fn from(value: u16) -> Self {
        if value == 0xFFFF {
            Channel::Control
        } else {
            Channel::Other(value)
        }
    }
}
