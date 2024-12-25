type Props = {
  freeMemory: number;
  usedMemory: number;
  trackNumber: number;
  moduleName: string;
}

function Header(props: Props) {
  const {
    freeMemory,
    usedMemory,
    trackNumber,
    moduleName
  } = props;

  return (
    <div className="flex items-start gap-x-[7px]">
      <div className="flex gap-x-1.5 *:bg-sted-blue *:pl-2">
        <div className="w-[113px] pr-[9px]">FREE: {freeMemory}</div>
        <div className="w-[114px] pr-2.5">USED: {usedMemory}</div>
        <div className="w-[66px] pr-2.5">TR.: {trackNumber}</div>
        <div className="w-[178px] pr-2.5">MODULE: {moduleName}</div>
      </div>

      {/* Title area */}
      <div className="h-8 w-[272px] bg-sted-blue border border-sted-gray">

      </div>
    </div>
  );
}

export default Header;
