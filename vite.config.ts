import { defineConfig } from 'vite'
import vue from '@vitejs/plugin-vue'
import monacoEditorPlugin from 'vite-plugin-monaco-editor'
import { resolve } from 'path'

// https://vitejs.dev/config/
export default defineConfig({
    plugins: [vue(), monacoEditorPlugin],
    resolve: {
        alias: [
            {
                find: '@',
                replacement: resolve(__dirname, './src'),
            },
            {
                find: '#',
                replacement: resolve(__dirname, './src/components'),
            },
            {
                find: '^',
                replacement: resolve(__dirname, './src/views'),
            }
        ]
    },
})
