import type { Meta, StoryObj } from "@storybook/react-vite";
import TrackEditScreen from "./TrackEditScreen";

const meta = {
  component: TrackEditScreen,
  parameters: {
    layout: "centered",
  },
} satisfies Meta<typeof TrackEditScreen>;

export default meta;
type Story = StoryObj<typeof meta>;

export const Default: Story = {};
