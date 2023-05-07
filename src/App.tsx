import { createSignal, onCleanup, onMount } from "solid-js";
import { invoke } from "@tauri-apps/api/tauri";
import "./App.css";

function App() {
  const [bytes, setBytes] = createSignal<number[]>([]);
  const [playPosition, setPlayPosition] = createSignal<number>(0);

  function play() {
    invoke("play").then(() => console.log("hoge"));
  }

  function loadFile() {
    invoke(
      "load_file", 
      { filePath: "C:\\Users\\mucho\\workspace\\sted2\\sted2-clone\\src-tauri\\test\\ALLSTARS.MID" }
    );
  }

  async function getPlayPosition() {
    setPlayPosition(await invoke("get_play_position"));
  }

  onMount(() => {
    let frame = requestAnimationFrame(loop);

  function loop() {
    frame = requestAnimationFrame(loop);
    getPlayPosition();
  }
    
    onCleanup(() => cancelAnimationFrame(frame))
  })

  return (
    <div class="container">
      <div>
        <button type="button" onClick={() => loadFile()}>
          Load file
        </button>
        <button type="button" onClick={() => play()}>
          Play
        </button>
        <button type="button" onClick={() => getPlayPosition()}>
          Get play position
        </button>
      </div>
      <h2>Binary viewer</h2>
      <p>Play position: {playPosition()}</p>
      <div class="binary-viewer">
        {bytes().map(byte => <span>{byte.toString(16).toUpperCase().padStart(2, "0")}</span>)}
      </div>
    </div>
  );
}

export default App;
