use std::sync::{mpsc::Receiver, Arc, Mutex};

pub enum PlayStatusMessage {
    NoteOn((u8, u8)),
    NoteOff((u8, u8)),
}

pub fn play_status_thread(
    receiver: Receiver<PlayStatusMessage>,
    key_state: Arc<Mutex<[Vec<u8>; 16]>>,
) -> Result<(), String> {
    loop {
        match receiver.recv() {
            Ok(PlayStatusMessage::NoteOn((channel, note))) => {
                let mut note_on_keys = key_state.lock().expect("Failed to lock key_state");
                note_on_keys[channel as usize].push(note);
            }
            Ok(PlayStatusMessage::NoteOff((channel, note))) => {
                let mut note_on_keys = key_state.lock().expect("Failed to lock key_state");
                note_on_keys[channel as usize].retain(|&k| k != note);
            }
            Err(_) => break,
        }
    }
    Ok(())
}
