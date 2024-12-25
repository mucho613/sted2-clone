import { invoke } from "@tauri-apps/api/core";

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

  return {
    loadFile,
    play,
    stop,
  };
}
