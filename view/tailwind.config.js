/** @type {import('tailwindcss').Config} */
export default {
  content: [
    './public/**/*.{html,js,ts}',
    './src/**/*.{html,js,ts}',
  ],
  theme: {
    extend: {},
  },
  plugins: [
    require('@tailwindcss/container-queries'),
  ],
}

