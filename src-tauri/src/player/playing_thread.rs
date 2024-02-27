use std::sync::{Arc, Mutex};

use crate::midi::send_message;
use crate::song::Song;
use recomposer_file::event::types::TrackEvent;
use recomposer_file::RcpFile;

use super::play_status_thread::PlayStatusMessage;
use super::util::get_step_time_from_event;

struct NoteOnKey {
    channel: u8,
    note: u8,
    remain_to_note_off: u32,
}

pub fn playing_thread(
    midi_output_connection: Arc<Mutex<Option<midir::MidiOutputConnection>>>,
    receiver: std::sync::mpsc::Receiver<&str>,
    play_status_sender: std::sync::mpsc::Sender<PlayStatusMessage>,
    song: Song,
) -> Result<(), String> {
    let mut midi_output = midi_output_connection
        .lock()
        .expect("Failed to lock midi_output_connection")
        .take()
        .expect("Failed to take midi_output_connection");

    let play_start_time = std::time::Instant::now();
    let mut delta_time_counter: u32 = 0;
    let current_tempo: u32 = 500000 / 1000; // Default BPM = 120

    // 絶対時間に変換する
    let tracks = song
        .tracks
        .iter()
        .map(|track| {
            let mut prev_delta_time = 0;
            let mut track_delta_time_counter = 0u32;
            let track_events = track
                .track_events
                .iter()
                .map(|event| {
                    let delta_time = get_step_time_from_event(event);
                    track_delta_time_counter += prev_delta_time;
                    prev_delta_time = delta_time as u32;

                    (track_delta_time_counter, event)
                })
                .collect::<Vec<(u32, &TrackEvent)>>();
            (track.track_header.channel, track_events)
        })
        .collect::<Vec<(u8, Vec<(u32, &TrackEvent)>)>>();

    let mut pointer = [0usize; 18];

    loop {
        let current_time = std::time::Instant::now();
        let difference = current_time.duration_since(play_start_time).as_millis() as u32;
        delta_time_counter = song.header_block.time_base as u32 * difference / current_tempo;

        match receiver.try_recv() {
            Ok("stop") => break,
            _ => (),
        }

        // 再生しなければいけないイベントを Vec に入れる
        let mut reserved: Vec<(u8, &TrackEvent)> = vec![];
        // 現在の再生位置を過ぎた場所にあるイベントを再生する
        for (track_index, (channel, events)) in tracks.iter().enumerate() {
            loop {
                let taken = events.get(pointer[track_index] as usize);
                let (absolute_time, event) = match taken {
                    Some((absolute_time, event)) => (*absolute_time, *event),
                    None => break,
                };

                if absolute_time <= delta_time_counter {
                    pointer[track_index] += 1;
                    reserved.push((*channel, event));
                } else {
                    break;
                }
            }
        }

        for (channel, event) in reserved {
            println!("{:?}", event);
            match *event {
                TrackEvent::Note {
                    key_number,
                    velocity,
                    ..
                } => {
                    play_status_sender
                        .send(PlayStatusMessage::NoteOn((channel, key_number)))
                        .unwrap();
                    send_message(
                        &mut midi_output,
                        &[0x90 | channel, key_number, velocity].to_vec(),
                    );
                    send_message(&mut midi_output, &[0x80 | channel, key_number, 0].to_vec());
                }
                _ => (),
            }
        }

        // 5ms ぐらい待つ
        std::thread::sleep(std::time::Duration::from_millis(5));
    }

    // TODO: レコンポーザフォーマットでの再生機能が整ったら、以下のコードを有効にする
    // // Note on 状態のキーをすべて Note off にする
    // for key in note_on_keys {
    //     play_status_sender
    //         .send(PlayStatusMessage::NoteOff((key.channel, key.note)))
    //         .unwrap();
    //     send_message(
    //         &mut midi_output,
    //         &[0x80 | key.channel, key.note, 0x00].to_vec(),
    //     );
    // }

    midi_output_connection
        .lock()
        .expect("Failed to lock midi_output_connection")
        .replace(midi_output);

    Ok(())
}
