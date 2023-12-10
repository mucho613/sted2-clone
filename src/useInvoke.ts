import { invoke } from "@tauri-apps/api";
import { PlayStatus } from "./feature/TrackMonitor";

export function useInvoke() {
  function loadFile(filePath: string) {
    invoke("load_file", { filePath });
  }

  function play() {
    invoke("play");
  }

  function getPlayStatus(): Promise<PlayStatus> {
    return invoke("get_play_status");
  }

  return {
    loadFile,
    play,
    getPlayStatus,
  };
}
