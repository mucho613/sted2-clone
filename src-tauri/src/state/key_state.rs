use std::sync::{Arc, Mutex};

pub struct KeyState {
    pub note_on_keys: Arc<Mutex<[Vec<u8>; 16]>>,
}

// impl KeyState {
//     pub fn playing_notes(&self) -> [Vec<u8>; 16] {
//         self.note_on_keys.lock().expect("Failed to lock").clone()
//     }

//     pub fn note_on(&mut self, channel: usize, key: u8) {
//         self.note_on_keys[channel].push(key);
//     }

//     pub fn note_off(&mut self, channel: usize, key: u8) {
//         self.note_on_keys[channel].retain(|&k| k != key);
//     }
// }
