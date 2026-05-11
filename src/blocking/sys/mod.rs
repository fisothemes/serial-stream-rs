#[cfg(unix)]
mod unix;
#[cfg(windows)]
mod windows;

#[cfg(unix)]
pub(super) use unix::InnerStream;

#[cfg(windows)]
pub(super) use windows::InnerStream;
