use super::sys::InnerStream;
use super::{ReadHalf, WriteHalf};
use crate::{Direction, FullDuplex, HalfDuplex, Purge, SerialConfig};
use std::io::{Read, Write};

#[allow(private_bounds)]
pub struct SerialStream<P: AsRef<str>, D: Direction = FullDuplex> {
    _inner: InnerStream<P, D>,
}

#[allow(private_bounds)]
impl<P: AsRef<str>, D: Direction> SerialStream<P, D> {
    pub fn open(config: impl Into<SerialConfig<P, D>>) -> std::io::Result<Self> {
        let _ = config.into();
        todo!()
    }

    pub fn purge(&self, what: Purge) -> std::io::Result<()> {
        let _ = what;
        todo!()
    }
}

impl<P: AsRef<str>> SerialStream<P, FullDuplex> {
    pub fn try_split(self) -> std::io::Result<(ReadHalf<P>, WriteHalf<P>)> {
        todo!()
    }
}

impl<P: AsRef<str>> SerialStream<P, HalfDuplex> {
    pub fn set_rts(&mut self, level: bool) -> std::io::Result<()> {
        let _ = level;
        todo!()
    }

    pub fn flush_hardware(&mut self) -> std::io::Result<()> {
        todo!()
    }
}

impl<P: AsRef<str>, D: Direction> Read for SerialStream<P, D> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let _ = buf;
        todo!()
    }
}

impl<P: AsRef<str>, D: Direction> Write for SerialStream<P, D> {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let _ = buf;
        todo!()
    }

    fn flush(&mut self) -> std::io::Result<()> {
        todo!()
    }
}
