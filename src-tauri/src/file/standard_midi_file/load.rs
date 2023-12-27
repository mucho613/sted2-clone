use super::{
    parse_header_chunk::parse_header_chunk, Event, EventBody, StandardMidiFile, TrackChunk,
};

pub fn load(file: &[u8]) -> Result<StandardMidiFile, String> {
    let header_chunk_bytes = &file[0..14];
    let header_chunk = match parse_header_chunk(header_chunk_bytes) {
        Ok(header_chunk) => header_chunk,
        Err(error) => return Err(error),
    };

    let track_chunk_bytes = &file[14..];

    // let track_chunks = track_chunk_bytes_divided
    //     .iter();

    Ok(StandardMidiFile {
        header_chunk,
        track_chunk: vec![],
    })
}
