/** @type {import('tailwindcss').Config} */
module.exports = {
  content: {
    files: ["*.html", "./src/**/*.rs"],
  },
  theme: {
    colors: {
      'model_d': '#7aa2f7',
      'model_l': '#73daca',
      'user_d': '#7aa2f7',
      'user_l': '#bb9af7',
      'button_d': '#2ac3de',
      'button_l': '#7dcfff',
      'inp': '#24283b',
      'txt': '#9aa5ce',
      'background': '#1a1b26',
      'hover_d': '#f7768e',
      'hover_l': '#ff9e64'
    },
  },
  plugins: [],
}
