import { onCleanup, onMount } from "solid-js";
import { TRACK_GAP, WHITE_KEY_HEIGHT, WHITE_KEY_WIDTH } from "./constant";
import { calculateKeyPositionOnlyBlackKeys, calculateKeyPositionOnlyWhiteKeys } from "./logic";
import { useInvoke } from "../../useInvoke";

export type PlayStatus = number[][];

function TrackMonitor() {
  // biome-ignore lint/style/useConst: ref を利用するので let でないと書き換えできない
  let whiteKeys: HTMLCanvasElement | undefined = undefined;
  // biome-ignore lint/style/useConst: ref を利用するので let でないと書き換えできない
  let activeWhiteKeys: HTMLCanvasElement | undefined = undefined;
  // biome-ignore lint/style/useConst: ref を利用するので let でないと書き換えできない
  let blackKeys: HTMLCanvasElement | undefined = undefined;
  // biome-ignore lint/style/useConst: ref を利用するので let でないと書き換えできない
  let activeBlackKeys: HTMLCanvasElement | undefined = undefined;

  const WIDTH = 600 as const;
  const HEIGHT = 400 as const;

  const { getPlayStatus } = useInvoke();

  onMount(() => {
    if (!whiteKeys || !activeWhiteKeys || !blackKeys || !activeBlackKeys) return;
    const whiteKeysContext = (whiteKeys as HTMLCanvasElement).getContext("2d");
    const activeWhiteKeysContext = (activeWhiteKeys as HTMLCanvasElement).getContext("2d");
    const blackKeysContext = (blackKeys as HTMLCanvasElement).getContext("2d");
    const activeBlackKeysContext = (activeBlackKeys as HTMLCanvasElement).getContext("2d");

    if (whiteKeysContext && blackKeysContext) {
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
    }

    let frame = requestAnimationFrame(loop);

    async function loop() {
      if (!activeWhiteKeysContext || !activeBlackKeysContext) return;

      const playStatus = await getPlayStatus();

      activeWhiteKeysContext.clearRect(0, 0, WIDTH, HEIGHT);
      activeBlackKeysContext.clearRect(0, 0, WIDTH, HEIGHT);

      // 白鍵のうち、押下中の鍵盤を描画
      activeWhiteKeysContext.fillStyle = "white";
      for (let i = 0; i < 16; i++) {
        const noteOnKeys = playStatus[i];

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
        const noteOnKeys = playStatus[i];

        for (const noteOnKey of noteOnKeys) {
          const position = calculateKeyPositionOnlyBlackKeys(noteOnKey);
          const { x, y, width, height } = {
            ...position,
            y: (WHITE_KEY_HEIGHT + TRACK_GAP) * i + position.y,
          };
          activeBlackKeysContext.fillRect(x, y, width, height);
        }
      }

      frame = requestAnimationFrame(loop);
    }

    onCleanup(() => cancelAnimationFrame(frame));
  });

  return (
    <div class="m-6">
      <h2 class="uppercase mb-2">Track monitor</h2>

      <div class="relative">
        <table>
          <thead>
            <tr>
              <th>Ch.</th>
            </tr>
          </thead>
          <tbody>
            {[...Array.from({ length: 16 })].map((_, i) => (
              <tr class="h-6">
                <td class="text-right">{i + 1}</td>
              </tr>
            ))}
          </tbody>
        </table>

        <div class="absolute top-4 left-[70px] mt-[8px]">
          <canvas class="absolute" ref={whiteKeys} width={WIDTH} height={HEIGHT} />
          <canvas class="absolute" ref={activeWhiteKeys} width={WIDTH} height={HEIGHT} />
          <canvas class="absolute" ref={blackKeys} width={WIDTH} height={HEIGHT} />
          <canvas class="absolute" ref={activeBlackKeys} width={WIDTH} height={HEIGHT} />
        </div>
      </div>
    </div>
  );
}

export default TrackMonitor;
