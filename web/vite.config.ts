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
      '/qa/query': {
        target: 'https://ask.databend.rs',
        changeOrigin: true,
        headers: {
          'Origin': 'https://ask.databend.rs'
        }
      }
    }
  },
  css: {
    preprocessorOptions: {
      scss: {
        javascriptEnabled: true,
        additionalData: '@import "@/assets/css/mixin.scss";'
      }
    },
  },
  build: {
    rollupOptions: {
      output:{
        manualChunks: {
          react: ['react', 'react-dom'],
          redux: ['redux','react-redux', 'redux-thunk']
        }
      }
    },
    sourcemap: false,
    minify: 'terser', 
    terserOptions: { 
      compress: {
        drop_console: true,
        drop_debugger: true
      }
    }
  },
});
