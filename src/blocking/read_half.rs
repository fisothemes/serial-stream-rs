use super::sys::InnerStream;
use crate::{FullDuplex, Purge};
use std::io::{self, Read};
use std::time::Duration;

pub struct ReadHalf<P: AsRef<str>> {
    // This is scaffolding. Will likely need to wrap an OS-specific handle or Arc here
    _inner: InnerStream<P, FullDuplex>,
}

impl<P: AsRef<str>> ReadHalf<P> {
    pub fn purge(&self, what: Purge) {
        let _ = what;
        todo!()
    }

    pub fn set_read_timeout(&self, timeout: impl Into<Option<Duration>>) -> io::Result<()> {
        let _ = timeout;
        todo!()
    }

    pub fn read_timeout(&self) -> io::Result<Option<Duration>> {
        todo!()
    }
}

impl<P: AsRef<str>> Read for ReadHalf<P> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let _ = buf;
        todo!()
    }
}
