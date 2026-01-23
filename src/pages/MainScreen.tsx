import { useState } from "react";
import Header from "../features/Header/Header";
import Menu from "../features/Menu/Menu";
import TrackList from "../features/TrackList/TrackList";
import SongInfo from "../features/SongInfo/SongInfo";
import { MENU_ITEMS } from "../features/Menu/constant/menuItems";
import { useMenuHotkeys } from "../features/Menu/useMenuHotkeys";

function MainScreen() {
  const [error, _setError] = useState("");
  const [selectedItemId, setSelectedItemId] = useState<string>(MENU_ITEMS[0].id);

  useMenuHotkeys({ selectedItemId, setSelectedItemId });

  return (
    <div className="mt-[33px] ml-7 flex flex-col bg-sted-black">
      <div className="mb-[11px]">
        <Header
          size="default"
          freeMemory={589356}
          usedMemory={0}
          trackNumber={1}
          moduleName="MODULE: TestModuleName"
        />
      </div>

      <div className="flex gap-x-[5px]">
        {/* Left column */}
        <div className="flex flex-col gap-y-[11px]">
          <div className="w-[97px] h-[57px] border border-sted-gray" />

          <div className="ml-[7px]">
            <Menu selectedItemId={selectedItemId} />
          </div>
        </div>

        {/* Right column */}
        {/* File & song info */}
        <div className="flex flex-col gap-y-1.5">
          <div className="flex flex-col w-[643px] h-[41px] border border-sted-gray pl-[9px] pt-1">
            <dl className="flex flex-wrap">
              <dt className="after:content-[':'] after:ml-4 w-[112px]">MUSIC TITLE</dt>
              <dd className="w-[512px]">TEST</dd>
              <dt className="after:content-[':'] after:ml-8 w-[112px]">FILE NAME</dt>
              <dd className="w-[512px] relative">
                <span>TEST.RCP</span>
                <span className="absolute left-[184px]">/</span>
                <span className="absolute left-[336px]">/</span>
              </dd>
            </dl>
          </div>
          <div className="flex gap-x-[7px]">
            <SongInfo
              tempo={120}
              timebase={48}
              playBias={0}
              keyName="C MAJOR"
              beat="4 / 4"
              midiInputDeviceName="InDevice"
            />
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
