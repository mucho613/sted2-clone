import { onCleanup, onMount } from "solid-js";

type Props = {
  getNoteOnKeys: () => Promise<{[key: string]: number}>;
}

function TrackMonitor(props: Props) {
  let canvas: HTMLCanvasElement | undefined = undefined;

  const WIDTH = 600 as const;
  const HEIGHT = 400 as const;

  onMount(() => {
    if(!canvas) return;
    const context = canvas.getContext("2d");
    
    let frame = requestAnimationFrame(loop);

    async function loop() {
      frame = requestAnimationFrame(loop);

      if(!context) return;

      context.clearRect(0, 0, WIDTH, HEIGHT);

      const noteOnKeys = await props.getNoteOnKeys();
      context.fillStyle = "red"

      
            context.fillRect(j * 4, i * 20, 3, 15);
    }
    
    onCleanup(() => cancelAnimationFrame(frame))
  })

  return (
    <>
      <h2>Track monitor</h2>
      <canvas ref={canvas} width={WIDTH} height={HEIGHT} />
    </>
  );
}

export default TrackMonitor;