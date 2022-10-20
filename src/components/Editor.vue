<script setup lang="ts">
import { onMounted, shallowRef } from 'vue'
import * as monaco from 'monaco-editor'
import JsonWorker from 'monaco-editor/esm/vs/language/json/json.worker?worker'
import SqlWorker from 'monaco-editor/esm/vs/basic-languages/sql/sql.js?worker'
import EditorWorker from 'monaco-editor/esm/vs/editor/editor.worker.js?worker'
import { QuerySuggestionsOfRedis } from '@/data/data'


const props = defineProps<{
    value: any
    type?: string
    readOnly?: boolean
}>()
const emits = defineEmits<{
    (e: 'handle', val: null): void
    (e: 'change', val: string): void
}>()

if (props.type == undefined) {
    self.MonacoEnvironment = {
        getWorker: () => new JsonWorker()
    }
}

const monacoEditor = shallowRef<any>(null)
const editorRef = shallowRef<HTMLElement | null>(null)

onMounted(() => {
    let value = ''
    let language = ''
    if (props.type == 'redis_query') {
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
        value = props.value
        language = 'redis'
    } else if (props.type == 'postgres_query') {
        self.MonacoEnvironment = {
            getWorker: () => new SqlWorker()
        }
        value = props.value
        language = 'sql'
    } else if (props.type == 'json') {
        self.MonacoEnvironment = {
            getWorker: () => new JsonWorker()
        }
        let json = ''
        try {
            json = JSON.stringify(JSON.parse(props.value), null, '\t')
        } catch (e) {
            json = ''
        }
        value = json
        language = 'json'
    }
    if (editorRef.value) {
        monacoEditor.value = monaco.editor.create(editorRef.value, {
            mouseWheelZoom: true,
            value: value,
            readOnly: props.readOnly ? true : false,
            theme: 'vs-dark',
            selectOnLineNumbers: true,
            language: language,
            automaticLayout: true,
        })
        // monacoEditor.value?.trigger('format', 'editor.action.formatDocument')
        setTimeout(() => {
            monacoEditor.value?.getAction('editor.action.formatDocument').run()
        }, 100)
        // 监听值变化
        monacoEditor.value?.onDidChangeModelContent(() => {
            const currenValue = monacoEditor.value.getValue()
            emits('change', currenValue)
        })
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
    min-height: 300px;
}
</style>
    