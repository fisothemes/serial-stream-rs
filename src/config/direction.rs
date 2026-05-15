use std::time::Duration;

pub(crate) trait Direction {}

#[derive(Debug, Clone, Copy, Default)]
pub struct FullDuplex;
impl Direction for FullDuplex {}

#[derive(Debug, Clone, Copy, Default)]
pub struct HalfDuplex;

impl Direction for HalfDuplex {}

/// Automatic half-duplex mode. The OS kernel automatically toggles the RTS pin.
#[derive(Debug, Clone, Copy, Default)]
pub struct AutoHalfDuplex {
    delay_before_send: Duration,
    delay_after_send: Duration,
    invert_rts: bool,
    terminate_bus: bool,
}

impl AutoHalfDuplex {
    pub fn new(
        delay_before_send: Duration,
        delay_after_send: Duration,
        invert_rts: bool,
        terminate_bus: bool,
    ) -> Self {
        Self {
            delay_before_send,
            delay_after_send,
            invert_rts,
            terminate_bus,
        }
    }

    pub fn with_delay_before_send(mut self, value: Duration) -> Self {
        self.delay_before_send = value;
        self
    }

    pub fn with_delay_after_send(mut self, value: Duration) -> Self {
        self.delay_after_send = value;
        self
    }

    pub fn with_invert_rts(mut self, value: bool) -> Self {
        self.invert_rts = value;
        self
    }
    pub fn with_terminate_bus(mut self, value: bool) -> Self {
        self.terminate_bus = value;
        self
    }

    pub fn delay_before_send(&self) -> Duration {
        self.delay_before_send
    }

    pub fn delay_after_send(&self) -> Duration {
        self.delay_after_send
    }

    pub fn invert_rts(&self) -> bool {
        self.invert_rts
    }

    pub fn terminate_bus(&self) -> bool {
        self.terminate_bus
    }
}

impl Direction for AutoHalfDuplex {}
