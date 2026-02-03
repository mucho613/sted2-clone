use midir::MidiOutput;
pub use midir::{MidiOutputConnection, SendError};

#[derive(Debug)]
pub enum MidiOutputError {
    Init(midir::InitError),
    NoPorts,
    PortName(midir::PortInfoError),
    PortIndexOutOfRange { index: usize, count: usize },
    PortNameNotFound { name: String },
    Connect(midir::ConnectError<MidiOutput>),
}

impl std::fmt::Display for MidiOutputError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MidiOutputError::Init(err) => write!(f, "Failed to initialize MIDI output: {err}"),
            MidiOutputError::NoPorts => write!(f, "No MIDI output ports are available"),
            MidiOutputError::PortName(err) => write!(f, "Failed to read MIDI port name: {err}"),
            MidiOutputError::PortIndexOutOfRange { index, count } => write!(
                f,
                "MIDI output port index out of range: index={index}, count={count}"
            ),
            MidiOutputError::PortNameNotFound { name } => {
                write!(f, "MIDI output port not found: {name}")
            }
            MidiOutputError::Connect(err) => write!(f, "Failed to connect to MIDI output: {err}"),
        }
    }
}

impl std::error::Error for MidiOutputError {}

/// List available MIDI output port names.
pub fn midi_output_ports() -> Result<Vec<String>, MidiOutputError> {
    let midi_out = MidiOutput::new("sted-midi").map_err(MidiOutputError::Init)?;
    let ports = midi_out.ports();
    if ports.is_empty() {
        return Err(MidiOutputError::NoPorts);
    }

    let mut names = Vec::with_capacity(ports.len());
    for port in ports {
        let name = midi_out
            .port_name(&port)
            .map_err(MidiOutputError::PortName)?;
        names.push(name);
    }

    Ok(names)
}

/// Prepare a MIDI output connection by index.
pub fn midi_output_connection(port_index: usize) -> Result<MidiOutputConnection, MidiOutputError> {
    let midi_out = MidiOutput::new("sted-midi").map_err(MidiOutputError::Init)?;
    let ports = midi_out.ports();
    if ports.is_empty() {
        return Err(MidiOutputError::NoPorts);
    }
    if port_index >= ports.len() {
        return Err(MidiOutputError::PortIndexOutOfRange {
            index: port_index,
            count: ports.len(),
        });
    }

    let port = &ports[port_index];
    midi_out
        .connect(port, "sted-midi-output")
        .map_err(MidiOutputError::Connect)
}

/// Prepare a MIDI output connection by exact port name match.
pub fn midi_output_connection_by_name(
    port_name: &str,
) -> Result<MidiOutputConnection, MidiOutputError> {
    let midi_out = MidiOutput::new("sted-midi").map_err(MidiOutputError::Init)?;
    let ports = midi_out.ports();
    if ports.is_empty() {
        return Err(MidiOutputError::NoPorts);
    }

    for port in ports {
        let name = midi_out
            .port_name(&port)
            .map_err(MidiOutputError::PortName)?;
        if name == port_name {
            return midi_out
                .connect(&port, "sted-midi-output")
                .map_err(MidiOutputError::Connect);
        }
    }

    Err(MidiOutputError::PortNameNotFound {
        name: port_name.to_string(),
    })
}

/// Send MIDI bytes to an open MIDI output connection.
pub fn send_midi(connection: &mut MidiOutputConnection, data: &[u8]) -> Result<(), SendError> {
    connection.send(data)
}
