use nom::{
    bytes::streaming::{tag, take},
    error::Error,
};

use super::{
    event::parse_event, variable_length_bytes::parse_variable_length_bytes, Event, EventBody,
    TrackChunk,
};

pub fn parse_track_chunk(bytes: &[u8]) -> Result<(&[u8], TrackChunk), String> {
    let (bytes, _) = tag::<&str, &[u8], Error<&[u8]>>("MTrk")(bytes).expect("\"MTrk\" not found.");
    let (bytes, _) =
        take::<u8, &[u8], Error<&[u8]>>(4u8)(bytes).expect("Failed to read data length.");

    let mut events: Vec<Event> = vec![];

    loop {
        let (bytes, event) = parse_event(bytes).expect("Failed to parse event.");

        if event.event_body == EventBody::EndOfTrack {
            events.push(event);
            break;
        }

        events.push(event);
    }

    Ok((bytes, TrackChunk { data_body: events }))
}

#[test]
fn empty_track() {
    assert_eq!(
        parse_track_chunk(&[
            0x4D, 0x54, 0x72, 0x6B, // "MTrk"
            0x00, 0x00, 0x00, 0x00, // data length
        ]),
        Ok((vec![], TrackChunk { data_body: vec![] }))
    );
}

#[test]
fn only_end_of_track() {
    assert_eq!(
        parse_track_chunk(&[
            0x4D, 0x54, 0x72, 0x6B, // "MTrk"
            0x00, 0x00, 0x00, 0x04, // data length
            0x00, 0xFF, 0x2F, 0x00, // end of track
        ]),
        Ok(TrackChunk {
            data_body: vec![Event {
                delta_time: 0,
                event_body: EventBody::EndOfTrack
            }],
        })
    );
}
