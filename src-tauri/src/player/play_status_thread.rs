use std::sync::{mpsc::Receiver, Arc, Mutex};

use crate::state::midi_output_state::TrackStatus;

pub enum PlayStatusMessage {
    NoteOn((u8, u8)),
    NoteOff((u8, u8)),
    VolumeChange((u8, u8)),
    ExpressionChange((u8, u8)),
    PanChange((u8, u8)),
    ReverbChange((u8, u8)),
    ChorusChange((u8, u8)),
    CutOffFrequencyChange((u8, u8)),
    ResonanceChange((u8, u8)),
}

pub fn play_status_thread(
    receiver: Receiver<PlayStatusMessage>,
    tracks: Arc<Mutex<[TrackStatus; 16]>>,
) -> Result<(), String> {
    loop {
        match receiver.recv() {
            Ok(PlayStatusMessage::NoteOn((channel, note))) => {
                let mut tracks = tracks.lock().expect("Failed to lock key_state");
                tracks[channel as usize].note_on_keys.push(note);
            }
            Ok(PlayStatusMessage::NoteOff((channel, note))) => {
                let mut tracks = tracks.lock().expect("Failed to lock key_state");
                tracks[channel as usize].note_on_keys.retain(|&k| k != note);
            }
            Ok(PlayStatusMessage::VolumeChange((channel, volume))) => {
                let mut tracks = tracks.lock().expect("Failed to lock key_state");
                tracks[channel as usize].volume = volume;
            }
            Ok(PlayStatusMessage::ExpressionChange((channel, expression))) => {
                let mut tracks = tracks.lock().expect("Failed to lock key_state");
                tracks[channel as usize].expression = expression;
            }
            Ok(PlayStatusMessage::PanChange((channel, pan))) => {
                let mut tracks = tracks.lock().expect("Failed to lock key_state");
                tracks[channel as usize].pan = pan;
            }
            Ok(PlayStatusMessage::ReverbChange((channel, reverb))) => {
                let mut tracks = tracks.lock().expect("Failed to lock key_state");
                tracks[channel as usize].reverb = reverb;
            }
            Ok(PlayStatusMessage::ChorusChange((channel, chorus))) => {
                let mut tracks = tracks.lock().expect("Failed to lock key_state");
                tracks[channel as usize].chorus = chorus;
            }
            Ok(PlayStatusMessage::CutOffFrequencyChange((channel, cut_off_frequency))) => {
                let mut tracks = tracks.lock().expect("Failed to lock key_state");
                tracks[channel as usize].cut_off_frequency = cut_off_frequency;
            }
            Ok(PlayStatusMessage::ResonanceChange((channel, resonance))) => {
                let mut tracks = tracks.lock().expect("Failed to lock key_state");
                tracks[channel as usize].resonance = resonance;
            }
            Err(_) => break,
        }
    }
    Ok(())
}
