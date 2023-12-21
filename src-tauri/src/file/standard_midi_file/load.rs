use crate::song::song::EventBody;

use StandardMidiFile::Event;

use super::{HeaderChunk, StandardMidiFile, TrackChunk};

pub fn load(file: &Vec<u8>) -> StandardMidiFile {
    let header_chunk_bytes = &file[0..14];

    let header_chunk = HeaderChunk {
        chunk_type: &header_chunk_bytes[0..4],
        data_length: u32::from(&header_chunk_bytes[5]) << 24
            | u32::from(&header_chunk_bytes[6]) << 16
            | u32::from(&header_chunk_bytes[7]) << 8
            | u32::from(&header_chunk_bytes[8]),
        format: &header_chunk_bytes[10],
        number_of_tracks: &header_chunk_bytes[12],
        time_base: u16::from(&header_chunk_bytes[13]) << 8 | u16::from(&header_chunk_bytes[14]),
    };

    let track_chunk_bytes = &file[14..];

    // Divide bytes to tracks
    track_chunk_bytes.

    let track_chunks = track_chunk_bytes.map(|bytes| load_track(bytes));

    let time_base = u32::from(header_chunk[12]) << 8 | u32::from(header_chunk[13]);

    StandardMidiFile {
        header_chunk,
        track_chunk: todo!(),
    }
}

pub fn load_track(bytes: Vec<u8>) -> TrackChunk {
    let mut events: Vec<Event> = vec![];

    let mut index = 8;

    while index < track_chunk.len() {
        let byte_0 = u32::from(track_chunk[index]);
        let byte_1 = u32::from(track_chunk[index + 1]);
        let byte_2 = u32::from(track_chunk[index + 2]);
        let byte_3 = u32::from(track_chunk[index + 3]);

        let delta_time = if byte_0 & 0x80 == 0x00 {
            index += 1;
            byte_0
        } else if byte_1 & 0x80 == 0x00 {
            index += 2;
            (byte_0 & 0x7F) << 7 | (byte_1 & 0x7F)
        } else if byte_2 & 0x80 == 0x00 {
            index += 3;
            (byte_0 & 0x7F) << 14 | (byte_1 & 0x7F) << 7 | (byte_2 & 0x7F)
        } else if byte_3 & 0x80 == 0x00 {
            index += 4;
            (byte_0 & 0x7F) << 21 | (byte_1 & 0x7F) << 14 | (byte_2 & 0x7F) << 7 | (byte_3 & 0x7F)
        } else {
            panic!("Parsing variable-length quantity failed.");
        };

        // Event type
        match track_chunk[index] & 0xF0 {
            // 3 bytes message
            0x80 | 0x90 | 0xA0 | 0xB0 | 0xE0 => {
                let message = track_chunk[index..index + 3].to_vec();
                events.push(Event {
                    delta_time: delta_time,
                    event_body: EventBody::ChannelMessage(message),
                });
                index += 3;
            }
            // 2 bytes message
            0xC0 | 0xD0 => {
                let message = track_chunk[index..index + 2].to_vec();
                events.push(Event {
                    delta_time: delta_time,
                    event_body: EventBody::ChannelMessage(message),
                });
                index += 2;
            }
            0xF0 => {
                match &track_chunk[index] {
                    // System exclusive
                    0xF0 => {
                        let length = &track_chunk[index + 1];

                        let mut data: Vec<u8> = track_chunk
                            [index..index + 2 + usize::from(*length)]
                            .to_vec()
                            .clone();

                        data.remove(1);

                        events.push(Event {
                            delta_time: delta_time,
                            event_body: EventBody::ChannelMessage(data),
                        });

                        index += usize::from(*length) + 2
                    }

                    // Meta event
                    0xFF => {
                        index += 1;

                        let meta_event_type = track_chunk[index];
                        if meta_event_type == 0x2F {
                            // End of track
                            break;
                        } else if meta_event_type == 0x51 {
                            // Tempo changed
                            let tempo = u32::from(track_chunk[index + 2]) << 16
                                | u32::from(track_chunk[index + 3]) << 8
                                | u32::from(track_chunk[index + 4]);

                            events.push(Event {
                                delta_time: delta_time,
                                event_body: EventBody::TempoChangeEvent(tempo),
                            });
                        } else if meta_event_type == 0x58 {
                            // Signature changed
                        }
                        index += 1;

                        let length = &track_chunk[index];

                        index += usize::from(*length) + 1;
                    }

                    _ => println!("Unknown event - {}", &track_chunk[index]),
                }
            }
            _ => panic!("Unknown event - {:?}", &track_chunk[index..index + 5]),
        }
    }

    TrackChunk {
        chunk_type: todo!(),
        data_length: todo!(),
        data_body: todo!(),
    }

}
