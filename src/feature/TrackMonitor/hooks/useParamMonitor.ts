import { MidiOutputStatus } from "../TrackMonitor";
import { TRACK_GAP, TRACK_HEIGHT } from "../constant";

type ReturnType = {
  width: number;
  height: number;
  render: (context: CanvasRenderingContext2D, MidiOutputStatus: MidiOutputStatus) => void;
};

const WIDTH = 300 as const;
const HEIGHT = TRACK_HEIGHT * 16 + TRACK_GAP * 15;

export const useParamMonitor = (): ReturnType => {
  const CONTROL_CHANGE_GRAPH_WIDTH = 28 as const;
  const CONTROL_CHANGE_GRAPH_GAP = 4 as const;

  const render = (context: CanvasRenderingContext2D, midiOutputStatus: MidiOutputStatus) => {
    context.clearRect(0, 0, WIDTH, HEIGHT);

    // Control change 周りの描画
    context.fillStyle = "#2a7a2a";
    for (let i = 0; i < 16; i++) {
      const track = midiOutputStatus[i];
      const y = (TRACK_HEIGHT + TRACK_GAP) * i + TRACK_HEIGHT;
      const height = -4;
      {
        const { x, width } = {
          x: 0,
          width: (track.volume * CONTROL_CHANGE_GRAPH_WIDTH) / 127,
        };
        context.fillRect(x, y, width, height);
      }
      {
        const { x, width } = {
          x: CONTROL_CHANGE_GRAPH_WIDTH + CONTROL_CHANGE_GRAPH_GAP,
          width: (track.expression * CONTROL_CHANGE_GRAPH_WIDTH) / 127,
        };
        context.fillRect(x, y, width, height);
      }
      context.fillStyle = "red";
      {
        const baseX = (CONTROL_CHANGE_GRAPH_WIDTH + CONTROL_CHANGE_GRAPH_GAP) * 2;
        const { x, width, height } = {
          x: baseX + CONTROL_CHANGE_GRAPH_WIDTH / 2,
          width: ((track.pan - 64) * CONTROL_CHANGE_GRAPH_WIDTH) / 127,
          height: -4,
        };
        context.fillRect(x, y, width, height);
      }
      context.fillStyle = "#2a7a2a";
      {
        const { x, width } = {
          x: (CONTROL_CHANGE_GRAPH_WIDTH + CONTROL_CHANGE_GRAPH_GAP) * 3,
          width: (track.reverb * CONTROL_CHANGE_GRAPH_WIDTH) / 127,
        };
        context.fillRect(x, y, width, height);
      }
      {
        const { x, width } = {
          x: (CONTROL_CHANGE_GRAPH_WIDTH + CONTROL_CHANGE_GRAPH_GAP) * 4,
          width: (track.chorus * CONTROL_CHANGE_GRAPH_WIDTH) / 127,
        };
        context.fillRect(x, y, width, height);
      }
      {
        const { x, width } = {
          x: (CONTROL_CHANGE_GRAPH_WIDTH + CONTROL_CHANGE_GRAPH_GAP) * 5,
          width: (track.cut_off_frequency * CONTROL_CHANGE_GRAPH_WIDTH) / 127,
        };
        context.fillRect(x, y, width, height);
      }
      {
        const { x, width } = {
          x: (CONTROL_CHANGE_GRAPH_WIDTH + CONTROL_CHANGE_GRAPH_GAP) * 6,
          width: (track.resonance * CONTROL_CHANGE_GRAPH_WIDTH) / 127,
        };
        context.fillRect(x, y, width, height);
      }
    }
  };

  return {
    width: WIDTH,
    height: HEIGHT,
    render,
  };
};
