import MenuItem from "../MenuItem/MenuItem";

function Menu() {
  const menuItems = [
    "LOAD",
    "SAVE",
    "EDIT&SET",
    "PLAY",
    "RECORD",
    "FILTER",
    "TIME&KEY",
    "TITLE",
    "MEMO",
    "PART ASS",
    "RHY ASSI",
    "USER EXC",
    "CM64 CON",
    "SC55 CON",
    "OPTION",
    "UNIT SEL",
    "INIT",
    "EXIT"
  ]

  return (
    <div className="flex flex-col w-[66px] gap-y-1">
      {menuItems.map(menuItem => <MenuItem key={menuItem} label={menuItem} />)}
    </div>
  );
}

export default Menu;
