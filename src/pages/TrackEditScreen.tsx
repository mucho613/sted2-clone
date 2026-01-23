import { useState } from "react";
import Header from "../features/Header/Header";
import { MENU_ITEMS } from "../features/Menu/constant/menuItems";
import { useMenuHotkeys } from "../features/Menu/useMenuHotkeys";

function TrackEditScreen() {
  const [error, _setError] = useState("");
  const [selectedItemId, setSelectedItemId] = useState<string>(MENU_ITEMS[0].id);

  useMenuHotkeys({ selectedItemId, setSelectedItemId });

  return (
    <div className="mt-[33px] ml-7 flex flex-col bg-sted-black">
      <div className="mb-[11px]">
        <Header
          size="half"
          freeMemory={589356}
          usedMemory={0}
          trackNumber={1}
          moduleName="MODULE: TestModuleName"
        />

        {/* Message area */}
        <div className="w-full h-4" />
      </div>

      <p className="fixed bottom-4 left-4">{error}</p>
    </div>
  );
}

export default TrackEditScreen;
