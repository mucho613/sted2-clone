pub fn open_port() -> Result<midir::MidiOutputConnection, String> {
    let midi_outputs = midir::MidiOutput::new("hoge").unwrap();
    let midi_output = &midi_outputs.ports()[0];
    let connect_out = midi_outputs
        .connect(&midi_output, "Komplete Audio 6 MK2 MIDI")
        .unwrap();

    Ok(connect_out)
}

pub fn send_message(connect_out: &mut midir::MidiOutputConnection, message: Vec<u8>) {
    connect_out.send(&message).unwrap();
}
