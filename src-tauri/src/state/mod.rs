use std::sync::Mutex;

use midir::MidiOutputConnection;

pub struct FileBuffer {
    pub file: Mutex<Vec<u8>>,
}

pub struct MidiOutput {
    pub midi_output_connection: Mutex<Option<MidiOutputConnection>>,
}
