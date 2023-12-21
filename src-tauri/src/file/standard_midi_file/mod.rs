mod load;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct StandardMidiFile {
    header_chunk: HeaderChunk,
    track_chunk: Vec<TrackChunk>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HeaderChunk {
    chunk_type: [u8; 4],
    data_length: u32,
    format: u16,
    number_of_tracks: u16,
    time_base: u16,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TrackChunk {
    chunk_type: [u8; 4],
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

impl HeaderChunk {
    fn validate_header_chunk(&self) -> bool {
        self.chunk_type[0] == b'M' && // 0x4D
        self.chunk_type[1] == b'T' && // 0x54
        self.chunk_type[2] == b'h' && // 0x68
        self.chunk_type[3] == b'd' // 0x64
    }
}

impl TrackChunk {
    fn validate_track_chunk(&self) -> bool {
        self.chunk_type[0] == b'M' && // 0x4D
        self.chunk_type[1] == b'T' && // 0x54
        self.chunk_type[2] == b'r' && // 0x72
        self.chunk_type[3] == b'k' // 0x6B
    }
}
