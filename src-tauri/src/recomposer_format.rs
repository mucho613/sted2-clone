mod recomposer_format;

pub struct Header {
    header_string: [u8; 32],
    title: [u8; 64],
    title_2: [u8; 336],
    time_base_lsb: u8,
    tempo: u8,
    time_measurement_a: u8,
    time_measurement_b: u8,
    key: u8,
    play_bias: u8,
    number_of_tracks: u8,
    time_base_msb: u8,
}

pub struct UserExclusive {
    user_exclusive_1: [u8; 256],
}

pub struct TrackHeader {
    size: u16,
    channel: u8,
    key: u8,
    step: u8,
    mode: u8,
    comment: u8,
    data: Vec<u8>,
}

pub struct TrackData {
    event: u8,
    step_time: u8,
    gate_time: u8,
    velocity: u8,
}
