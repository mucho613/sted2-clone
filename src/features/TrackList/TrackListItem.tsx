import type { Track } from "./types/track";

type Props = {
  track: Track;
};

function TrackListItem(props: Props) {
  const { track } = props;

  const {
    trackNumber,
    mode,
    port,
    channelNumber,
    stepOffset,
    keyOffset,
    rhythmTrack,
    trackMemo,
    step,
    totalStep,
  } = track;

  return (
    <tr className="h-4 *:border-r *:border-sted-gray *:py-0 first:pt-[3px]">
      <td className="text-right bg-sted-blue border-none">{trackNumber}</td>
      <td className="text-right capitalize pr-[3px]">{mode}</td>
      <td className="pl-1">
        <span className="inline-block w-2">{port}</span>
        <span className="inline-block w-4 text-right">{channelNumber}</span>
      </td>
      <td className="text-right pr-[3px]">{stepOffset}</td>
      <td className="text-right pr-[3px]">{keyOffset}</td>
      <td className="pl-1">{rhythmTrack ? "ON" : "OFF"}</td>
      <td className="pl-1">{trackMemo}</td>
      <td className="text-right pr-[5px]">{step}</td>
      <td className="text-right pr-2">{totalStep}</td>
    </tr>
  );
}

export default TrackListItem;
