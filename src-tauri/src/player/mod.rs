use std::sync::Mutex;

pub mod play;
pub mod playing_thread;

pub struct PlayerState {
    pub playing_thread: Mutex<Option<std::thread::JoinHandle<()>>>,
}
