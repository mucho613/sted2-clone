import type { Track } from "./types/track";

type Props = {
  track: Track;
};

function TrackListItem(props: Props) {
  const { track } = props;

  const {
    trackNumber,
    mode,
    midiOutput,
    stepOffset,
    keyOffset,
    rhythmTrack,
    trackMemo,
    step,
    totalStep,
  } = track;

  return (
    <tr className="h-4 *:border-r *:border-sted-gray *:py-0 first:h-5 first:align-bottom last:h-[21px] last:align-top">
      <td className="text-right bg-sted-blue border-none w-[18px]">{trackNumber}</td>
      <td className="text-right capitalize pr-[3px] text-sted-extralightgray">{mode}</td>
      <td className="pl-1">
        {midiOutput ? (
          <>
            <span className="inline-block w-2">{midiOutput.port}</span>
            <span className="inline-block w-4 text-right">{midiOutput.channel}</span>
          </>
        ) : (
          "OFF"
        )}
      </td>
      <td className="text-right pr-[3px]">{stepOffset}</td>
      <td className="text-right pr-[3px]">{keyOffset ?? "OFF"}</td>
      <td className="pl-1">{rhythmTrack ? "ON" : "OFF"}</td>
      <td className="pl-1">{trackMemo}</td>
      <td className="text-right pr-[5px]">{step}</td>
      <td className="text-right pr-2">{totalStep}</td>
    </tr>
  );
}

export default TrackListItem;
