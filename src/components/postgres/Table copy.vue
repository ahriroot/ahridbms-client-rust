<script setup lang="ts">
import { select, update } from '@/api/postgres';
import { Connection } from '@/types/Connection';
import { PostgresConnect } from '@/types/postgres';
import { h, ref, shallowRef, onBeforeMount } from 'vue'
import {
    useLoadingBar, NDataTable, NTable, NButton, NSpace, useMessage
} from 'naive-ui'
import IntVue from '@/components/postgres/cell/Int.vue'
import VarCharVue from '@/components/postgres/cell/VarChar.vue'
import BoolVue from '@/components/postgres/cell/Bool.vue'
import TimestampTZVue from '@/components/postgres/cell/TimestampTZ.vue'


const props = defineProps<{
    conn: Connection<PostgresConnect>
    data: any
}>()
const emits = defineEmits<{
    (e: 'handle', val: null): void
}>()

const loadingBar = useLoadingBar()
const skip = ref(0)
const limit = ref(10)
const loadingCount = ref(0)
const message = useMessage()

const loadingStart = () => {
    loadingCount.value++
    if (loadingCount.value == 1) {
        loadingBar.start()
    }
}
const loadingFinish = () => {
    if (loadingCount.value > 0) {
        loadingCount.value--
    }
    if (loadingCount.value == 0) {
        loadingBar.finish()
    }
}
const format = (time: number) => {
    const date = new Date(time * 1000)
    const year = date.getUTCFullYear()
    const month = date.getUTCMonth() + 1
    const day = date.getUTCDate()
    const hour = date.getUTCHours()
    const minute = date.getUTCMinutes()
    const second = date.getUTCSeconds()
    return `${year}-${month}-${day} ${hour}:${minute}:${second}`
}

const typeRender = shallowRef<{ [key: string]: any; }>({
    Int: {
        width: 160,
        component: IntVue
    },
    VarChar: {
        width: 200,
        component: VarCharVue
    },
    Bool: {
        width: 200,
        component: BoolVue
    },
    TimestampTZ: {
        width: 200,
        component: TimestampTZVue
    }
})
const data = ref([])
const columns = ref<any>([])
const pagination = ref(false as const)
onBeforeMount(async () => {
    loadingStart()
    let res = await select({
        conn: props.conn,
        skip: skip.value,
        limit: limit.value,
        page: 0,
        size: 0,
        database: props.data.database,
        table: props.data.table
    })
    loadingFinish()
    if (res.length > 0) {
        let first = res[0]
        console.log(first)
        columns.value = []
        constData.value = []
        first.forEach((field: any) => {
            for (const k in field) {
                let fieldType = typeRender.value[k]
                constData.value.push({
                    [field[k].key]: {
                        type: k,
                        key: field[k].key,
                        value: field[k].value,
                        create: true
                    },
                } as never)
                let tmp: any = {
                    title: field[k].key,
                    key: field[k].key,
                }
                if (fieldType) {
                    tmp.width = fieldType.width
                    tmp.render = (row: any, index: number) => {
                        return h(fieldType.component, {
                            field: row[field[k].key],
                            isEdit: false,
                            onUpdateValue(v: any) {
                                let c = []
                                for (let s in row) {
                                    if (v.key == s && v.value === row[s].value) {
                                        return
                                    }
                                    if ([
                                        'VarChar',
                                        'CharN',
                                        'Text',
                                        'Citext',
                                        'Name',
                                        'Unknown',
                                        'Json',
                                        'Xml',
                                        'Aclitem',
                                        'Ignore'
                                    ].includes(row[s].type)) {
                                        c.push(`"${s}" = '${row[s].value}'`)
                                    } else if (row[s].type == 'TimestampTZ') {
                                        c.push(`"${s}" = '${format(row[s].value)}'`)
                                    } else {
                                        c.push(`"${s}" = ${row[s].value}`)
                                    }
                                }
                                let sql = `UPDATE "public".${props.data.table} SET ${v.key} = ${v.value} WHERE ${c.join(' AND ')}`
                                loadingStart()
                                update({
                                    conn: props.conn,
                                    database: props.data.database,
                                    sql: sql
                                }).then(res => {
                                    message.success(`Affected Rows: ${res}`)
                                    loadingFinish()
                                }).finally(() => {
                                    loadingFinish()
                                })
                                data.value[index][field[k].key] = v as never
                            }
                        })
                    }
                } else {
                    tmp.render = (row: any) => {
                        return h('span', row[field[k].key.value])
                    }
                }
                columns.value.push(tmp)
            }
        })
        columns.value.push({
            title: 'Action',
            key: 'actions',
            fixed: 'right',
            width: '140',
            render(row: any) {
                return h(
                    NSpace,
                    {},
                    {
                        default: () => [h(
                            NButton,
                            {
                                strong: true,
                                tertiary: true,
                                size: 'small',
                                onClick: () => {

                                    console.log(row)
                                }
                            },
                            { default: () => 'Edit' }
                        ), h(
                            NButton,
                            {
                                strong: true,
                                tertiary: true,
                                size: 'small',
                                onClick: () => {
                                    let c = []
                                    for (let s in row) {
                                        if ([
                                            'VarChar',
                                            'CharN',
                                            'Text',
                                            'Citext',
                                            'Name',
                                            'Unknown',
                                            'Json',
                                            'Xml',
                                            'Aclitem',
                                            'Ignore'
                                        ].includes(row[s].type)) {
                                            c.push(`"${s}" = '${row[s].value}'`)
                                        } else if (row[s].type == 'TimestampTZ') {
                                            c.push(`"${s}" = '${format(row[s].value)}'`)
                                        } else {
                                            c.push(`"${s}" = ${row[s].value}`)
                                        }
                                    }
                                    let sql = `DELETE FROM "public".${props.data.table} WHERE ${c.join(' AND ')}`
                                    loadingStart()
                                    update({
                                        conn: props.conn,
                                        database: props.data.database,
                                        sql: sql
                                    }).then(res => {
                                        message.success(`Affected Rows: ${res}`)
                                        loadingFinish()
                                    }).finally(() => {
                                        loadingFinish()
                                    })
                                }
                            },
                            { default: () => 'Delete' }
                        )]
                    }
                )
            }
        })

        let tbs: any = []
        res.forEach((data: any) => {
            let table: any = {}
            data.forEach((field: any) => {
                for (const k in field) {
                    table[field[k].key] = {
                        type: k,
                        key: field[k].key,
                        value: field[k].value
                    }
                }
            })
            tbs.push(table)
        })
        data.value = tbs
        console.log(tbs)
        loadingFinish()
    }
})

const constData = ref([])

const handleCreateData = () => {
    data.value.unshift(constData.value as never)
}
</script>
    
<template>
    <n-button @click="handleCreateData">Create</n-button>
    <n-data-table :columns="columns" :data="data" :pagination="pagination" :single-line="false" size="small" />
    <n-table :bordered="false" :single-line="false">
        <thead>
            <tr>
                <th>Abandon</th>
                <th>Abormal</th>
                <th>Abolish</th>
                <th>...</th>
                <th>万事开头难</th>
            </tr>
        </thead>
        <tbody>
            <tr>
                <td>放弃</td>
                <td>反常的</td>
                <td>彻底废除</td>
                <td>...</td>
                <td>干！我刚才背的是啥</td>
            </tr>
            <tr>
                <td>...</td>
                <td>...</td>
                <td>...</td>
                <td>...</td>
                <td>...</td>
            </tr>
        </tbody>
    </n-table>
</template>
    
<style scoped>

</style>
    