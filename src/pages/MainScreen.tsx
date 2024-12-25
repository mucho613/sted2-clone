import { useState } from "react";
import { useInvoke } from "../useInvoke";
import MenuItem from "../features/MenuItem/MenuItem";

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
        <div className="flex flex-col w-[66px] gap-1">
          <MenuItem label="LOAD" isSelected={false} onClick={handleLoad} />
          <MenuItem label="PLAY" isSelected={false} onClick={handlePlay} />
        </div>
      </div>

      <p className="fixed bottom-4 left-4">{error}</p>
    </div>
  );
}

export default MainScreen;
