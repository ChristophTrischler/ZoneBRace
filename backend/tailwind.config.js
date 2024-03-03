/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ["./templates/**/*.html"],
  theme: {
    extend: {
      fontFamily: {
        display: ['IBM Plex Mono', 'Menlo', 'monospace'],
        body: ['IBM Plex Mono', 'Menlo', 'monospace'],
      },
      colors: {
        'khaki': {
          DEFAULT: '#ceb992',
          100: '#312716',
          200: '#614e2c',
          300: '#927641',
          400: '#b89a61',
          500: '#ceb992',
          600: '#d8c7a8',
          700: '#e2d5be',
          800: '#ebe3d3',
          900: '#f5f1e9'
        },
        'secondary': {
          DEFAULT: '#73937e',
          100: '#171d19',
          200: '#2d3b32',
          300: '#44584b',
          400: '#5b7664',
          500: '#73937e',
          600: '#8ea897',
          700: '#aabeb1',
          800: '#c7d4cb',
          900: '#e3e9e5'
        },
        'primary':
        {
          DEFAULT: '#d64045',
          100: '#2e0a0b',
          200: '#5d1416',
          300: '#8b1d21',
          400: '#b9272c',
          500: '#d64045',
          600: '#df686c',
          700: '#e78d90',
          800: '#efb3b5',
          900: '#f7d9da'
        },
        'gray': { DEFAULT: '#2a2b2a', 100: '#090909', 200: '#111211', 300: '#1a1a1a', 400: '#222322', 500: '#2a2b2a', 600: '#555755', 700: '#7f817f', 800: '#a9aba9', 900: '#d4d5d4' }
      },
    },
    plugins: [],
  }
}
