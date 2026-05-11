#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(u8)]
pub enum DataBits {
    #[default]
    Eight = 8,
    Seven = 7,
    Six = 6,
    Five = 5,
}

impl From<u8> for DataBits {
    fn from(value: u8) -> Self {
        match value {
            8 => Self::Eight,
            7 => Self::Seven,
            6 => Self::Six,
            5 => Self::Five,
            _ => unreachable!(),
        }
    }
}

impl From<DataBits> for u8 {
    fn from(value: DataBits) -> Self {
        value as u8
    }
}
