use nom::{bytes::streaming::take, error::Error, number::streaming::be_u24};

use crate::file::standard_midi_file::MetaEvent;

use super::{
    status_byte::parse_status_byte, variable_length_bytes::parse_variable_length_bytes, Event,
    EventBody,
};

/// SMF のイベントをパースする
pub fn parse_event(bytes: &[u8], prev_status_byte: Option<u8>) -> Result<(&[u8], Event), &str> {
    let (bytes, delta_time) =
        parse_variable_length_bytes(&bytes).expect("Failed to read delta time.");

    let (bytes, event_type_byte) =
        parse_status_byte(bytes, prev_status_byte).expect("Failed to read status byte.");

    // Event type
    let (bytes, event) = match event_type_byte {
        // 3 bytes message
        0x80..=0xBF | 0xE0..=0xEF => {
            let mut message = bytes[0..2].to_vec();
            message.insert(0, event_type_byte);
            (
                &bytes[2..],
                Event {
                    delta_time,
                    event_body: EventBody::ChannelMessage(message),
                },
            )
        }
        // 2 bytes message
        0xC0..=0xDF => {
            let mut message = bytes[0..1].to_vec();
            message.insert(0, event_type_byte);
            (
                &bytes[1..],
                Event {
                    delta_time,
                    event_body: EventBody::ChannelMessage(message),
                },
            )
        }
        // System exclusive
        0xF0 => {
            let (bytes, data_length) =
                parse_variable_length_bytes(bytes).expect("Failed to read message.");
            let data_length = data_length as usize;
            let (bytes, message) = take::<u8, &[u8], Error<&[u8]>>(data_length as u8)(bytes)
                .expect("Failed to read system exclusive value.");
            let mut message = message.to_vec();
            message.insert(0, event_type_byte);

            (
                bytes,
                Event {
                    delta_time,
                    event_body: EventBody::SystemExclusiveMessage(message),
                },
            )
        }
        // Meta event
        0xFF => {
            let (bytes, meta_event_type) =
                take::<u8, &[u8], Error<&[u8]>>(1u8)(bytes).expect("Failed to read event type.");

            let meta_event_type = meta_event_type[0];

            let (bytes, data_length) =
                parse_variable_length_bytes(bytes).expect("Failed to read message.");

            if meta_event_type == 0x2F {
                // End of track
                (
                    bytes,
                    Event {
                        delta_time,
                        event_body: EventBody::MetaEvent(MetaEvent::EndOfTrack),
                    },
                )
            } else if meta_event_type == 0x51 {
                // Tempo changed
                let (bytes, value) = take::<u8, &[u8], Error<&[u8]>>(3u8)(bytes)
                    .expect("Failed to read tempo value.");

                let (_, tempo) =
                    be_u24::<&[u8], Error<&[u8]>>(value).expect("Failed to parse tempo value.");

                (
                    bytes,
                    Event {
                        delta_time,
                        event_body: EventBody::MetaEvent(MetaEvent::TempoChangeEvent(tempo)),
                    },
                )
            } else {
                // Other event
                // TODO: Implement other meta events
                let (bytes, value) = take::<u8, &[u8], Error<&[u8]>>(data_length as u8)(bytes)
                    .expect("Failed to read value.");

                (
                    bytes,
                    Event {
                        delta_time,
                        event_body: EventBody::MetaEvent(MetaEvent::TextEvent(format!(
                            "{:?}",
                            value
                        ))),
                    },
                )
            }
        }
        _ => return Err("Invalid event type."),
    };

    Ok((&bytes, event))
}
