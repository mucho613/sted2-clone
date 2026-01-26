import type { Meta, StoryObj } from "@storybook/react-vite";
import SongInfo from "./SongInfo";

const meta = {
  component: SongInfo,
  parameters: {
    layout: "centered",
  },
} satisfies Meta<typeof SongInfo>;

export default meta;
type Story = StoryObj<typeof meta>;

export const Default: Story = {
  args: {
    tempo: 120,
    timebase: 96,
    playBias: 0,
    keyName: "C MAJOR",
    beat: "4 / 4",
    midiInputDeviceName: "InDevice",
  },
};
