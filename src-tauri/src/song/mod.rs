pub mod from_rcp_file;

pub struct Song {
    name: String,
    tracks: Vec<Track>,
}

pub struct Track {
    events: Vec<recomposer_file::event::types::TrackEvent>,
    measures: Vec<Measure>,
    structure: Structure,
}

pub struct Measure {
    events: Vec<recomposer_file::event::types::TrackEvent>,
}

pub struct Structure {
    events: Vec<Event>,
}

pub struct Event {
    absolute_time: u64,
    event: recomposer_file::event::types::TrackEvent,
}
