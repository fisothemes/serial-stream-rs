use super::sys::InnerStream;
use crate::{FullDuplex, Purge};
use std::io::Write;

pub struct WriteHalf<P: AsRef<str>> {
    // This is scaffolding. Will likely need to wrap an OS-specific handle or Arc here
    _inner: InnerStream<P, FullDuplex>,
}

impl<P: AsRef<str>> WriteHalf<P> {
    pub fn purge(&self, purge: Purge) -> std::io::Result<()> {
        let _ = purge;
        todo!()
    }
}

impl<P: AsRef<str>> Write for WriteHalf<P> {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let _ = buf;
        todo!()
    }

    fn flush(&mut self) -> std::io::Result<()> {
        todo!()
    }
}
