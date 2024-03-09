/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ["./src/**/*.rs"],
  theme: {
    extend: {},
    fontFamily: {
      eb_garamond: ["EB Garamond", "serif"],
      signika_negative: ["Signika Negative", "sans-serif"],
    },
  },
  plugins: [
    require("@catppuccin/tailwindcss")({
      // which flavour of colours to use by default, in the `:root`
      defaultFlavour: "mocha",
    }),
  ], 
}

