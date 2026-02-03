use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let path = args.get(1).expect("Please provide a path to the RCP file");
    let mut output: Option<String> = None;
    let mut port_name: Option<String> = None;
    let mut list_midi_ports = false;
    let mut list_serial_ports = false;

    let mut i = 2;
    while i < args.len() {
        match args[i].as_str() {
            "--output" => {
                let value = args.get(i + 1).expect("Missing value for --output");
                output = Some(value.to_string());
                i += 2;
            }
            "--port" => {
                let value = args.get(i + 1).expect("Missing value for --port");
                port_name = Some(value.to_string());
                i += 2;
            }
            "--list-midi-ports" => {
                list_midi_ports = true;
                i += 1;
            }
            "--list-serial-ports" => {
                list_serial_ports = true;
                i += 1;
            }
            "--help" | "-h" => {
                print_usage_and_exit();
            }
            _ => {
                eprintln!("Unknown argument: {}", args[i]);
                print_usage_and_exit();
            }
        }
    }

    if list_midi_ports {
        list_midi_ports_and_exit();
    }
    if list_serial_ports {
        list_serial_ports_and_exit();
    }

    let output = output.expect("Missing --output (serial or midi)");
    let port_name = port_name.expect("Missing --port");

    let binary = fs::read(path).expect("Failed to read the RCP file");
    let rcp_file = sted_core::load(&binary);

    let target = match output.as_str() {
        "serial" => sted_core::OutputTarget::Serial { port_name },
        "midi" => sted_core::OutputTarget::Midi { port_name },
        other => {
            eprintln!("Unknown output type: {}", other);
            print_usage_and_exit();
        }
    };

    if let Err(e) = sted_core::play(&rcp_file, target) {
        eprintln!("Playback failed: {e}");
        std::process::exit(1);
    }
}

fn print_usage_and_exit() -> ! {
    eprintln!(
        "Usage: sted-cli <file.rcp> --output <serial|midi> --port <PORT_NAME>\n\
         Options:\n\
         --list-midi-ports\n\
         --list-serial-ports"
    );
    std::process::exit(1);
}

fn list_midi_ports_and_exit() -> ! {
    match sted_core::midi_output_ports() {
        Ok(ports) => {
            for port in ports {
                println!("{port}");
            }
        }
        Err(err) => {
            eprintln!("Failed to list MIDI ports: {err}");
            std::process::exit(1);
        }
    }
    std::process::exit(0);
}

fn list_serial_ports_and_exit() -> ! {
    match sted_core::serial_ports() {
        Ok(ports) => {
            for port in ports {
                println!("{port}");
            }
        }
        Err(err) => {
            eprintln!("Failed to list serial ports: {err}");
            std::process::exit(1);
        }
    }
    std::process::exit(0);
}
