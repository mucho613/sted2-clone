import MenuItem from "../MenuItem/MenuItem";
import { MENU_ITEMS } from "./constant/menuItems";

type Props = {
  selectedItemId: string;
}

function Menu(props: Props) {
  const { selectedItemId } = props;

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
