use serial_stream::blocking::SerialStream;
use std::io::{self, Read, Write};

fn main() -> io::Result<()> {
    let mut stream = SerialStream::open("COM3")?;

    stream.write_all(b"AT\r\n")?;

    let mut buf = [0; 64];
    let n = stream.read(&mut buf)?;

    println!("Received: {:?}", &buf[..n]);

    Ok(())
}
