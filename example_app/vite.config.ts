import { defineConfig } from "vite";
import solid from "vite-plugin-solid";

export default defineConfig({
  plugins: [solid()],
  build: {
    outDir: "dist",
    lib: {
      entry: "client/islands.tsx",
      name: "islands",
      fileName: () => "islands.js",
      formats: ["es"],
    },
    rollupOptions: {
      output: {
        // Keep it as a single file
        inlineDynamicImports: true,
      },
    },
  },
});
