// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod file;
mod menu;
mod midi;
mod player;
mod state;

use std::sync::Mutex;

use file::load::load_file;
use menu::midi_output_menu_event;
use player::{play::play, play_status::get_play_status, stop::stop, SequencerState};
use state::{FileState, KeyStatus, MidiConnectionState};
use tauri::{Manager, Menu};

fn main() {
    let midi_output_menu = menu::midi_output_menu();
    let menu = Menu::new().add_submenu(midi_output_menu);

    tauri::Builder::default()
        .menu(menu)
        .on_menu_event(midi_output_menu_event)
        .setup(|app| {
            app.manage(FileState {
                file: Mutex::new(None),
                smf: Mutex::new(None),
            });
            app.manage(MidiConnectionState {
                midi_output_port_index: Mutex::new(None),
            });
            app.manage(SequencerState {
                sender: Mutex::new(None),
            });
            app.manage(KeyStatus {
                note_on_keys: Default::default(),
            });

            #[cfg(debug_assertions)]
            app.get_window("main").unwrap().open_devtools();

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            play,
            stop,
            load_file,
            get_play_status,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
