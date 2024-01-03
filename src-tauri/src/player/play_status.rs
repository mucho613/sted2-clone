use tauri::State;

use crate::state::key_state::KeyStatus;

#[tauri::command]
pub fn get_play_status(key_status: State<'_, KeyStatus>) -> Result<[Vec<u8>; 16], String> {
    Ok(key_status.playing_notes())
}
