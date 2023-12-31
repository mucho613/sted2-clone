import { invoke } from "@tauri-apps/api";
import { PlayStatus } from "./feature/TrackMonitor";

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

  function getPlayStatus(): Promise<PlayStatus> {
    return invoke("get_play_status");
  }

  return {
    loadFile,
    play,
    stop,
    getPlayStatus,
  };
}
