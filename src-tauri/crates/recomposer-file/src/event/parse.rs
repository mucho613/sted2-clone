use nom::{error::Error, number::complete::be_u8, IResult};

use super::{
    comment_event::take_comment_event, exclusive_event::take_track_exclusive_event,
    types::TrackEvent,
};

fn take_single_length_event(i: &[u8]) -> IResult<&[u8], TrackEvent, Error<&[u8]>> {
    let (i, event_type) = be_u8(i)?;
    let (i, byte_0) = be_u8(i)?;
    let (i, byte_1) = be_u8(i)?;
    let (i, byte_2) = be_u8(i)?;

    let track_event = match event_type {
        0x00..=0x7F => TrackEvent::Note {
            step_time: byte_0,
            key_number: event_type,
            gate_time: byte_1,
            velocity: byte_2,
        },

        0x90..=0x97 => TrackEvent::UserExclusive {
            step_time: byte_0,
            template_gt: byte_1,
            template_ve: byte_2,
            number: event_type & 0x0F,
        },

        0xDD => TrackEvent::RolandBaseAddress {
            step_time: byte_0,
            gate_time: byte_1,
            velocity: byte_2,
        },
        0xDE => TrackEvent::RolandAddressParameter {
            step_time: byte_0,
            address: byte_1,
            description: byte_2,
        },
        0xDF => TrackEvent::RolandDeviceNumberAndModelId {
            step_time: byte_0,
            device_number: byte_1,
            model_id: byte_2,
        },

        0xE2 => TrackEvent::BankPrg {
            step_time: byte_0,
            gate_time: byte_1,
            velocity: byte_2,
        },
        0xE5 => TrackEvent::Keyin {
            step_time: byte_0,
            gate_time: byte_1,
            velocity: byte_2,
        },
        0xE6 => TrackEvent::MidiChannel {
            step_time: byte_0,
            channel: byte_1,
        },
        0xE7 => TrackEvent::Tempo {
            step_time: byte_0,
            tempo: (byte_2 as u16) << 7 | byte_1 as u16,
        },
        0xEA => TrackEvent::AfterTouch {
            step_time: byte_0,
            pressure: byte_1,
        },
        0xEB => TrackEvent::ControlChange {
            step_time: byte_0,
            controller_number: byte_1,
            value: byte_2,
        },
        0xEC => TrackEvent::ProgramChange {
            step_time: byte_0,
            program_number: byte_1,
        },
        0xED => TrackEvent::PolyphonicAfterTouch {
            step_time: byte_0,
            key_number: byte_1,
            pressure: byte_2,
        },
        0xEE => TrackEvent::PitchBend {
            step_time: byte_0,
            value: (byte_2 as i16) << 7 | byte_1 as i16,
        },

        0xF5 => TrackEvent::Key { offset: byte_0 },
        0xF8 => TrackEvent::RepeatEnd { count: byte_0 },
        0xF9 => TrackEvent::RepeatStart,
        0xFC => TrackEvent::SameMeasure {
            measure: byte_0,
            track_offset: (byte_2 as u16) << 7 | byte_1 as u16,
        },
        0xFD => TrackEvent::MeasureEnd,
        0xFE => TrackEvent::EndOfTrack,

        _ => panic!("Unknown track event type"),
    };

    Ok((i, track_event))
}

pub fn parse_track_event(i: &[u8]) -> IResult<&[u8], TrackEvent, Error<&[u8]>> {
    let (_, event_type) = be_u8(i)?;

    let (i, track_event) = match event_type {
        0x98 => take_track_exclusive_event(i)?,
        0xF6 => take_comment_event(i)?,
        // イベントの区切りで 0xF7 が見つかるのは不正
        0xF7 => panic!("Invalid track event"),
        _ => take_single_length_event(i)?,
    };

    Ok((i, track_event))
}
