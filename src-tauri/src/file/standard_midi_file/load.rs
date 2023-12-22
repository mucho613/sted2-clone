use super::{Event, EventBody, HeaderChunk, StandardMidiFile, TrackChunk};

pub fn load(file: &[u8]) -> StandardMidiFile {
    let header_chunk_bytes = &file[0..14];

    let header_chunk = HeaderChunk {
        chunk_type: [
            header_chunk_bytes[0],
            header_chunk_bytes[1],
            header_chunk_bytes[2],
            header_chunk_bytes[3],
        ],
        data_length: u32::from(header_chunk_bytes[5]) << 24
            | u32::from(header_chunk_bytes[6]) << 16
            | u32::from(header_chunk_bytes[7]) << 8
            | u32::from(header_chunk_bytes[8]),
        format: u16::from(header_chunk_bytes[9]) << 8 | u16::from(header_chunk_bytes[10]),
        number_of_tracks: u16::from(header_chunk_bytes[11]) << 8
            | u16::from(header_chunk_bytes[12]),
        time_base: u16::from(header_chunk_bytes[13]) << 8 | u16::from(header_chunk_bytes[14]),
    };

    let track_chunk_bytes = &file[14..];

    // バイト列をトラックごとに分割する
    let track_chunk_bytes_divided = divide_track_chunk_bytes(track_chunk_bytes);

    // let track_chunks = track_chunk_bytes_divided
    //     .iter();

    let time_base = u32::from(header_chunk_bytes[12]) << 8 | u32::from(header_chunk_bytes[13]);

    StandardMidiFile {
        header_chunk,
        track_chunk: vec![todo!()],
    }
}

pub fn divide_track_chunk_bytes(bytes: &[u8]) -> Vec<&[u8]> {
    let mut index: usize = 0;

    while index < bytes.len() {
        // MTrk を見つける
        // データ長を読む(4 bytes)
        // MTrk からデータ末尾まで切り出す
        // その先の MTrk を見つける
        // ...
    }

    todo!()
}

pub fn load_track(bytes: &[u8]) -> TrackChunk {
    let mut events: Vec<Event> = vec![];

    let mut index = 8;

    while index < bytes.len() {
        let byte_0 = u32::from(bytes[index]);
        let byte_1 = u32::from(bytes[index + 1]);
        let byte_2 = u32::from(bytes[index + 2]);
        let byte_3 = u32::from(bytes[index + 3]);

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
        match bytes[index] & 0xF0 {
            // 3 bytes message
            0x80 | 0x90 | 0xA0 | 0xB0 | 0xE0 => {
                let message = bytes[index..index + 3].to_vec();
                events.push(Event {
                    delta_time: delta_time,
                    event_body: EventBody::ChannelMessage(message),
                });
                index += 3;
            }
            // 2 bytes message
            0xC0 | 0xD0 => {
                let message = bytes[index..index + 2].to_vec();
                events.push(Event {
                    delta_time: delta_time,
                    event_body: EventBody::ChannelMessage(message),
                });
                index += 2;
            }
            0xF0 => {
                match &bytes[index] {
                    // System exclusive
                    0xF0 => {
                        let length = &bytes[index + 1];

                        let mut data: Vec<u8> = bytes[index..index + 2 + usize::from(*length)]
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

                        let meta_event_type = bytes[index];
                        if meta_event_type == 0x2F {
                            // End of track
                            break;
                        } else if meta_event_type == 0x51 {
                            // Tempo changed
                            let tempo = u32::from(bytes[index + 2]) << 16
                                | u32::from(bytes[index + 3]) << 8
                                | u32::from(bytes[index + 4]);

                            events.push(Event {
                                delta_time: delta_time,
                                event_body: EventBody::TempoChangeEvent(tempo),
                            });
                        } else if meta_event_type == 0x58 {
                            // Signature changed
                        }
                        index += 1;

                        let length = &bytes[index];

                        index += usize::from(*length) + 1;
                    }

                    _ => println!("Unknown event - {}", &bytes[index]),
                }
            }
            _ => panic!("Unknown event - {:?}", &bytes[index..index + 5]),
        }
    }

    TrackChunk {
        chunk_type: todo!(),
        data_length: todo!(),
        data_body: todo!(),
    }
}
