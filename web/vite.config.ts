import { defineConfig } from 'vite';
import react from '@vitejs/plugin-react';
import path from 'path';

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [react()],
  resolve: {
    alias: {
      '@': path.resolve(__dirname, 'src'),
      'components': path.resolve(__dirname, 'src/components'),
      'assets': path.resolve(__dirname, 'src/assets'),
      'utils': path.resolve(__dirname, 'src/utils'),
      'apis': path.resolve(__dirname, 'src/apis')
    },
    extensions: ['.mjs', '.js', '.ts', '.jsx', '.tsx', '.json']
  },
  server: {
    open: true,
    proxy: {
      '/query': {
        target: 'http://ask.databend.rs:8081',
        changeOrigin: true,
      }
    }
  },
});
