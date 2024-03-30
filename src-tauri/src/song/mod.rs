pub mod from_rcp_file;

#[derive(Debug)]
pub struct Song {
    name: String,
    tracks: Vec<Track>,
}

#[derive(Debug)]
pub struct Track {
    measures: Vec<Measure>,
}

#[derive(Debug)]
pub struct Measure {
    step_time: u32,
    events: Vec<recomposer_file::event::types::TrackEvent>,
}
