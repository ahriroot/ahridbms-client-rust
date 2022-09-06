<script setup lang="ts">
import { onMounted, shallowRef } from 'vue'
import * as monaco from 'monaco-editor'
import JsonWorker from 'monaco-editor/esm/vs/language/json/json.worker?worker'
import EditorWorker from 'monaco-editor/esm/vs/editor/editor.worker.js?worker'
import { QuerySuggestionsOfRedis } from '@/data/data'


const props = defineProps<{
    value: any
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
const editorRef = shallowRef<HTMLElement | null>(null)

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
                automaticLayout: true
            })
            // monacoEditor.value?.trigger('format', 'editor.action.formatDocument')
            setTimeout(() => {
                monacoEditor.value?.getAction('editor.action.formatDocument').run()
            }, 100)
            // 监听值变化
            monacoEditor.value?.onDidChangeModelContent(() => {
                const currenValue = monacoEditor.value.getValue()
                emits('handle', currenValue)
            })
        }
    } else if (props.type == 'json') {
        if (editorRef.value) {
            let json = ''
            try {
                json = JSON.stringify(JSON.parse(props.value), null, '\t')
            } catch (e) {
                json = ''
            }
            self.MonacoEnvironment = {
                getWorker: () => new JsonWorker()
            }
            monacoEditor.value = monaco.editor.create(editorRef.value, {
                value: json,
                readOnly: false,
                theme: 'vs-dark',
                selectOnLineNumbers: true,
                language: "json",
                automaticLayout: true
            })
            setTimeout(() => {
                monacoEditor.value?.getAction('editor.action.formatDocument').run()
            }, 100)
            monacoEditor.value?.onDidChangeModelContent(() => {
                const currenValue = monacoEditor.value.getValue()
                emits('handle', currenValue)
            })
        }
    }
})

const setValue = async (val: string) => {
    let json = ''
    try {
        json = JSON.stringify(JSON.parse(val), null, '\t')
    } catch (e) {
        json = ''
    }
    monacoEditor.value?.setValue(json)
    monacoEditor.value?.getAction('editor.action.formatDocument').run()
}

const getValue = async (): Promise<string | undefined> => {
    return monacoEditor.value?.getValue()
}

const getSelectedValue = async () => {
    monacoEditor.value?.trigger('source', 'editor.action.clipboardCopyAction')
}

defineExpose({
    setValue, getValue, getSelectedValue
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
    