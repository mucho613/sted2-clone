use recomposer_file::{
    RcpFile,
    event::types::TrackEvent,
    track_block::types::Track,
};

#[derive(Debug, Clone)]
pub enum OutputTarget {
    Serial { port_name: String },
    Midi { port_name: String },
}

#[derive(Debug)]
pub enum PlayError {
    SerialPort(sted_midi::SerialPortError),
    SerialSend(std::io::Error),
    MidiPort(sted_midi::MidiOutputError),
    MidiSend(sted_midi::SendError),
    InvalidTrackSequence { track_number: u8, reason: String },
}

impl std::fmt::Display for PlayError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PlayError::SerialPort(err) => write!(f, "Failed to open serial port: {err}"),
            PlayError::SerialSend(err) => write!(f, "Failed to send serial MIDI data: {err}"),
            PlayError::MidiPort(err) => write!(f, "Failed to open MIDI output: {err}"),
            PlayError::MidiSend(err) => write!(f, "Failed to send MIDI data: {err}"),
            PlayError::InvalidTrackSequence {
                track_number,
                reason,
            } => write!(f, "Invalid track sequence (track {track_number}): {reason}"),
        }
    }
}

impl std::error::Error for PlayError {}

use crate::song_info::display_song_info;

struct MidiEvent {
    tick: u32,
    bytes: Vec<u8>,
}

fn split_measures(track_events: &[TrackEvent]) -> Vec<Vec<TrackEvent>> {
    let mut measures = Vec::new();
    let mut current_measure = Vec::new();

    for event in track_events {
        match event {
            TrackEvent::MeasureEnd => {
                measures.push(std::mem::take(&mut current_measure));
            }
            _ => current_measure.push(event.clone()),
        }
    }

    if !current_measure.is_empty() {
        measures.push(current_measure);
    }

    measures
}

fn resolve_measure(
    index: usize,
    measures: &[Vec<TrackEvent>],
    cache: &mut [Option<Vec<TrackEvent>>],
    visiting: &mut [bool],
    track_number: u8,
) -> Result<Vec<TrackEvent>, PlayError> {
    if let Some(cached) = &cache[index] {
        return Ok(cached.clone());
    }

    if visiting[index] {
        return Err(PlayError::InvalidTrackSequence {
            track_number,
            reason: format!("SameMeasure loop detected at measure {}", index + 1),
        });
    }

    visiting[index] = true;

    let mut same_measure: Option<(u8, u16)> = None;
    for event in &measures[index] {
        if let TrackEvent::SameMeasure {
            measure,
            track_offset,
        } = event
        {
            if same_measure.is_some() {
                return Err(PlayError::InvalidTrackSequence {
                    track_number,
                    reason: format!(
                        "Multiple SameMeasure events found in measure {}",
                        index + 1
                    ),
                });
            }
            same_measure = Some((*measure, *track_offset));
        }
    }

    let resolved = if let Some((measure, track_offset)) = same_measure {
        if track_offset != 0 {
            return Err(PlayError::InvalidTrackSequence {
                track_number,
                reason: format!(
                    "SameMeasure track_offset {track_offset} is not supported (measure {})",
                    index + 1
                ),
            });
        }

        if measure == 0 {
            return Err(PlayError::InvalidTrackSequence {
                track_number,
                reason: format!("SameMeasure references invalid measure 0 (measure {})", index + 1),
            });
        }

        let target_index = (measure - 1) as usize;
        if target_index >= measures.len() {
            return Err(PlayError::InvalidTrackSequence {
                track_number,
                reason: format!(
                    "SameMeasure references out-of-range measure {} (measure {})",
                    measure,
                    index + 1
                ),
            });
        }

        resolve_measure(target_index, measures, cache, visiting, track_number)?
    } else {
        measures[index]
            .iter()
            .filter(|event| !matches!(event, TrackEvent::SameMeasure { .. }))
            .cloned()
            .collect()
    };

    visiting[index] = false;
    cache[index] = Some(resolved.clone());

    Ok(resolved)
}

fn resolve_same_measures(
    measures: &[Vec<TrackEvent>],
    track_number: u8,
) -> Result<Vec<Vec<TrackEvent>>, PlayError> {
    let mut cache = vec![None; measures.len()];
    let mut visiting = vec![false; measures.len()];
    let mut resolved = Vec::with_capacity(measures.len());

    for index in 0..measures.len() {
        let measure_events = resolve_measure(index, measures, &mut cache, &mut visiting, track_number)?;
        resolved.push(measure_events);
    }

    Ok(resolved)
}

fn expand_repeats(
    track_events: &[TrackEvent],
    track_number: u8,
) -> Result<Vec<TrackEvent>, PlayError> {
    let mut output: Vec<TrackEvent> = Vec::new();
    let mut repeat_stack: Vec<usize> = Vec::new();

    for event in track_events {
        match event {
            TrackEvent::RepeatStart => {
                repeat_stack.push(output.len());
            }
            TrackEvent::RepeatEnd { count } => {
                let start_index = match repeat_stack.pop() {
                    Some(index) => index,
                    None => {
                        return Err(PlayError::InvalidTrackSequence {
                            track_number,
                            reason: "RepeatEnd found without matching RepeatStart".to_string(),
                        });
                    }
                };

                let segment: Vec<TrackEvent> = output[start_index..].to_vec();
                output.truncate(start_index);

                for _ in 0..(*count as usize) {
                    output.extend(segment.iter().cloned());
                }
            }
            _ => output.push(event.clone()),
        }
    }

    if !repeat_stack.is_empty() {
        return Err(PlayError::InvalidTrackSequence {
            track_number,
            reason: "RepeatStart found without matching RepeatEnd".to_string(),
        });
    }

    Ok(output)
}

fn expand_track_events(track: &Track) -> Result<Vec<TrackEvent>, PlayError> {
    let measures = split_measures(&track.track_events);
    let resolved_measures = resolve_same_measures(&measures, track.track_header.track_number)?;

    let mut flattened: Vec<TrackEvent> = Vec::new();
    for measure in resolved_measures {
        flattened.extend(measure);
    }

    expand_repeats(&flattened, track.track_header.track_number)
}

/// Play the loaded Recomposer format file
pub fn play(rcp_file: &RcpFile, output: OutputTarget) -> Result<(), PlayError> {
    display_song_info(&rcp_file);

    let mut track_events: Vec<Vec<MidiEvent>> = Vec::new();

    // 各トラックを走査
    for track in &rcp_file.track_block.tracks {
        let mut current_time = 0u32;

        let mut midi_events: Vec<MidiEvent> = Vec::new();

        let expanded_events = expand_track_events(track)?;

        for event in &expanded_events {
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
                TrackEvent::RolandAddressAndParameter { step_time, .. } => {
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
                TrackEvent::AfterTouch {
                    step_time,
                    pressure,
                } => {
                    current_time += *step_time as u32;
                    // AfterTouch イベント
                    midi_events.push(MidiEvent {
                        tick: current_time,
                        bytes: vec![0xA0 | (track.track_header.channel & 0x0F), *pressure],
                    });
                }
                TrackEvent::ControlChange {
                    step_time,
                    controller_number,
                    value,
                } => {
                    // Control Change イベント
                    midi_events.push(MidiEvent {
                        tick: current_time,
                        bytes: vec![
                            0xB0 | (track.track_header.channel & 0x0F),
                            *controller_number,
                            *value,
                        ],
                    });
                    current_time += *step_time as u32;
                }
                TrackEvent::ProgramChange {
                    step_time,
                    program_number,
                } => {
                    // Program Change イベント
                    midi_events.push(MidiEvent {
                        tick: current_time,
                        bytes: vec![0xC0 | (track.track_header.channel & 0x0F), *program_number],
                    });
                    current_time += *step_time as u32;
                }
                TrackEvent::PolyphonicAfterTouch { step_time, .. } => {
                    current_time += *step_time as u32;
                }
                TrackEvent::PitchBend { step_time, value } => {
                    // Pitch Bend イベント
                    midi_events.push(MidiEvent {
                        tick: current_time,
                        bytes: vec![
                            0xE0 | (track.track_header.channel & 0x0F),
                            *value as u8 & 0x7F,
                            (*value as u8 >> 7) & 0x7F,
                        ],
                    });
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

    enum OutputConnection {
        Serial(Box<dyn std::io::Write + Send>),
        Midi(sted_midi::MidiOutputConnection),
    }

    let mut output_connection = match output {
        OutputTarget::Serial { port_name } => {
            let port = sted_midi::serial_port(&port_name).map_err(PlayError::SerialPort)?;
            OutputConnection::Serial(port)
        }
        OutputTarget::Midi { port_name } => {
            let connection = sted_midi::midi_output_connection_by_name(&port_name)
                .map_err(PlayError::MidiPort)?;
            OutputConnection::Midi(connection)
        }
    };

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

        let send_result = match &mut output_connection {
            OutputConnection::Serial(port) => {
                sted_midi::send(port.as_mut(), &event.bytes).map_err(PlayError::SerialSend)
            }
            OutputConnection::Midi(connection) => {
                sted_midi::send_midi(connection, &event.bytes).map_err(PlayError::MidiSend)
            }
        };

        if let Err(e) = send_result {
            return Err(e);
        }

        previous_tick = event.tick;
    }

    Ok(())
}
