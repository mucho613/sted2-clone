use std::vec;

use nom::{
    bytes::complete::take,
    error::Error,
    number::complete::{le_u16, le_u8},
    IResult,
};

use crate::event::{parse::parse_track_event, types::TrackEvent};

use super::types::{Track, TrackBlock, TrackHeader, TrackType};

fn parse_track_header(i: &[u8]) -> IResult<&[u8], TrackHeader, Error<&[u8]>> {
    let (i, size) = le_u16(i)?;
    let (i, track_number) = le_u8(i)?;
    let (i, track_type) = le_u8(i)?;
    let (i, channel) = le_u8(i)?;
    let (i, key_bias) = le_u8(i)?;
    let (i, step_bias) = le_u8(i)?;
    let (i, is_muting) = le_u8(i)?;
    let (i, comment) = take(36usize)(i)?;

    let track_type = match track_type {
        0x00 => TrackType::Normal,
        0x80 => TrackType::Rhythm,
        _ => panic!("Unknown track type"),
    };

    let is_muting = match is_muting {
        0x00 => false,
        0x01 => true,
        _ => panic!("Unknown value for is_muting"),
    };

    Ok((
        i,
        TrackHeader {
            size,
            track_number,
            track_type,
            channel,
            key_bias,
            step_bias,
            muting: is_muting,
            comment: comment.try_into().unwrap(),
        },
    ))
}

fn parse_track(i: &[u8]) -> IResult<&[u8], Track, Error<&[u8]>> {
    let (mut i, track_header) = parse_track_header(i)?;

    let mut track_events = vec![];

    loop {
        let (i2, event) = parse_track_event(i)?;
        i = i2;

        if let TrackEvent::EndOfTrack = event {
            break;
        }

        track_events.push(event);
    }

    Ok((
        i,
        Track {
            track_header,
            track_events,
        },
    ))
}

pub fn parse_track_block(i: &[u8]) -> IResult<&[u8], TrackBlock, Error<&[u8]>> {
    let mut i = i;

    let mut tracks = vec![];

    (0..18).for_each(|_| {
        let (i2, track) = parse_track(i).unwrap();
        i = i2;
        tracks.push(track);
    });

    Ok((i, TrackBlock { tracks }))
}
