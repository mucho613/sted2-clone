use std::io::Write;
use std::time::Duration;

fn main() {
    let port_name = "COM1";
    let baud_rate = 38_400u32;

    let mut port = match serialport::new(port_name, baud_rate)
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

    // GS Reset command payload
    let test_payload: &[u8] = &[
        0xF0, 0x41, 0x10, 0x42, 0x12, 0x40, 0x00, 0x7F, 0x00, 0x41, 0xF7,
    ];

    if let Err(e) = port.write_all(test_payload) {
        eprintln!("write_all failed: {}", e);
        std::process::exit(1);
    }
    if let Err(e) = port.flush() {
        eprintln!("flush failed: {}", e);
        std::process::exit(1);
    }

    println!("Sent {} bytes: {:02X?}", test_payload.len(), test_payload);
}
