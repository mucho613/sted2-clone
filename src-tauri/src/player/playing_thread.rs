use std::sync::{Arc, Mutex};

use crate::midi::send_message;
use recomposer_file::track_block::types::{Track, TrackEvent};
use recomposer_file::RcpFile;

use super::play_status_thread::PlayStatusMessage;
use super::prepare::PlayEvent;

struct NoteOnKey {
    channel: u8,
    note: u8,
    remain_to_note_off: u32,
}

pub fn playing_thread(
    midi_output_connection: Arc<Mutex<Option<midir::MidiOutputConnection>>>,
    receiver: std::sync::mpsc::Receiver<&str>,
    song: Vec<PlayEvent>,
    time_base: u32,
    play_status_sender: std::sync::mpsc::Sender<PlayStatusMessage>,
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

    let mut note_on_keys: Vec<NoteOnKey> = Vec::new();

    for event in song.iter() {
        match receiver.try_recv() {
            Ok("stop") => break,
            _ => (),
        }

        // Wait for delta time
        loop {
            let now = std::time::Instant::now();

            let elapsed_time = now - last_tempo_changed_time;

            let elapsed_time = elapsed_time.as_millis();

            // Todo: Wait

            if elapsed_time >= u128::from(wait) {
                break;
            }

            std::thread::sleep(std::time::Duration::from_millis(5));
        }

        match event.event {
            TrackEvent::Note(key_number, _, gate_time, velocity) => {
                for key in &note_on_keys {
                    play_status_sender
                        .send(PlayStatusMessage::NoteOff((key.channel, key.note)))
                        .unwrap();
                    send_message(
                        &mut midi_output,
                        &[0x80 | key.channel, key.note, 0x00].to_vec(),
                    );
                }
                note_on_keys.clear();

                play_status_sender
                    .send(PlayStatusMessage::NoteOn((0, key_number)))
                    .unwrap();
                send_message(
                    &mut midi_output,
                    &[0x90 | 0 /* channel */, key_number, velocity].to_vec(),
                );

                note_on_keys.push(NoteOnKey {
                    channel: 0,
                    note: key_number,
                    remain_to_note_off: u32::from(gate_time),
                });
            }
            _ => (),
        }

        println!("Event!: {}", delta_time_counter);

        delta_time_counter += u32::from(step_time);
    }

    // Note on 状態のキーをすべて Note off にする
    for key in note_on_keys {
        play_status_sender
            .send(PlayStatusMessage::NoteOff((key.channel, key.note)))
            .unwrap();
        send_message(
            &mut midi_output,
            &[0x80 | key.channel, key.note, 0x00].to_vec(),
        );
    }

    midi_output_connection
        .lock()
        .expect("Failed to lock midi_output_connection")
        .replace(midi_output);

    Ok(())
}
