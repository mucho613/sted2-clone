use midir::MidiOutputConnection;
use tauri::State;

use crate::file::standard_midi_file::{EventBody, MetaEvent, StandardMidiFile};
use crate::midi::send_message;
use crate::state::{FileState, MidiOutputState};

pub fn playing_thread(
    smf: StandardMidiFile,
    midi_output: &mut MidiOutputConnection,
) -> Result<(), String> {
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
            EventBody::ChannelMessage(message) => send_message(midi_output, &message),
            EventBody::SystemExclusiveMessage(message) => send_message(midi_output, message),
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
