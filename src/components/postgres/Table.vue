<script setup lang="ts">
import { select, update } from '@/api/postgres';
import { Connection } from '@/types/Connection';
import { PostgresConnect } from '@/types/postgres';
import { h, ref, shallowRef, onBeforeMount } from 'vue'
import {
    useLoadingBar, NTable, NButton, NIcon, useMessage
} from 'naive-ui'
import { Trash, } from '@vicons/ionicons5'

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

interface Column {
    name: string
    type: string
}
const columns = ref<Column[]>([])
const data = ref<any[]>([])
const changedData = ref<any[]>([])
const handleLoadData = async () => {
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
        columns.value = []
        let first = res[0]
        first.forEach((field: any) => {
            for (let k in field) {
                columns.value.push({
                    name: field[k].key,
                    type: k
                })
            }
        })

        let tmp: any[] = []
        res.forEach((row: any) => {
            let rowData: any = []
            row.forEach((field: any) => {
                for (const type in field) {
                    rowData.push({
                        key: field[type].key,
                        value: field[type].value,
                        type: type
                    })
                }
            })
            tmp.push(rowData)
        })
        data.value = tmp
    }
}
onBeforeMount(async () => {
    await handleLoadData()
})

const handleUpdateValue = (value: any, row: any) => {
    let c: string[] = []
    row.forEach((field: any) => {
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
        ].includes(field.type)) {
            c.push(`"${field.key}" = '${field.value}'`)
        } else if (field.type == 'TimestampTZ') {
            c.push(`"${field.key}" = '${format(field.value)}'`)
        } else {
            c.push(`"${field.key}" = ${field.value}`)
        }
    })
    let where = c.join(' AND ')
    let has = changedData.value.some((item: any) => {
        return item.where == where
    })
    if (has) {
        changedData.value = changedData.value.map((item: any) => {
            if (item.where == where) {
                if (item.value.some((v: any) => {
                    return v.key == value.key
                })) {
                    item.value = item.value.map((v: any) => {
                        if (v.key == value.key) {
                            return value
                        }
                        return v
                    })
                } else {
                    item.value.push(value)
                }
            }
            return item
        })
    } else {
        changedData.value.push({
            where: where,
            value: [value]
        })
    }
}
const handleUpdate = () => {
    let sqls: string[] = []
    changedData.value.forEach((row: any) => {
        let data: string[] = []
        row.value.forEach((field: any) => {
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
            ].includes(field.type)) {
                data.push(`"${field.key}" = '${field.value}'`)
            } else if (field.type == 'TimestampTZ') {
                data.push(`"${field.key}" = '${format(field.value)}'`)
            } else {
                data.push(`"${field.key}" = ${field.value}`)
            }
        })
        sqls.push(`UPDATE "${props.data.database}"."${props.data.table}" SET ${data.join(', ')} WHERE ${row.where};`)
    })
    console.log(sqls)
}

const handleDelete = (row: any[]) => {
    let c: string[] = []
    row.forEach((field: any) => {
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
        ].includes(field.type)) {
            c.push(`"${field.key}" = '${field.value}'`)
        } else if (field.type == 'TimestampTZ') {
            c.push(`"${field.key}" = '${format(field.value)}'`)
        } else {
            c.push(`"${field.key}" = ${field.value}`)
        }
    })
    let sql = `DELETE FROM "public".${props.data.table} WHERE ${c.join(' AND ')}`
    loadingStart()
    update({
        conn: props.conn,
        database: props.data.database,
        sql: sql
    }).then(async res => {
        message.success(`Affected Rows: ${res}`)
        await handleLoadData()
        loadingFinish()
    }).finally(() => {
        loadingFinish()
    })
}
</script>
    
<template>
    <n-button @click="handleUpdate">Update</n-button>
    <n-table :single-line="false" size="small">
        <thead>
            <tr>
                <th v-for="i in data[0]">{{ i.key }} ({{ i.type }})</th>
                <th>Opera</th>
            </tr>
        </thead>
        <tbody>
            <tr v-for="r in data">
                <td v-for="c in r">
                    <component v-if="typeRender[c.type]" :is="typeRender[c.type].component" :field="c" :isEdit="false"
                        :handleUpdateValue="(value:any)=>handleUpdateValue(value, r)"></component>
                </td>
                <td>
                    <n-button strong secondary type="info" size="small" @click="handleDelete(r)">
                        <template #icon>
                            <n-icon>
                                <trash />
                            </n-icon>
                        </template>
                    </n-button>
                </td>
            </tr>
        </tbody>
    </n-table>
</template>
    
<style scoped>

</style>
    