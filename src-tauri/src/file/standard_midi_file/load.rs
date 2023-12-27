use super::{
    parse_header_chunk::parse_header_chunk, parse_track_chunk::parse_track_chunk, StandardMidiFile,
};

pub fn load(file: &[u8]) -> Result<StandardMidiFile, String> {
    let header_chunk_bytes = &file[0..14];
    let header_chunk = match parse_header_chunk(header_chunk_bytes) {
        Ok(header_chunk) => header_chunk,
        Err(error) => return Err(error),
    };

    let track_chunk = match parse_track_chunk(&file[14..]) {
        Ok(track_chunk) => track_chunk,
        Err(error) => return Err(error),
    };

    Ok(StandardMidiFile {
        header_chunk,
        track_chunk: vec![track_chunk.1],
    })
}
