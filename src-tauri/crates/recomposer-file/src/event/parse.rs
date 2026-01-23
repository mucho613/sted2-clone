use nom::{IResult, error::Error, number::complete::le_u8};

use super::{
    comment_event::take_comment_event, exclusive_event::take_track_exclusive_event,
    types::TrackEvent,
};

fn take_single_length_event(i: &[u8]) -> IResult<&[u8], TrackEvent, Error<&[u8]>> {
    let (i, event_type) = le_u8(i)?;
    let (i, byte_0) = le_u8(i)?;
    let (i, byte_1) = le_u8(i)?;
    let (i, byte_2) = le_u8(i)?;

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
        // 未知のイベントタイプの場合は読み飛ばす
        _ => TrackEvent::Unknown {
            event_type,
            data: [byte_0, byte_1, byte_2],
        },
    };

    Ok((i, track_event))
}

pub fn parse_track_event(i: &[u8]) -> IResult<&[u8], TrackEvent, Error<&[u8]>> {
    let (_, event_type) = le_u8(i)?;

    let (i, track_event) = match event_type {
        0x98 => take_track_exclusive_event(i)?,
        0xF6 => take_comment_event(i)?,
        _ => take_single_length_event(i)?,
    };

    Ok((i, track_event))
}

#[cfg(test)]
mod tests {
    use super::parse_track_event;

    #[test]
    fn test_parse_note_event() {
        let data = [0x3C, 0x30, 0x60, 0x64];
        let (_, event) = parse_track_event(&data).expect("Failed to parse track event");

        assert_eq!(
            event,
            super::TrackEvent::Note {
                step_time: 0x30,
                key_number: 0x3C,
                gate_time: 0x60,
                velocity: 0x64,
            }
        );
    }

    #[test]
    fn test_parse_user_exclusive_event() {
        let data = [0x92, 0x10, 0x20, 0x30];
        let (_, event) = parse_track_event(&data).expect("Failed to parse track event");

        assert_eq!(
            event,
            super::TrackEvent::UserExclusive {
                step_time: 0x10,
                template_gt: 0x20,
                template_ve: 0x30,
                number: 0x02,
            }
        );
    }

    #[test]
    fn test_parse_roland_base_address_event() {
        let data = [0xDD, 0x05, 0x10, 0x20];
        let (_, event) = parse_track_event(&data).expect("Failed to parse track event");

        assert_eq!(
            event,
            super::TrackEvent::RolandBaseAddress {
                step_time: 0x05,
                gate_time: 0x10,
                velocity: 0x20,
            }
        );
    }

    #[test]
    fn test_parse_roland_address_parameter_event() {
        let data = [0xDE, 0x07, 0x15, 0x25];
        let (_, event) = parse_track_event(&data).expect("Failed to parse track event");

        assert_eq!(
            event,
            super::TrackEvent::RolandAddressParameter {
                step_time: 0x07,
                address: 0x15,
                description: 0x25,
            }
        );
    }

    #[test]
    fn test_parse_roland_device_number_and_model_id_event() {
        let data = [0xDF, 0x09, 0x1A, 0x2A];
        let (_, event) = parse_track_event(&data).expect("Failed to parse track event");

        assert_eq!(
            event,
            super::TrackEvent::RolandDeviceNumberAndModelId {
                step_time: 0x09,
                device_number: 0x1A,
                model_id: 0x2A,
            }
        );
    }

    #[test]
    fn test_parse_bank_prg_event() {
        let data = [0xE2, 0x08, 0x12, 0x18];
        let (_, event) = parse_track_event(&data).expect("Failed to parse track event");

        assert_eq!(
            event,
            super::TrackEvent::BankPrg {
                step_time: 0x08,
                gate_time: 0x12,
                velocity: 0x18,
            }
        );
    }

    #[test]
    fn test_parse_keyin_event() {
        let data = [0xE5, 0x0C, 0x24, 0x36];
        let (_, event) = parse_track_event(&data).expect("Failed to parse track event");

        assert_eq!(
            event,
            super::TrackEvent::Keyin {
                step_time: 0x0C,
                gate_time: 0x24,
                velocity: 0x36,
            }
        );
    }

    #[test]
    fn test_parse_midi_channel_event() {
        let data = [0xE6, 0x02, 0x04, 0x00];
        let (_, event) = parse_track_event(&data).expect("Failed to parse track event");

        assert_eq!(
            event,
            super::TrackEvent::MidiChannel {
                step_time: 0x02,
                channel: 0x04,
            }
        );
    }

    #[test]
    fn test_parse_tempo_event() {
        let data = [0xE7, 0x10, 0x40, 0x01];
        let (_, event) = parse_track_event(&data).expect("Failed to parse track event");

        assert_eq!(
            event,
            super::TrackEvent::Tempo {
                step_time: 0x10,
                tempo: (0x01u16 << 7) | 0x40u16,
            }
        );
    }

    #[test]
    fn test_parse_after_touch_event() {
        let data = [0xEA, 0x06, 0x50, 0x00];
        let (_, event) = parse_track_event(&data).expect("Failed to parse track event");

        assert_eq!(
            event,
            super::TrackEvent::AfterTouch {
                step_time: 0x06,
                pressure: 0x50,
            }
        );
    }

    #[test]
    fn test_parse_control_change_event() {
        let data = [0xEB, 0x14, 0x07, 0x7F];
        let (_, event) = parse_track_event(&data).expect("Failed to parse track event");

        assert_eq!(
            event,
            super::TrackEvent::ControlChange {
                step_time: 0x14,
                controller_number: 0x07,
                value: 0x7F,
            }
        );
    }

    #[test]
    fn test_parse_program_change_event() {
        let data = [0xEC, 0x20, 0x2B, 0x00];
        let (_, event) = parse_track_event(&data).expect("Failed to parse track event");

        assert_eq!(
            event,
            super::TrackEvent::ProgramChange {
                step_time: 0x20,
                program_number: 0x2B,
            }
        );
    }

    #[test]
    fn test_parse_polyphonic_after_touch_event() {
        let data = [0xED, 0x18, 0x3C, 0x5A];
        let (_, event) = parse_track_event(&data).expect("Failed to parse track event");

        assert_eq!(
            event,
            super::TrackEvent::PolyphonicAfterTouch {
                step_time: 0x18,
                key_number: 0x3C,
                pressure: 0x5A,
            }
        );
    }

    #[test]
    fn test_parse_pitch_bend_event() {
        let data = [0xEE, 0x22, 0x30, 0x02];
        let (_, event) = parse_track_event(&data).expect("Failed to parse track event");

        assert_eq!(
            event,
            super::TrackEvent::PitchBend {
                step_time: 0x22,
                value: (0x02i16 << 7) | 0x30i16,
            }
        );
    }

    #[test]
    fn test_parse_key_event() {
        let data = [0xF5, 0x03, 0x00, 0x00];
        let (_, event) = parse_track_event(&data).expect("Failed to parse track event");

        assert_eq!(event, super::TrackEvent::Key { offset: 0x03 });
    }

    #[test]
    fn test_parse_repeat_end_event() {
        let data = [0xF8, 0x05, 0x00, 0x00];
        let (_, event) = parse_track_event(&data).expect("Failed to parse track event");

        assert_eq!(event, super::TrackEvent::RepeatEnd { count: 0x05 });
    }

    #[test]
    fn test_parse_repeat_start_event() {
        let data = [0xF9, 0x00, 0x00, 0x00];
        let (_, event) = parse_track_event(&data).expect("Failed to parse track event");

        assert_eq!(event, super::TrackEvent::RepeatStart);
    }

    #[test]
    fn test_parse_same_measure_event() {
        let data = [0xFC, 0x08, 0x20, 0x01];
        let (_, event) = parse_track_event(&data).expect("Failed to parse track event");

        assert_eq!(
            event,
            super::TrackEvent::SameMeasure {
                measure: 0x08,
                track_offset: (0x01u16 << 7) | 0x20u16,
            }
        );
    }

    #[test]
    fn test_parse_measure_end_event() {
        let data = [0xFD, 0x00, 0x00, 0x00];
        let (_, event) = parse_track_event(&data).expect("Failed to parse track event");

        assert_eq!(event, super::TrackEvent::MeasureEnd);
    }

    #[test]
    fn test_parse_end_of_track_event() {
        let data = [0xFE, 0x00, 0x00, 0x00];
        let (_, event) = parse_track_event(&data).expect("Failed to parse track event");

        assert_eq!(event, super::TrackEvent::EndOfTrack);
    }
}
