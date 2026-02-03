use recomposer_file::{RcpFile, event::types::TrackEvent};

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
}

impl std::fmt::Display for PlayError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PlayError::SerialPort(err) => write!(f, "Failed to open serial port: {err}"),
            PlayError::SerialSend(err) => write!(f, "Failed to send serial MIDI data: {err}"),
            PlayError::MidiPort(err) => write!(f, "Failed to open MIDI output: {err}"),
            PlayError::MidiSend(err) => write!(f, "Failed to send MIDI data: {err}"),
        }
    }
}

impl std::error::Error for PlayError {}

use crate::song_info::display_song_info;

struct MidiEvent {
    tick: u32,
    bytes: Vec<u8>,
}

/// Play the loaded Recomposer format file
pub fn play(rcp_file: &RcpFile, output: OutputTarget) -> Result<(), PlayError> {
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
