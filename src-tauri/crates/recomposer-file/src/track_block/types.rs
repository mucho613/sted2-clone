use crate::event::types::TrackEvent;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TrackBlock {
    pub tracks: Vec<Track>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Track {
    pub track_header: TrackHeader,
    pub track_events: Vec<TrackEvent>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TrackType {
    Normal,
    Rhythm,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TrackHeader {
    pub size: u16,
    pub track_number: u8,
    pub track_type: TrackType,
    pub channel: u8,
    pub key_bias: u8,
    pub step_bias: u8,
    pub muting: bool,
    pub comment: [u8; 36],
}
