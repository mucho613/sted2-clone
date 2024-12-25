import { useState } from "react";
import { useInvoke } from "../useInvoke";
import MenuItem from "../features/MenuItem/MenuItem";
import Header from "../features/Header/Header";

function MainScreen() {
  const [filePath] = useState(
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
    <div className="p-4 flex flex-col">
      <div className="mb-[11px]">
        <Header freeMemory={0} usedMemory={0} trackNumber={0} moduleName={"Roland SC-55"} />
      </div>
      <div>
        {/* Left column */}
        <div className="flex flex-col w-[66px] gap-1">
          <MenuItem label="LOAD" isSelected={false} onClick={handleLoad} />
          <MenuItem label="PLAY" isSelected={false} onClick={handlePlay} />
        </div>

        {/* Right column */}
      </div>

      <p className="fixed bottom-4 left-4">{error}</p>
    </div>
  );
}

export default MainScreen;
