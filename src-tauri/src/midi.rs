use midir::MidiOutputConnection;

const MIDI_INITIALIZE_ERROR_MESSAGE: &str = "MIDI 入出力機能を初期化できませんでした。";

pub fn midi_output() -> midir::MidiOutput {
    let midi_outputs =
        midir::MidiOutput::new("STed2-clone MIDI Output").expect(MIDI_INITIALIZE_ERROR_MESSAGE);
    midi_outputs
}

pub fn open_port(index: usize) -> MidiOutputConnection {
    let midi_output = midi_output();

    let selected_port = &midi_output.ports()[index];

    let device_name = midi_output
        .port_name(&selected_port)
        .expect("ポート名の取得に失敗しました。");

    println!("Connected to: {}", device_name);

    midi_output
        .connect(&selected_port, "Primary port")
        .expect(MIDI_INITIALIZE_ERROR_MESSAGE)
}

pub fn send_message(connect_out: &mut midir::MidiOutputConnection, message: Vec<u8>) {
    connect_out.send(&message).unwrap();
}
