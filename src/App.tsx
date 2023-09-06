import { createSignal } from "solid-js";
import TrackMonitor from "./feature/TrackMonitor";
import { useInvoke } from "./useInvoke";

function App() {
  const [filePath, setFilePath] = createSignal("");

  const { loadFile, play, getPlayStatus } = useInvoke();

  return (
    <div class="container">
      <div>
        <input type="text" onInput={(e) => setFilePath(e.target.value)}/>
        <button type="button" onClick={() => loadFile(filePath())}>
          Load file
        </button>
        <button type="button" onClick={play}>
          Play
        </button>
      </div>

      <TrackMonitor getPlayStatus={getPlayStatus}/>
    </div>
  );
}

export default App;
