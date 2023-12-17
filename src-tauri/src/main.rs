// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod file;
mod midi;
mod player;
mod state;

use file::load::load_file;
use midi::{midi_output, open_port};
use player::play;
use state::{FileBuffer, MidiOutput};
use tauri::{CustomMenuItem, Manager, Menu, MenuEntry, Submenu};
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

            *midi_output
                .midi_output_connection
                .lock()
                .expect("Mutex error") = Some(open_port(parsed));
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
