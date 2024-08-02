use std::collections::HashMap;

use tokio::sync::mpsc::{self, Receiver, Sender};

use super::{
    mixer::Mixer,
    protocol::{
        requests::{GetConfigWord, LinkTransmit},
        Channel, Command, CommandId, ConfigVariable, PHYSIM_MAGIC,
    },
    qemu::connection::CommandWriter,
};

pub struct Device {
    id: u32,
    variables: HashMap<ConfigVariable, u32>,
    handle: Sender<Command>,
    sink: Receiver<Command>,
}

impl Device {
    pub fn new(device_id: u32) -> Self {
        let (handle, sink) = mpsc::channel(1);

        Self {
            id: device_id,
            variables: HashMap::from([
                (ConfigVariable::Magic, PHYSIM_MAGIC),
                (ConfigVariable::DeviceId, device_id),
            ]),
            handle,
            sink,
        }
    }

    pub fn get_id(&self) -> u32 {
        self.id
    }

    pub fn get_config_word(&self, variable: ConfigVariable) -> Option<u32> {
        self.variables.get(&variable).copied()
    }

    pub fn set_config_word(&mut self, variable: ConfigVariable, value: u32) {
        self.variables.insert(variable, value);
    }

    pub fn get_handle(&self) -> Sender<Command> {
        self.handle.clone()
    }

    pub async fn run(&mut self, mixer: &Mixer, mut conn: impl CommandWriter) {
        while let Some(cmd) = self.sink.recv().await {
            match cmd.command_id {
                CommandId::GetConfigWord => {
                    self.do_get_config_word(GetConfigWord::from(cmd), &mut conn)
                        .await
                }
                CommandId::LinkSend => mixer.link_send(LinkTransmit::from(cmd)).await,
                CommandId::LinkRecv => self.do_link_recv(LinkTransmit::from(cmd), &mut conn).await,
                CommandId::Quit => break,
                CommandId::Unknown(_) => todo!(),
            }
        }
    }

    async fn do_get_config_word(&self, req: GetConfigWord, conn: &mut impl CommandWriter) {
        let GetConfigWord { word_id } = req;
        let value = if let Some(value) = self.get_config_word(ConfigVariable::from(word_id)) {
            value
        } else {
            println!("[HANDLER {}] GET_CONFIG_WORD(word_id=0x{word_id:08X}) -> not found (reads 0xFFFFFFFF)", self.id);
            0xFFFF_FFFF
        };

        conn.write_command(&Command {
            command_id: CommandId::GetConfigWord,
            channel: Channel::Control,
            payload: value.to_le_bytes().into(),
        })
        .await;
    }

    async fn do_link_recv(&self, req: LinkTransmit, conn: &mut impl CommandWriter) {
        conn.write_command(&Command {
            command_id: CommandId::LinkRecv,
            channel: Channel::Other(req.link_id),
            payload: req.payload,
        })
        .await;
    }
}
