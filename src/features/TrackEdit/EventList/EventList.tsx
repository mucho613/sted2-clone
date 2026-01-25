type Props = {
  trackMemo: string;
  trackNumber: number;
  measures: number;
  port: string;
  channel: number;
  usedMemory: number;
};

function EventList(props: Props) {
  const {
    trackMemo,
    trackNumber,
    measures,
    port,
    channel,
    usedMemory
  } = props;

  return <div className="w-[306px]">
    <div className="bg-sted-blue h-[17px]">M:{trackMemo}</div>
    <dl className="h-4 ml-2 flex">
      <dt>TR.:</dt>
      <dd className="w-4 text-right mr-2">{trackNumber}</dd>
      <dt>MEAS:</dt>
      <dd className="w-8 text-right mr-4">{measures}</dd>
      <dt>CH.:</dt>
      <dd className="w-6 mr-2 flex space-between">
        <span>{port}</span>
        <span className="w-4 text-right">{channel}</span>
      </dd>
      <dt>USED:</dt>
      <dd className="w-10 text-right">{usedMemory}</dd>
    </dl>
  </div>
}

export default EventList;
