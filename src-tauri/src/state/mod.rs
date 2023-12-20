use std::sync::Mutex;

use midir::MidiOutputConnection;

use crate::song::song::Song;

pub struct FileState {
    pub file: Mutex<Vec<u8>>,
    pub song: Mutex<Option<Song>>,
}

pub struct MidiOutputState {
    pub midi_output_connection: Mutex<Option<MidiOutputConnection>>,
}
