// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod midi;
mod player

use player::play;
use std::{fs::File, io::Read, sync::Mutex};
use midi::{open_port, midi_output};
use midir::MidiOutputConnection;
use tauri::{CustomMenuItem, Mana::playger, Menu, MenuEntry, State, Submenu};
;
struct FileBuffer {
    file: Mutex<Vec<u8>>,
}

struct MidiOutput {
    midi_output_connection: Mutex<Option<MidiOutputConnection>>,
}

#[tauri::command]
fn load_file(file_path: String, file_buffer: State<'_, FileBuffer>) -> Result<(), String> {
    let mut file: File = File::open(file_path).expect("ファイルの読み込みに失敗しました。");
    let mut buffer: Vec<u8> = vec![];
    file.read_to_end(&mut buffer).unwrap();
    println!("{:?}", buffer);
    *file_buffer.file.lock().unwrap() = buffer;
    Ok(())
}

fn main() {
    // MIDI output open
    let midi_output = midi_output();

    let ports = midi_output.ports();
    let port_items: Vec<MenuEntry> = ports
        .iter()
        .enumerate()
        .map(|(index, port)| {
            let name = midi_output
                .port_name(port)
                .expect("Failed to get port name.");
            MenuEntry::CustomItem(CustomMenuItem::new(index.to_string(), &name))
        })
        .collect();

    let submenu = Submenu::new("MIDI Output", Menu::with_items(port_items));
    let menu = Menu::new().add_submenu(submenu);

    tauri::Builder::default()
        .menu(menu)
        .on_menu_event(|event| {
            let parsed = event
                .menu_item_id()
                .parse::<usize>()
                .expect("Failed to parse");

            let midi_output = event.window().state::<MidiOutput>();

            *midi_output.midi_output_connection.lock().expect("Mutex error") = Some(open_port(parsed));
        })
        .manage(FileBuffer {
            file: Default::default(),
        })
        .manage(MidiOutput {
            midi_output_connection: Default::default(),
        })
        .invoke_handler(tauri::generate_handler![play, load_file])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
