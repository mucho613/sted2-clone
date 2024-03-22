pub mod from_rcp_file;

#[derive(Debug)]
pub struct Song {
    name: String,
    tracks: Vec<Track>,
}

#[derive(Debug)]
pub struct Track {
    events: Vec<recomposer_file::event::types::TrackEvent>,
    measures: Vec<Measure>,
}

#[derive(Debug)]
pub struct Measure {
    events: Vec<recomposer_file::event::types::TrackEvent>,
}
