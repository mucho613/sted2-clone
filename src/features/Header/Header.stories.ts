import type { Meta, StoryObj } from "@storybook/react";
import Header from "./Header";

const meta = {
  component: Header,
  parameters: {
    layout: "centered",
  },
} satisfies Meta<typeof Header>;

export default meta;
type Story = StoryObj<typeof meta>;

export const Default: Story = {
  args: {
    freeMemory: 589356,
    usedMemory: 0,
    trackNumber: 1,
    moduleName: "MODULE: TestModuleName",
  },
};
