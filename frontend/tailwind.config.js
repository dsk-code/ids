/** @type {import('tailwindcss').Config} */
module.exports = {
  content: [
    "./index.html",
    "./src/*.{js,jsx,ts,tsx}",
    "./src/**/*.{js,jsx,ts,tsx}",
    "./src/**/**/*.{js,jsx,ts,tsx}",
    './node_modules/preline/preline.js',
  ],
  theme: {
    extend: {
      colors: {
        "dark-purple": "#081A51",
        "light-white": 'rgba(255,255,255,0.18)',
        "home-bg": "#050505",
        "selected-text": "#A3A3FF",
        theme: "#5c318c",
      },
    },
    fontFamily: {
      poppins: ["Poppins", "sans-serif"],
    },
  },
  plugins: [
    require('@tailwindcss/forms'),
    require('preline/plugin'),
  ],
}