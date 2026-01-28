import { defineConfig } from 'vite'
import vue from '@vitejs/plugin-vue'

// https://vite.dev/config/
export default defineConfig({
  optimizeDeps: {
    include: [ "vue-router" ]
  },
  plugins: [vue()],
  server: {
    proxy: {
      "/api/": {
        target: "http://127.0.0.1:2299",
        changeOrigin: true,
        rewrite: (path) => path.replace(/^\/api/, "")
      }
    }
  }
})
