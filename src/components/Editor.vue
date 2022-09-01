<script setup lang="ts">
import { ref, getCurrentInstance, onMounted, watch, shallowRef } from 'vue'
import * as monaco from 'monaco-editor'
import JsonWorker from 'monaco-editor/esm/vs/language/json/json.worker?worker'
import EditorWorker from 'monaco-editor/esm/vs/editor/editor.worker.js?worker'
import { QuerySuggestionsOfRedis } from '@/data/data';

// self.MonacoEnvironment = {
//     getWorker() {
//         return new JsonWorker();
//     },
// }

const props = defineProps<{
    value: string
    type?: string
}>()
const emits = defineEmits<{
    (e: 'handle', val: null): void
}>()

if (props.type == undefined) {
    self.MonacoEnvironment = {
        getWorker: () => new JsonWorker()
    }
}

const monacoEditor = shallowRef<any>(null)
const editorRef = ref<HTMLElement | null>(null)

onMounted(() => {

    if (props.type == 'redis_query') {
        console.log('redis_query')
        self.MonacoEnvironment = {
            getWorker: () => new EditorWorker()
        }
        monaco.languages.register({ id: "redis" })
        monaco.languages.registerCompletionItemProvider("redis", {
            provideCompletionItems: async () => ({
                suggestions: await QuerySuggestionsOfRedis(
                    monaco.languages.CompletionItemKind,
                    monaco.languages.CompletionItemInsertTextRule.InsertAsSnippet
                )
            } as any)
        })
        if (editorRef.value) {
            monacoEditor.value = monaco.editor.create(editorRef.value, {
                value: props.value,
                readOnly: false,
                theme: 'vs-dark',
                selectOnLineNumbers: true,
                language: "redis",
            });
            // monacoEditor.value?.trigger('format', 'editor.action.formatDocument')
            setTimeout(() => {
                monacoEditor.value?.getAction('editor.action.formatDocument').run();
            }, 100)
            // 监听值变化
            monacoEditor.value?.onDidChangeModelContent(() => {
                const currenValue = monacoEditor.value.getValue();
                emits('handle', currenValue);
            });
        }
    } else if (props.type == 'json') {
        if (editorRef.value) {
            self.MonacoEnvironment = {
                getWorker: () => new JsonWorker()
            }
            monacoEditor.value = monaco.editor.create(editorRef.value, {
                value: JSON.stringify(JSON.parse(props.value), null, '\t'),
                readOnly: false,
                theme: 'vs-dark',
                selectOnLineNumbers: true,
                language: "json",
                automaticLayout: true
            });
            setTimeout(() => {
                monacoEditor.value?.getAction('editor.action.formatDocument').run();
            }, 100)
            monacoEditor.value?.onDidChangeModelContent(() => {
                const currenValue = monacoEditor.value.getValue();
                emits('handle', currenValue);
            });
        }
    }
})
</script>
    
<template>
    <div ref="editorRef" class="code-editor"></div>
</template>
    
<style scoped>
.code-editor {
    width: 100%;
    height: 100%;
    min-height: 500px;
}
</style>
    