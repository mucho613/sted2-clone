import { createSignal, onCleanup, onMount } from "solid-js";
import { TRACK_GAP, WHITE_KEY_HEIGHT, WHITE_KEY_WIDTH } from "./constant";
import {
  calculateKeyPositionOnlyBlackKeys,
  calculateKeyPositionOnlyWhiteKeys,
} from "./logic";
import { useInvoke } from "../../useInvoke";
import { createSign } from "crypto";
import { x, y } from "@tauri-apps/api/path-9b1e7ad5";

export type PlayStatus = {
  note_on_keys: number[];
  volume: number;
  expression: number;
  pan: number;
  cut_off_frequency: number;
  resonance: number;
}[];

function TrackMonitor() {
  // biome-ignore lint/style/useConst: ref を利用するので let でないと書き換えできない
  let controlChangeMonitor: HTMLCanvasElement | undefined = undefined;
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

  const [midiOutputStatus, setMidiOutputStatus] =
    createSignal<PlayStatus | null>(null);

  onMount(() => {
    if (
      !controlChangeMonitor ||
      !whiteKeys ||
      !activeWhiteKeys ||
      !blackKeys ||
      !activeBlackKeys
    )
      return;
    const controlChangeMonitorContext = (
      controlChangeMonitor as HTMLCanvasElement
    ).getContext("2d");
    const whiteKeysContext = (whiteKeys as HTMLCanvasElement).getContext("2d");
    const activeWhiteKeysContext = (
      activeWhiteKeys as HTMLCanvasElement
    ).getContext("2d");
    const blackKeysContext = (blackKeys as HTMLCanvasElement).getContext("2d");
    const activeBlackKeysContext = (
      activeBlackKeys as HTMLCanvasElement
    ).getContext("2d");

    if (whiteKeysContext && blackKeysContext) {
      // 白鍵を描画
      whiteKeysContext.fillStyle = "#777";
      for (let i = 0; i < 16; i++) {
        for (let j = 0; j < 128; j++) {
          const { x, y, width, height } = calculateKeyPositionOnlyWhiteKeys(j);
          whiteKeysContext.fillRect(
            x,
            (WHITE_KEY_HEIGHT + TRACK_GAP) * i + y,
            width,
            height
          );
        }
      }
      // 黒鍵を描画
      blackKeysContext.fillStyle = "#222";
      for (let i = 0; i < 16; i++) {
        for (let j = 0; j < 128; j++) {
          const { x, y, width, height } = calculateKeyPositionOnlyBlackKeys(j);
          blackKeysContext.fillRect(
            x,
            (WHITE_KEY_HEIGHT + TRACK_GAP) * i + y,
            width,
            height
          );
        }
      }
    }

    let frame = requestAnimationFrame(loop);

    async function loop() {
      if (
        !controlChangeMonitorContext ||
        !activeWhiteKeysContext ||
        !activeBlackKeysContext
      )
        return;

      const midiOutputStatus = await getPlayStatus();
      setMidiOutputStatus(midiOutputStatus);

      controlChangeMonitorContext.clearRect(0, 0, WIDTH, HEIGHT);
      activeWhiteKeysContext.clearRect(0, 0, WIDTH, HEIGHT);
      activeBlackKeysContext.clearRect(0, 0, WIDTH, HEIGHT);

      // Control change 周りの描画
      controlChangeMonitorContext.fillStyle = "#ccc";
      for (let i = 0; i < 16; i++) {
        const track = midiOutputStatus[i];
        const y = (WHITE_KEY_HEIGHT + TRACK_GAP) * i + WHITE_KEY_HEIGHT;
        {
          const { x, width, height } = {
            x: 0,
            width: 12,
            height: -(WHITE_KEY_HEIGHT * track.volume) / 127,
          };
          controlChangeMonitorContext.fillRect(x, y, width, height);
        }
        {
          const { x, width, height } = {
            x: 16,
            width: 12,
            height: -(WHITE_KEY_HEIGHT * track.expression) / 127,
          };
          controlChangeMonitorContext.fillRect(x, y, width, height);
        }
        {
          const { x, width, height } = {
            x: 32,
            width: 12,
            height: -(WHITE_KEY_HEIGHT * track.cut_off_frequency) / 127,
          };
          controlChangeMonitorContext.fillRect(x, y, width, height);
        }
        {
          const { x, width, height } = {
            x: 48,
            width: 12,
            height: -(WHITE_KEY_HEIGHT * track.resonance) / 127,
          };
          controlChangeMonitorContext.fillRect(x, y, width, height);
        }
      }

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
              {["Ch.", "Vol.", "Exp.", "Pan", "C.of", "Res."].map((label) => (
                <th class="w-8 font-normal font-12 text-xs">{label}</th>
              ))}
            </tr>
          </thead>
          <tbody>
            {[...Array.from({ length: 16 })].map((_, i) => {
              const status = midiOutputStatus();
              if (!status) return <></>;

              const track = status[i];
              return (
                <tr class="h-6">
                  <td class="text-right">{i + 1}</td>
                  <td class="text-right">{track.volume}</td>
                  <td class="text-right">{track.expression}</td>
                  <td class="text-right">{track.pan}</td>
                  <td class="text-right">{track.cut_off_frequency}</td>
                  <td class="text-right">{track.resonance}</td>
                </tr>
              );
            })}
          </tbody>
        </table>

        <div class="absolute top-4 left-[200px] mt-[8px]">
          <canvas
            class="absolute"
            ref={controlChangeMonitor}
            width={300}
            height={HEIGHT}
          />
        </div>

        <div class="absolute top-4 left-[250px] mt-[8px]">
          <canvas
            class="absolute"
            ref={whiteKeys}
            width={WIDTH}
            height={HEIGHT}
          />
          <canvas
            class="absolute"
            ref={activeWhiteKeys}
            width={WIDTH}
            height={HEIGHT}
          />
          <canvas
            class="absolute"
            ref={blackKeys}
            width={WIDTH}
            height={HEIGHT}
          />
          <canvas
            class="absolute"
            ref={activeBlackKeys}
            width={WIDTH}
            height={HEIGHT}
          />
        </div>
      </div>
    </div>
  );
}

export default TrackMonitor;
