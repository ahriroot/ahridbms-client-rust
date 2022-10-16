<script setup lang="ts">
import TableViewVue from '@/components/postgres/TableView.vue'
import { Connection } from '@/types/Connection'
import { PostgresConnect } from '@/types/postgres'
import EditorVue from '@/components/Editor.vue'
import { ref, onBeforeMount } from 'vue'
import { executeSelectSql } from '@/api/postgres'
import { NTabs, NTabPane, NButton, NIcon } from 'naive-ui'
import { Checkmark } from '@vicons/ionicons5'
import { nanoid } from 'nanoid'

const props = defineProps<{
    conn: Connection<PostgresConnect>
    data: any
}>()
const emits = defineEmits<{
    (e: 'handleCloseTab', val: null): void
}>()

onBeforeMount(() => {
    let cfg = localStorage.getItem(`postgres:query:${props.conn.id}`)
    if (cfg) {
        config.value = JSON.parse(cfg)
    }
    localStorage.setItem(`postgres:query:${props.conn.id}`, JSON.stringify(config.value))
})

const config = ref({
    expanded: [],
    resultList: [],
    query: ''
})

const handleChange = (val: string) => {
    config.value.query = val
    localStorage.setItem(`postgres:query:${props.conn.id}`, JSON.stringify(config.value))
}

const editorRef = ref<any>()
const results = ref<any[]>([])
const tab = ref<string>('')
const handleSelect = async () => {
    let sql_str = window.getSelection()?.toString() || ''
    if (!sql_str) {
        sql_str = await editorRef.value?.getValue()
    }
    let sql_str_arr = sql_str.trim().split(/;(?=([^\\']*\\'[^\\']*\\')*[^\\']*$)/)
    let sqls = sql_str_arr.filter((sql: string) => {
        return sql && sql.trim() && !sql.trim().startsWith('--')
    })
    if (sqls) {
        results.value = []
        for (let index = 0; index < sqls.length; index++) {
            let sql = sqls[index].trim()
            if (sql.toLowerCase().startsWith('select')) {
                let res = await executeSelectSql({
                    conn: props.conn,
                    database: props.data.database,
                    sql: sql
                }, false)
                results.value.push({
                    id: nanoid(),
                    type: 'select',
                    sql: sql,
                    data: res
                })
            } else {
                results.value.push({
                    id: nanoid(),
                    type: 'other',
                    sql: sql,
                    data: ''
                })
            }
            if (results.value.length == 1) {
                tab.value = results.value[0].id
            }
        }
    }
}
</script>
    
<template>
    <div class="page">
        <div class="menu">
            <n-button strong secondary size="small" @click="handleSelect">
                <template #icon>
                    <n-icon>
                        <Checkmark />
                    </n-icon>
                </template>
            </n-button>
        </div>
        <div class="input">
            <EditorVue ref="editorRef" @change="handleChange" :value="config.query" :type="'postgres_query'" />
        </div>
        <div class="output">
            <n-tabs v-model:value="tab" type="card" closable tab-style="min-width: 80px;" size="small">
                <n-tab-pane display-directive="show" v-for="(i, index) in results" :key="i.id" :tab="`Result ${index}`"
                    :name="i.id">
                    {{i.sql}}
                    <TableViewVue v-if="i.type == 'select'" :data="i.data" />
                    <div v-else>{{i}}</div>
                </n-tab-pane>
            </n-tabs>
        </div>
    </div>
</template>

<style scoped>
.page {
    position: absolute;
    top: 0;
    left: 0;
    right: 0;
    bottom: 36px;
    display: flex;
    flex-direction: column;
}

.page .menu {
    position: absolute;
    top: 0;
    left: 0;
    right: 0;
    height: 36px;
    padding: 0 4px;
    display: flex;
    justify-content: flex-start;
    align-items: center;
}

.page .input {
    position: absolute;
    top: 36px;
    left: 0;
    right: 0;
    height: 400px;
}

.page .output {
    position: absolute;
    top: 436px;
    left: 0;
    right: 0;
    bottom: -36px;
}
</style>
    