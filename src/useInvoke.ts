import { invoke } from "@tauri-apps/api";
import { MidiOutputStatus } from "./feature/TrackMonitor/TrackMonitor";
import { MidiPort } from "./feature/Midi/type/port";

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

  function getMidiOutputPorts(): Promise<MidiPort[]> {
    return invoke("get_midi_output_ports");
  }

  function openMidiOutputPort(id: string): Promise<null> {
    return invoke("open_midi_output_port", { id });
  }

  return {
    loadFile,
    play,
    stop,
    getPlayStatus,
    getMidiOutputPorts,
    openMidiOutputPort,
  };
}
