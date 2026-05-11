pub(crate) trait Direction {}

#[derive(Debug, Clone, Copy, Default)]
pub struct FullDuplex;
impl Direction for FullDuplex {}

#[derive(Debug, Clone, Copy, Default)]
pub struct HalfDuplex {
    rts_turnaround_delay_ms: u32,
    invert_rts_pin: bool,
}

impl HalfDuplex {
    pub fn new(rts_turnaround_delay_ms: u32, invert_rts_pin: bool) -> Self {
        Self {
            rts_turnaround_delay_ms,
            invert_rts_pin,
        }
    }

    pub fn rts_turnaround_delay_ms(&self) -> u32 {
        self.rts_turnaround_delay_ms
    }

    pub fn invert_rts_pin(&self) -> bool {
        self.invert_rts_pin
    }
}

impl Direction for HalfDuplex {}
