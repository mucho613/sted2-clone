import { createSignal } from "solid-js";
import TrackMonitor from "../feature/TrackMonitor/TrackMonitor";
import { useInvoke } from "../useInvoke";
import { useNavigate } from "@solidjs/router";

function PlayPanel() {
  const [error, setError] = createSignal("");

  const { stop } = useInvoke();

  const navigate = useNavigate();

  const handleStop = () => {
    stop()
      .then(() => setError(""))
      .catch((error) => setError(error));
    navigate("/");
  };

  return (
    <div class="container m-4">
      <div class="flex gap-1 mb-4">
        <button
          class="pl-px w-[66px] text-left h-[17px] bg-sted-blue leading-none"
          type="button"
          onClick={handleStop}
        >
          STOP
        </button>
      </div>
      <TrackMonitor />

      <p class="fixed bottom-4 left-4">{error()}</p>
    </div>
  );
}

export default PlayPanel;
