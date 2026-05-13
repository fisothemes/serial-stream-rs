use serial_stream::{BaudRate, blocking::SerialStream};
use std::io::{self, BufRead, BufReader, Read, Write};
use std::thread;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let stream = SerialStream::open(("COM3", BaudRate::B9600))?;

    let (reader, mut writer) = stream.try_split()?;

    let reader_handle = thread::spawn(move || -> io::Result<()> {
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
