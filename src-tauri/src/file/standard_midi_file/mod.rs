mod event;
pub mod load;
mod parse_header_chunk;
mod parse_track_chunk;
mod variable_length_bytes;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct StandardMidiFile {
    pub header_chunk: HeaderChunk,
    pub track_chunks: Vec<TrackChunk>,
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
    MetaEvent(MetaEvent),
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum MetaEvent {
    SequenceNumber(u16),
    TextEvent(String),
    CopyRightNotice(String),
    TrackName(String),
    InstrumentName(String),
    Lyric(String),
    Marker(String),
    CuePoint(String),
    ProgramName(String),
    DeviceName(String),
    MidiChannelPrefix(u8),
    MidiPort(u8),
    EndOfTrack,
    TempoChangeEvent(u32),
    SmpteOffset(u8, u8, u8, u8, u8),
    TimeSignature(u8, u8, u8, u8),
    KeySignature(i8, u8),
    SequencerSpecificEvent(Vec<u8>),
}
