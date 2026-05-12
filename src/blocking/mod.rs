mod sys;
use crate::config::{Direction, FullDuplex, SerialConfig};
use std::io::{Read, Result, Write};
use sys::InnerStream;

#[allow(private_bounds)]
pub struct SerialStream<P: AsRef<str>, D: Direction = FullDuplex> {
    _inner: InnerStream<P, D>,
}

#[allow(private_bounds)]
impl<P: AsRef<str>, D: Direction> SerialStream<P, D> {
    pub fn open(config: impl Into<SerialConfig<P, D>>) -> Result<Self> {
        let _ = config.into();
        todo!()
    }
}

impl<P: AsRef<str>, D: Direction> Read for SerialStream<P, D> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        let _ = buf;
        todo!()
    }
}

impl<P: AsRef<str>, D: Direction> Write for SerialStream<P, D> {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        let _ = buf;
        todo!()
    }

    fn flush(&mut self) -> Result<()> {
        todo!()
    }
}
