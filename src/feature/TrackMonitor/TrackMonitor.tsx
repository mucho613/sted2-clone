import { onCleanup, onMount } from "solid-js";
import { TRACK_GAP, WHITE_KEY_HEIGHT, WHITE_KEY_WIDTH } from "./constant";
import { calculateKeyPositionOnlyBlackKeys, calculateKeyPositionOnlyWhiteKeys } from "./logic";
import { useInvoke } from "../../useInvoke";

export type PlayStatus = number[][];

function TrackMonitor() {
  // biome-ignore lint/style/useConst: ref を利用するので let でないと書き換えできない
  let canvas: HTMLCanvasElement | undefined = undefined;

  const WIDTH = 600 as const;
  const HEIGHT = 400 as const;

  const { getPlayStatus } = useInvoke();

  onMount(() => {
    if (!canvas) return;
    const context = (canvas as HTMLCanvasElement).getContext("2d");

    let frame = requestAnimationFrame(loop);

    async function loop() {
      frame = requestAnimationFrame(loop);

      if (!context) return;

      const playStatus = await getPlayStatus();

      // console.log(playStatus);
      context.clearRect(0, 0, WIDTH, HEIGHT);

      // 白鍵を描画
      context.fillStyle = "white";
      for (let i = 0; i < 16; i++) {
        for (let j = 0; j < 128; j++) {
          const { x, y, width, height } = calculateKeyPositionOnlyWhiteKeys(j);
          context.fillRect(x, (WHITE_KEY_HEIGHT + TRACK_GAP) * i + y, width, height);
        }
      }

      // 白鍵のうち、押下中の鍵盤を描画
      context.fillStyle = "red";
      for (let i = 0; i < 16; i++) {
        const noteOnKeys = playStatus[i];

        for (const noteOnKey of noteOnKeys) {
          const position = calculateKeyPositionOnlyWhiteKeys(noteOnKey);
          const { x, y, width, height } = {
            ...position,
            y: (WHITE_KEY_HEIGHT + TRACK_GAP) * i + position.y,
          };
          context.fillRect(x, y, width, height);
        }
      }

      // 黒鍵を描画
      context.fillStyle = "#222";
      for (let i = 0; i < 16; i++) {
        for (let j = 0; j < 128; j++) {
          const { x, y, width, height } = calculateKeyPositionOnlyBlackKeys(j);
          context.fillRect(x, (WHITE_KEY_HEIGHT + TRACK_GAP) * i + y, width, height);
        }
      }

      // 黒鍵のうち、押下中の鍵盤を描画
      context.fillStyle = "red";
      for (let i = 0; i < 16; i++) {
        const noteOnKeys = playStatus[i];

        for (const noteOnKey of noteOnKeys) {
          const position = calculateKeyPositionOnlyBlackKeys(noteOnKey);
          const { x, y, width, height } = {
            ...position,
            y: (WHITE_KEY_HEIGHT + TRACK_GAP) * i + position.y,
          };
          context.fillRect(x, y, width, height);
        }
      }
    }

    onCleanup(() => cancelAnimationFrame(frame));
  });

  return <canvas ref={canvas} width={WIDTH} height={HEIGHT} />;
}

export default TrackMonitor;
