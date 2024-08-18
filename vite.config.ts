import { defineConfig } from "vite";

export default defineConfig({
  clearScreen: false,
  server: {
    port: 1420,
    strictPort: true,
    watch: {
      ignored: ["**/src-tauri/**"],
    },
  },
  build: {
    rollupOptions: {
      input: {
        main: 'index.html', 
        crosshair: 'src/crosshair.html', 
      },
    },
  },
  publicDir: "public", 
});
