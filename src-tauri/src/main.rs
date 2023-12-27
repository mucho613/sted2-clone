// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod file;
mod midi;
mod player;
mod state;

use std::sync::Mutex;

use file::load::load_file;
use midi::{midi_output, open_port};
use player::play;
use state::{FileState, MidiOutputState};
use tauri::{CustomMenuItem, Manager, Menu, MenuEntry, Submenu};

fn main() {
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

            let midi_output_state = event.window().state::<MidiOutputState>();

            let port = open_port(parsed).unwrap();

            *midi_output_state
                .midi_output_connection
                .lock()
                .expect("Mutex error") = Some(port);
        })
        .manage(FileState {
            file: Default::default(),
            smf: Mutex::new(None),
        })
        .manage(MidiOutputState {
            midi_output_connection: Default::default(),
        })
        .invoke_handler(tauri::generate_handler![play, load_file])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
