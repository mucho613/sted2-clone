use std::sync::Arc;
use std::sync::Mutex;

use crate::song::Song;

pub struct SongState {
    pub song: Arc<Mutex<Option<Song>>>,
}
