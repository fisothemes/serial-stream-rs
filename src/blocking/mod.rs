mod sys;
use crate::config::{Direction, FullDuplex, HalfDuplex, SerialConfig};
use std::io::{Read, Result, Write};
use sys::InnerStream;

#[allow(private_bounds)]
pub struct SerialStream<P: AsRef<str>, D: Direction = FullDuplex> {
    inner: InnerStream<P, D>,
}

impl<P: AsRef<str>> SerialStream<P, FullDuplex> {
    pub fn open(config: impl Into<SerialConfig<P>>) -> Result<Self> {
        todo!()
    }
}

impl<P: AsRef<str>> SerialStream<P, HalfDuplex> {
    pub fn open(config: impl Into<SerialConfig<P, HalfDuplex>>) -> Result<Self> {
        todo!()
    }
}

impl<P: AsRef<str>, D: Direction> Read for SerialStream<P, D> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        todo!()
    }
}

impl<P: AsRef<str>, D: Direction> Write for SerialStream<P, D> {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        todo!()
    }

    fn flush(&mut self) -> Result<()> {
        todo!()
    }
}
