#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CommandId {
    GetConfigWord,
    LinkSend,
    LinkRecv,
    Quit,
    Unknown(u16),
}

impl From<CommandId> for u16 {
    fn from(value: CommandId) -> Self {
        match value {
            CommandId::GetConfigWord => 0xaa00,
            CommandId::LinkSend => 0xbb00,
            CommandId::LinkRecv => 0xbb01,
            CommandId::Quit => 0xFFFF,
            CommandId::Unknown(other) => other,
        }
    }
}

impl From<u16> for CommandId {
    fn from(value: u16) -> Self {
        match value {
            0xaa00 => CommandId::GetConfigWord,
            0xbb00 => CommandId::LinkSend,
            0xbb01 => CommandId::LinkRecv,
            other => CommandId::Unknown(other),
        }
    }
}
