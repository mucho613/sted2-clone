// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{fs::File, io::Read, sync::Mutex, thread, time};
mod midi;
use crate::midi::{open_port, send_message};
use tauri::State;

struct FileBuffer {
    file: Mutex<Vec<u8>>,
}

#[tauri::command]
async fn play(
    file_buffer: State<'_, FileBuffer>,
) -> Result<(), String> {
    // MIDI output open
    let mut midi_output = open_port().unwrap();

    let file_buffer = file_buffer.file.lock().unwrap();

    let header_chunk = &file_buffer[0..14];

    let song_delta_time = u32::from(header_chunk[12]) << 8 | u32::from(header_chunk[13]);
    let track_chunk = &file_buffer[14..];

    let mut index = 8;

    let play_start_time = time::Instant::now();
    let mut delta_time_counter: u32 = 0;
    let mut last_tempo_changed_time = play_start_time;
    let mut last_tempo_changed_delta_time = delta_time_counter;
    let mut tempo: u32 = 500000; // Default BPM = 120

    while index < track_chunk.len() {
        let byte_0 = u32::from(track_chunk[index]);
        let byte_1 = u32::from(track_chunk[index + 1]);
        let byte_2 = u32::from(track_chunk[index + 2]);
        let byte_3 = u32::from(track_chunk[index + 3]);

        let delta_time = if byte_0 & 0x80 == 0x00 {
            index += 1;
            byte_0
        } else if byte_1 & 0x80 == 0x00 {
            index += 2;
            (byte_0 & 0x7F) << 7 | (byte_1 & 0x7F)
        } else if byte_2 & 0x80 == 0x00 {
            index += 3;
            (byte_0 & 0x7F) << 14 | (byte_1 & 0x7F) << 7 | (byte_2 & 0x7F)
        } else if byte_3 & 0x80 == 0x00 {
            index += 4;
            (byte_0 & 0x7F) << 21 | (byte_1 & 0x7F) << 14 | (byte_2 & 0x7F) << 7 | (byte_3 & 0x7F)
        } else {
            panic!("Parsing variablel-length quantity failed.");
        };

        // println!("Delta time: {}", delta_time);

        loop {
            let now = time::Instant::now();

            let elapsed_time = now - last_tempo_changed_time;

            let elapsed_time = elapsed_time.as_millis();

            let wait = (tempo / 1000)
                * (delta_time_counter - last_tempo_changed_delta_time + delta_time)
                / song_delta_time;

            if elapsed_time >= u128::from(wait) {
                break;
            }

            thread::sleep(time::Duration::from_millis(1));
        }

        delta_time_counter += delta_time;

        // Event type
        match track_chunk[index] & 0xF0 {
            // 3 bytes message
            0x80 | 0x90 | 0xA0 | 0xB0 | 0xE0 => {
                // println!("3 bytes message: {:02X?}", &track_chunk[index..index + 3]);

                let message: &[u8] = &track_chunk[index..index + 3];
                send_message(&mut midi_output, message.to_vec());

                index += 3;
            }
            // 2 bytes message
            0xC0 | 0xD0 => {
                // println!("2 bytes message: {:02X?}", &track_chunk[index..index + 2]);

                let message = &track_chunk[index..index + 2];
                send_message(&mut midi_output, message.to_vec());

                index += 2;
            }
            0xF0 => {
                match &track_chunk[index] {
                    // System exclusive
                    0xF0 => {
                        // println!("System exclusive");
                        let length = &track_chunk[index + 1];

                        let mut data: Vec<u8> = track_chunk
                            [index..index + 2 + usize::from(*length)]
                            .to_vec()
                            .clone();

                        data.remove(1);

                        send_message(&mut midi_output, data);

                        index += usize::from(*length) + 2
                    }

                    // Meta event
                    0xFF => {
                        // println!("Meta event");
                        index += 1;

                        let meta_event_type = track_chunk[index];
                        if meta_event_type == 0x2F {
                            // End of track
                            break;
                        } else if meta_event_type == 0x51 {
                            tempo = u32::from(track_chunk[index + 2]) << 16
                                | u32::from(track_chunk[index + 3]) << 8
                                | u32::from(track_chunk[index + 4]);

                            last_tempo_changed_time = time::Instant::now();
                            last_tempo_changed_delta_time = delta_time_counter;

                            // println!("Tempo changed: {}", tempo);
                        } else if meta_event_type == 0x58 {
                            // println!(
                            //     "Signature changed: {}, {}, {}, {}",
                            //     track_chunk[index + 2],
                            //     track_chunk[index + 3],
                            //     track_chunk[index + 4],
                            //     track_chunk[index + 5]
                            // )
                        }
                        index += 1;

                        let length = &track_chunk[index];

                        index += usize::from(*length) + 1;
                    }

                    _ => println!("Unknown event - {}", &track_chunk[index]),
                }
            }
            _ => panic!("Unknown event - {:?}", &track_chunk[index..index + 5]),
        }
    }

    midi_output.close();

    Ok(())
}

#[tauri::command]
fn load_file(file_path: String, file_buffer: State<'_, FileBuffer>) -> Result<(), String> {
    let mut file = File::open(file_path).expect("ファイルの読み込みに失敗しました。");
    let mut buffer: Vec<u8> = vec![];
    file.read_to_end(&mut buffer).unwrap();
    *file_buffer.file.lock().unwrap() = buffer;
    Ok(())
}

fn main() {
    tauri::Builder::default()
        .manage(FileBuffer {
            file: Default::default(),
        })
        .invoke_handler(tauri::generate_handler![play, load_file])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
