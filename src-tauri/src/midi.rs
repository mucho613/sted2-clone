use midir::MidiOutputConnection;

pub fn send_message(connect_out: &mut MidiOutputConnection, message: Vec<u8>) {
    connect_out.send(&message).unwrap();
}
