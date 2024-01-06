/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ["./src/**/*.{ts,tsx}"],
  theme: {
    colors: {
      blue: "#2C4A7E",
    },
    extend: {
      fontFamily: {
        "kodenmachou-12": ['KHドット小伝馬町12'],
      },
    },
  },
  plugins: [],
};
