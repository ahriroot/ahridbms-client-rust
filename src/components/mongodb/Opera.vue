<script setup lang="ts">
import CollectionView from '@/components/mongodb/CollectionView.vue'
import { Connection } from '@/types/Connection'
import { MongodbConnect } from '@/types/mongodb'
import EditorVue from '@/components/Editor.vue'
import { ref, onBeforeMount, shallowRef } from 'vue'
import { NSpace, NSelect, NButton, NIcon, NTabs, NTabPane, useMessage } from 'naive-ui'
import { Checkmark } from '@vicons/ionicons5'
import { databases, find, insertMany, insertOne, deleteOne, deleteMany, updateOne, updateMany, collections } from '@/api/mongodb'
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
    const resdbs = await databases({ conn: props.conn })
    dbs.value = []
    resdbs.forEach((db: any) => {
        dbs.value.push({
            label: db,
            value: db,
        })
    })
    const rescols = await collections({ conn: props.conn, database: db.value })
    cols.value = []
    rescols.forEach((db: any) => {
        cols.value.push({
            label: db,
            value: db,
        })
    })
    if (rescols.length > 0) {
        col.value = rescols[0]
    } else {
        col.value = ''
    }
})

const config = ref({
    expanded: [],
    resultList: [],
    query: ''
})

const handleDBChanged = async (val: string) => {
    db.value = val
    const rescols = await collections({ conn: props.conn, database: db.value })
    cols.value = []
    rescols.forEach((db: any) => {
        cols.value.push({
            label: db,
            value: db,
        })
    })
    if (rescols.length > 0) {
        col.value = rescols[0]
    } else {
        col.value = ''
    }
}

const handleChange = async (val: string) => {
    config.value.query = val
}

const db = ref<string>(props.data.database)
const dbs = ref<any[]>([])
const col = ref<string>("")
const cols = shallowRef<any[]>([])
const method = ref<string>("find")
const methods = shallowRef<any[]>([
    {
        label: "find",
        value: "find"
    },
    {
        label: "insertOne",
        value: "insertOne"
    },
    {
        label: "insertMany",
        value: "insertMany"
    },
    {
        label: "updateOne",
        value: "updateOne"
    },
    {
        label: "updateMany",
        value: "updateMany"
    },
    {
        label: "deleteOne",
        value: "deleteOne"
    },
    {
        label: "deleteMany",
        value: "deleteMany"
    },
])

const editorRefA = ref<any>()
const editorRefB = ref<any>()
const editorRefC = ref<any>()
const results = ref<any[]>([])
const tab = ref<string>('')
const loading = ref<boolean>(false)
const result = ref<any>(null)
const handleSelect = async () => {
    result.value = null
    loading.value = true
    if (db.value == '') {
        loading.value = false
        message.error('No database selected')
        return
    }
    if (col.value == '') {
        loading.value = false
        message.error('No collection selected')
        return
    }
    let a = await editorRefA.value?.getValue()
    let b = await editorRefB.value?.getValue()
    let c = await editorRefC.value?.getValue()
    try {
        a = JSON.parse(a)
        b = JSON.parse(b)
        c = JSON.parse(c)
    } catch (e) {
        loading.value = false
        message.error('Invalid JSON')
        return
    }
    switch (method.value) {
        case 'find':
            result.value = await find({
                conn: props.conn,
                database: db.value,
                collection: col.value,
                document: a,
                options: b,
            })
            break
        case 'insertOne':
            result.value = await insertOne({
                conn: props.conn,
                database: db.value,
                collection: col.value,
                document: a,
                options: b,
            })
            break
        case 'insertMany':
            result.value = await insertMany({
                conn: props.conn,
                database: db.value,
                collection: col.value,
                document: a,
                options: b,
            })
            break
        case 'updateOne':
            result.value = await updateMany({
                conn: props.conn,
                database: db.value,
                collection: col.value,
                filter: a,
                update: b,
                options: c,
            })
            break
        case 'updateMany':
            result.value = await updateMany({
                conn: props.conn,
                database: db.value,
                collection: col.value,
                filter: a,
                update: b,
                options: c,
            })
            break
        case 'deleteOne':
            result.value = await deleteOne({
                conn: props.conn,
                database: db.value,
                collection: col.value,
                document: a,
                options: b,
            })
            break
        case 'deleteMany':
            result.value = await deleteMany({
                conn: props.conn,
                database: db.value,
                collection: col.value,
                document: a,
                options: b,
            })
            break
    }
    loading.value = false
}
</script>
    
<template>
    <div class="page">
        <div class="menu">
            <n-space>
                <n-space>
                    <n-select style="min-width: 120px;" size="small" @on-update:value="handleDBChanged" filterable tag
                        v-model:value="db" :options="dbs" />
                    <n-select style="min-width: 120px;" size="small" filterable tag v-model:value="col" :options="cols" />
                    <n-select style="min-width: 140px;" size="small" v-model:value="method" :options="methods" />
                </n-space>
                <n-button strong secondary size="small" @click="handleSelect" :loading="loading">
                    <template #icon>
                        <n-icon>
                            <Checkmark />
                        </n-icon>
                    </template>
                </n-button>
            </n-space>
        </div>
        <div class="input">
            <div class="editorA">
                <EditorVue ref="editorRefA" @change="handleChange" value="{}" :type="'json'" />
            </div>
            <div class="editorB">
                <EditorVue ref="editorRefB" @change="handleChange" value="{}" :type="'json'" />
            </div>
            <div class="editorC">
                <EditorVue ref="editorRefC" @change="handleChange" value="{}" :type="'json'" />
            </div>
        </div>
        <div class="output">
            <div class="res">
                <CollectionView v-if="method == 'find' && result" :data="result" />
                <div v-else>{{ result }}</div>
            </div>
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

.page .input .editorA {
    position: absolute;
    top: 0;
    left: 0;
    width: calc(33% - 4px);
    bottom: 0;
}

.page .input .editorB {
    position: absolute;
    top: 0;
    left: 33%;
    right: 33%;
    bottom: 0;
}

.page .input .editorC {
    position: absolute;
    top: 0;
    width: calc(33% - 4px);
    right: 0;
    bottom: 0;
}

.page .output {
    position: absolute;
    top: 410px;
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
    