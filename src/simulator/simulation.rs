use std::sync::Arc;

use crate::logs::service::LogsService;

use super::{device::Device, mixer::Mixer, protocol::ConfigVariable, qemu::instance::QemuInstance};

#[derive(Default)]
pub struct Simulation {
    devices: Vec<Device>,
    mixer: Mixer,
    flash_image: Option<String>,
}

impl Simulation {
    pub fn create_link(&mut self) -> u16 {
        self.mixer.create_link()
    }

    pub fn create_device(&mut self) -> u32 {
        let device_id = self.devices.len();
        self.devices.push(Device::new(device_id as u32));
        device_id as u32
    }

    pub fn set_device_config_word(&mut self, device_id: u32, word_id: u32, value: u32) -> bool {
        let device = &mut self.devices[device_id as usize];
        let variable = ConfigVariable::from(word_id);

        if !variable.is_user_defined() {
            return false;
        }

        device.set_config_word(variable, value);
        true
    }

    pub fn connect_link_output(&mut self, link_id: u16, device_id: u32) {
        let device = &mut self.devices[device_id as usize];

        self.mixer.connect_link_output(link_id, device);

        let attached_links = device
            .get_config_word(ConfigVariable::AttachedLinksCount)
            .unwrap_or(0) as u8;
        device.set_config_word(ConfigVariable::AttachedLink(attached_links), link_id as u32);
        device.set_config_word(
            ConfigVariable::AttachedLinksCount,
            (attached_links + 1) as u32,
        );
    }

    pub fn set_flash_image_path(&mut self, path: impl Into<String>) {
        self.flash_image = Some(path.into());
    }

    pub fn device_count(&self) -> usize {
        self.devices.len()
    }

    pub async fn run(self, logs: Arc<LogsService>) {
        let flash_image_path = self.flash_image.as_ref().unwrap();
        let mixer = Arc::new(self.mixer);
        let mut instances = Vec::new();
        for device in self.devices {
            let instance = QemuInstance::new(device, logs.clone());

            instances.push(tokio::spawn(
                instance.run(mixer.clone(), flash_image_path.into()),
            ));
        }

        for instance in instances {
            instance.await.unwrap();
        }
    }
}
