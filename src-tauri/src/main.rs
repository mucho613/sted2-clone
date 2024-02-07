// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod config;
mod file;
mod menu;
mod midi;
mod player;
mod song;
mod state;

use crate::file::load_file;
use menu::menu_event_handler;
use player::{play::play, play_status::get_play_status, stop::stop};
use std::{
    sync::{Arc, Mutex},
    vec,
};

use midi::{get_midi_output_ports, open_midi_output_port};
use state::{
    file_state::FileState, midi_connection_state::MidiConnectionState,
    midi_output_state::MidiOutputState, sequencer_state::SequencerState, song_state::SongState,
};
use tauri::Manager;

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let midi_output_connection = initialize();

            let menu = menu::menu();
            tauri::WindowBuilder::new(app, "main".to_string(), tauri::WindowUrl::App("/".into()))
                .menu(menu)
                .build()?;

            app.manage(FileState {
                rcpFile: Mutex::new(None),
            });
            app.manage(SongState {
                song: Arc::new(Mutex::new(None)),
            });
            app.manage(MidiConnectionState {
                midi_output_connection: if midi_output_connection.is_some() {
                    Arc::new(Mutex::new(Some(midi_output_connection.unwrap())))
                } else {
                    Arc::new(Mutex::new(None))
                },
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

fn initialize() -> Option<midir::MidiOutputConnection> {
    let config = config::load_config();

    // config に記載された MIDI デバイスを自動で開く
    let port_name = config.midi_output_port?;

    let midi_output = midi::midi_output();
    let ports = midi_output.ports();
    let selected_port = ports
        .iter()
        .position(|port| midi_output.port_name(port).unwrap() == port_name)?;

    let port = midi_output
        .connect(&ports[selected_port], "Primary port")
        .unwrap();

    Some(port)
}
