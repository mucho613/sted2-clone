use nom::{bytes::streaming::take, error::Error};

/// SMF 内の event に含まれる先頭の byte を読み取り、Status byte として返す
/// running status に対応するため、前回のステータスバイトを引数に取る
pub fn parse_status_byte(bytes: &[u8], prev_status_byte: Option<u8>) -> Result<(&[u8], u8), &str> {
    let (bytes_head_readed, head_byte) =
        take::<u8, &[u8], Error<&[u8]>>(1u8)(bytes).expect("Failed to read status byte.");

    // ステータスバイトだったら、そのまま返す
    if head_byte[0] >= 128 {
        return Ok((bytes_head_readed, head_byte[0]));
    }

    // ステータスバイトでない(ランニングステータスだった)場合は、前回のステータスバイトを使う
    // 前回のステータスバイトがない場合はエラー
    if prev_status_byte.is_none() {
        return Err("Invalid event type, or running status is not set.");
    }

    let status_byte = prev_status_byte.unwrap();
    return Ok((bytes, status_byte));
}

#[test]
fn parse_status_byte_test() {
    // prev_status_byte が None で、Status byte が含まれるバイト列だった場合
    assert_eq!(parse_status_byte(&[0x80], None), Ok((&[][..], 0x80)));

    // prev_status_byte に u8 値が渡され、Status byte が含まれるバイト列だった場合
    assert_eq!(parse_status_byte(&[0x80], Some(0x90)), Ok((&[][..], 0x80)));

    // prev_status_byte が None で、Status byte でない値が含まれるバイト列だった場合
    assert_eq!(
        parse_status_byte(&[0x00], None),
        Err("Invalid event type, or running status is not set.")
    );

    // prev_status_byte に u8 値が渡され、Status byte でない値が含まれるバイト列だった場合
    assert_eq!(
        parse_status_byte(&[0x00], Some(0x80)),
        Ok((&[0x00][..], 0x80))
    );
}
