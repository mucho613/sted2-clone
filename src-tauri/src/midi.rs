const MIDI_INITIALIZE_ERROR_MESSAGE: &str = "MIDI 入出力機能を初期化できませんでした。";

pub fn open_port() -> Result<midir::MidiOutputConnection, String> {
    let midi_outputs = midir::MidiOutput::new("STed2-clone client").expect(MIDI_INITIALIZE_ERROR_MESSAGE);
    let midi_output = &midi_outputs.ports()[0]; // TODO: ポートを選択可能にする
    let connect_out = midi_outputs
        .connect(&midi_output, "Primary port")
        .expect(MIDI_INITIALIZE_ERROR_MESSAGE);

    Ok(connect_out)
}

pub fn send_message(connect_out: &mut midir::MidiOutputConnection, message: Vec<u8>) {
    connect_out.send(&message).unwrap();
}
