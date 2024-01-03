use std::sync::Mutex;

use crate::file::standard_midi_file::StandardMidiFile;

pub struct FileState {
    pub file: Mutex<Option<Vec<u8>>>,
    pub smf: Mutex<Option<StandardMidiFile>>,
}
