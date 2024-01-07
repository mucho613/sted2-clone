use midir::{MidiOutput, MidiOutputPorts};
use serde::{Deserialize, Serialize};
use tauri::State;

use crate::state::midi_connection_state::MidiConnectionState;

const MIDI_INITIALIZE_ERROR_MESSAGE: &str = "MIDI 入出力機能を初期化できませんでした。";

#[derive(Debug, Deserialize, Serialize)]
pub struct MidiPort {
    id: String,
    name: String,
}

pub fn midi_output() -> midir::MidiOutput {
    MidiOutput::new("STed2 Clone MIDI Output").expect(MIDI_INITIALIZE_ERROR_MESSAGE)
}

#[tauri::command]
pub fn get_midi_output_ports() -> Result<Vec<MidiPort>, String> {
    let midi_output = midi_output();
    let mut midi_ports: Vec<MidiPort> = Vec::new();

    for (i, port) in midi_output.ports().iter().enumerate() {
        let port_name = midi_output.port_name(port).unwrap();
        midi_ports.push(MidiPort {
            id: i.to_string(),
            name: port_name,
        });
    }

    Ok(midi_ports)
}

#[tauri::command]
pub fn open_midi_output_port(
    id: String,
    midi_connection_state: State<'_, MidiConnectionState>,
) -> Result<(), String> {
    let midi_output = midi_output();
    let ports = midi_output.ports();
    let selected_port = ports.get(id.parse::<usize>().unwrap()).unwrap();
    let port = midi_output.connect(selected_port, "Primary port").unwrap();

    midi_connection_state
        .midi_output_connection
        .lock()
        .unwrap()
        .replace(port);

    Ok(())
}

pub fn send_message(connect_out: &mut midir::MidiOutputConnection, message: &Vec<u8>) {
    connect_out.send(message).unwrap();
}
