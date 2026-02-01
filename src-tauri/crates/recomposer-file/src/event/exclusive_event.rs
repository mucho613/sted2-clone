use nom::{IResult, bytes::complete::take, error::Error};

use super::types::TrackEvent;

pub fn take_track_exclusive_event(i: &[u8]) -> IResult<&[u8], TrackEvent, Error<&[u8]>> {
    let mut buffer = vec![];

    // 先頭の 4 バイトを取得
    let (i, head_bytes) = take(4usize)(i)?;

    // イベントの2バイト目が Step time (8bit)
    let step_time = head_bytes[1];
    // イベントの3バイト目が Template(GT) (8bit)
    let template_gt = head_bytes[2];
    // イベントの4バイト目が Template(VE) (8bit)
    let template_ve = head_bytes[3];

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
