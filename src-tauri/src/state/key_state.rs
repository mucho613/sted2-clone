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
