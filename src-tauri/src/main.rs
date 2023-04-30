// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{fs::File, io::Read, thread, time};

mod midi;
use midir::MidiOutput;

use crate::midi::send_message;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn play() {
    // MIDI output open
    let midi_outputs = MidiOutput::new("hoge").unwrap();
    let midi_output = &midi_outputs.ports()[1];
    let mut connect_out = midi_outputs
        .connect(&midi_output, "Komplete Audio 6 MK2 MIDI")
        .unwrap();

    // File open
    let mut file =
        // File::open("C:\\Users\\mucho\\workspace\\sted2\\sted2-clone\\src-tauri\\test\\test.smf")
        File::open("C:\\Users\\mucho\\workspace\\sted2\\sted2-clone\\src-tauri\\test\\ALLSTARS.MID")
        // File::open("C:\\Users\\mucho\\workspace\\sted2\\sted2-clone\\src-tauri\\test\\midi602_format0.MID")
            .unwrap();
    let mut file_read_buffer = Vec::new();
    file.read_to_end(&mut file_read_buffer).unwrap();

    let header_chunk = &file_read_buffer[0..14];

    let song_delta_time = u32::from(header_chunk[12]) << 8 | u32::from(header_chunk[13]);
    let track_chunk = &file_read_buffer[14..];

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
            byte_0 & 0x7F << 7 | byte_1 & 0x7F
        } else if byte_2 & 0x80 == 0x00 {
            index += 3;
            byte_0 & 0x7F << 14 | byte_1 & 0x7F << 7 | byte_2 & 0x7F
        } else if byte_3 & 0x80 == 0x00 {
            index += 4;
            byte_0 & 0x7F << 21 | byte_1 & 0x7F << 14 | byte_2 & 0x7F << 7 | byte_3 & 0x7F
        } else {
            panic!();
        };

        println!("Delta time: {}", delta_time);

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
                println!("3 bytes message: {:X?}", &track_chunk[index..index + 4]);

                let message: &[u8] = &track_chunk[index..index + 3];
                send_message(&mut connect_out, message.to_vec());

                index += 3;
            }
            // 2 bytes message
            0xC0 | 0xD0 => {
                println!("2 bytes message: {:X?}", &track_chunk[index..index + 3]);

                let message = &track_chunk[index..index + 2];
                send_message(&mut connect_out, message.to_vec());

                index += 2;
            }
            0xF0 => {
                match &track_chunk[index] {
                    // System exclusive
                    0xF0 => {
                        let length = &track_chunk[index + 1];

                        let mut data: Vec<u8> = track_chunk
                            [index..index + 2 + usize::from(*length)]
                            .to_vec()
                            .clone();

                        data.remove(1);

                        send_message(&mut connect_out, data);

                        index += usize::from(*length) + 2
                    }

                    // Meta event
                    0xFF => {
                        println!("Meta event: {:X?}", &track_chunk[index..index + 10]);
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

                            println!("Tempo changed: {}", tempo);
                        } else if meta_event_type == 0x58 {
                            println!(
                                "Signature changed: {}, {}, {}, {}",
                                track_chunk[index + 2],
                                track_chunk[index + 3],
                                track_chunk[index + 4],
                                track_chunk[index + 5]
                            )
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

    connect_out.close();
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![play])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
