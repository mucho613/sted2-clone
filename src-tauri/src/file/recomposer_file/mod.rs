pub(crate) mod load;

#[derive(Debug)]
pub struct RCPFile {
    header: Header,
}

#[derive(Debug)]
pub struct Header {
    version: [u8; 32],
    title: [u8; 64],
    memo: [u8; 336],

    time_base: u8,
    tempo: u8,
    time_signature: TimeSignature,
    key: u8,
    play_bias: u8,
    cm6_file_name: [u8; 16],
    gsd_file_name: [u8; 16],
    number_of_tracks: u8,

    rhythm_notes: [RhythmNote; 32],
    user_exclusives: [UserExclusive; 8],
}

#[derive(Debug)]
pub struct TimeSignature {
    numerator: u8,
    denominator: u8,
}

#[derive(Debug)]
pub struct RhythmNote {
    name: [u8; 14],
    note_number: u8,
    gate_type: u8,
}

#[derive(Debug)]
pub struct UserExclusive {
    message: [u8; 48],
}

#[derive(Debug)]
pub struct TrackHeader {
    size: u16,
    channel: u8,
    key: u8,
    step: u8,
    mode: u8,
    comment: u8,
    data: Vec<u8>,
}

#[derive(Debug)]
pub struct TrackEvent {
    event_type: u8,
    step_time: u8,
    gate_time: u8,
    velocity: u8,
}
