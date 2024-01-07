import { createSignal } from "solid-js";
import { useInvoke } from "../useInvoke";
import { useNavigate } from "@solidjs/router";

function MainScreen() {
  const [filePath, setFilePath] = createSignal("");

  const [error, setError] = createSignal("");

  const { loadFile, play } = useInvoke();

  const navigate = useNavigate();

  const handleLoad = () => {
    loadFile(filePath())
      .then(() => setError(""))
      .catch((error) => setError(error));
  };

  const handlePlay = () => {
    play()
      .then(() => {
        setError("");
        navigate("/play-panel");
      })
      .catch((error) => setError(error));
  };

  return (
    <div class="container m-4">
      <div>
        <input class="mb-1 w-full" type="text" onInput={(e) => setFilePath(e.target.value)} />
        <div class="flex gap-1">
          <div class="flex flex-col w-[66px] gap-1">
            <button
              class="pl-px text-left h-[17px] bg-sted-blue leading-none"
              type="button"
              onClick={handleLoad}
            >
              LOAD
            </button>
            <button
              class="pl-px text-left h-[17px] bg-sted-blue leading-none"
              type="button"
              onClick={handlePlay}
            >
              PLAY
            </button>
          </div>
        </div>
      </div>

      <p class="fixed bottom-4 left-4">{error()}</p>
    </div>
  );
}

export default MainScreen;
