use nom::{
    bytes::{complete::tag, streaming::take},
    error::Error,
};

use super::{Header, RecomposerFormatFile};

// pub fn load(file: &[u8]) -> Result<RecomposerFormatFile, String> {
//     let (bytes, file_header) = match parse_file_header(file) {
//         Ok(file_header) => file_header,
//         Err(error) => return Err(error),
//     };

//     Ok(RecomposerFormatFile {
//         header: file_header,
//     })
// }

fn parse_version(file: &[u8]) -> IResult<&[u8], &[u8]> {
    let (bytes, version) = take(32u8)(file).expect("Failed to read version");

    Ok((bytes, version))
}

// fn parse_file_header(file: &[u8]) -> Result<Header, String> {
//     let (bytes, header_string) = parse_version(file).map_err(|_| "Failed to parse version")?;

//     Ok(Header {
//         version: header_string,
//         title: todo!(),
//         memo: todo!(),
//         time_base: todo!(),
//         tempo: todo!(),
//         time_signature: todo!(),
//         key: todo!(),
//         play_bias: todo!(),
//         cm6_file_name: todo!(),
//         gsd_file_name: todo!(),
//         number_of_tracks: todo!(),
//         rhythm_note: todo!(),
//         user_exclusive: todo!(),
//     })
// }

#[test]
fn test_parse_version() {
    let file = "RCM-PC98V2.0(C)COME ON MUSIC\r\nTrack memo area".as_slice();

    assert_eq!(
        parse_version(file),
        Ok((
            vec![].as_slice(),
            "RCM-PC98V2.0(C)COME ON MUSIC\r\n".as_bytes()
        ))
    );
}
