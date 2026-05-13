use serial_stream::{
    BaudRate, Result,
    blocking::{BufRead, BufReader, SerialStream, Write},
};
use std::thread;

fn main() -> Result<()> {
    let stream = SerialStream::open(("COM3", BaudRate::B9600))?;

    let (reader, mut writer) = stream.try_split()?;

    let reader_handle = thread::spawn(move || -> Result<()> {
        let reader = BufReader::new(reader);

        for line in reader.lines() {
            match line?.trim() {
                "exit" => break,
                line => println!("Reader: {}", line),
            }
        }

        Ok(())
    });

    writer.write_all(b"Hello, there!\r\n")?;

    reader_handle.join().unwrap()?;

    Ok(())
}
