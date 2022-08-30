<script setup lang="ts">
import { ref, getCurrentInstance, onMounted, watch, shallowRef } from 'vue'
import * as monaco from 'monaco-editor'
import JsonWorker from 'monaco-editor/esm/vs/language/json/json.worker?worker'
import EditorWorker from 'monaco-editor/esm/vs/editor/editor.worker.js?worker'

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
            provideCompletionItems: () => ({
                suggestions: [
                    {
                        label: "GET",
                        kind: monaco.languages.CompletionItemKind.Function,
                        insertText: "GET ${0:key}",
                        insertTextRules: monaco.languages.CompletionItemInsertTextRule.InsertAsSnippet,
                        detail: "Get the value of a key",
                    },
                    {
                        label: "JSON.GET",
                        kind: monaco.languages.CompletionItemKind.Function,
                        insertText: "GET ${0:key}",
                        insertTextRules: monaco.languages.CompletionItemInsertTextRule.InsertAsSnippet,
                        detail: "Get Json data",
                    },
                    {
                        label: "JSON.SET",
                        kind: monaco.languages.CompletionItemKind.Function,
                        insertText: "GET ${1:key} $ ${0:json_str}",
                        insertTextRules: monaco.languages.CompletionItemInsertTextRule.InsertAsSnippet,
                        detail: "SET Json data",
                    },
                ]
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
    