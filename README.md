# Serial Stream

A simple, ergonomic rust idiomatic cross-platform serial port library. This crate introduces standardised data streaming behaviour for serial coms similar to that of `std::net::TcpStream`.

## Usage

### Basic Read and Write

It's easy to open a serial port, then send and receive data.

```rust
use serial_stream::blocking::{SerialStream, BaudRate};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let mut stream = SerialStream::open(("COM3", BaudRate::B9600))?;
    
    stream.write_all(b"AT\r\n")?;
    
    let mut buf = [0;64];
    let n = stream.read(&mut buf)?;
    
    println!("Received: {:?}", &buf[..n]);
    
    Ok(())
}
```

### Reading Lines and Splitting Streams

For complex applications, you can split the stream into independent read and write halves. You can then intergrate these with standard library tools like `BufReader`. 

```rust
use serial_stream::blocking::{SerialStream, BaudRate};
use std::io::{BufRead, BufReader, Write};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let mut stream = SerialStream::open(("COM3", BaudRate::new(38400)))?;
    
    let (reader, mut writer) = stream.try_split()?;
    
    let mut reader = BufReader::new(reader);
    let mut line = String::new();
    
    writer.write_all(b"Hello, world!\r\n")?;
    reader.read_line(&mut line)?;
    
    println!("Device says: {}", line);
    
    Ok(())
}
```

### Advanced Configuration

If your extra configuration like hardware flow control, parity, or data terminal readiness, you can paa a full `SerialConfig` into the `SerialStream::open` method. The default configuration is the same as the python `serial` library's.

```rust
use serial_stream::blocking::{SerialStream, SerialConfig, BuadRate, Parity, FlowControl};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let config = SerialConfig::new("COM3", BuadRate::B115200)
        .with_parity(Parity::Even)
        .with_flow_control(FlowControl::Hardware)
        .with_data_terminal_ready(true);
    
    let mut stream = SerialStream::open(config)?;
    
    // ... use stream here
    
    Ok(())
}
```

## Status

> [!WARNING]
> This project is currently under active development. APIs are subject to change.