import { createSignal } from "solid-js";
import { useInvoke } from "../useInvoke";

type MidiPort = {
  id: string;
  name: string;
};

function Settings() {
  const [midiOutputPorts, setMidiOutputPorts] = createSignal<MidiPort[]>([]);

  const { getMidiOutputPorts, openMidiOutputPort } = useInvoke();

  const handleChange = (event: Event) => {
    const target = event.target as HTMLSelectElement;
    openMidiOutputPort(target.value);
  };

  getMidiOutputPorts().then((ports) => {
    setMidiOutputPorts(ports);
  });

  return (
    <div class="container m-4">
      <header class="mt-8 mb-4">
        <h1>設定</h1>
      </header>
      <section>
        <h2>MIDI出力先ポート</h2>

        {midiOutputPorts().length > 0 && (
          <select class="mt-2" onChange={handleChange}>
            {midiOutputPorts().map((port) => {
              return <option value={port.name}>{port.name}</option>;
            })}
          </select>
        )}
      </section>
    </div>
  );
}

export default Settings;
