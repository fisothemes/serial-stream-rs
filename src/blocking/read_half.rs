use super::sys::InnerStream;
use crate::{FullDuplex, Purge};
use std::io::Read;

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
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let _ = buf;
        todo!()
    }
}
