type Props = {
  freeMemory: number;
  usedMemory: number;
  trackNumber: number;
  moduleName: string;
};

function Header(props: Props) {
  const { freeMemory, usedMemory, trackNumber, moduleName } = props;

  return (
    <div className="flex items-start">
      <dl className="flex *:bg-sted-blue mr-1.5">
        <dt className="pl-2 after:content-[':']">FREE</dt>
        <dd className="w-[65px] text-right pr-[9px] mr-1.5">{freeMemory}</dd>
        <dt className="pl-[9px] after:content-[':']">USED</dt>
        <dd className="w-[65px] text-right pr-[9px] mr-1.5">{usedMemory}</dd>
        <dt className="pl-[9px] after:content-[':']">TR.</dt>
        <dd className="w-[25px] text-right pr-[9px]">{trackNumber}</dd>
      </dl>

      <p className="w-[178px] h-4 px-px bg-sted-blue overflow-hidden whitespace-pre mr-[7px]">
        {moduleName}
      </p>

      {/* Title area */}
      <div className="h-8 w-[272px] bg-sted-blue border border-sted-gray" />
    </div>
  );
}

export default Header;
