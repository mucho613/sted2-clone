import TrackListItem from "./TrackListItem";
import type { Track } from "./types/track";

function TrackList() {
  const tracks: Track[] = [...new Array(18)].map((_, index) => ({
    trackNumber: index + 1,
    mode: "play",
    port: "A",
    channelNumber: index < 16 ? index + 1 : 0,
    stepOffset: 0,
    keyOffset: 0,
    rhythmTrack: false,
    trackMemo: "",
    step: 0,
    totalStep: 0,
  }));

  return (
    <table className="table-fixed border border-sted-gray">
      <thead>
        <tr className="*:font-normal bg-sted-gray text-left *:py-0 oy-">
          <th className="w-[19px] pl-0.5 pr-px">TR</th>
          <th className="w-[43px] pl-[7px] pr-1">MODE</th>
          <th className="w-8 px-1">CH.</th>
          <th className="w-8 px-1">ST+</th>
          <th className="w-8 px-1">K#+</th>
          <th className="w-8 px-1">RHY</th>
          <th className="w-[298px] pl-7">TRACK MEMO</th>
          <th className="w-12 pl-2.5">STEP</th>
          <th className="w-[106px] pl-[34px]">TOTAL ST</th>
        </tr>
      </thead>
      <tbody>
        {tracks.map((track) => (
          <TrackListItem key={track.trackNumber} track={track} />
        ))}
      </tbody>
    </table>
  );
}

export default TrackList;
