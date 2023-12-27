use nom::{bytes::streaming::take, error::Error, number::streaming::be_u24};

use super::{variable_length_bytes::parse_variable_length_bytes, Event, EventBody};

pub fn parse_event(bytes: &[u8]) -> Result<(&[u8], Event), &str> {
    let (bytes, delta_time) =
        parse_variable_length_bytes(&bytes).expect("Failed to read delta time.");

    let (bytes, event_type_byte) =
        take::<u8, &[u8], Error<&[u8]>>(1u8)(bytes).expect("Failed to read event type.");

    let event_type_byte = event_type_byte[0];

    // Event type
    let event = match event_type_byte {
        // 3 bytes message
        0x80..=0xBF | 0xE0..=0xEF => {
            let message = bytes[0..3].to_vec();
            Event {
                delta_time,
                event_body: EventBody::ChannelMessage(message),
            }
        }
        // 2 bytes message
        0xC0..=0xDF => {
            let message = bytes[0..2].to_vec();
            Event {
                delta_time,
                event_body: EventBody::ChannelMessage(message),
            }
        }
        // System exclusive
        0xF0 => Event {
            delta_time,
            event_body: EventBody::SystemExclusiveMessage(vec![]),
        },
        // Meta event
        0xFF => {
            let (bytes, meta_event_type) =
                take::<u8, &[u8], Error<&[u8]>>(1u8)(bytes).expect("Failed to read event type.");

            let meta_event_type = meta_event_type[0];

            if meta_event_type == 0x2F {
                // End of track
                Event {
                    delta_time,
                    event_body: EventBody::EndOfTrack,
                }
            } else if meta_event_type == 0x51 {
                // Tempo changed
                let (bytes, value) = take::<u8, &[u8], Error<&[u8]>>(3u8)(bytes)
                    .expect("Failed to read tempo value.");

                let tempo = be_u24::<&[u8], Error<&[u8]>>(value)
                    .expect("Failed to parse tempo value.")
                    .1;

                Event {
                    delta_time,
                    event_body: EventBody::TempoChangeEvent(tempo),
                }
            } else {
                todo!();
            }
        }
        _ => panic!("Invalid event type."),
    };

    Ok((bytes, event))
}
