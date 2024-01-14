use recomposer_file::track_block::types::{Track, TrackEvent};

use super::util::get_step_time_from_event;

pub struct PlayEvent {
    pub time: u32,
    pub port: u8,
    pub channel: u8,
    pub track_number: u8,
    pub event: TrackEvent,
}

pub fn merge_tracks(tracks: Vec<Track>) -> Vec<PlayEvent> {
    let mut merged = vec![];

    for (track_number, track) in tracks.iter().enumerate() {
        let mut time: u32 = 0;

        for event in track.track_events.iter() {
            let event = *event.clone();

            let step_time: u32 = get_step_time_from_event(&event).into();

            time += step_time;

            merged.push(PlayEvent {
                time,
                port: 0,    // Port A 固定
                channel: 0, // ひとまず Channel 1 固定
                track_number: 0,
                event,
            });
        }
    }

    merged
}
