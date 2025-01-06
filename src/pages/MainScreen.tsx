import { useState } from "react";
import Header from "../features/Header/Header";
import Menu from "../features/Menu/Menu";
import TrackList from "../features/TrackList/TrackList";

function MainScreen() {
  const [error, _setError] = useState("");

  return (
    <div className="mt-[33px] ml-7 flex flex-col bg-sted-black">
      <div className="mb-[11px]">
        <Header freeMemory={589356} usedMemory={0} trackNumber={1} moduleName={"Roland SC-55"} />
      </div>

      <div className="flex gap-x-[5px]">
        {/* Left column */}
        <div className="flex flex-col gap-y-[11px]">
          <div className="w-[97px] h-[57px] border border-sted-gray" />

          <div className="ml-[7px]">
            <Menu />
          </div>
        </div>

        {/* Right column */}
        {/* File & song info */}
        <div className="flex flex-col gap-y-1.5">
          <div className="flex flex-col w-[643px] h-[41px] border border-sted-gray" />
          <div className="flex gap-x-[7px]">
            <div className="flex flex-col w-[323px] h-[75px] border border-sted-gray" />
            <div className="flex flex-col w-[313px] h-[75px] border border-sted-gray" />
          </div>
          <TrackList />
        </div>
      </div>

      <p className="fixed bottom-4 left-4">{error}</p>
    </div>
  );
}

export default MainScreen;
