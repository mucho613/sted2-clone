import { invoke } from "@tauri-apps/api";
import { MidiOutputStatus } from "./feature/TrackMonitor/TrackMonitor";

export function useInvoke() {
  function loadFile(filePath: string): Promise<null> {
    return invoke("load_file", { filePath });
  }

  function play() {
    return invoke("play");
  }

  function stop() {
    return invoke("stop");
  }

  function getPlayStatus(): Promise<MidiOutputStatus> {
    return invoke("get_play_status");
  }

  return {
    loadFile,
    play,
    stop,
    getPlayStatus,
  };
}
