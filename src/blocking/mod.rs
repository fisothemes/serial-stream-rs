mod sys;
use crate::{Direction, FullDuplex, HalfDuplex, Purge, SerialConfig};
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

    pub fn purge(&self, what: Purge) -> Result<()> {
        let _ = what;
        todo!()
    }
}

impl<P: AsRef<str>> SerialStream<P, FullDuplex> {
    pub fn try_split(self) -> Result<(ReadHalf<P>, WriteHalf<P>)> {
        todo!()
    }
}

impl<P: AsRef<str>> SerialStream<P, HalfDuplex> {
    pub fn set_rts(&mut self, level: bool) -> Result<()> {
        let _ = level;
        todo!()
    }

    pub fn flush_hardware(&mut self) -> Result<()> {
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

pub struct ReadHalf<P: AsRef<str>> {
    // This is scaffolding. Will likely need to wrap an OS-specific handle or Arc here
    _inner: InnerStream<P, FullDuplex>,
}

impl<P: AsRef<str>> ReadHalf<P> {
    pub fn purge(&self, what: Purge) {
        let _ = what;
        todo!()
    }
}

impl<P: AsRef<str>> Read for ReadHalf<P> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        let _ = buf;
        todo!()
    }
}

pub struct WriteHalf<P: AsRef<str>> {
    // This is scaffolding. Will likely need to wrap an OS-specific handle or Arc here
    _inner: InnerStream<P, FullDuplex>,
}

impl<P: AsRef<str>> WriteHalf<P> {
    pub fn purge(&self, purge: Purge) -> Result<()> {
        let _ = purge;
        todo!()
    }
}

impl<P: AsRef<str>> Write for WriteHalf<P> {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        let _ = buf;
        todo!()
    }

    fn flush(&mut self) -> Result<()> {
        todo!()
    }
}
