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
      alpha: 'lower-alpha',
      'upper-alpha': 'upper-alpha',
    },
    fontFamily: {
      'serif': ['BespokeSerif', defaultTheme.fontFamily.serif],
      'mono': ['IBMPlex', defaultTheme.fontFamily.mono],
    },
    fontSize: {
      sm: '0.8rem',
      base: '1rem',
      body: '1.175rem',
      xl: '1.25rem',
      '2xl': '1.563rem',
      '3xl': '1.953rem',
      '4xl': '2.441rem',
      '5xl': '3.052rem',
    }
  },
  plugins: [],
}
