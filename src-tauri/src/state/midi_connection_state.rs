use std::sync::{Arc, Mutex};

use midir::MidiOutputConnection;

pub struct MidiConnectionState {
    pub midi_output_connection: Arc<Mutex<Option<MidiOutputConnection>>>,
}
