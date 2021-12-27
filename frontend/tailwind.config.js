module.exports = {
  content: ["./index.html", "./src/**/*.rs"],
  theme: {
    fontFamily: {
      sans: ["Prompt", "sans-serif"],
    },
    colors: {
      transparent: "transparent",
      current: "currentColor",
      red: "#ff2f00",
      grey: {
        light: "#ccc",
        DEFAULT: "#aaa",
      }
    },
    extend: {},
  },
  plugins: [],
}
