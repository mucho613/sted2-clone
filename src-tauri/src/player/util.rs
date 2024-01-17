use recomposer_file::event::types::TrackEvent;

pub fn get_step_time_from_event(event: &TrackEvent) -> u8 {
    match *event {
        TrackEvent::Note { step_time, .. } => step_time,
        TrackEvent::UserExclusive { step_time, .. } => step_time,
        TrackEvent::RolandBaseAddress { step_time, .. } => step_time,
        TrackEvent::RolandParameter { step_time, .. } => step_time,
        TrackEvent::RolandDeviceNumber { step_time, .. } => step_time,
        TrackEvent::BankPrg { step_time, .. } => step_time,
        TrackEvent::Keyin { step_time, .. } => step_time,
        TrackEvent::MidiChannel { step_time, .. } => step_time,
        TrackEvent::Tempo { step_time, .. } => step_time,
        TrackEvent::AfterTouch { step_time, .. } => step_time,
        TrackEvent::ControlChange { step_time, .. } => step_time,
        TrackEvent::ProgramChange { step_time, .. } => step_time,
        TrackEvent::PolyphonicAfterTouch { step_time, .. } => step_time,
        TrackEvent::PitchBend { step_time, .. } => step_time,
        _ => 0u8,
    }
}
