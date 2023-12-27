use super::{
    parse_header_chunk::parse_header_chunk, parse_track_chunk::parse_track_chunk, StandardMidiFile,
    TrackChunk,
};

pub fn load(file: &[u8]) -> Result<StandardMidiFile, String> {
    let (bytes, header_chunk) = match parse_header_chunk(file) {
        Ok(header_chunk) => header_chunk,
        Err(error) => return Err(error),
    };

    let mut track_chunks: Vec<TrackChunk> = vec![];
    let mut bytes = bytes;

    for i in 0..header_chunk.number_of_tracks {
        println!("Track {}", i);
        let (bytes_track_readed, track_chunk) = match parse_track_chunk(bytes) {
            Ok(track_chunk) => track_chunk,
            Err(error) => return Err(error),
        };
        bytes = bytes_track_readed;
        track_chunks.push(track_chunk);
    }

    Ok(StandardMidiFile {
        header_chunk,
        track_chunks,
    })
}
