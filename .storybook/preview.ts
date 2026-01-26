import type { Preview } from "@storybook/react-vite";

// Import global styles of STed2 Clone
import "../src/input.css";

const preview: Preview = {
  parameters: {
    controls: {
      matchers: {
        color: /(background|color)$/i,
        date: /Date$/i,
      },
    },
  },
};

export default preview;
