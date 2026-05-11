# Serial Stream

A simple, ergonomic rust idiomatic cross-platform serial port library. This crate introduces standardised data streaming behaviour for serial coms similar to that of `std::net::TcpStream`.

## Usage

### Basic Read and Write

It's easy to open a serial port, then send and receive data.

```rust
use serial_stream::blocking::{SerialStream, BaudRate};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let mut stream = SerialStream::open("COM3")?;
    
    stream.write_all(b"AT\r\n")?;
    
    let mut buf = [0;64];
    let n = stream.read(&mut buf)?;
    
    println!("Received: {:?}", &buf[..n]);
    
    Ok(())
}
```

### Reading Lines and Splitting Streams

For complex applications, you can simply split a `FullDuplex` stream into independent read and write halves. You can then intergrate these with standard library tools like `BufReader` or `Send` them to another thread.

```rust
use serial_stream::blocking::{SerialStream, BaudRate};
use std::io::{BufRead, BufReader, Write};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let mut stream = SerialStream::open(("COM3", BaudRate::new(38400)))?;

    // Split into owned halves for lock-free concurrency.
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

If you need specific settings like hardware flow control, parity, or data terminal readiness, you can use the `SerialConfig` struct to build your configuration and pass it into the `SerialStream::open` method. The default configuration is the same as the python `serial` library's.

```rust
use serial_stream::blocking::{SerialStream, SerialConfig, BuadRate, Parity, FlowControl};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let config = SerialConfig::new("COM3")
        .with_baud_rate(BuadRate::B115200)
        .with_parity(Parity::Even)
        .with_flow_control(FlowControl::Hardware)
        .with_dtr(true);
    
    let mut stream = SerialStream::open(config)?;
    
    // ... use stream here
    
    Ok(())
}
```

### Half-Duplex and RS-485 (Typestate Safety)

`serial-stream` uses generic marker types (`SerialStream<D>`) to enforce hardware safety. This allows you to open a type-safe `HalfDuplex` port. This alters the available methods on the stream.

```rust
use serial_stream::blocking::{SerialStream, SerialConfig, BaudRate};
use std::io::{Read, Write, Result};

fn main() -> Result<()> {
    // Transition the config to Half-Duplex (with a 5ms RTS turnaround delay)
    let config = SerialConfig::new("/dev/ttyUSB0")
        .with_baud_rate(BaudRate::B38400)
        .into_half_duplex()
        .with_rts_turnaround_delay_ms(5);

    let mut bus = SerialStream::open(config)?;

    // `bus.try_split()?` doesn't exist for HalfDuplex streams.

    // 1. Take control of the hardware bus (Transmit Mode)
    bus.set_rts(true)?;
    
    // 2. Write data using standard traits
    bus.write_all(b"POLL_SENSOR\n")?;
    bus.flush_hardware()?; // Blocks until the last bit leaves the UART
    
    // 3. Release the bus (Receive Mode)
    bus.set_rts(false)?;
    
    // 4. Safely read the response
    let mut response = [0; 32];
    bus.read(&mut response)?;

    Ok(())
}
```

## Status

> [!WARNING]
> This project is currently under active development. APIs are subject to change.