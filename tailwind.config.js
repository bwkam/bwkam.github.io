/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ["./src/**/*.rs"],
  theme: {
    extend: {},
    fontFamily: {
      garamond: ["EB Garamond", "serif"]
    }
  },
  plugins: [
    require("@catppuccin/tailwindcss")({
      // which flavour of colours to use by default, in the `:root`
      defaultFlavour: "mocha",
    }),
  ], 
}

