import { MidiOutputStatus } from "../TrackMonitor";
import { TRACK_GAP, TRACK_HEIGHT, WHITE_KEY_HEIGHT, WHITE_KEY_WIDTH } from "../constant";
import { calculateKeyPositionOnlyBlackKeys, calculateKeyPositionOnlyWhiteKeys } from "../logic";

type ReturnType = {
  width: number;
  height: number;
  initialRender: (
    whiteKeysContext: CanvasRenderingContext2D,
    blackKeysContext: CanvasRenderingContext2D,
  ) => void;
  render: (
    activeWhiteKeysContext: CanvasRenderingContext2D,
    activeBlackKeysContext: CanvasRenderingContext2D,
    midiOutputStatus: MidiOutputStatus,
  ) => void;
};

const WIDTH = WHITE_KEY_WIDTH * 75; // 75 = 12 * 6 + 3
const HEIGHT = TRACK_HEIGHT * 16 + TRACK_GAP * 15;

export const useKeyboardMonitor = (): ReturnType => {
  const initialRender = (
    whiteKeysContext: CanvasRenderingContext2D,
    blackKeysContext: CanvasRenderingContext2D,
  ) => {
    // 白鍵を描画
    whiteKeysContext.fillStyle = "#777";
    for (let i = 0; i < 16; i++) {
      for (let j = 0; j < 128; j++) {
        const { x, y, width, height } = calculateKeyPositionOnlyWhiteKeys(j);
        whiteKeysContext.fillRect(x, (WHITE_KEY_HEIGHT + TRACK_GAP) * i + y, width, height);
      }
    }
    // 黒鍵を描画
    blackKeysContext.fillStyle = "#222";
    for (let i = 0; i < 16; i++) {
      for (let j = 0; j < 128; j++) {
        const { x, y, width, height } = calculateKeyPositionOnlyBlackKeys(j);
        blackKeysContext.fillRect(x, (WHITE_KEY_HEIGHT + TRACK_GAP) * i + y, width, height);
      }
    }
  };

  const render = (
    activeWhiteKeysContext: CanvasRenderingContext2D,
    activeBlackKeysContext: CanvasRenderingContext2D,
    midiOutputStatus: MidiOutputStatus,
  ) => {
    if (!activeWhiteKeysContext || !activeBlackKeysContext) return;

    activeWhiteKeysContext.clearRect(0, 0, WIDTH, HEIGHT);
    activeBlackKeysContext.clearRect(0, 0, WIDTH, HEIGHT);

    // 白鍵のうち、押下中の鍵盤を描画
    activeWhiteKeysContext.fillStyle = "white";

    const tracks = midiOutputStatus.map((track) => track.note_on_keys);
    for (let i = 0; i < 16; i++) {
      const noteOnKeys = tracks[i];

      for (const noteOnKey of noteOnKeys) {
        const position = calculateKeyPositionOnlyWhiteKeys(noteOnKey);
        const { x, y, width, height } = {
          ...position,
          y: (WHITE_KEY_HEIGHT + TRACK_GAP) * i + position.y,
        };
        activeWhiteKeysContext.fillRect(x, y, width, height);
      }
    }

    // 黒鍵のうち、押下中の鍵盤を描画
    activeBlackKeysContext.fillStyle = "white";
    for (let i = 0; i < 16; i++) {
      const noteOnKeys = tracks[i];

      for (const noteOnKey of noteOnKeys) {
        const position = calculateKeyPositionOnlyBlackKeys(noteOnKey);
        const { x, y, width, height } = {
          ...position,
          y: (WHITE_KEY_HEIGHT + TRACK_GAP) * i + position.y,
        };
        activeBlackKeysContext.fillRect(x, y, width, height);
      }
    }
  };

  return {
    width: WIDTH,
    height: HEIGHT,
    initialRender,
    render,
  };
};
