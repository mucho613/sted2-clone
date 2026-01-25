const MENU_ITEMS = [
  { id: "LOAD", label: "LOAD" },
  { id: "SAVE", label: "SAVE" },
  { id: "EDIT_SET", label: "EDIT&SET" },
  { id: "PLAY", label: "PLAY" },
  { id: "RECORD", label: "RECORD" },
  { id: "FILTER", label: "FILTER" },
  { id: "TIME_KEY", label: "TIME&KEY" },
  { id: "TITLE", label: "TITLE" },
  { id: "MEMO", label: "MEMO" },
  { id: "PART_ASSIGN", label: "PART ASS" },
  { id: "RHYTHM_ASSIGN", label: "RHY ASSI" },
  { id: "USER_EXCLUSIVE", label: "USER EXC" },
  { id: "CM64_CONTROLLER", label: "CM64 CON" },
  { id: "SC55_CONTROLLER", label: "SC55 CON" },
  { id: "OPTION", label: "OPTION" },
  { id: "UNIT_SELECT", label: "UNIT SEL" },
  { id: "INIT", label: "INIT" },
  { id: "EXIT", label: "EXIT" },
] as const;

export type MenuItemId = (typeof MENU_ITEMS)[number]["id"];

export { MENU_ITEMS };
