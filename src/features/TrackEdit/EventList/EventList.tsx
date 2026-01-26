import EventListItem from "./EventListItem";

type Props = {
  trackMemo: string;
  trackNumber: number;
  measures: number;
  port: string;
  channel: number;
  usedMemory: number;
};

export type TrackEvent = {
  id: string;
  noteNumber: number;
  stepTime: number;
  gateTime: number;
  velocity: number;
};

function EventList(props: Props) {
  const { trackMemo, trackNumber, measures, port, channel, usedMemory } = props;

  const events: TrackEvent[] = [
    {
      id: "1",
      noteNumber: 60,
      stepTime: 0,
      gateTime: 36,
      velocity: 100,
    },
    {
      id: "2",
      noteNumber: 62,
      stepTime: 48,
      gateTime: 36,
      velocity: 100,
    },
    {
      id: "3",
      noteNumber: 64,
      stepTime: 96,
      gateTime: 36,
      velocity: 100,
    },
  ];

  return (
    <div className="w-[306px]">
      <div className="bg-sted-blue h-[17px] px-px">M:{trackMemo}</div>
      <dl className="h-4 ml-[9px] flex">
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

      <div className="w-[306px] h-[412px] border border-sted-gray">
        <table className="w-full">
          <thead>
            <tr className="h-4 bg-sted-gray">
              <th className="w-10 text-right">MEAS</th>
              <th className="w-10 text-right">STEP</th>
              <th className="w-2">:</th>
              <th className="w-16 text-left">NOTE K#</th>
              <th className="w-12 text-right pr-2">ST</th>
              <th className="w-12 text-right pr-2">GT</th>
              <th className="w-14 text-right pr-4">VEL</th>
            </tr>
          </thead>
          <tbody>
            {events.map((event) => (
              <EventListItem key={event.id} event={event} />
            ))}
          </tbody>
        </table>
      </div>
    </div>
  );
}

export default EventList;
