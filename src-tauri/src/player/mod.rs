use tauri::State;
mod playing_thread;
use crate::file::standard_midi_file::{EventBody, MetaEvent};
use crate::midi::send_message;
use crate::state::{FileState, MidiOutputState};
use playing_thread::playing_thread;

#[tauri::command]
pub fn play(
    file_state: State<'_, FileState>,
    midi_output_state: State<'_, MidiOutputState>,
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

    std::thread::spawn(move || {
        playing_thread(smf, &mut midi_output).unwrap();
    });

    Ok(())
}
