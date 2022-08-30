import { defineConfig } from 'vite'
import path from 'path'
import react from '@vitejs/plugin-react'

// https://vitejs.dev/config/
export default defineConfig({
  resolve: {
    alias: {
      '@atoms': path.resolve('src/components/atoms'),
      '@molecules': path.resolve('src/components/molecules'),
      '@layouts': path.resolve('src/components/layouts'),
      '@templates': path.resolve('src/components/templates'),
      '@assets': path.resolve('src/assets'),
      '@lib': path.resolve('src/lib'),
      '@constants': path.resolve('src/constants'),
    },
  },
  plugins: [react()],
  define: {
    global: 'window',
  },
})
