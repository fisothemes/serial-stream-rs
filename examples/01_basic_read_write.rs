use serial_stream::{
    Result,
    blocking::{Read, SerialStream, Write},
};

fn main() -> Result<()> {
    let mut stream = SerialStream::open("COM3")?;

    stream.write_all(b"AT\r\n")?;

    let mut buf = [0; 64];
    let n = stream.read(&mut buf)?;

    println!("Received: {:?}", &buf[..n]);

    Ok(())
}
