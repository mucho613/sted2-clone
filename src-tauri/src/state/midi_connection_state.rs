use std::sync::Mutex;

pub struct MidiConnectionState {
    pub midi_output_port_index: Mutex<Option<usize>>,
}
