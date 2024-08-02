#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum ConfigVariable {
    UserDefined(u32),
    DeviceId,
    AttachedLinksCount,
    AttachedLink(u8),
    Reserved,
    Magic,
}

impl ConfigVariable {
    pub fn is_user_defined(&self) -> bool {
        matches!(self, Self::UserDefined(_))
    }
}

impl From<u32> for ConfigVariable {
    fn from(value: u32) -> Self {
        match value {
            0xFFFF_FF00 => Self::DeviceId,
            0xFFFF_FF01 => Self::AttachedLinksCount,
            0xFFFF_FF02..=0xFFFF_FF0F => Self::AttachedLink((value - 0xFFFF_FF02) as u8),
            0xFFFF_FF10..=0xFFFF_FFFE => Self::Reserved,
            0xFFFF_FFFF => Self::Magic,
            other => Self::UserDefined(other),
        }
    }
}
