use recomposer_file::{RcpFile, event::types::TrackEvent};

use crate::song_info::display_song_info;

struct MidiEvent {
    tick: u32,
    bytes: Vec<u8>,
}

/// Play the loaded Recomposer format file
pub fn play(rcp_file: &RcpFile) {
    display_song_info(&rcp_file);

    let mut track_events: Vec<Vec<MidiEvent>> = Vec::new();

    // 各トラックを走査
    for track in &rcp_file.track_block.tracks {
        let mut current_time = 0u32;

        let mut midi_events: Vec<MidiEvent> = Vec::new();

        for event in &track.track_events {
            match event {
                TrackEvent::Note {
                    key_number,
                    velocity,
                    step_time,
                    gate_time,
                } => {
                    // Note On イベント
                    midi_events.push(MidiEvent {
                        tick: current_time,
                        bytes: vec![
                            0x90 | (track.track_header.channel & 0x0F),
                            *key_number,
                            *velocity,
                        ],
                    });

                    // Note Off イベント
                    midi_events.push(MidiEvent {
                        tick: current_time + *gate_time as u32,
                        bytes: vec![0x80 | (track.track_header.channel & 0x0F), *key_number, 0],
                    });

                    current_time += *step_time as u32;
                }
                TrackEvent::UserExclusive { step_time, .. } => {
                    current_time += *step_time as u32;
                }
                TrackEvent::TrackExclusive { step_time, .. } => {
                    current_time += *step_time as u32;
                }
                TrackEvent::RolandBaseAddress { step_time, .. } => {
                    current_time += *step_time as u32;
                }
                TrackEvent::RolandDeviceNumberAndModelId { step_time, .. } => {
                    current_time += *step_time as u32;
                }
                TrackEvent::RolandAddressParameter { step_time, .. } => {
                    current_time += *step_time as u32;
                }
                TrackEvent::BankPrg { step_time, .. } => {
                    current_time += *step_time as u32;
                }
                TrackEvent::Keyin { step_time, .. } => {
                    current_time += *step_time as u32;
                }
                TrackEvent::MidiChannel { step_time, .. } => {
                    current_time += *step_time as u32;
                }
                TrackEvent::Tempo { step_time, .. } => {
                    current_time += *step_time as u32;
                }
                TrackEvent::AfterTouch { step_time, .. } => {
                    current_time += *step_time as u32;
                }
                TrackEvent::ControlChange { step_time, .. } => {
                    current_time += *step_time as u32;
                }
                TrackEvent::ProgramChange { step_time, .. } => {
                    current_time += *step_time as u32;
                }
                TrackEvent::PolyphonicAfterTouch { step_time, .. } => {
                    current_time += *step_time as u32;
                }
                TrackEvent::PitchBend { step_time, .. } => {
                    current_time += *step_time as u32;
                }
                TrackEvent::Key { offset: _ } => {}
                TrackEvent::Comment { text: _ } => {}
                TrackEvent::RepeatEnd { count: _ } => {}
                TrackEvent::RepeatStart => {}
                TrackEvent::SameMeasure {
                    measure: _,
                    track_offset: _,
                } => {}
                TrackEvent::MeasureEnd => {}
                TrackEvent::EndOfTrack => {}
                TrackEvent::Unknown {
                    event_type: _,
                    data: _,
                } => {}
            }
        }

        track_events.push(midi_events);
    }

    // トラックごとのイベントをマージして、タイムライン上でソート
    let mut all_events: Vec<MidiEvent> = Vec::new();
    for events in track_events {
        all_events.extend(events);
    }

    all_events.sort_by_key(|e| e.tick);

    let serial_port = sted_midi::serial_port();

    // sted-midi の send 関数で、MIDI イベントを送信する
    let tempo = rcp_file.header_block.tempo;
    let timebase = rcp_file.header_block.time_base;

    // timebase の値は、4分音符あたりのティック数を示す
    // 例えば、timebase が 48 の場合、1ティックは ((60[sec] / tempo) / 48) 秒となる
    let tick_duration_ms = (60_000f32 / tempo as f32) / timebase as f32;

    let mut previous_tick = 0u32;
    for event in &all_events {
        let wait_ticks = event.tick - previous_tick;
        let wait_duration_ms = (wait_ticks as f32 * tick_duration_ms) as u64;

        std::thread::sleep(std::time::Duration::from_millis(wait_duration_ms));

        if let Err(e) = sted_midi::send(serial_port.try_clone().unwrap(), &event.bytes) {
            eprintln!("Error sending MIDI data: {}", e);
        }

        previous_tick = event.tick;
    }
}
