pub(super) mod play;
pub(super) mod play_status;
pub(super) mod play_status_thread;
pub(super) mod playing_thread;
pub(super) mod stop;
pub(super) mod util;

struct Player {
    song: Song,
}

impl Player {
    /// 再生開始する
    fn play(&self) {
        todo!();

        // spawn するなど
    }

    /// 再生を停止する
    fn stop(&self) {
        todo!();
    }

    /// 再生位置の変更
    fn seek(&self, position: u32) {
        todo!();
    }

    // song を更新する
    fn update_song(&mut self) {
        // self.song = song;
    }
}
