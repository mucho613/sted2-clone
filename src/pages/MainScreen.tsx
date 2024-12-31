import { useState } from "react";
import Header from "../features/Header/Header";
import Menu from "../features/Menu/Menu";
import TrackList from "../features/TrackList/TrackList";

function MainScreen() {
  const [error, _setError] = useState("");

  return (
    <div className="flex flex-col bg-sted-black">
      <div className="mb-[11px]">
        <Header freeMemory={0} usedMemory={0} trackNumber={0} moduleName={"Roland SC-55"} />
      </div>

      <div className="flex gap-x-[5px]">
        {/* Left column */}
        <div className="flex flex-col gap-y-[11px]">
          <div className="w-[97ox] h-[57px] border border-sted-gray" />

          <div className="ml-[7px]">
            <Menu />
          </div>
        </div>

        {/* Right column */}
        <div className="flex flex-col gap-y-1.5">
          {/* File & song info */}
          <div className="flex flex-col w-[643px] h-[122px] border border-sted-gray" />
          <TrackList />
        </div>
      </div>

      <p className="fixed bottom-4 left-4">{error}</p>
    </div>
  );
}

export default MainScreen;
