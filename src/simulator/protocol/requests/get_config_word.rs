use crate::simulator::protocol::{Command, CommandId};

pub struct GetConfigWord {
    pub word_id: u32,
}

impl From<Command> for GetConfigWord {
    fn from(value: Command) -> Self {
        if value.command_id != CommandId::GetConfigWord {
            panic!("Invalid command");
        }

        GetConfigWord {
            word_id: u32::from_le_bytes(value.payload.try_into().unwrap()),
        }
    }
}
