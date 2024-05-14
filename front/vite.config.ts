import { defineConfig } from 'vite';
import preact from '@preact/preset-vite';

export default defineConfig({
  plugins: [preact()],
  root: 'public',
  server: {
    proxy: {
      '/api': 'http://localhost:8000',
    },
  },
});
