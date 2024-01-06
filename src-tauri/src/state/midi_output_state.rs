use std::sync::{Arc, Mutex};

use serde::{Deserialize, Serialize};

pub struct MidiOutputState {
    pub tracks: Arc<Mutex<[TrackStatus; 16]>>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TrackStatus {
    pub channel: u8,
    pub volume: u8,
    pub expression: u8,
    pub pan: u8,
    pub reverb: u8,
    pub chorus: u8,
    pub cut_off_frequency: u8,
    pub resonance: u8,
    pub pitch_bend: u16,
    pub note_on_keys: Vec<u8>,
}

impl Default for TrackStatus {
    fn default() -> Self {
        Self {
            channel: 0,
            volume: 100,
            expression: 127,
            pan: 64,
            reverb: 0,
            chorus: 0,
            cut_off_frequency: 127,
            resonance: 0,
            pitch_bend: 8192,
            note_on_keys: vec![],
        }
    }
}
