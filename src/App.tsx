import { createSignal } from "solid-js";
import TrackMonitor from "./feature/TrackMonitor";
import { useInvoke } from "./useInvoke";

function App() {
  const [filePath, setFilePath] = createSignal("");

  const [error, setError] = createSignal("");

  const { loadFile, play, getPlayStatus } = useInvoke();

  const handleLoad = () => {
    loadFile(filePath())
      .then(() => setError(""))
      .catch((error) => setError(error));
  };

  const handlePlay = () => {
    play()
      .then(() => setError(""))
      .catch((error) => setError(error));
  };

  return (
    <div class="container m-4">
      <div>
        <input class="mb-1 w-full" type="text" onInput={(e) => setFilePath(e.target.value)} />
        <div class="flex gap-1">
          <div class="flex flex-col w-[66px] gap-1">
            <button
              class="pl-px text-left h-[17px] bg-blue leading-none"
              type="button"
              onClick={handleLoad}
            >
              LOAD
            </button>
            <button
              class="pl-px text-left h-[17px] bg-blue leading-none"
              type="button"
              onClick={handlePlay}
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

      <p class="fixed bottom-4 left-4">{error()}</p>
    </div>
  );
}

export default App;
