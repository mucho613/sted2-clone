use std::sync::Mutex;

pub struct SequencerState<'a> {
    pub sender: Mutex<Option<std::sync::mpsc::Sender<&'a str>>>,
}
