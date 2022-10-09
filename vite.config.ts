import { defineConfig, PluginOption } from 'vite'
import vue from '@vitejs/plugin-vue'
import monacoEditorPlugin from 'vite-plugin-monaco-editor'
import { resolve } from 'path'

// https://vitejs.dev/config/
export default defineConfig({
    plugins: [vue(), monacoEditorPlugin as unknown as PluginOption],
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
            },
            {
                find: 'vue-i18n',
                replacement: 'vue-i18n/dist/vue-i18n.cjs.js',
            }
        ]
    },
})
