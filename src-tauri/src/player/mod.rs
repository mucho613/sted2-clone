use std::sync::Mutex;

pub mod play;
pub mod play_status;
pub mod playing_thread;
pub mod stop;

pub struct SequencerState<'a> {
    pub sender: Mutex<Option<std::sync::mpsc::Sender<&'a str>>>,
}
