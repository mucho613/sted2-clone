#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HeaderBlock {
    pub version: [u8; 32],
    pub title: [u8; 64],
    pub memo: [u8; 336],

    pub time_base: u8,
    pub tempo: u8,
    pub time_signature: TimeSignature,
    pub key: u8,
    pub play_bias: u8,
    pub cm6_file_name: [u8; 16],
    pub gsd_file_name: [u8; 16],
    pub number_of_tracks: u8,

    pub rhythm_notes: [RhythmNote; 32],
    pub user_exclusives: [UserExclusive; 8],
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TimeSignature {
    pub numerator: u8,
    pub denominator: u8,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RhythmNote {
    pub name: [u8; 14],
    pub note_number: u8,
    pub gate_type: u8,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UserExclusive {
    pub memo: [u8; 24],
    pub body: [u8; 24],
}
