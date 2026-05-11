mod baud_rate;
mod data_bits;
mod direction;
mod flow_control;
mod parity;
mod port;
mod stop_bits;

pub(crate) use direction::Direction;

pub use baud_rate::BaudRate;
pub use data_bits::DataBits;
pub use direction::{FullDuplex, HalfDuplex};
pub use flow_control::FlowControl;
pub use parity::Parity;
pub use port::Port;
pub use stop_bits::StopBits;

#[derive(Debug, Clone)]
#[allow(private_bounds)]
pub struct SerialConfig<P: AsRef<str>, D: Direction = FullDuplex> {
    port: Port<P>,
    baud_rate: BaudRate,
    data_bits: DataBits,
    parity: Parity,
    stop_bits: StopBits,
    flow_control: FlowControl,
    dtr: bool,
    rts: bool,
    mode: D,
}

#[allow(private_bounds)]
impl<P: AsRef<str>, D: Direction> SerialConfig<P, D> {
    pub fn with_baud_rate(mut self, value: BaudRate) -> Self {
        self.baud_rate = value;
        self
    }

    pub fn with_data_bits(mut self, value: DataBits) -> Self {
        self.data_bits = value;
        self
    }

    pub fn with_parity(mut self, value: Parity) -> Self {
        self.parity = value;
        self
    }

    pub fn with_stop_bits(mut self, value: StopBits) -> Self {
        self.stop_bits = value;
        self
    }

    pub fn with_flow_control(mut self, value: FlowControl) -> Self {
        self.flow_control = value;
        self
    }

    pub fn with_dtr(mut self, value: bool) -> Self {
        self.dtr = value;
        self
    }

    pub fn with_rts(mut self, value: bool) -> Self {
        self.rts = value;
        self
    }

    pub fn port(&self) -> &Port<P> {
        &self.port
    }

    pub fn baud_rate(&self) -> BaudRate {
        self.baud_rate
    }

    pub fn data_bits(&self) -> DataBits {
        self.data_bits
    }

    pub fn parity(&self) -> Parity {
        self.parity
    }

    pub fn stop_bits(&self) -> StopBits {
        self.stop_bits
    }

    pub fn flow_control(&self) -> FlowControl {
        self.flow_control
    }

    pub fn dtr(&self) -> bool {
        self.dtr
    }

    pub fn rts(&self) -> bool {
        self.rts
    }

    pub fn mode(&self) -> &D {
        &self.mode
    }
}

impl<P: AsRef<str>> SerialConfig<P, FullDuplex> {
    pub fn new(port: P) -> Self {
        Self {
            port: port.into(),
            baud_rate: BaudRate::default(),
            data_bits: DataBits::default(),
            parity: Parity::default(),
            stop_bits: StopBits::default(),
            flow_control: FlowControl::default(),
            dtr: true,
            rts: true,
            mode: FullDuplex::default(),
        }
    }

    pub fn into_half_duplex(self) -> SerialConfig<P, HalfDuplex> {
        SerialConfig {
            port: self.port,
            baud_rate: self.baud_rate,
            data_bits: self.data_bits,
            parity: self.parity,
            stop_bits: self.stop_bits,
            flow_control: self.flow_control,
            dtr: self.dtr,
            rts: self.rts,
            mode: HalfDuplex::default(),
        }
    }
}

impl<P: AsRef<str>> SerialConfig<P, HalfDuplex> {
    pub fn new(port: P) -> Self {
        Self {
            port: port.into(),
            baud_rate: BaudRate::default(),
            data_bits: DataBits::default(),
            parity: Parity::default(),
            stop_bits: StopBits::default(),
            flow_control: FlowControl::default(),
            dtr: true,
            rts: true,
            mode: HalfDuplex::default(),
        }
    }

    pub fn with_rts_turnaround_delay_ms(mut self, value: u32) -> Self {
        self.mode = HalfDuplex::new(value, self.mode.invert_rts_pin());
        self
    }

    pub fn with_invert_rts_pin(mut self, value: bool) -> Self {
        self.mode = HalfDuplex::new(self.mode.rts_turnaround_delay_ms(), value);
        self
    }

    pub fn rts_turnaround_delay_ms(&self) -> u32 {
        self.mode.rts_turnaround_delay_ms()
    }

    pub fn invert_rts_pin(&self) -> bool {
        self.mode.invert_rts_pin()
    }

    pub fn into_full_duplex(self) -> SerialConfig<P, FullDuplex> {
        SerialConfig {
            port: self.port,
            baud_rate: self.baud_rate,
            data_bits: self.data_bits,
            parity: self.parity,
            stop_bits: self.stop_bits,
            flow_control: self.flow_control,
            dtr: self.dtr,
            rts: self.rts,
            mode: FullDuplex::default(),
        }
    }
}

impl<P: AsRef<str>> From<SerialConfig<P, FullDuplex>> for SerialConfig<P, HalfDuplex> {
    fn from(config: SerialConfig<P, FullDuplex>) -> Self {
        config.into_half_duplex()
    }
}

impl<P: AsRef<str>> From<SerialConfig<P, HalfDuplex>> for SerialConfig<P, FullDuplex> {
    fn from(config: SerialConfig<P, HalfDuplex>) -> Self {
        config.into_full_duplex()
    }
}

impl<T: AsRef<str>> From<T> for SerialConfig<T, FullDuplex> {
    fn from(port: T) -> Self {
        Self::new(port)
    }
}

impl<T: AsRef<str>> From<T> for SerialConfig<T, HalfDuplex> {
    fn from(port: T) -> Self {
        Self::new(port)
    }
}

impl<T: AsRef<str>> From<(T, BaudRate)> for SerialConfig<T, FullDuplex> {
    fn from((port, baud_rate): (T, BaudRate)) -> Self {
        Self::new(port).with_baud_rate(baud_rate)
    }
}
