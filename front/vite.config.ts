import { resolve } from 'node:path';

import { defineConfig } from 'vite';
import preact from '@preact/preset-vite';

export default defineConfig({
  plugins: [preact({})],
  root: 'public',
  server: { proxy: { '/api': 'http://localhost:8000' } },
  build: { outDir: '../../back/record_keeper/dist' },
  resolve: {
    alias: {
      'react-dom/test-utils': 'preact/test-utils',
      '/src': resolve(process.cwd(), 'src'),
    },
  },
});
