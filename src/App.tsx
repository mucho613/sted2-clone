import { createSignal } from "solid-js";
import logo from "./assets/logo.svg";
import { invoke } from "@tauri-apps/api/tauri";
import "./App.css";
import { emit, listen } from '@tauri-apps/api/event'

function App() {
  const [bytes, setBytes] = createSignal<number[]>([]);

  function play() {
    invoke("play");

  }

  async function loadFile() {
    emit("front-to-back", "hello!");
    await invoke(
      "load_file", 
      { filePath: "C:\\Users\\mucho\\workspace\\sted2\\sted2-clone\\src-tauri\\test\\ALLSTARS.MID" }
    ) as number[];
  }

  listen('back-to-front', event => {
    console.log('back-to-front');
  });

  return (
    <div class="container">
      <div>
        <button type="button" onClick={() => loadFile()}>
          Load file
        </button>
        <button type="button" onClick={() => play()}>
          Play
        </button>
      </div>
      <h2>Binary viewer</h2>
      <div class="binary-viewer">
        {bytes().map(byte => <span>{byte.toString(16).toUpperCase().padStart(2, "0")}</span>)}
      </div>
    </div>
  );
}

export default App;
