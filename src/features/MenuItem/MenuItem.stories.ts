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
    isSelected: false,
    onClick: fn(),
  },
};

export const Selected: Story = {
  args: {
    label: "STed2",
    isSelected: true,
    onClick: fn(),
  },
};
