/** @type {import('tailwindcss').Config} */
export default {
  content: ['./src/**/*.{html,js,svelte,ts}'],
  darkMode: true, // or 'media' or 'class'
  theme: {

    extend: {},

  },

  variants: {

    extend: {},

  },
  plugins: [

    require('@tailwindcss/forms'),

    require('@tailwindcss/typography'),
  ],    
};
