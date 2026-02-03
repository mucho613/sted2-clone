mod midi;
mod serial;

pub use midi::{
    MidiOutputConnection, MidiOutputError, SendError, midi_output_connection,
    midi_output_connection_by_name, midi_output_ports, send_midi,
};
pub use serial::{SerialPortError, send, serial_port, serial_ports};
