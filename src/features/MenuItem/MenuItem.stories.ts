import type { Meta, StoryObj } from "@storybook/react-vite";
import MenuItem from "./MenuItem";

const meta = {
  component: MenuItem,
  parameters: {
    layout: "centered",
  },
} satisfies Meta<typeof MenuItem>;

export default meta;
type Story = StoryObj<typeof meta>;

export const Default: Story = {
  args: {
    label: "STed2",
    state: "normal",
  },
};

export const Selected: Story = {
  args: {
    label: "STed2",
    state: "selected",
  },
};

export const Active: Story = {
  args: {
    label: "STed2",
    state: "active",
  },
};
