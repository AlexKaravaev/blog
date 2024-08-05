/** @type {import('tailwindcss').Config} */
const defaultTheme = require('tailwindcss/defaultTheme')
module.exports = {
  theme: {
    extend: {
    fontFamily: {
      eurostile: ['Eurostile', "sans-serif"],
    },
    backgroundColor: {
      'custom-blue': '#5A67D8',
    }
  }
  },
  content: [
    "./src/**/*.rs",
  ],
  plugins: [
    require("@tailwindcss/typography"),
  ],
};
