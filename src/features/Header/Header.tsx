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
      <dl className="flex *:bg-sted-blue">
        <dt className="pl-2 after:content-[':']">FREE</dt>
        <dd className="w-[65px] text-right pr-[9px] mr-1.5">{freeMemory}</dd>
        <dt className="pl-[9px] after:content-[':']">USED</dt>
        <dd className="w-[65px] text-right pr-[9px] mr-1.5">{usedMemory}</dd>
        <dt className="pl-[9px] after:content-[':']">TR.</dt>
        <dd className="w-[25px] text-right pr-[9px] mr-1.5">{trackNumber}</dd>
        <dt className="pl-[9px] after:content-[':']">MODULE</dt>
        <dd className="w-[113px] text-right pr-[9px]">{moduleName}</dd>
      </dl>

      {/* Title area */}
      <div className="h-8 w-[272px] bg-sted-blue border border-sted-gray">

      </div>
    </div>
  );
}

export default Header;
