use tauri::State;

use crate::state::midi_output_state::{MidiOutputState, TrackStatus};

#[tauri::command]
pub fn get_play_status(
    midi_output_status: State<'_, MidiOutputState>,
) -> Result<[TrackStatus; 16], String> {
    Ok(midi_output_status
        .tracks
        .lock()
        .expect("Failed to lock")
        .clone())
}
