use tauri::State;

use crate::file::standard_midi_file::{EventBody, MetaEvent};
use crate::midi::send_message;
use crate::state::{FileState, MidiOutputState};

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

    let play_start_time = std::time::Instant::now();
    let mut delta_time_counter: u32 = 0;
    let mut last_tempo_changed_time = play_start_time;
    let mut last_tempo_changed_delta_time = delta_time_counter;
    let mut current_tempo: u32 = 500000; // Default BPM = 120

    let time_base = u32::from(smf.header_chunk.time_base);

    let track = &smf.track_chunks[0];

    for event in track.data_body.iter() {
        loop {
            let now = std::time::Instant::now();

            let elapsed_time = now - last_tempo_changed_time;

            let elapsed_time = elapsed_time.as_millis();

            let wait = (current_tempo / 1000)
                * (delta_time_counter - last_tempo_changed_delta_time + event.delta_time)
                / time_base;

            if elapsed_time >= u128::from(wait) {
                break;
            }

            std::thread::sleep(std::time::Duration::from_millis(1));
        }

        delta_time_counter += event.delta_time;

        // Event type
        match &event.event_body {
            // Channel message
            EventBody::ChannelMessage(message) => {
                send_message(&mut midi_output, &message);
            }
            EventBody::SystemExclusiveMessage(message) => send_message(&mut midi_output, message),
            EventBody::MetaEvent(MetaEvent::TempoChangeEvent(tempo)) => {
                current_tempo = *tempo;
                last_tempo_changed_time = std::time::Instant::now();
                last_tempo_changed_delta_time = delta_time_counter;
            }
            EventBody::MetaEvent(MetaEvent::EndOfTrack) => (),
            _ => (),
        }
    }

    Ok(())
}
