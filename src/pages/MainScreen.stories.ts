import type { Meta, StoryObj } from "@storybook/react";
import MainScreen from "./MainScreen";

const meta = {
  title: "MainScreen",
  component: MainScreen,
  parameters: {
    layout: "centered",
  },
} satisfies Meta<typeof MainScreen>;

export default meta;
type Story = StoryObj<typeof meta>;

export const Default: Story = {};
