mod load;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct StandardMidiFile {
    header_chunk: HeaderChunk,
    track_chunk: Vec<TrackChunk>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HeaderChunk {
    format: u16,
    number_of_tracks: u16,
    time_base: u16,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TrackChunk {
    data_length: u32,
    data_body: Vec<Event>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Event {
    delta_time: u32,
    event_body: EventBody,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum EventBody {
    ChannelMessage(Vec<u8>),
    SystemExclusiveMessage(Vec<u8>),
    TempoChangeEvent(u32),
}
