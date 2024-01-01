use tauri::State;

use crate::state::{FileState, MidiOutputState};

use super::{playing_thread::playing_thread, PlayerState};

#[tauri::command]
pub fn play(
    file_state: State<'_, FileState>,
    midi_output_state: State<'_, MidiOutputState>,
    player_state: State<'_, PlayerState>,
) -> Result<(), String> {
    let smf = file_state.smf.lock().unwrap().take();
    let smf = match smf {
        Some(smf) => smf,
        None => return Err("ファイルが読み込まれていません。".to_string()),
    };

    let mut midi_output = midi_output_state.midi_output_connection.lock().unwrap();
    let mut midi_output = match midi_output.take() {
        Some(port) => port,
        None => return Err("MIDI 出力ポートが選択されていません。".to_string()),
    };

    let (sender, receiver) = std::sync::mpsc::channel();
    player_state.sender.lock().unwrap().replace(sender);

    std::thread::spawn(move || {
        playing_thread(receiver, smf, &mut midi_output).unwrap();
    });

    Ok(())
}
