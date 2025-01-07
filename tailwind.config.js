/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ["./templates/*.html"],
  theme: {
    extend: {
	fontFamily: {
	    dm: ["Dank Mono", "Consolas", "Courier New", "Courier", "monospace"]
	},
    },
  plugins: [],
}

