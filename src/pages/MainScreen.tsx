import { useState } from "react";
import { useInvoke } from "../useInvoke";

function MainScreen() {
  const [filePath, setFilePath] = useState(
    "C:\\TEST.RCP",
  );

  const [error, setError] = useState("");

  const { loadFile, play } = useInvoke();

  const handleLoad = () => {
    loadFile(filePath)
      .then(() => setError(""))
      .catch((error) => setError(error));
  };

  const handlePlay = () => {
    play()
      .then(() => {
        setError("");
      })
      .catch((error) => setError(error));
  };

  return (
    <div className="container m-4">
      <div>
        <input
          className="mb-1 w-full"
          value={filePath}
          type="text"
          onInput={(e) => setFilePath("A")}
        />
        <div className="flex gap-1">
          <div className="flex flex-col w-[66px] gap-1">
            <button
              className="pl-px text-left h-[17px] bg-sted-blue leading-none"
              type="button"
              onClick={handleLoad}
            >
              LOAD
            </button>
            <button
              className="pl-px text-left h-[17px] bg-sted-blue leading-none"
              type="button"
              onClick={handlePlay}
            >
              PLAY
            </button>
          </div>
        </div>
      </div>

      <p className="fixed bottom-4 left-4">{error}</p>
    </div>
  );
}

export default MainScreen;
