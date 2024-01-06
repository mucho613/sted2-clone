import { createSignal, onCleanup, onMount } from "solid-js";
import { BLACK_KEY_WIDTH, TRACK_GAP, WHITE_KEY_HEIGHT, WHITE_KEY_WIDTH } from "./constant";
import { calculateKeyPositionOnlyBlackKeys, calculateKeyPositionOnlyWhiteKeys } from "./logic";
import { useInvoke } from "../../useInvoke";
import { panText } from "./logic/pan";
import { useParamMonitor } from "./hooks/useParamMonitor";
import { useKeyboardMonitor } from "./hooks/useKeyboardMonitor";

export type MidiOutputStatus = {
  volume: number;
  expression: number;
  pan: number;
  reverb: number;
  chorus: number;
  cutoff_frequency: number;
  resonance: number;
  pitch_bend: number;
  note_on_keys: number[];
}[];

function TrackMonitor() {
  // biome-ignore lint/style/useConst: <explanation>
  let paramMonitor: HTMLCanvasElement | undefined = undefined;
  // biome-ignore lint/style/useConst: <explanation>
  let whiteKeys: HTMLCanvasElement | undefined = undefined;
  // biome-ignore lint/style/useConst: <explanation>
  let activeWhiteKeys: HTMLCanvasElement | undefined = undefined;
  // biome-ignore lint/style/useConst: <explanation>
  let blackKeys: HTMLCanvasElement | undefined = undefined;
  // biome-ignore lint/style/useConst: <explanation>
  let activeBlackKeys: HTMLCanvasElement | undefined = undefined;

  const { getPlayStatus } = useInvoke();

  const {
    width: paramMonitorWidth,
    height: paramMonitorHeight,
    render: renderParamMonitor,
  } = useParamMonitor();
  const {
    width: keyboardMonitorWidth,
    height: keyboardMonitorHeight,
    initialRender: initialRenderKeyboards,
    render: renderKeyboards,
  } = useKeyboardMonitor();

  const [midiOutputStatus, setMidiOutputStatus] = createSignal<MidiOutputStatus | null>(null);

  onMount(() => {
    if (!paramMonitor || !whiteKeys || !activeWhiteKeys || !blackKeys || !activeBlackKeys) return;

    const paramMonitorContext = (paramMonitor as HTMLCanvasElement).getContext("2d");
    const whiteKeysContext = (whiteKeys as HTMLCanvasElement).getContext("2d");
    const activeWhiteKeysContext = (activeWhiteKeys as HTMLCanvasElement).getContext("2d");
    const blackKeysContext = (blackKeys as HTMLCanvasElement).getContext("2d");
    const activeBlackKeysContext = (activeBlackKeys as HTMLCanvasElement).getContext("2d");

    if (!whiteKeysContext || !blackKeysContext) return;

    initialRenderKeyboards(whiteKeysContext, blackKeysContext);

    let frame = requestAnimationFrame(loop);

    onCleanup(() => cancelAnimationFrame(frame));

    async function loop() {
      const midiOutputStatus = await getPlayStatus();
      setMidiOutputStatus(midiOutputStatus);

      if (!paramMonitorContext || !activeWhiteKeysContext || !activeBlackKeysContext) return;

      renderParamMonitor(paramMonitorContext, midiOutputStatus);
      renderKeyboards(activeWhiteKeysContext, activeBlackKeysContext, midiOutputStatus);

      frame = requestAnimationFrame(loop);
    }
  });

  return (
    <>
      <h2 class="uppercase mb-2">Track monitor</h2>

      <div class="relative">
        <div class="absolute top-4 left-[34px] mt-[8px]">
          <canvas
            class="absolute"
            ref={paramMonitor}
            width={paramMonitorWidth}
            height={paramMonitorHeight}
          />
        </div>

        <div class="absolute top-4 left-[240px] mt-[8px]">
          <canvas
            class="absolute"
            ref={whiteKeys}
            width={keyboardMonitorWidth}
            height={keyboardMonitorHeight}
          />
          <canvas
            class="absolute"
            ref={activeWhiteKeys}
            width={keyboardMonitorWidth}
            height={keyboardMonitorHeight}
          />
          <canvas
            class="absolute"
            ref={blackKeys}
            width={keyboardMonitorWidth}
            height={keyboardMonitorHeight}
          />
          <canvas
            class="absolute"
            ref={activeBlackKeys}
            width={keyboardMonitorWidth}
            height={keyboardMonitorHeight}
          />
        </div>

        <table class="absolute">
          <thead>
            <tr>
              {["Ch.", "Vol.", "Exp.", "Pan", "Rev.", "Cho.", "Pitch"].map((label) => (
                <th class="w-8 font-normal font-kodenmachou-12 text-xs">{label}</th>
              ))}
            </tr>
          </thead>
          <tbody>
            {[...Array.from({ length: 16 })].map((_, i) => {
              const status = midiOutputStatus();
              if (!status) return <></>;

              const track = status[i];
              return (
                <tr class="h-6 font-kodenmachou-12 text-xs">
                  <td class="text-center">{i + 1}</td>
                  <td class="text-center">{track.volume}</td>
                  <td class="text-center">{track.expression}</td>
                  <td class="text-center">{panText(track.pan)}</td>
                  <td class="text-center">{track.reverb}</td>
                  <td class="text-center">{track.chorus}</td>
                  <td class="text-center">{track.pitch_bend - 8192}</td>
                </tr>
              );
            })}
          </tbody>
        </table>
      </div>
    </>
  );
}

export default TrackMonitor;
