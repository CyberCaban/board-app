import { defineConfig } from "vite";
import react from "@vitejs/plugin-react-swc";
import process from "process";

// https://vitejs.dev/config/
export default defineConfig(() => {
  const api = process.env.VITE_API_BASE_URL || "http://localhost:5000";
  return {
    plugins: [react()],
    server: {
      host: "0.0.0.0",
      proxy: {
        "/api": {
          target: api,
          changeOrigin: true,
        },
        "/uploads": {
          target: api,
          changeOrigin: true,
        },
      },
    },
  };
});
