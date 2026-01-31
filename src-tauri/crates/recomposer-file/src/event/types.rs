#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TrackEvent {
    Note {
        step_time: u8,
        key_number: u8,
        gate_time: u8,
        velocity: u8,
    },
    UserExclusive {
        step_time: u8,
        template_gt: u8,
        template_ve: u8,
        number: u8,
    },
    TrackExclusive {
        step_time: u8,
        template_gt: u8,
        template_ve: u8,
        message_body: Vec<u8>,
    },
    RolandBaseAddress {
        step_time: u8,
        gate_time: u8,
        velocity: u8,
    },
    RolandDeviceNumberAndModelId {
        step_time: u8,
        device_number: u8,
        model_id: u8,
    },
    RolandAddressAndParameter {
        step_time: u8,
        address: u8,
        description: u8,
    },
    BankPrg {
        step_time: u8,
        gate_time: u8,
        velocity: u8,
    },
    Keyin {
        step_time: u8,
        gate_time: u8,
        velocity: u8,
    },
    MidiChannel {
        step_time: u8,
        channel: u8,
    },
    Tempo {
        step_time: u8,
        tempo: u16,
    },
    AfterTouch {
        step_time: u8,
        pressure: u8,
    },
    ControlChange {
        step_time: u8,
        controller_number: u8,
        value: u8,
    },
    ProgramChange {
        step_time: u8,
        program_number: u8,
    },
    PolyphonicAfterTouch {
        step_time: u8,
        key_number: u8,
        pressure: u8,
    },
    PitchBend {
        step_time: u8,
        value: i16,
    },
    Key {
        offset: u8,
    },
    Comment {
        text: [u8; 20],
    },
    RepeatEnd {
        count: u8,
    },
    RepeatStart,
    SameMeasure {
        measure: u8,
        track_offset: u16,
    },
    MeasureEnd,
    EndOfTrack,
    Unknown {
        event_type: u8,
        data: [u8; 3],
    },
}
