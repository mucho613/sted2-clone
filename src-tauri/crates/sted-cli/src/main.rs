use std::env;
use std::fs;

fn main() {
    let mut args = env::args();
    let program = args.next().unwrap_or_else(|| "sted-cli".to_string());
    let command = match args.next() {
        Some(command) => command,
        None => print_usage_and_exit(&program, None),
    };

    match command.as_str() {
        "play" => play_command(program, args.collect()),
        "list-midi-ports" => list_midi_ports_and_exit(),
        "list-serial-ports" => list_serial_ports_and_exit(),
        "--help" | "-h" => print_usage_and_exit(&program, None),
        _ => {
            eprintln!("Unknown command: {command}");
            print_usage_and_exit(&program, None);
        }
    }
}

fn play_command(program: String, args: Vec<String>) {
    if args.is_empty() {
        eprintln!("Missing file path");
        print_usage_and_exit(&program, Some("play"));
    }

    let path = &args[0];
    let mut output: Option<String> = None;
    let mut port_name: Option<String> = None;

    let mut i = 1;
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
            "--help" | "-h" => {
                print_usage_and_exit(&program, Some("play"));
            }
            _ => {
                eprintln!("Unknown argument: {}", args[i]);
                print_usage_and_exit(&program, Some("play"));
            }
        }
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
            print_usage_and_exit(&program, Some("play"));
        }
    };

    if let Err(e) = sted_core::play(&rcp_file, target) {
        eprintln!("Playback failed: {e}");
        std::process::exit(1);
    }
}

fn print_usage_and_exit(program: &str, command: Option<&str>) -> ! {
    match command {
        Some("play") => {
            eprintln!(
                "Usage:\n\
                 {program} play <file.rcp> --output <serial|midi> --port <PORT_NAME>"
            );
        }
        _ => {
            eprintln!(
                "Usage:\n\
                 {program} play <file.rcp> --output <serial|midi> --port <PORT_NAME>\n\
                 {program} list-midi-ports\n\
                 {program} list-serial-ports"
            );
        }
    }
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
