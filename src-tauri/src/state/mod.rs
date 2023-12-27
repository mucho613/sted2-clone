use std::sync::Mutex;

use midir::MidiOutputConnection;

use crate::file::standard_midi_file::StandardMidiFile;

pub struct FileState {
    pub file: Mutex<Vec<u8>>,
    pub smf: Mutex<Option<StandardMidiFile>>,
}

pub struct MidiOutputState {
    pub midi_output_connection: Mutex<Option<MidiOutputConnection>>,
}
