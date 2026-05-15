# Serial Stream

A simple, ergonomic rust idiomatic cross-platform serial port library. This crate introduces standardised data streaming behaviour for serial coms similar to that of `std::net::TcpStream`.

## Usage

### Finding Available Ports

You can easily query the operating system for available serial ports. To the query returns a lazy iterator.

```rust
use serial_stream::{Port, Result, blocking::SerialStream};

fn main() -> Result<()> {
    // The compiler will automatically infer the type based on the binding.
    let ports: Vec<Port<String>> = SerialStream::available_ports()?.collect();

    for port in ports {
        println!("Found active port: {}", port.as_ref());
    }

    Ok(())
}
```

### Basic Read and Write

It's easy to open a serial port, then send and receive data.

```rust
use serial_stream::{Result, blocking::{Read, Write, SerialStream}};

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
use serial_stream::{Result, BaudRate, blocking::SerialStream};
use std::io::{BufRead, BufReader, Write};

fn main() -> Result<()> {
    let stream = SerialStream::open(("COM3", BaudRate::B38400))?;

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

If you need specific settings like hardware flow control, parity, or data terminal readiness, you can use the `SerialConfig` struct to build your configuration and pass it into the `SerialStream::open` method. The default configuration is the same as the python [`serial`](https://pyserial.readthedocs.io/en/latest/pyserial_api.html) library's.

```rust
use serial_stream::{
    BaudRate, FlowControl, Parity, Result, SerialConfig, 
    blocking::SerialStream
};
use std::time::Duration;

fn main() -> Result<()> {
    let config = SerialConfig::new("COM3")
        .with_baud_rate(BaudRate::B115200)
        .with_parity(Parity::Even)
        .with_flow_control(FlowControl::Hardware)
        .with_dtr(true)
        .with_read_timeout(Duration::from_millis(500));

    let mut stream = SerialStream::open(config)?;

    // ... use stream here

    Ok(())
}
```

### Half-Duplex and RS-485 (Typestate Safety)

`serial-stream` uses generic marker types to enforce hardware safety. `FullDuplex` ports allow splitting across threads, while half-duplex configurations strictly enforce turn-taking on the bus.

You can operate in **Manual Half-Duplex** (where you manually toggle the **RTS** pin) or **Automatic Half-Duplex** (where the OS kernel handles the toggling for **RS-485**). The typestate pattern allows you to transition between them at any point closing the port.

```rust
use serial_stream::{
    BaudRate, Result, SerialConfig,
    blocking::{Read, SerialStream, Write},
};

fn main() -> Result<()> {
    // Open the port in Manual Half-Duplex mode
    let config = SerialConfig::new("/dev/ttyUSB0")
        .with_baud_rate(BaudRate::B38400)
        .into_half_duplex(); // Manual mode

    let mut bus = SerialStream::open(config)?;

    // `bus.try_split()?` doesn't exist for HalfDuplex streams, so you don't need to
    // worry about bus collisions.

    // Manually wake up a sleeping sensor by holding RTS high.
    bus.set_rts(true)?;
    bus.write_all(b"WAKEUP_PULSE\n")?;
    bus.flush_hardware()?; // Blocks until the last bit physically leaves the UART.
    bus.set_rts(false)?;

    // Now that the sensor is awake, hand control over to the OS kernel.
    // The port is not dropped; it moved into `AutoHalfDuplex`.
    let mut auto_bus = bus.try_into_auto()?;

    // (If you wanted to configure RS-485 specific delays, you can do so in the config beforehand)
    // let auto_config = SerialConfig::new("/dev/ttyUSB0")
    //     .into_half_duplex().into_auto()
    //     .with_delay_before_send(std::time::Duration::from_millis(5));

    // `.set_rts()` is now impossible to call on `auto_bus`.
    // The Rust compiler protects you from racing the OS kernel!

    auto_bus.write_all(b"POLL_DATA\n")?;

    let mut response = [0; 32];
    auto_bus.read(&mut response)?;

    Ok(())
}
```

### Handling Thread Interrupts and Stream Closures

Under normal circumstances the port should automatically close when streams are dropped. However, if a stream is split across threads, simply dropping the writer will not wake up a permanently blocked reader thread.

To handle graceful shutdowns in that situation, there is a `.purge()` API. This mirrors the ergonomics of `TcpStream::shutdown()`, which will safely interrupt blocking OS operations from another thread.

```rust
use serial_stream::{
    Purge, Result,
    blocking::{BufRead, BufReader, SerialStream, Write},
};

fn main() -> Result<()> {
    let stream = SerialStream::open("COM1")?;

    let (reader, mut writer) = stream.try_split()?;

    let reader_handle = std::thread::spawn(move || {
        let mut reader = BufReader::new(reader);
        let mut line = String::new();

        // This will block indefinitely until data arrives OR the hardware is purged.
        match reader.read_line(&mut line) {
            Ok(_) => println!("Received: {}", line),
            Err(e) => println!("Reader safely interrupted: {}", e),
        }

        println!("Reader thread shutting down.");
    });

    writer.write_all(b"Disconnecting...\n")?;

    // Send a hardware-level interrupt to the OS.
    // This instantly aborts the pending `.read_line()` on the reader thread,
    // waking it up with an io::Error.
    writer.purge(Purge::Read)?;

    // Wait for the reader thread to gracefully exit!
    let _ = reader_handle.join();

    Ok(())
}
```

## Status

> [!WARNING]
> This project is currently under active development. APIs are subject to change.