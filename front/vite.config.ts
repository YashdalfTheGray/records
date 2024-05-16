import { resolve } from 'node:path';

import { defineConfig } from 'vite';
import preact from '@preact/preset-vite';

export default defineConfig({
  plugins: [preact({ reactAliasesEnabled: false })],
  root: 'public',
  server: {
    proxy: {
      '/api': 'http://localhost:8000',
    },
  },
  build: {
    outDir: '../back/record_keeper/dist',
  },
  resolve: {
    alias: {
      'react-dom/test-utils': 'preact/test-utils',
      'react-dom': 'preact/compat',
      react: 'preact/compat',
      '/src': resolve(process.cwd(), 'src'),
    },
  },
});
