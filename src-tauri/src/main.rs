// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod file;
mod menu;
mod midi;
mod player;
mod state;

use std::sync::Mutex;

use file::load::load_file;
use player::stop::stop;
use player::{play::play, PlayerState};
use state::{FileState, MidiOutputState};
use tauri::Menu;

fn main() {
    let midi_output_menu = menu::midi_output_menu();
    let menu = Menu::new().add_submenu(midi_output_menu);

    tauri::Builder::default()
        .menu(menu)
        .on_menu_event(|event| {
            menu::midi_output_menu_event(event);
        })
        .manage(FileState {
            file: Default::default(),
            smf: Mutex::new(None),
        })
        .manage(MidiOutputState {
            midi_output_connection: Default::default(),
        })
        .manage(PlayerState {
            playing_thread: Default::default(),
        })
        .invoke_handler(tauri::generate_handler![play, stop, load_file])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
