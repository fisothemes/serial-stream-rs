use serial_stream::{Result, blocking::SerialStream};

fn main() -> Result<()> {
    let ports = SerialStream::<String>::available_ports()?;

    for port in ports {
        println!("Found port: {}", port);
    }

    Ok(())
}
