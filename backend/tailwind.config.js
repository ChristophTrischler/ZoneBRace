/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ["./templates/**/*.html", './templates/inede.html'],
  theme: {
    extend: {
      fontFamily: {
        display: ['IBM Plex Mono', 'Menlo', 'monospace'],
        body: ['IBM Plex Mono', 'Menlo', 'monospace'],
      },
      colors: {
        primary: {
        50: '#f6f6f5',
        100: '#e7e7e6',
        200: '#d1d1d0',
        300: '#b1b0af',
        400: '#8a8a86',
        500: '#6f6f6b',
        600: '#5f5f5b',
        700: '#504f4e',
        800: '#464644',
        900: '#3d3d3c',
        950: '#242423',
        },
        secondary: {
        50: '#f0fdfa',
        100: '#ccfbf1',
        200: '#99f6e4',
        300: '#5eead4',
        400: '#2dd4bf',
        500: '#14b8a6',
        600: '#0d9488',
        700: '#0f766e',
        800: '#115e59',
        900: '#134e4a',
        },
        red: '#C1292E', 
        black: {
          800: '#242423', 
          1000: '#000000'
        }
      },
    },
  },
  plugins: [],
}
