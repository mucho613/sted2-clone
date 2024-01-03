use std::sync::Mutex;

use crate::file::standard_midi_file::StandardMidiFile;

pub struct FileState {
    pub file: Mutex<Option<Vec<u8>>>,
    pub smf: Mutex<Option<StandardMidiFile>>,
}

pub struct MidiConnectionState {
    pub midi_output_port_index: Mutex<Option<usize>>,
}

pub struct KeyStatus {
    pub note_on_keys: [Vec<u8>; 16],
}

impl KeyStatus {
    pub fn playing_notes(&self) -> [Vec<u8>; 16] {
        self.note_on_keys.clone()
    }

    pub fn note_on(&mut self, channel: usize, key: u8) {
        self.note_on_keys[channel].push(key);
    }

    pub fn note_off(&mut self, channel: usize, key: u8) {
        self.note_on_keys[channel].retain(|&k| k != key);
    }
}
