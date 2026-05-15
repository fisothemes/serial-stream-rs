use super::sys::InnerStream;
use super::{ReadHalf, WriteHalf};
use crate::{AutoHalfDuplex, Direction, FullDuplex, HalfDuplex, Port, Purge, SerialConfig};
use std::io::{self, Read, Write};
use std::time::Duration;

#[allow(private_bounds)]
pub struct SerialStream<P: AsRef<str>, D: Direction = FullDuplex> {
    _inner: InnerStream<P, D>,
}

#[allow(private_bounds)]
impl<P: AsRef<str>, D: Direction> SerialStream<P, D> {
    pub fn open(config: impl Into<SerialConfig<P, D>>) -> io::Result<Self> {
        let _ = config.into();
        todo!()
    }

    pub fn purge(&self, what: Purge) -> io::Result<()> {
        let _ = what;
        todo!()
    }

    pub fn set_read_timeout(&self, timeout: impl Into<Option<Duration>>) -> io::Result<()> {
        let _ = timeout;
        todo!()
    }

    pub fn set_write_timeout(&self, timeout: impl Into<Option<Duration>>) -> io::Result<()> {
        let _ = timeout;
        todo!()
    }

    pub fn read_timeout(&self) -> io::Result<Option<Duration>> {
        todo!()
    }

    pub fn write_timeout(&self) -> io::Result<Option<Duration>> {
        todo!()
    }

    pub fn flush_hardware(&mut self) -> io::Result<()> {
        todo!()
    }
}

impl<P: AsRef<str>, D: Direction> Read for SerialStream<P, D> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let _ = buf;
        todo!()
    }
}

impl<P: AsRef<str>, D: Direction> Write for SerialStream<P, D> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let _ = buf;
        todo!()
    }

    fn flush(&mut self) -> io::Result<()> {
        todo!()
    }
}

impl<P: AsRef<str> + From<String>> SerialStream<P> {
    pub fn available_ports() -> io::Result<impl Iterator<Item = Port<P>>> {
        // Placeholder implementation. Makes compiler happy.
        let raw_ports = Vec::new();
        Ok(raw_ports.into_iter().map(|s| Port::new(P::from(s))))
    }
}

impl<P: AsRef<str>> SerialStream<P, FullDuplex> {
    pub fn try_split(self) -> io::Result<(ReadHalf<P>, WriteHalf<P>)> {
        todo!()
    }
}

impl<P: AsRef<str>> SerialStream<P, HalfDuplex> {
    pub fn try_into_auto(self) -> io::Result<SerialStream<P, AutoHalfDuplex>> {
        todo!()
    }

    pub fn set_rts(&mut self, level: bool) -> io::Result<()> {
        let _ = level;
        todo!()
    }
}

impl<P: AsRef<str>> TryFrom<SerialStream<P, HalfDuplex>> for SerialStream<P, AutoHalfDuplex> {
    type Error = io::Error;
    fn try_from(value: SerialStream<P, HalfDuplex>) -> Result<Self, Self::Error> {
        value.try_into_auto()
    }
}

impl<P: AsRef<str>> SerialStream<P, AutoHalfDuplex> {
    pub fn try_into_manual(self) -> io::Result<SerialStream<P, HalfDuplex>> {
        todo!()
    }
}

impl<P: AsRef<str>> TryFrom<SerialStream<P, AutoHalfDuplex>> for SerialStream<P, HalfDuplex> {
    type Error = io::Error;
    fn try_from(value: SerialStream<P, AutoHalfDuplex>) -> Result<Self, Self::Error> {
        value.try_into_manual()
    }
}
