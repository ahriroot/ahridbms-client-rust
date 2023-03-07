<script setup lang="ts">
import TableViewVue from '@/components/postgres/TableView.vue'
import { Connection } from '@/types/Connection'
import { PostgresConnect } from '@/types/postgres'
import EditorVue from '@/components/Editor.vue'
import { ref, onBeforeMount } from 'vue'
import { executeSelectSql } from '@/api/postgres'
import { NTabs, NTabPane, NButton, NIcon } from 'naive-ui'
import { Checkmark } from '@vicons/ionicons5'
import { uuid } from '@/utils/crypto'

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
const loading = ref<boolean>(false)
const handleSelect = async () => {
    let sql_str = window.getSelection()?.toString() || ''
    if (!sql_str) {
        sql_str = await editorRef.value?.getValue()
    }
    sql_str = sql_str.split('\n').filter((sql: string) => {
        return sql && sql.trim() && !sql.trim().startsWith('//')
    }).join('\n')
    let sql_str_arr = sql_str.trim().split(/;(?=(?:[^'"]|'[^']*'|"[^"]*")*$)/)
    let regMap = [
        {
            type: 'use',
            reg: /use\s*(\w+)/,
        },
        {
            type: 'db',
            reg: /db\.(\w+).(\w+)\((.*),\s*(.*)\)/,
        },
    ]
    let exec: any[] = []
    sql_str_arr.forEach((sql: string) => {
        for (let i = 0; i < regMap.length; i++) {
            let rm = regMap[i]
            let match = sql.match(rm.reg)
            if (match) {
                switch (rm.type) {
                    case 'use':
                        exec.push({
                            type: 'use',
                            db: match[1],
                        })
                        break
                    case 'db':
                        exec.push({
                            type: 'db',
                            col: match[1],
                            func: match[2],
                            document: match[3],
                            options: match[4],
                        })
                        break
                }
            }
        }
    })
    console.log(exec)
}
</script>
    
<template>
    <div class="page">
        <div class="menu">
            <n-button strong secondary size="small" @click="handleSelect" :loading="loading">
                <template #icon>
                    <n-icon>
                        <Checkmark />
                    </n-icon>
                </template>
            </n-button>
        </div>
        <div class="input">
            <EditorVue ref="editorRef" @change="handleChange" :value="config.query" :type="'mongo_query'" />
        </div>
        <div class="output">
            <!-- <n-tabs v-model:value="tab" type="card" tab-style="min-width: 80px;" size="small">
                                                                                                <n-tab-pane display-directive="show" v-for="(i, index) in results" :key="i.id" :tab="`Result ${index}`"
                                                                                                    :name="i.id">
                                                                                                    <div class="sql">{{ i.sql }}</div>
                                                                                                    <div class="res">
                                                                                                        <TableViewVue v-if="i.type == 'select'" :data="i.data" />
                                                                                                        <div v-else>{{ i }}</div>
                                                                                                    </div>
                                                                                                </n-tab-pane>
                                                                                            </n-tabs> -->
        </div>
    </div>
</template>

<style scoped>
.page {
    position: absolute;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
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
    bottom: 0;
}

.page .output .sql {
    position: absolute;
    top: 0;
    left: 0;
    right: 0;
    height: 30px;
    line-height: 30px;
    padding: 0 6px;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
}

.page .output .res {
    position: absolute;
    top: 30px;
    left: 0;
    right: 0;
    bottom: 0;
}
</style>
    