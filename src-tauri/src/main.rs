// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod file;
mod menu;
mod midi;
mod player;
mod state;

use std::{sync::Mutex, vec};

use file::load::load_file;
use menu::menu_event_handler;
use player::{play::play, play_status::get_play_status, stop::stop};

use midi::{get_midi_output_ports, open_midi_output_port};
use state::{
    file_state::FileState, midi_connection_state::MidiConnectionState,
    midi_output_state::MidiOutputState, sequencer_state::SequencerState,
};
use tauri::Manager;

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let menu = menu::menu();

            tauri::WindowBuilder::new(app, "main".to_string(), tauri::WindowUrl::App("/".into()))
                .menu(menu)
                .build()?;

            app.manage(FileState {
                file: Mutex::new(None),
                smf: Mutex::new(None),
            });
            app.manage(MidiConnectionState {
                midi_output_connection: Default::default(),
            });
            app.manage(SequencerState {
                sender: Mutex::new(None),
            });
            app.manage(MidiOutputState {
                tracks: Default::default(),
            });

            Ok(())
        })
        .on_menu_event(menu_event_handler)
        .invoke_handler(tauri::generate_handler![
            play,
            stop,
            load_file,
            get_play_status,
            get_midi_output_ports,
            open_midi_output_port,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
