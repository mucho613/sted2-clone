use crate::file::load::load_file;
use crate::midi::{midi_output, open_port};
use crate::state::MidiOutputState;
use tauri::{CustomMenuItem, Manager, Menu, MenuEntry, Submenu};

pub fn midi_output_menu() -> Submenu {
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

    Submenu::new("MIDI Output", Menu::with_items(port_items))
}

pub fn midi_output_menu_event(event: tauri::WindowMenuEvent) {
    let parsed = event
        .menu_item_id()
        .parse::<usize>()
        .expect("Failed to parse");

    let midi_output_state = event.window().state::<MidiOutputState>();

    let port = match open_port(parsed) {
        Ok(port) => port,
        Err(error) => {
            println!("{}", error);
            return;
        }
    };

    *midi_output_state
        .midi_output_connection
        .lock()
        .expect("Mutex error") = Some(port);
}
