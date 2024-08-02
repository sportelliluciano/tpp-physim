use std::collections::HashMap;

use tokio::sync::mpsc::Sender;

use super::{
    device::Device,
    protocol::{requests::LinkTransmit, Channel, Command, CommandId},
};

#[derive(Default)]
pub struct Mixer {
    connections: HashMap<u16, Sender<Command>>,
    links: u16,
}

impl Mixer {
    pub fn create_link(&mut self) -> u16 {
        let link_id = self.links;
        self.links += 1;
        link_id
    }

    pub fn connect_link_output(&mut self, link_id: u16, device: &Device) {
        self.connections.insert(link_id, device.get_handle());
    }

    pub async fn link_send(&self, req: LinkTransmit) {
        let LinkTransmit { link_id, payload } = req;

        if let Some(tx) = self.connections.get(&link_id) {
            if let Err(e) = tx
                .send(Command {
                    command_id: CommandId::LinkRecv,
                    channel: Channel::Other(link_id),
                    payload,
                })
                .await
            {
                println!("[CONTROLLER] Could not deliver packet through link {link_id} -- link down {e:?}")
            }
        } else {
            println!("[CONTROLLER] Dropping packet to link_id={link_id} (no one is listening)",);
        }
    }
}
