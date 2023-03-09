<script setup lang="ts">
import CollectionView from '@/components/mongodb/CollectionView.vue'
import { Connection } from '@/types/Connection'
import { MongodbConnect } from '@/types/mongodb'
import EditorVue from '@/components/Editor.vue'
import { ref, onBeforeMount } from 'vue'
import { NSpace, NSelect, NButton, NIcon, NTabs, NTabPane, useMessage } from 'naive-ui'
import { Checkmark } from '@vicons/ionicons5'
import { databases, find, insertMany, insertOne, deleteOne, deleteMany, updateOne, updateMany } from '@/api/mongodb'
import { uuid } from '@/utils/crypto'

const props = defineProps<{
    conn: Connection<MongodbConnect>
    data: any
}>()
const emits = defineEmits<{
    (e: 'handleCloseTab', val: null): void
}>()
const message = useMessage()

onBeforeMount(async () => {
    let cfg = localStorage.getItem(`mongo:query:${props.conn.id}`)
    if (cfg) {
        config.value = JSON.parse(cfg)
    }
    const res = await databases({ conn: props.conn })
    res.forEach((db: any) => {
        options.value.push({
            label: db,
            value: db,
        })
    })
})

const config = ref({
    expanded: [],
    resultList: [],
    query: ''
})

const handleChange = async (val: string) => {
    config.value.query = val
    localStorage.setItem(`mongo:query:${props.conn.id}`, JSON.stringify(config.value))
}

const db = ref<string>(props.data.database)
const options = ref<any[]>([])

const handleUse = async (match: RegExpMatchArray, command: string) => {
    db.value = match[1]
}

const handleDB = async (match: RegExpMatchArray, command: string) => {
    try {
        let col = match[1]
        let func = match[2]
        let m1 = JSON.parse(match[3])
        let m2 = {}
        if (match[4]) {
            m2 = JSON.parse(match[4])
        }
        let m3 = {}
        if (match[5]) {
            m3 = JSON.parse(match[5])
        }
        let res: any;
        let type = 'other'
        switch (func) {
            case 'insertOne':
                res = await insertOne({
                    conn: props.conn,
                    database: db.value,
                    collection: col,
                    document: m1,
                    options: m2,
                })
                type = 'insert'
                break
            case 'insertMany':
                res = await insertMany({
                    conn: props.conn,
                    database: db.value,
                    collection: col,
                    document: m1,
                    options: m2,
                })
                type = 'insert'
                break
            case 'updateOne':
                res = await updateOne({
                    conn: props.conn,
                    database: db.value,
                    collection: col,
                    filter: m1,
                    update: m2,
                    options: m3,
                })
                type = 'update'
                break
            case 'updateMany':
                res = await updateMany({
                    conn: props.conn,
                    database: db.value,
                    collection: col,
                    filter: m1,
                    update: m2,
                    options: m3,
                })
                type = 'update'
                break
            case 'deleteOne':
                res = await deleteOne({
                    conn: props.conn,
                    database: db.value,
                    collection: col,
                    document: m1,
                    options: m2,
                })
                type = 'delete'
                break
            case 'deleteMany':
                res = await deleteMany({
                    conn: props.conn,
                    database: db.value,
                    collection: col,
                    document: m1,
                    options: m2,
                })
                type = 'delete'
                break
            case 'find':
                res = await find({
                    conn: props.conn,
                    database: db.value,
                    collection: col,
                    document: m1,
                    options: m2,
                })
                type = 'find'
                break
        }
        results.value.push({
            id: await uuid(),
            type: type,
            sql: command,
            data: res
        })
        if (results.value.length == 1) {
            tab.value = results.value[0].id
        }
    }
    catch (e) {
        // message.error((e as any).message)
    }
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
            reg: /db\.(\w+).(\w+)\((.*)\)/,
        },
        {
            type: 'db',
            reg: /db\.(\w+).(\w+)\((.*),\s*(.*)\)/,
        },
        {
            type: 'db',
            reg: /db\.(\w+).(\w+)\((.*),\s*(.*),\s*(.*)\)/,
        },
    ]
    loading.value = true
    results.value = []
    sql_str_arr.forEach(async (command: string) => {
        for (let i = 0; i < regMap.length; i++) {
            let rm = regMap[i]
            let match = command.match(rm.reg)
            if (match) {
                switch (rm.type) {
                    case 'use':
                        await handleUse(match, command)
                        break
                    case 'db':
                        await handleDB(match, command)
                        break
                }
            }
        }
    })
    loading.value = false
}
</script>
    
<template>
    <div class="page">
        <div class="menu">
            <n-space>
                <n-button strong secondary size="small" @click="handleSelect" :loading="loading">
                    <template #icon>
                        <n-icon>
                            <Checkmark />
                        </n-icon>
                    </template>
                </n-button>
                <div>
                    <n-select style="min-width: 120px;" size="small" v-model:value="db" :options="options" />
                </div>
            </n-space>
        </div>
        <div class="input">
            <EditorVue ref="editorRef" @change="handleChange" :value="config.query" :type="'mongo_query'" />
        </div>
        <div class="output">
            <n-tabs v-model:value="tab" type="card" tab-style="min-width: 80px;" size="small">
                <n-tab-pane display-directive="show" v-for="(i, index) in results" :key="i.id" :tab="`Result ${index}`"
                    :name="i.id">
                    <div class="sql">{{ i.sql }}</div>
                    <div class="res">
                        <CollectionView v-if="i.type == 'find'" :data="i.data" />
                        <div v-else>{{ i.data }}</div>
                    </div>
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
    