mod event;
mod load;
mod parse_header_chunk;
mod parse_track_chunk;
mod variable_length_bytes;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct StandardMidiFile {
    header_chunk: HeaderChunk,
    track_chunk: Vec<TrackChunk>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct HeaderChunk {
    format: u16,
    number_of_tracks: u16,
    time_base: u16,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct TrackChunk {
    data_body: Vec<Event>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Event {
    delta_time: u32,
    event_body: EventBody,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum EventBody {
    ChannelMessage(Vec<u8>),
    SystemExclusiveMessage(Vec<u8>),
    TempoChangeEvent(u32),
    EndOfTrack,
}
