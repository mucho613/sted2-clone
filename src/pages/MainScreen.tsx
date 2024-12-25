import { useState } from "react";
import { useInvoke } from "../useInvoke";
import MenuItem from "../features/MenuItem/MenuItem";
import Header from "../features/Header/Header";
import Menu from "../features/Menu/Menu";

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

      <div className="flex">
        {/* Left column */}
        <div className="flex flex-col gap-y-[11px]">
          <div className="w-[97ox] h-[57px] border border-sted-gray" />

          <div className="ml-[7px]">
            <Menu />
          </div>
        </div>

        {/* Right column */}
      </div>

      <p className="fixed bottom-4 left-4">{error}</p>
    </div>
  );
}

export default MainScreen;
