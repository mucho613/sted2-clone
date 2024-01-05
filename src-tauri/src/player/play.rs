use std::sync::Arc;

use tauri::State;

use crate::{
    midi::{midi_output, send_message},
    state::{
        file_state::FileState,
        key_state::KeyState,
        midi_connection_state::{self, MidiConnectionState},
        sequencer_state::SequencerState,
    },
};

use super::{play_status_thread::play_status_thread, playing_thread::playing_thread};

#[tauri::command]
pub fn play(
    file_state: State<'_, FileState>,
    midi_output_state: State<'_, MidiConnectionState>,
    player_state: State<'_, SequencerState>,
    key_state: State<'_, KeyState>,
) -> Result<(), String> {
    let smf = file_state.smf.lock().expect("Failed to lock smf").clone();
    let smf = match smf {
        Some(smf) => smf,
        None => return Err("ファイルが読み込まれていません。".to_string()),
    };

    let midi_output_connection = Arc::clone(&midi_output_state.midi_output_connection);
    if midi_output_connection.lock().unwrap().is_none() {
        return Err("MIDI 出力ポートが選択されていません。".to_string());
    }

    let (sender, receiver) = std::sync::mpsc::channel();
    let (play_status_sender, play_status_receiver) = std::sync::mpsc::channel();
    player_state.sender.lock().unwrap().replace(sender);
    // MIDI メッセージの送信を行うスレッド
    std::thread::spawn(move || {
        playing_thread(midi_output_connection, receiver, smf, play_status_sender).unwrap();
    });

    let key_state = Arc::clone(&key_state.note_on_keys);
    // 演奏モニタの状態管理を行うスレッド
    std::thread::spawn(move || {
        play_status_thread(play_status_receiver, key_state).unwrap();
    });

    Ok(())
}
