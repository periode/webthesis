/** @type {import('tailwindcss').Config} */

const defaultTheme = require('tailwindcss/defaultTheme')

module.exports = {
  content: ['./src/**/*.{html,js,svelte,ts}'],
  theme: {
    extend: {},
    fontFamily: {
      'serif': ['BespokeSerif', defaultTheme.fontFamily.serif],
      'mono': ['IBMPlex', defaultTheme.fontFamily.mono],
    }
  },
  plugins: [],
}
