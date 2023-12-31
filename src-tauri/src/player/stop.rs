use tauri::State;

use crate::state::{FileState, MidiOutputState};

#[tauri::command]
pub fn stop(player_state: State<'_, super::PlayerState>) -> Result<(), String> {
    let mut playing_thread = player_state.playing_thread.lock().unwrap();
    let playing_thread = match playing_thread.take() {
        Some(playing_thread) => playing_thread,
        None => return Err("再生中のファイルがありません。".to_string()),
    };
    playing_thread.park

    Ok(())
}
