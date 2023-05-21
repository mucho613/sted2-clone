import { createSignal, onCleanup, onMount } from "solid-js";
import { invoke } from "@tauri-apps/api/tauri";
import "./App.css";
import TrackMonitor from "./feature/TrackMonitor";

function App() {
  const [bytes, setBytes] = createSignal<number[]>([]);
  const [playPosition, setPlayPosition] = createSignal<number>(0);
  const [noteOnKeys, setNoteOnKeys] = createSignal<any>();

  function play() {
    invoke("play").then(() => console.log("Play end."));
  }

  function loadFile() {
    invoke(
      "load_file", 
      { filePath: "C:\\Users\\mucho\\workspace\\sted2\\sted2-clone\\src-tauri\\test\\ALLSTARS.MID" }
    );
  }

  async function getNoteOnKeys(): Promise<{[key: string]: number}> {
    // return invoke("get_note_on_keys");
    const noteOnKeys = await invoke("get_note_on_keys") as {[key: string]: number};
    // console.log(noteOnKeys);
    return noteOnKeys;
  }

  return (
    <div class="container">
      <div>
        <button type="button" onClick={() => loadFile()}>
          Load file
        </button>
        <button type="button" onClick={() => play()}>
          Play
        </button>
        <button type="button" onClick={() => getNoteOnKeys()}>
          Get note on keys
        </button>
      </div>

      <TrackMonitor getNoteOnKeys={getNoteOnKeys}/>
    </div>
  );
}

export default App;
