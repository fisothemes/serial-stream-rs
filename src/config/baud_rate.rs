#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BaudRate(u32);
impl BaudRate {
    pub const B110: BaudRate = BaudRate(110);
    pub const B300: BaudRate = BaudRate(300);
    pub const B600: BaudRate = BaudRate(600);
    pub const B1200: BaudRate = BaudRate(1200);
    pub const B2400: BaudRate = BaudRate(2400);
    pub const B4800: BaudRate = BaudRate(4800);
    pub const B9600: BaudRate = BaudRate(9600);
    pub const B19200: BaudRate = BaudRate(19200);
    pub const B31250: BaudRate = BaudRate(31250);
    pub const B38400: BaudRate = BaudRate(38400);
    pub const B57600: BaudRate = BaudRate(57600);
    pub const B115200: BaudRate = BaudRate(115200);
    pub const B250000: BaudRate = BaudRate(250000);
    pub const B230400: BaudRate = BaudRate(230400);
    pub const B460800: BaudRate = BaudRate(460800);
    pub const B921600: BaudRate = BaudRate(921600);

    pub fn new(baud_rate: u32) -> Self {
        Self(baud_rate)
    }

    pub fn as_u32(self) -> u32 {
        self.0
    }
}

impl From<u32> for BaudRate {
    fn from(value: u32) -> Self {
        Self(value)
    }
}

impl From<BaudRate> for u32 {
    fn from(value: BaudRate) -> Self {
        value.0
    }
}

impl Default for BaudRate {
    fn default() -> Self {
        Self::B9600
    }
}
