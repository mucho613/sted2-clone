use tauri::State;

use crate::state::{FileState, MidiConnectionState};

use super::{playing_thread::playing_thread, PlayerState};

#[tauri::command]
pub fn play(
    file_state: State<'_, FileState>,
    midi_output_state: State<'_, MidiConnectionState>,
    player_state: State<'_, PlayerState>,
) -> Result<(), String> {
    let smf = file_state.smf.lock().expect("Failed to lock smf").clone();
    let smf = match smf {
        Some(smf) => smf,
        None => return Err("ファイルが読み込まれていません。".to_string()),
    };

    let midi_output_port_index = midi_output_state
        .midi_output_port_index
        .lock()
        .expect("Failed to lock midi_output_connection")
        .clone();
    let midi_output_port_index = match midi_output_port_index {
        Some(midi_output_port_index) => midi_output_port_index,
        None => return Err("MIDIポートが選択されていません。".to_string()),
    };

    let (sender, receiver) = std::sync::mpsc::channel();
    player_state.sender.lock().unwrap().replace(sender);

    std::thread::spawn(move || {
        playing_thread(midi_output_port_index, receiver, smf).unwrap();
    });

    Ok(())
}
