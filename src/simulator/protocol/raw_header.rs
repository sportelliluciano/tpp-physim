pub struct RawHeader {
    pub command: u16,
    pub channel: u16,
    pub payload_size: u32,
}

impl From<[u8; 8]> for RawHeader {
    fn from(value: [u8; 8]) -> Self {
        Self {
            command: u16::from_le_bytes(value[..2].try_into().unwrap()),
            channel: u16::from_le_bytes(value[2..4].try_into().unwrap()),
            payload_size: u32::from_le_bytes(value[4..].try_into().unwrap()),
        }
    }
}

impl RawHeader {
    pub fn to_bytes(&self) -> [u8; 8] {
        let cmd_bytes = self.command.to_le_bytes();
        let chan_bytes = self.channel.to_le_bytes();
        let psz_bytes = self.payload_size.to_le_bytes();

        [
            cmd_bytes[0],
            cmd_bytes[1],
            chan_bytes[0],
            chan_bytes[1],
            psz_bytes[0],
            psz_bytes[1],
            psz_bytes[2],
            psz_bytes[3],
        ]
    }
}
