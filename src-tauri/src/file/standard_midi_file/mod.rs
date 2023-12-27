mod event;
pub mod load;
mod parse_header_chunk;
mod parse_track_chunk;
mod variable_length_bytes;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct StandardMidiFile {
    pub header_chunk: HeaderChunk,
    pub track_chunk: Vec<TrackChunk>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct HeaderChunk {
    pub format: u16,
    pub number_of_tracks: u16,
    pub time_base: u16,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct TrackChunk {
    pub data_body: Vec<Event>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Event {
    pub delta_time: u32,
    pub event_body: EventBody,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum EventBody {
    ChannelMessage(Vec<u8>),
    SystemExclusiveMessage(Vec<u8>),
    TempoChangeEvent(u32),
    NoImplementEvent,
    EndOfTrack,
}
