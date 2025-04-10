import { defineConfig } from "@rsbuild/core";
import { pluginPreact } from "@rsbuild/plugin-preact";

export default defineConfig({
  plugins: [pluginPreact()],
  server: {
    open: false,
    base: "/",
  },
  output: {
    assetPrefix: "/",
  },
  html: {
    template: "./template/index.html",
  },
});
