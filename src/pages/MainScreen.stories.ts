import type { Meta, StoryObj } from "@storybook/react-vite";
import MainScreen from "./MainScreen";

const meta = {
  component: MainScreen,
  parameters: {
    layout: "centered",
  },
} satisfies Meta<typeof MainScreen>;

export default meta;
type Story = StoryObj<typeof meta>;

export const Default: Story = {};
