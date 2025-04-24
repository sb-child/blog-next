import { defineConfig } from "@rsbuild/core";
import { pluginLess } from "@rsbuild/plugin-less";
import { pluginPreact } from "@rsbuild/plugin-preact";
import { pluginTypedCSSModules } from "@rsbuild/plugin-typed-css-modules";

export default defineConfig({
  plugins: [pluginPreact(), pluginLess(), pluginTypedCSSModules()],
  server: {
    open: false,
    base: "/",
  },
  output: {
    assetPrefix: "/",
    cssModules: {
      namedExport: true,
    },
  },
  html: {
    template: "./template/index.html",
  },
});
