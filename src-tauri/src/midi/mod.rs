use midir::{ConnectError, MidiOutput, MidiOutputConnection};

const MIDI_INITIALIZE_ERROR_MESSAGE: &str = "MIDI 入出力機能を初期化できませんでした。";

pub fn midi_output() -> midir::MidiOutput {
    midir::MidiOutput::new("STed2 Clone MIDI Output").expect(MIDI_INITIALIZE_ERROR_MESSAGE)
}

pub fn open_port(index: usize) -> Result<MidiOutputConnection, ConnectError<MidiOutput>> {
    let midi_output = midi_output();

    let selected_port = &midi_output.ports()[index];

    midi_output.connect(&selected_port, "Primary port")
}

pub fn send_message(connect_out: &mut midir::MidiOutputConnection, message: &Vec<u8>) {
    connect_out.send(message).unwrap();
}
