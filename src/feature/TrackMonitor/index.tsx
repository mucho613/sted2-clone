import { onCleanup, onMount } from "solid-js";
import { TRACK_GAP, WHITE_KEY_HEIGHT, WHITE_KEY_WIDTH } from "./constant";
import { calculateKeyPositionOnlyBlackKeys, calculateKeyPositionOnlyWhiteKeys } from "./logic";

export type PlayStatus = {
  channels: {
    noteOnKeys: number[];
    pitchbend: number;
  }[];
};

type Props = {
  getPlayStatus: () => Promise<PlayStatus>;
};

function TrackMonitor(props: Props) {
  const canvas: HTMLCanvasElement | undefined = undefined;

  const WIDTH = 600 as const;
  const HEIGHT = 580 as const;

  onMount(() => {
    if (!canvas) return;
    const context = canvas.getContext("2d");

    let frame = requestAnimationFrame(loop);

    function loop() {
      frame = requestAnimationFrame(loop);

      if (!context) return;

      // props.getPlayStatus().then(
      //   (playStatus) => {
      //     const { channels } = playStatus;
      //     context.clearRect(0, 0, WIDTH, HEIGHT);

      //     // 白鍵を描画
      //     context.fillStyle = "white";
      //     for(let i = 0; i < 16; i++) {
      //       for(let j = 0; j < 128; j++) {
      //         const { x, y, width, height } = calculateKeyPositionOnlyWhiteKeys(j);
      //         context.fillRect(
      //           x,
      //           (WHITE_KEY_HEIGHT + TRACK_GAP) * i + y,
      //           width,
      //           height
      //         );
      //       }
      //     }

      //     // 白鍵のうち、押下中の鍵盤を描画
      //     context.fillStyle = "red";
      //     channels.forEach((channel, channelNumber) => {
      //       const { noteOnKeys } = channel;

      //       noteOnKeys.forEach((noteOnKey) => {
      //         const position = calculateKeyPositionOnlyWhiteKeys(noteOnKey);
      //         const {x, y, width, height} = {
      //           ...position,
      //           y: (WHITE_KEY_HEIGHT + TRACK_GAP) * channelNumber + position.y,
      //         }
      //         context.fillRect(x, y, width, height);
      //       });
      //     });

      //     // 黒鍵を描画
      //     context.fillStyle = "#222";
      //     for(let i = 0; i < 16; i++) {
      //       for(let j = 0; j < 128; j++) {
      //         const { x, y, width, height } = calculateKeyPositionOnlyBlackKeys(j);
      //         context.fillRect(
      //           x,
      //           (WHITE_KEY_HEIGHT + TRACK_GAP) * i + y,
      //           width,
      //           height
      //         );
      //       }
      //     }

      //     // 黒鍵のうち、押下中の鍵盤を描画
      //     context.fillStyle = "red";
      //     channels.forEach((channel, channelNumber) => {
      //       const { noteOnKeys } = channel;

      //       noteOnKeys.forEach((noteOnKey) => {
      //         const position = calculateKeyPositionOnlyWhiteKeys(noteOnKey);
      //         const {x, y, width, height} = {
      //           ...position,
      //           y: (WHITE_KEY_HEIGHT + TRACK_GAP) * channelNumber + position.y,
      //         }
      //         context.fillRect(x, y, width, height);
      //       });
      //     });
      //   }
      // );
    }

    onCleanup(() => cancelAnimationFrame(frame));
  });

  return (
    <>
      <h2>Track monitor</h2>
      <canvas ref={canvas} width={WIDTH} height={HEIGHT} />
    </>
  );
}

export default TrackMonitor;
