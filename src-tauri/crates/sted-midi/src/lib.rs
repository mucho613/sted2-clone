use std::io::Write;
use std::time::Duration;

/// Prepare serial port for sending MIDI data
pub fn serial_port() -> Box<dyn serialport::SerialPort> {
    let port_name = "COM1";
    let baud_rate = 38_400u32;

    let port = match serialport::new(port_name, baud_rate)
        .timeout(Duration::from_millis(500))
        .data_bits(serialport::DataBits::Eight)
        .parity(serialport::Parity::None)
        .stop_bits(serialport::StopBits::One)
        .flow_control(serialport::FlowControl::None)
        .open()
    {
        Ok(p) => p,
        Err(e) => {
            eprintln!("Failed to open {}: {}", port_name, e);
            std::process::exit(1);
        }
    };

    println!("Opened {} at {} bps", port_name, baud_rate);

    port
}

pub fn send(port: Box<dyn serialport::SerialPort>, data: &[u8]) -> std::io::Result<()> {
    let mut port = port;

    port.write_all(data)?;
    port.flush()?;

    Ok(())
}
