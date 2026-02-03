#[derive(Debug)]
pub enum MidiPortsError {
    MidiOutput(sted_midi::MidiOutputError),
}

impl std::fmt::Display for MidiPortsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MidiPortsError::MidiOutput(err) => {
                write!(f, "Failed to list MIDI output ports: {err}")
            }
        }
    }
}

impl std::error::Error for MidiPortsError {}

#[derive(Debug)]
pub enum SerialPortsError {
    SerialPort(sted_midi::SerialPortError),
}

impl std::fmt::Display for SerialPortsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SerialPortsError::SerialPort(err) => {
                write!(f, "Failed to list serial ports: {err}")
            }
        }
    }
}

impl std::error::Error for SerialPortsError {}

pub fn midi_output_ports() -> Result<Vec<String>, MidiPortsError> {
    sted_midi::midi_output_ports().map_err(MidiPortsError::MidiOutput)
}

pub fn serial_ports() -> Result<Vec<String>, SerialPortsError> {
    sted_midi::serial_ports().map_err(SerialPortsError::SerialPort)
}
