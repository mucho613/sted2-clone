use nom::{bytes::complete::take, error::Error, IResult};

use super::types::TrackEvent;

pub fn take_track_exclusive_event(i: &[u8]) -> IResult<&[u8], TrackEvent, Error<&[u8]>> {
    let mut buffer = vec![];
    let mut i_in_loop = i;
    loop {
        let (i, bytes) = take(4usize)(i_in_loop)?;
        i_in_loop = i;

        if bytes[2] == 0xF7 {
            break;
        } else {
            buffer.push(bytes[2]);
        }

        if bytes[3] == 0xF7 {
            break;
        } else {
            buffer.push(bytes[3]);
        }
    }

    let step_time = buffer[1];
    let template_gt = buffer[2];
    let template_ve = buffer[3];

    Ok((
        i_in_loop,
        TrackEvent::TrackExclusive {
            step_time,
            template_gt,
            template_ve,
            message_body: buffer,
        },
    ))
}
