type Props = {
  tempo: number;
  timebase: number;
  playBias: number;
  keyName: string;
  beat: string;
  midiInputDeviceName: string;
};

function SongInfo(props: Props) {
  const { tempo, timebase, playBias, keyName, beat, midiInputDeviceName } = props;

  return (
    <div className="flex flex-col w-[323px] h-[75px] border border-sted-gray pl-[9px] pt-[5px]">
      <dl className="flex flex-wrap">
        <dt className="after:content-[':'] after:ml-8">TEMPO</dt>
        <dd className="w-8 mr-12 text-right">{tempo}</dd>
        <dt className="after:content-[':'] after:ml-2">TIMEBASE</dt>
        <dd className="w-8 text-right">{timebase}</dd>
        <dt className="after:content-[':']">PLAY BIAS</dt>
        <dd className="w-8 mr-[200px] text-right">{playBias}</dd>
        <dt className="after:content-[':'] after:ml-4">KEY</dt>
        <dd className="w-16 mr-[200px] text-right">{keyName}</dd>
        <dt className="after:content-[':'] after:ml-4">BEAT</dt>
        <dd className="w-14 mr-12 text-right">{beat}</dd>
        <dt className="after:content-[':'] after:ml-2">MIDI IN</dt>
        <dd className="ml-2">{midiInputDeviceName}</dd>
      </dl>
    </div>
  );
}

export default SongInfo;
