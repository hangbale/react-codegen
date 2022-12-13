import { defineConfig } from 'vite'
import react from '@vitejs/plugin-react'
import legacy from '@vitejs/plugin-legacy'
import eslint from 'vite-plugin-eslint'
import { resolve } from 'path'
// https://vitejs.dev/config/
export default defineConfig({
  resolve: {
    alias: {
      '@': resolve(__dirname, './src')
    }
  },
  // esbuild: { //https://github.com/vitejs/vite/discussions/3112
  //   loader: 'jsx',
  //   include: /src\/.*\.jsx?$/,
  // },
  plugins: [
    react(),
    {
      apply: 'build',
      ...legacy({
        targets: ['defaults', 'not IE 11']
      })
    },
    eslint()

  ],
  server: {
    proxy: {
      '/api': {
        target: 'http://yapi.dev.mchz.com.cn/mock/509/api/',
        changeOrigin: true,
        rewrite: (path) => {
          return path.replace(/^\/api/, '')
        }
      }
    }
  }
})
