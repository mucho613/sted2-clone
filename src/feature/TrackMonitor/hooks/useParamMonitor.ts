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
      const { volume, expression, pan, reverb, chorus, pitch_bend } = midiOutputStatus[i];
      const params = Object.entries({ volume, expression, pan, reverb, chorus, pitch_bend });

      for (let j = 0; j < params.length; j++) {
        const [key, value] = params[j];
        const x = (CONTROL_CHANGE_GRAPH_WIDTH + CONTROL_CHANGE_GRAPH_GAP) * j;
        const y = (TRACK_HEIGHT + TRACK_GAP) * i + TRACK_HEIGHT;
        context.fillStyle = "#444";
        context.fillRect(x, y, CONTROL_CHANGE_GRAPH_WIDTH, -2);

        if (key === "pan") {
          const xBase =
            (CONTROL_CHANGE_GRAPH_WIDTH + CONTROL_CHANGE_GRAPH_GAP) * j +
            CONTROL_CHANGE_GRAPH_WIDTH / 2;
          const offset = ((value - 64) * CONTROL_CHANGE_GRAPH_WIDTH) / 127;
          const width = 2;
          const height = -8;

          context.fillStyle = "red";
          context.fillRect(xBase + offset - 1, y, width, height);
        }
        if (key === "pitch_bend") {
          const xBase =
            (CONTROL_CHANGE_GRAPH_WIDTH + CONTROL_CHANGE_GRAPH_GAP) * j +
            CONTROL_CHANGE_GRAPH_WIDTH / 2;
          const offset = ((value - 8192) * CONTROL_CHANGE_GRAPH_WIDTH) / 16384;
          const width = 2;
          const height = -8;

          context.fillStyle = "red";
          context.fillRect(xBase + offset - 1, y, width, height);
        } else {
          const width = (value * CONTROL_CHANGE_GRAPH_WIDTH) / 127;

          context.fillStyle = "#2a7a2a";
          context.fillRect(x, y, width, -2);
        }
      }
    }
  };

  return {
    width: WIDTH,
    height: HEIGHT,
    render,
  };
};
