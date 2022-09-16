<script setup lang="ts">
import { executeWithTransaction, getPrimaryKeys, select, update } from '@/api/postgres'
import { Connection } from '@/types/Connection'
import { PostgresConnect } from '@/types/postgres'
import { ref, shallowRef, onBeforeMount, getCurrentInstance } from 'vue'
import {
    useLoadingBar, NTable, NButton, NIcon, NPopover, NPagination, useMessage
} from 'naive-ui'
import { Trash, Add } from '@vicons/ionicons5'
import useClipboard from "vue-clipboard3"

import BigIntVue from '@/components/postgres/cell/BigInt.vue'
import IntVue from '@/components/postgres/cell/Int.vue'
import SmallIntVue from '@/components/postgres/cell/SmallInt.vue'
import VarCharVue from '@/components/postgres/cell/VarChar.vue'
import BoolVue from '@/components/postgres/cell/Bool.vue'
import TimestampVue from '@/components/postgres/cell/Timestamp.vue'
import TimestampTZVue from '@/components/postgres/cell/TimestampTZ.vue'


const props = defineProps<{
    conn: Connection<PostgresConnect>
    data: any
}>()
const emits = defineEmits<{
    (e: 'handle', val: null): void
}>()

const { toClipboard } = useClipboard()
const loadingBar = useLoadingBar()
const loadingCount = ref(0)
const message = useMessage()
const page = ref(1)
const size = ref(10)

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
/**
 * 复制数据到剪切板
 * @param val 要复制的数据
 */
const handleCopy = async (val: any, tip: string = '') => {
    if (val != null && val != undefined) {
        let str
        // 判断 val 是否为字符串
        if (typeof val === 'string') {
            str = val
        } else if (typeof val === 'object') {
            try {
                str = JSON.stringify(val)
            } catch (error) {
                str = val.toString()
            }
        } else {
            str = val.toString()
        }
        await toClipboard(str)
        message.success(tip || 'Cpoied!', { duration: 800 })
    }
}

const typeRender = shallowRef<{ [key: string]: any }>({
    BigInt: {
        width: 160,
        component: BigIntVue
    },
    Int: {
        width: 160,
        component: IntVue
    },
    SmallInt: {
        width: 160,
        component: SmallIntVue
    },
    VarChar: {
        width: 200,
        component: VarCharVue
    },
    Bool: {
        width: 200,
        component: BoolVue
    },
    Timestamp: {
        width: 200,
        component: TimestampVue
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
        skip: 0,
        limit: 0,
        page: page.value,
        size: size.value,
        database: props.data.database,
        table: props.data.table
    })
    loadingFinish()
    if (res.length > 0) {
        columns.value = []
        newData.value = []
        let first = res[0]
        first.forEach((field: any) => {
            for (let type in field) {
                columns.value.push({
                    name: field[type].key,
                    type: type
                })
                newData.value.push({
                    key: field[type].key,
                    value: null,
                    type: type
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
const pks = ref<string[]>([])
onBeforeMount(async () => {
    getPrimaryKeys({
        conn: props.conn,
        database: props.data.database,
        table: props.data.table
    }).then(res => {
        pks.value = []
        res.forEach((fields: any) => {
            fields.forEach((field: any) => {
                for (let k in field) {
                    if (field[k].key == 'colname') {
                        pks.value.push(field[k].value)
                    }
                }
            })
        })
    })
    await handleLoadData()
})

const handleUpdateValue = (value: any, row: any) => {
    let c: string[] = []
    row.forEach((field: any) => {
        if (pks.value.includes(field.key)) {
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

const handleInsert = () => {
    let fields: any[] = []
    let values: any[] = []
    newData.value.forEach((field: any) => {
        fields.push(`"${field.key}"`)
        if (field.value == null) {
            values.push('NULL')
        } else {
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
                values.push(`'${field.value}'`)
            } else if (field.type == 'TimestampTZ' || field.type == 'Timestamp') {
                values.push(`'${format(field.value)}'`)
            } else {
                values.push(field.value)
            }
        }
    })
    let sql = `INSERT INTO "public"."${props.data.table}" (${fields.join(', ')}) VALUES (${values.join(', ')})`
    loadingStart()
    update({
        conn: props.conn,
        database: props.data.database,
        sql: sql
    }).then(async res => {
        message.success(`Affected Rows: ${res}`)
        await handleLoadData()
        showNewData.value = false
        newData.value.forEach((field: any) => {
            field.value = null
        })
        loadingFinish()
    }).finally(() => {
        loadingFinish()
    })
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
                if (field.value == null) {
                    data.push(`"${field.key}" = ${field.value}`)
                } else {
                    data.push(`"${field.key}" = '${field.value}'`)
                }
            } else if (field.type == 'TimestampTZ' || field.type == 'Timestamp') {
                data.push(`"${field.key}" = '${format(field.value)}'`)
            } else {
                data.push(`"${field.key}" = ${field.value}`)
            }
        })
        sqls.push(`UPDATE "public"."${props.data.table}" SET ${data.join(', ')} WHERE ${row.where}`)
    })
    console.log(sqls)
    loadingStart()
    executeWithTransaction({
        conn: props.conn,
        database: props.data.database,
        sqls: sqls
    }).then(async (res: number) => {
        console.log(res)
        message.success(`Affected Rows: ${res}`)
        changedData.value = []
        // nextTick(async () => {
        //     loadingFinish()
        // await handleLoadData()
        // })
        await handleLoadData()
        const instance = getCurrentInstance();
        instance?.proxy?.$forceUpdate();

        loadingFinish()
    }).catch((err: any) => {
        console.log(err)
        loadingFinish()
    })
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

const showNewData = ref(false)
const newData = ref<any[]>([])
const handleNewValue = (value: any) => {
    if (newData.value.some((item: any) => {
        return item.key == value.key
    })) {
        newData.value = newData.value.map((item: any) => {
            if (item.key == value.key) {
                return value
            }
            return item
        })
    } else {
        newData.value.push(value)
    }
}
</script>
    
<template>
    <n-button size="small" @click="handleUpdate">Update</n-button>
    <n-button size="small" @click="showNewData = !showNewData">NewData</n-button>
    <n-pagination v-model:page="page" :item-count="100" :page-sizes="[10,20,50,100,1000]" :page-size="size" size="small"
        show-quick-jumper show-size-picker />
    <n-table :single-line="false" size="small">
        <thead>
            <tr>
                <th v-for="i in data[0]" @click="handleCopy(i.key, 'key name cpoied')">
                    <n-popover trigger="hover">
                        <template #trigger>
                            <div>{{ i.key }}</div>
                        </template>
                        <span>{{ i.key }} ({{ i.type }})</span>
                    </n-popover>
                </th>
                <th></th>
            </tr>
        </thead>
        <tbody>
            <tr v-if="showNewData">
                <td v-for="i in newData">
                    <component :is="typeRender[i.type].component" :handleUpdateValue="handleNewValue" :field="i"
                        :isEdit="true" :newData="true">
                    </component>
                </td>
                <td>
                    <n-button strong secondary type="info" size="small" @click="handleInsert">
                        <template #icon>
                            <n-icon>
                                <Add />
                            </n-icon>
                        </template>
                    </n-button>
                </td>
            </tr>
            <tr v-for="r in data">
                <td v-for="c in r" :style="`width: ${typeRender[c.type].width}px`">
                    <component v-if="typeRender[c.type]" :key="c.id" :is="typeRender[c.type].component" :field="c"
                        :isEdit="false" :handleUpdateValue="(value:any)=>handleUpdateValue(value, r)"></component>
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
    