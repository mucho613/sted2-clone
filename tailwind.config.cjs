/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ["./src/**/*.{ts,tsx}"],
  theme: {
    colors: {
      sted: {
        white: "#E7E7E7", // デフォルトのテキスト、選択中のメニュー項目
        blue: "#2C4A7E", // ボタンの背景
        gray: "#686877", // 枠線
        lightgray: "#686877", // Edit 画面のカーソル
        extralightgray: "#D1D1D1", // トラック一覧の "Play" の文字
        black: "#070016", // 背景、white が背景の場合のテキスト
      },
    },
    extend: {
      fontFamily: {
        "kodenmachou-12": ["KHドット小伝馬町12"],
      },
    },
  },
  plugins: [],
};
