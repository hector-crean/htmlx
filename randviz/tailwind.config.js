/** @type {import('tailwindcss').Config} */
export default {
  content: [
    './public/**/*.{html,js,ts}',
    './src/**/*.{html,js,ts}',
  ],
  theme: {
    extend: {
      containers: {
        "6xs": '1rem',
        "5xs": '2rem',
        "4xs": '4rem',
        "3xs": '8rem',
        "2xs": '16rem'
        // etc...
      },
    },
  },
  plugins: [
    require('@tailwindcss/container-queries'),
  ],
}

