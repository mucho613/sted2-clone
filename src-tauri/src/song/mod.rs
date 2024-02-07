pub struct Song {
    name: String,
    tracks: Vec<Track>,
}

pub struct Track {
    events: Vec<recomposer_file::event::types::TrackEvent>,
}
