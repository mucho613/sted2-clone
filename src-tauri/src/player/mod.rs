use std::sync::Mutex;

pub mod play;
pub mod playing_thread;
pub mod stop;

pub struct PlayerState<'a> {
    pub sender: Mutex<Option<std::sync::mpsc::Sender<&'a str>>>,
}
