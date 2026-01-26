import type { TrackEvent } from "../EventList/EventList";
import { formatNoteNumber } from "./logic/noteNumber";

type Props = {
  event: TrackEvent;
};

function EventListItem(props: Props) {
  const { event } = props;

  const { noteNumber, stepTime, gateTime, velocity } = event;

  return (
    <tr className="h-4 *:overflow-hidden">
      <td className="text-right" />
      <td className="text-right" />
      <td className="w-2 text-right">:</td>
      <td className="w-14 flex justify-between">
        <span>{formatNoteNumber(noteNumber)}</span>
        <span>{noteNumber}</span>
      </td>
      <td className="text-right pr-2">{stepTime}</td>
      <td className="text-right pr-2">{gateTime}</td>
      <td className="text-right pr-4">{velocity}</td>
    </tr>
  );
}

export default EventListItem;
