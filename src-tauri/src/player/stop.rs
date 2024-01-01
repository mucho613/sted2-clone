use tauri::State;

use super::PlayerState;

#[tauri::command]
pub fn stop(player_state: State<'_, PlayerState>) -> Result<(), String> {
    let sender = player_state.sender.lock().unwrap().take();
    let sender = match sender {
        Some(sender) => sender,
        None => return Err("Player is not running.".to_string()),
    };

    match sender.send("stop") {
        Ok(_) => Ok(()),
        Err(_) => Err("Failed to send stop signal.".to_string()),
    }
}
