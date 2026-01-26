import type { Meta, StoryObj } from "@storybook/react-vite";
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
    size: "default",
    freeMemory: 589356,
    usedMemory: 0,
    trackNumber: 1,
    moduleName: "MODULE: TestModuleName",
  },
};

export const HalfSize: Story = {
  args: {
    size: "half",
    freeMemory: 589356,
    usedMemory: 0,
    trackNumber: 1,
    moduleName: "MODULE: TestModuleName",
  },
};
