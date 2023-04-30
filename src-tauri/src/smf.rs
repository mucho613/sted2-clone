use serde::Serialize;

mod smf_format;

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
    data_body: Vec<u8>,
}
