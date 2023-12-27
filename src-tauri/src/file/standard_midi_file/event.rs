use nom::{bytes::streaming::take, error::Error, number::streaming::be_u24};

use super::{variable_length_bytes::parse_variable_length_bytes, Event, EventBody};

/// SMF のイベントをパースする
pub fn parse_event(bytes: &[u8]) -> Result<(&[u8], Event), &str> {
    println!("{:?}", bytes);
    let (bytes, delta_time) =
        parse_variable_length_bytes(&bytes).expect("Failed to read delta time.");

    let (bytes, event_type_byte) =
        take::<u8, &[u8], Error<&[u8]>>(1u8)(bytes).expect("Failed to read event type.");

    let event_type_byte = event_type_byte[0];

    // Event type
    let (number_bytes, event) = match event_type_byte {
        // 3 bytes message
        0x80..=0xBF | 0xE0..=0xEF => {
            let mut message = bytes[0..2].to_vec();
            message.insert(0, event_type_byte);
            (
                2,
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
                1,
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
            let mut data = bytes[0..data_length as usize].to_vec();
            data.insert(0, 0xF0);

            (
                data_length + 1, // TODO: variable length bytes の長さが 1 と仮定してしまっているので、ちゃんと計算する
                Event {
                    delta_time,
                    event_body: EventBody::SystemExclusiveMessage(data),
                },
            )
        }
        // Meta event
        0xFF => {
            let (bytes, meta_event_type) =
                take::<u8, &[u8], Error<&[u8]>>(1u8)(bytes).expect("Failed to read event type.");

            let meta_event_type = meta_event_type[0];

            if meta_event_type == 0x2F {
                // End of track
                (
                    2,
                    Event {
                        delta_time,
                        event_body: EventBody::EndOfTrack,
                    },
                )
            } else if meta_event_type == 0x51 {
                // Tempo changed
                let (_, value) = take::<u8, &[u8], Error<&[u8]>>(3u8)(&bytes[1..])
                    .expect("Failed to read tempo value.");

                let tempo = be_u24::<&[u8], Error<&[u8]>>(value)
                    .expect("Failed to parse tempo value.")
                    .1;

                (
                    5,
                    Event {
                        delta_time,
                        event_body: EventBody::TempoChangeEvent(tempo),
                    },
                )
            } else {
                // Other event
                let (_, data_length) =
                    parse_variable_length_bytes(bytes).expect("Failed to read message.");

                println!("Data length: {}", data_length);

                (
                    data_length + 2,
                    Event {
                        delta_time,
                        event_body: EventBody::NoImplementEvent,
                    },
                )
            }
        }
        _ => {
            println!("Invalid event type: {}", event_type_byte);
            panic!("Invalid event type.")
        }
    };

    Ok((&bytes[number_bytes as usize..], event))
}
