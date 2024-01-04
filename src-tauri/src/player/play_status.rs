use tauri::State;

use crate::state::key_state::KeyState;

#[tauri::command]
pub fn get_play_status(key_status: State<'_, KeyState>) -> Result<[Vec<u8>; 16], String> {
    Ok(key_status
        .note_on_keys
        .lock()
        .expect("Failed to lock")
        .clone())
}
