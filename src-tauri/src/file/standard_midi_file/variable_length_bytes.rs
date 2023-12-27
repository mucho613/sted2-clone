use nom::{
    bytes::streaming::{tag, take, take_till1},
    error::Error,
};

pub fn parse_variable_length_bytes(bytes: &[u8]) -> Result<(&[u8], u32), &str> {
    let mut length = 0;
    let delta_time = if bytes[0] & 0x80 == 0 {
        length = 1;
        u32::from(bytes[0])
    } else if bytes[1] & 0x80 == 0 {
        length = 2;
        u32::from(bytes[0] & 0x7F) << 7 | u32::from(bytes[1] & 0x7F)
    } else if bytes[2] & 0x80 == 0 {
        length = 3;
        u32::from(bytes[0] & 0x7F) << 14
            | u32::from(bytes[1] & 0x7F) << 7
            | u32::from(bytes[2] & 0x7F)
    } else if bytes[3] & 0x80 == 0 {
        length = 4;
        u32::from(bytes[0] & 0x7F) << 21
            | u32::from(bytes[1] & 0x7F) << 14
            | u32::from(bytes[2] & 0x7F) << 7
            | u32::from(bytes[3] & 0x7F)
    } else {
        return Err("Invalid delta time.");
    };

    Ok((&bytes[length..], delta_time))
}

#[test]
fn single_byte_test() {
    assert_eq!(parse_variable_length_bytes(&[0x00]).unwrap().1, 0u32);
    assert_eq!(parse_variable_length_bytes(&[0x7F]).unwrap().1, 127u32);
}

#[test]
fn double_byte_test() {
    assert_eq!(
        parse_variable_length_bytes(&[0x81, 0x00]).unwrap().1,
        128u32
    );
    assert_eq!(
        parse_variable_length_bytes(&[0xFF, 0x7F]).unwrap().1,
        16383u32
    );
}

#[test]
fn triple_byte_test() {
    assert_eq!(
        parse_variable_length_bytes(&[0x81, 0x80, 0x00]).unwrap().1,
        16384u32
    );
    assert_eq!(
        parse_variable_length_bytes(&[0xFF, 0xFF, 0x7F]).unwrap().1,
        2097151u32
    );
}

#[test]
fn quadruple_byte_test() {
    assert_eq!(
        parse_variable_length_bytes(&[0x81, 0x80, 0x80, 0x00])
            .unwrap()
            .1,
        2097152u32
    );
    assert_eq!(
        parse_variable_length_bytes(&[0xFF, 0xFF, 0xFF, 0x7F])
            .unwrap()
            .1,
        268435455u32
    );
}
