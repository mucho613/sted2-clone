use nom::{bytes::complete::take, error::Error, IResult};

use super::types::TrackEvent;

pub fn take_comment_event(i: &[u8]) -> IResult<&[u8], TrackEvent, Error<&[u8]>> {
    let (i, bytes) = take(40usize)(i)?;

    // 40 bytes のうち、4 bytes ごとの 3, 4 byte 目だけを取り出して結合する
    let mut buffer = vec![];
    for i in (0..40).step_by(4) {
        buffer.push(bytes[i + 2]);
        buffer.push(bytes[i + 3]);
    }

    let mut text = [0u8; 20];
    text.copy_from_slice(&buffer[0..20]);

    Ok((i, TrackEvent::Comment { text }))
}
