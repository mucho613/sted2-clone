use nom::{bytes::complete::take, error::Error, IResult};

use super::{Header, RCPFile, RhythmNote, TimeSignature, UserExclusive};

pub fn load(file: &[u8]) -> Result<RCPFile, String> {
    let (_, file_header) = match parse_file_header(file) {
        Ok(file_header) => file_header,
        Err(_) => return Err("Error parsing file header".to_string()),
    };

    Ok(RCPFile {
        header: file_header,
    })
}

fn parse_file_header(file: &[u8]) -> IResult<&[u8], Header, Error<&[u8]>> {
    let (file, version) = take(32usize)(file)?;
    let (file, title) = take(64usize)(file)?;
    let (file, memo) = take(336usize)(file)?;

    let (file, _) = take(16usize)(file)?;

    let (file, time_base) = take(1usize)(file)?;
    let (file, tempo) = take(1usize)(file)?;
    let (file, time_signature) = take(2usize)(file)?;
    let (file, key) = take(1usize)(file)?;
    let (file, play_bias) = take(1usize)(file)?;
    let (file, cm6_file_name) = take(16usize)(file)?;
    let (file, gsd_file_name) = take(16usize)(file)?;
    let (file, number_of_tracks) = take(1usize)(file)?;

    let (file, _) = take(31usize)(file)?;

    let (file, rhythm_notes) = [0..32].iter().fold((file, Vec::new()), |mut acc, x| {
        let (file, rhythm_note) = take::<usize, &[u8], Error<&[u8]>>(16usize)(acc.0).unwrap();
        acc.1.push(RhythmNote {
            name: rhythm_note[0..14].try_into().unwrap(),
            note_number: rhythm_note[14],
            gate_type: rhythm_note[15],
        });
        (file, acc.1)
    });

    let (file, user_exclusives) = [0..32].iter().fold((file, Vec::new()), |mut acc, x| {
        let (file, user_exclusive) = take::<usize, &[u8], Error<&[u8]>>(16usize)(acc.0).unwrap();
        acc.1.push(UserExclusive {
            message: user_exclusive.try_into().unwrap(),
        });
        (file, acc.1)
    });

    let time_signature = TimeSignature {
        numerator: time_signature[0],
        denominator: time_signature[1],
    };

    Ok((
        file,
        Header {
            version: version.try_into().unwrap(),
            title: title.try_into().unwrap(),
            memo: memo.try_into().unwrap(),
            time_base: time_base[0],
            tempo: tempo[0],
            time_signature,
            key: key[0],
            play_bias: play_bias[0],
            cm6_file_name: cm6_file_name.try_into().unwrap(),
            gsd_file_name: gsd_file_name.try_into().unwrap(),
            number_of_tracks: number_of_tracks[0],
            rhythm_notes: rhythm_notes.try_into().unwrap(),
            user_exclusives: user_exclusives.try_into().unwrap(),
        },
    ))
}

// #[test]
// fn parse_file_header() {
//     let input = "RCM-PC98V2.0(C)COME ON MUSIC\r\n\0\0Trailing bytes".as_bytes();

//     assert_eq!(
//         take_version(input),
//         Ok((
//             "Trailing bytes".as_bytes(),
//             "RCM-PC98V2.0(C)COME ON MUSIC\r\n\0\0".as_bytes(),
//         ))
//     );
// }
