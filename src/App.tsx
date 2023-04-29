import { createSignal } from "solid-js";
import logo from "./assets/logo.svg";
import { invoke } from "@tauri-apps/api/tauri";
import "./App.css";

function App() {
  async function play() {
    await invoke("play");
  }

  return (
    <div class="container">
      <h1>STed2 Clone</h1>

      <div class="row">
        <div>
          <button type="button" onClick={() => play()}>
            Play
          </button>
        </div>
      </div>
    </div>
  );
}

export default App;
