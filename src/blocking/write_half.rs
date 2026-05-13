use super::sys::InnerStream;
use crate::{FullDuplex, Purge};
use std::io::{self, Write};
use std::time::Duration;

pub struct WriteHalf<P: AsRef<str>> {
    // This is scaffolding. Will likely need to wrap an OS-specific handle or Arc here
    _inner: InnerStream<P, FullDuplex>,
}

impl<P: AsRef<str>> WriteHalf<P> {
    pub fn purge(&self, purge: Purge) -> io::Result<()> {
        let _ = purge;
        todo!()
    }

    pub fn set_write_timeout(&self, timeout: impl Into<Option<Duration>>) -> io::Result<()> {
        let _ = timeout;
        todo!()
    }

    pub fn write_timeout(&self) -> io::Result<Option<Duration>> {
        todo!()
    }
}

impl<P: AsRef<str>> Write for WriteHalf<P> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let _ = buf;
        todo!()
    }

    fn flush(&mut self) -> io::Result<()> {
        todo!()
    }
}
