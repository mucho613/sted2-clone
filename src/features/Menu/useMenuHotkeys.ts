import { useHotkeys } from "react-hotkeys-hook";
import { MENU_ITEMS } from "./constant/menuItems";

type Params = {
  selectedItemId: string;
  setSelectedItemId: (id: string) => void;
};

export const useMenuHotkeys = (params: Params) => {
  const { selectedItemId, setSelectedItemId } = params;

  // 前の項目に移動
  useHotkeys(
    "ArrowUp",
    () => {
      const currentIndex = MENU_ITEMS.findIndex((item) => item.id === selectedItemId);
      const newIndex = (currentIndex - 1 + MENU_ITEMS.length) % MENU_ITEMS.length;
      setSelectedItemId(MENU_ITEMS[newIndex].id);
    },
    [selectedItemId],
  );

  // 次の項目に移動
  useHotkeys(
    "ArrowDown",
    () => {
      const currentIndex = MENU_ITEMS.findIndex((item) => item.id === selectedItemId);
      const newIndex = (currentIndex + 1) % MENU_ITEMS.length;
      setSelectedItemId(MENU_ITEMS[newIndex].id);
    },
    [selectedItemId],
  );

  // 各メニュー項目のショートカット
  useHotkeys("l", () => setSelectedItemId("LOAD"), []);
  useHotkeys("s", () => setSelectedItemId("SAVE"), []);
  useHotkeys("e", () => setSelectedItemId("EDIT_SET"), []);
  useHotkeys("p", () => setSelectedItemId("PLAY"), []);
  useHotkeys("r", () => setSelectedItemId("RECORD"), []);
  useHotkeys("f", () => setSelectedItemId("FILTER"), []);
  useHotkeys("k", () => setSelectedItemId("TIME_KEY"), []);
  useHotkeys("t", () => setSelectedItemId("TITLE"), []);
  useHotkeys("m", () => setSelectedItemId("MEMO"), []);
  useHotkeys("g", () => setSelectedItemId("PART_ASSIGN"), []);
  useHotkeys("a", () => setSelectedItemId("RHYTHM_ASSIGN"), []);
  useHotkeys("u", () => setSelectedItemId("USER_EXCLUSIVE"), []);
  useHotkeys("c", () => setSelectedItemId("CM64_CONTROLLER"), []);
  useHotkeys("shift+c", () => setSelectedItemId("SC55_CONTROLLER"), []);
  useHotkeys("o", () => setSelectedItemId("OPTION"), []);
  useHotkeys("d", () => setSelectedItemId("UNIT_SELECT"), []);
  useHotkeys("i", () => setSelectedItemId("INIT"), []);
  useHotkeys("x", () => setSelectedItemId("EXIT"), []);
};
