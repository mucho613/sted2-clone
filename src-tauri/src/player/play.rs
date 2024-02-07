use std::sync::Arc;

use tauri::State;

use crate::state::{
    file_state::FileState, midi_connection_state::MidiConnectionState,
    midi_output_state::MidiOutputState, sequencer_state::SequencerState,
};

use super::{play_status_thread::play_status_thread, playing_thread::playing_thread};

#[tauri::command]
pub fn play(
    file_state: State<'_, FileState>,
    midi_connection_state: State<'_, MidiConnectionState>,
    player_state: State<'_, SequencerState>,
    midi_output_state: State<'_, MidiOutputState>,
) -> Result<(), String> {
    let song = file_state
        .rcpFile
        .lock()
        .expect("Failed to lock smf")
        .take();
    let song = match song {
        Some(smf) => smf,
        None => return Err("ファイルが読み込まれていません。".to_string()),
    };

    let midi_output_connection = Arc::clone(&midi_connection_state.midi_output_connection);
    if midi_output_connection.lock().unwrap().is_none() {
        return Err("MIDI 出力ポートが選択されていません。".to_string());
    }

    let (player_control_sender, player_control_receiver) = std::sync::mpsc::channel();
    let (play_status_sender, play_status_receiver) = std::sync::mpsc::channel();
    player_state
        .sender
        .lock()
        .unwrap()
        .replace(player_control_sender);
    // MIDI メッセージの送信を行うスレッド
    std::thread::spawn(move || {
        playing_thread(
            midi_output_connection,
            player_control_receiver,
            play_status_sender,
            song,
        )
        .unwrap();
    });

    let tracks = Arc::clone(&midi_output_state.tracks);
    // 演奏モニタの状態管理を行うスレッド
    std::thread::spawn(move || {
        play_status_thread(play_status_receiver, tracks).unwrap();
    });

    Ok(())
}
