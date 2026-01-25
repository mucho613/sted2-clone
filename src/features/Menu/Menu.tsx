import { useState } from "react";
import MenuItem from "../MenuItem/MenuItem";
import { MENU_ITEMS, type MenuItemId } from "./constant/menuItems";
import { useMenuHotkeys } from "./useMenuHotkeys";
import { useNavigate } from "react-router";

function Menu() {
  const [selectedItemId, setSelectedItemId] = useState<MenuItemId>(MENU_ITEMS[0].id);

  const navigate = useNavigate();

  const handleMenuItemEnter = (menuItemId: MenuItemId): void => {
    switch (menuItemId) {
      case "EDIT_SET":
        navigate("/track-edit");
        break;
      default:
        break;
    }
  };

  useMenuHotkeys({
    selectedItemId,
    setSelectedItemId,
    onEnter: handleMenuItemEnter,
  });

  return (
    <div className="flex flex-col w-[66px] gap-y-1">
      {MENU_ITEMS.map((menuItem) => (
        <MenuItem
          key={menuItem.id}
          state={selectedItemId === menuItem.id ? "selected" : "normal"}
          label={menuItem.label}
        />
      ))}
    </div>
  );
}

export default Menu;
