type Track = {
  trackNumber: number;
  mode: string;
  midiOutput?: TrackMidiOutput;
  stepOffset: number;
  keyOffset?: number;
  rhythmTrack?: boolean;
  trackMemo: string;
  step: number;
  totalStep: number;
};

type TrackMidiOutput = {
  port: string;
  channel: number;
};

export type { Track, TrackMidiOutput };
