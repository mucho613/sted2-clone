use recomposer_file::track_block::types::TrackEvent;

pub fn get_step_time_from_event(event: &TrackEvent) -> u8 {
    match *event {
        TrackEvent::Note(_, step_time, _, _) => step_time,
        TrackEvent::UserExclusive(step_time, _) => step_time,
        TrackEvent::TrackExclusive(step_time, _) => step_time,
        TrackEvent::RolBase(step_time, _, _) => step_time,
        TrackEvent::RolPara(step_time, _, _) => step_time,
        TrackEvent::RolDev(step_time, _, _) => step_time,
        TrackEvent::BankPrg(step_time, _, _) => step_time,
        TrackEvent::Keyin(step_time, _, _) => step_time,
        TrackEvent::MidiChannel(step_time, _) => step_time,
        TrackEvent::Tempo(step_time, _, _) => step_time,
        TrackEvent::AfterTouch(step_time, _) => step_time,
        TrackEvent::Control(step_time, _, _) => step_time,
        TrackEvent::ProgramChange(step_time, _) => step_time,
        TrackEvent::PolyphonicAfterTouch(step_time, _, _) => step_time,
        TrackEvent::PitchBend(step_time, _) => step_time,
        _ => 0u8,
    }
}
