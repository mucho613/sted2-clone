use std::time::Duration;

#[derive(Debug)]
pub enum SerialPortError {
    Serial(serialport::Error),
}

impl std::fmt::Display for SerialPortError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SerialPortError::Serial(err) => write!(f, "Serial port error: {err}"),
        }
    }
}

impl std::error::Error for SerialPortError {}

pub fn serial_ports() -> Result<Vec<String>, SerialPortError> {
    let ports = serialport::available_ports().map_err(SerialPortError::Serial)?;
    Ok(ports.into_iter().map(|port| port.port_name).collect())
}

/// Prepare serial port for sending MIDI data
pub fn serial_port(
    port_name: &str,
) -> Result<Box<dyn std::io::Write + Send>, SerialPortError> {
    let baud_rate = 38_400u32;

    let port: Box<dyn std::io::Write + Send> = serialport::new(port_name, baud_rate)
        .timeout(Duration::from_millis(500))
        .data_bits(serialport::DataBits::Eight)
        .parity(serialport::Parity::None)
        .stop_bits(serialport::StopBits::One)
        .flow_control(serialport::FlowControl::None)
        .open()
        .map_err(SerialPortError::Serial)?;

    Ok(port)
}

pub fn send(port: &mut dyn std::io::Write, data: &[u8]) -> std::io::Result<()> {
    port.write_all(data)?;
    port.flush()?;

    Ok(())
}
