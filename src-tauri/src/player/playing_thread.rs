use std::sync::{Arc, Mutex};

use crate::file::standard_midi_file::{EventBody, MetaEvent, StandardMidiFile};
use crate::midi::send_message;

pub fn playing_thread(
    midi_output_connection: Arc<Mutex<Option<midir::MidiOutputConnection>>>,
    receiver: std::sync::mpsc::Receiver<&str>,
    smf: StandardMidiFile,
) -> Result<(), String> {
    let mut midi_output = midi_output_connection
        .lock()
        .expect("Failed to lock midi_output_connection")
        .take()
        .expect("Failed to take midi_output_connection");

    let play_start_time = std::time::Instant::now();
    let mut delta_time_counter: u32 = 0;
    let mut last_tempo_changed_time = play_start_time;
    let mut last_tempo_changed_delta_time = delta_time_counter;
    let mut current_tempo: u32 = 500000; // Default BPM = 120

    let time_base = u32::from(smf.header_chunk.time_base);

    let track = &smf.track_chunks[0];

    for event in track.data_body.iter() {
        match receiver.try_recv() {
            Ok("stop") => break,
            _ => (),
        }

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

            // std::thread::sleep(std::time::Duration::from_millis(1));
        }

        delta_time_counter += event.delta_time;

        // Event type
        match &event.event_body {
            EventBody::ChannelMessage(message) => send_message(&mut midi_output, &message),
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

    midi_output_connection
        .lock()
        .expect("Failed to lock midi_output_connection")
        .replace(midi_output);

    Ok(())
}
