<script setup lang="ts">
import { onBeforeMount, onMounted, ref, shallowRef } from 'vue'
import * as monaco from 'monaco-editor'
import JsonWorker from 'monaco-editor/esm/vs/language/json/json.worker?worker'
import SqlWorker from 'monaco-editor/esm/vs/basic-languages/sql/sql.js?worker'
import RedisWorker from 'monaco-editor/esm/vs/basic-languages/redis/redis.js?worker'
import EditorWorker from 'monaco-editor/esm/vs/editor/editor.worker.js?worker'
import { QuerySuggestionsOfRedis, QuerySuggestionsOfMongo } from '@/data/data'


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
const value = ref('')
const language = ref('')

onBeforeMount(() => {
    if (props.type == 'redis_query') {
        self.MonacoEnvironment = {
            getWorker: () => new RedisWorker()
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
        value.value = props.value
        language.value = 'redis'
    } else if (props.type == 'postgres_query') {
        self.MonacoEnvironment = {
            getWorker: () => new SqlWorker()
        }
        value.value = props.value
        language.value = 'sql'
    } else if (props.type == 'mongo_query') {
        self.MonacoEnvironment = {
            getWorker: () => new SqlWorker()
        }
        monaco.languages.register({ id: "mongo" })
        monaco.languages.registerCompletionItemProvider("mongo", {
            provideCompletionItems: async () => ({
                suggestions: await QuerySuggestionsOfMongo(
                    monaco.languages.CompletionItemKind,
                    monaco.languages.CompletionItemInsertTextRule.InsertAsSnippet
                )
            } as any)
        })
        value.value = props.value
        language.value = 'mongo'
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
        value.value = json
        language.value = 'json'
    }
})

onMounted(() => {
    if (editorRef.value) {
        monacoEditor.value = monaco.editor.create(editorRef.value, {
            mouseWheelZoom: true,
            value: value.value,
            readOnly: props.readOnly,
            theme: 'vs-dark',
            selectOnLineNumbers: true,
            language: language.value,
            automaticLayout: true,
            fontSize: 18,
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
    