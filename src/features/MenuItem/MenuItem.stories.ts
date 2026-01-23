import type { Meta, StoryObj } from "@storybook/react";
import MenuItem from "./MenuItem";
import { fn } from "@storybook/test";

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
    onClick: fn(),
  },
};

export const Selected: Story = {
  args: {
    label: "STed2",
    state: "selected",
    onClick: fn(),
  },
};

export const Active: Story = {
  args: {
    label: "STed2",
    state: "active",
    onClick: fn(),
  },
};
