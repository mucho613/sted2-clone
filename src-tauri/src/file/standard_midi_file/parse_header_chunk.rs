use super::{Event, EventBody, HeaderChunk, StandardMidiFile, TrackChunk};

use nom::error::Error;
use nom::{
    bytes::streaming::{tag, take},
    number::streaming::be_u16,
};

pub fn parse_header_chunk(bytes: &[u8]) -> Result<HeaderChunk, String> {
    let (bytes, _) = tag::<&str, &[u8], Error<_>>("MThd")(bytes).expect("\"MThd\" not found.");
    let (bytes, _) = take::<u8, &[u8], Error<_>>(4u8)(bytes).expect("Failed to read data length.");
    let (bytes, format) = take::<u8, &[u8], Error<_>>(2u8)(bytes).expect("Failed to read format");
    let (bytes, number_of_tracks) =
        take::<u8, &[u8], Error<_>>(2u8)(bytes).expect("Failed to read number of tracks");
    let (bytes, time_base) =
        take::<u8, &[u8], Error<_>>(2u8)(bytes).expect("Failed to read time base");

    let format = be_u16::<&[u8], Error<_>>(format)
        .expect("Failed to parse")
        .1;
    let number_of_tracks = be_u16::<&[u8], Error<_>>(number_of_tracks)
        .expect("Failed to parse")
        .1;
    let time_base = be_u16::<&[u8], Error<_>>(time_base)
        .expect("Failed to parse")
        .1;

    Ok(HeaderChunk {
        format,
        number_of_tracks,
        time_base,
    })
}

#[test]
fn parse_header_chunk_test() {
    assert_eq!(
        parse_header_chunk(&[
            0x4D, 0x54, 0x68, 0x64, // "MThd"
            0x00, 0x00, 0x00, 0x06, // data length
            0x00, 0x01, // format
            0x00, 0x02, // number of tracks
            0x00, 0x03, // time base
        ]),
        Ok(HeaderChunk {
            format: 1,
            number_of_tracks: 2,
            time_base: 3,
        })
    );
}
