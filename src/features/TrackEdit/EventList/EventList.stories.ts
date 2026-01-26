import type { Meta, StoryObj } from "@storybook/react-vite";
import EventList from "./EventList";

const meta = {
  component: EventList,
  parameters: {
    layout: "centered",
  },
} satisfies Meta<typeof EventList>;

export default meta;
type Story = StoryObj<typeof meta>;

export const Default: Story = {
  args: {
    trackMemo: "Test Memo",
    trackNumber: 1,
    measures: 1,
    port: "A",
    channel: 1,
    usedMemory: 0,
  },
};
