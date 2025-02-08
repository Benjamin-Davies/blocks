import { defineConfig } from 'vite';
import path from 'path';

// https://vite.dev/config/
export default defineConfig({
  build: {
    minify: false,
  },
  resolve: {
    alias: {
      'blocks-web': path.resolve(__dirname, 'blocks-web/pkg'),
    },
  },
});
