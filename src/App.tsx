import { createSignal } from "solid-js";
import TrackMonitor from "./feature/TrackMonitor";
import { useInvoke } from "./useInvoke";

function App() {
  const [filePath, setFilePath] = createSignal("");

  const { loadFile, play, getPlayStatus } = useInvoke();

  return (
    <div class="container m-4">
      <div>
        <input class="mb-1 w-full" type="text" onInput={(e) => setFilePath(e.target.value)} />
        <div class="flex gap-1">
          <div class="flex flex-col w-[66px] gap-1">
            <button
              class="pl-px text-left h-[17px] bg-blue leading-none"
              type="button"
              onClick={() => loadFile(filePath())}
            >
              LOAD
            </button>
            <button
              class="pl-px text-left h-[17px] bg-blue leading-none"
              type="button"
              onClick={play}
            >
              PLAY
            </button>
          </div>
          <div>
            <h2>Track Monitor</h2>
          </div>
        </div>
      </div>

      {/* <TrackMonitor getPlayStatus={getPlayStatus} /> */}
    </div>
  );
}

export default App;
