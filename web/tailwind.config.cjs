/** @type {import('tailwindcss').Config} */

const defaultTheme = require('tailwindcss/defaultTheme')

module.exports = {
  content: ['./src/**/*.{html,js,svelte,ts}'],
  darkMode: 'class',
  theme: {
    extend: {
      lineHeight: {
        'extra': '4.5',
        '12': '3rem',
      }
    },
    listStyleType: {
      none: 'none',
      disc: 'disc',
      decimal: 'decimal',
      alpha: 'lower-alpha'
    },
    fontFamily: {
      'serif': ['BespokeSerif', defaultTheme.fontFamily.serif],
      'mono': ['IBMPlex', defaultTheme.fontFamily.mono],
    }
  },
  plugins: [],
}
