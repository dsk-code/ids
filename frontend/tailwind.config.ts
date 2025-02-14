import type { Config } from "tailwindcss";

export default {
  content: ["./app/**/{**,.client,.server}/**/*.{js,jsx,ts,tsx}"],
  theme: {
    extend: {
      fontFamily: {
        sans: [
          "Inter",
          "ui-sans-serif",
          "system-ui",
          "sans-serif",
          "Apple Color Emoji",
          "Segoe UI Emoji",
          "Segoe UI Symbol",
          "Noto Color Emoji",
          "Poppins",
        ],
      },
      colors: {
        "dark-purple": "#081A51",
        "light-white": 'rgba(255,255,255,0.18)',
        "home-bg": "#050505",
        "selected-text": "#A3A3FF",
        theme: "#5c318c",
      },
      borderRadius: {
        'custom': '0.3rem', // 例: 0.3remの角丸
      },
    },
  },
  plugins: [],
} satisfies Config;
