<script setup lang="ts">
import { executeWithTransaction, getPrimaryKeys, getTableStruct, select, update } from '@/api/postgres'
import { Connection } from '@/types/Connection'
import { PostgresConnect } from '@/types/postgres'
import { h, ref, shallowRef, onBeforeMount, reactive } from 'vue'
import {
    useLoadingBar, NDataTable, NButton, NIcon, useDialog, useMessage
} from 'naive-ui'
import { Trash, Add, Checkmark } from '@vicons/ionicons5'
import useClipboard from "vue-clipboard3"

import BigIntVue from '@/components/postgres/cells/BigInt.vue'
import VarCharVue from '@/components/postgres/cells/VarChar.vue'
import BoolVue from '@/components/postgres/cells/Bool.vue'
import TimestampVue from '@/components/postgres/cells/Timestamp.vue'
import { useIndexStore } from '@/store'
import { useI18n } from 'vue-i18n'


const typeRender = shallowRef<{ [key: string]: any }>({
    int: {
        width: 160,
        component: BigIntVue
    },
    int2: {
        width: 120,
        component: BigIntVue
    },
    int4: {
        width: 120,
        component: BigIntVue
    },
    int8: {
        width: 160,
        component: BigIntVue
    },
    serial: {
        width: 160,
        component: BigIntVue
    },
    serial2: {
        width: 160,
        component: BigIntVue
    },
    serial4: {
        width: 160,
        component: BigIntVue
    },
    serial8: {
        width: 160,
        component: BigIntVue
    },
    varchar: {
        width: undefined,
        component: VarCharVue
    },
    name: {
        width: 200,
        component: VarCharVue
    },
    bool: {
        width: 90,
        component: BoolVue
    },
    timestamp: {
        width: 200,
        component: TimestampVue
    },
    timestamptz: {
        width: 200,
        component: TimestampVue
    }
})

const props = defineProps<{
    conn: Connection<PostgresConnect>
    data: any
}>()
const emits = defineEmits<{
    (e: 'handle', val: null): void
}>()

const { toClipboard } = useClipboard()
const { t } = useI18n()
const store = useIndexStore()
const loadingBar = useLoadingBar()
const dialog = useDialog()
const loadingCount = ref(0)
const message = useMessage()
const count = ref(0)

const pagination = reactive({
    page: 1,
    pageSize: 5,
    showSizePicker: true,
    itemCount: 0,
    showQuickJumper: true,
    pageSizes: [5, 10, 20, 50, 100, 500, 1000],
    onChange: async (page: number) => {
        pagination.page = page
        await handleLoadData()
    },
    onUpdatePageSize: async (pageSize: number) => {
        store.updateConfig({
            ...store.config,
            pageSize
        })
        pagination.pageSize = pageSize
        pagination.page = 1
        await handleLoadData()
    }
})

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

const sorts = ref<{
    field: string
    order: string
}[]>([])
const handleUpdateSorter = async (sorter: any) => {
    sorter.value = sorter
    sorts.value = []
    sorter.forEach((item: any) => {
        if (item.order === "ascend" || item.order === "descend") {
            sorts.value.push({
                field: item.columnKey,
                order: item.order === "ascend" ? "ASC" : "DESC"
            })
        }
    })
    await handleLoadData()
}

const data = ref<any[]>([])
const handleLoadData = async () => {
    loadingStart()
    let res = await select({
        conn: props.conn,
        skip: 0,
        limit: 0,
        page: pagination.page,
        size: pagination.pageSize,
        sorts: sorts.value,
        database: props.data.database,
        table: props.data.table
    })
    data.value = res.data
    pagination.itemCount = res.count
    count.value = res.count
    loadingFinish()
}

const columns = ref<any[]>([])
const struct = ref<any[]>([])
const newData = ref<any[]>([])
const pks = ref<string[]>([])
onBeforeMount(async () => {
    pagination.pageSize = store.config.pageSize
    getPrimaryKeys({
        conn: props.conn,
        database: props.data.database,
        table: props.data.table
    }).then((res: any) => {
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
    struct.value = await getTableStruct({
        conn: props.conn,
        database: props.data.database,
        table: props.data.table
    })
    struct.value.forEach((field: any) => {
        newData.value.push({
            type: field.typname,
            name: field.attname,
            value: null
        })
    })
    await handleLoadData()
    columns.value = []
    struct.value.forEach((column: any) => {
        let t = column.typname
        let comp = typeRender.value[t].component
        columns.value.push({
            title: column.attname,
            key: column.attname,
            width: typeRender.value[t].width,
            sorter: {
                multiple: 3
            },
            render(row: any, index: number) {
                let r = row.find((item: any) => item.field == column.attname)
                return h('div',
                    {
                        class: r.value !== r.old ? 'diff' : 'same'
                    },
                    [
                        h(comp, {
                            value: r.value,
                            onUpdateValue: (val: any) => {
                                r.value = val
                            },
                        })
                    ]
                )
            }
        })
    })
    columns.value.push({
        title: '',
        key: 'opera',
        width: 34,
        render: (row: any, index: number) => {
            if (isNaN(row[0].old)) {
                return h(
                    NButton,
                    {
                        size: 'small',
                        onClick: async () => await handleInsert(row)
                    },
                    {
                        default: () => h(
                            NIcon,
                            {},
                            {
                                default: () => h(Add)
                            }
                        )
                    }
                )
            } else {
                return h(
                    NButton,
                    {
                        size: 'small',
                        onClick: async () => await handleDelete(row)
                    },
                    {
                        default: () => h(
                            NIcon,
                            {},
                            {
                                default: () => h(Trash)
                            }
                        )
                    }
                )
            }
        }
    })
})

const where = (values: any[]) => {
    let wheres: string[] = []
    pks.value.forEach((pk, index) => {
        let tmp = values.find((item: any) => item.field == pk)
        let type = tmp.type;
        if (tmp.old === null) {
            wheres.push(`${pk} IS NULL`)
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
            ].includes(type)) {
                wheres.push(`${pk} = '${tmp.old}'`)
            } else if (type == 'TimestampTZ' || type == 'Timestamp') {
                wheres.push(`${pk} = '${format(tmp.old)}'`)
            } else {
                wheres.push(`${pk} = ${tmp.old}`)
            }
        }
    })
    return wheres.join(' AND ')
}

const handleUpdate = async () => {
    let sqls: string[] = []
    data.value.forEach((row: any) => {
        let w = where(row)
        let changeData: any[] = []
        row.forEach((column: any) => {
            if (column.value !== column.old) {
                let type = column.type;
                if (column.value === null) {
                    changeData.push(`${column.field} = NULL`)
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
                    ].includes(type)) {
                        changeData.push(`${column.field} = '${column.value}'`)
                    } else if (type == 'TimestampTZ' || type == 'Timestamp') {
                        changeData.push(`${column.field} = '${format(column.value)}'`)
                    } else {
                        changeData.push(`${column.field} = ${column.value}`)
                    }
                }
            }
        })
        if (changeData.length > 0) {
            sqls.push(`UPDATE "public".${props.data.table} SET ${changeData.join(', ')} WHERE ${w}`)
        }
    })
    if (sqls.length > 0) {
        loadingStart()
        let res = await executeWithTransaction({
            conn: props.conn,
            database: props.data.database,
            sqls: sqls
        })
        loadingFinish()
        if (!res.is_error) {
            message.success(`Affected Rows: ${res}`)
            await handleLoadData()
        }
    }
}

const handleDelete = async (row: any) => {
    if (store.config?.deleteNoConfirm) {
        let w = where(row)
        let sql = `DELETE FROM "public".${props.data.table} WHERE ${w}`
        loadingStart()
        let res = await update({
            conn: props.conn,
            database: props.data.database,
            sql: sql
        })
        loadingFinish()
        if (!res.is_error) {
            message.success(`Affected Rows: ${res}`)
            await handleLoadData()
        }
    } else {
        let w = where(row)
        dialog.warning({
            title: t('delete'),
            content: `${t('delete')} [WHERE ${w}] ?`,
            positiveText: t('delete'),
            onPositiveClick: async () => {
                let w = where(row)
                let sql = `DELETE FROM "public".${props.data.table} WHERE ${w}`
                loadingStart()
                let res = await update({
                    conn: props.conn,
                    database: props.data.database,
                    sql: sql
                })
                loadingFinish()
                if (!res.is_error) {
                    message.success(`Affected Rows: ${res}`)
                    await handleLoadData()
                }
            }
        })
    }


}

const handleInsert = async (row: any) => {
    let fields: any[] = []
    let values: any[] = []
    row.forEach((column: any) => {
        if (column.value !== column.old) {
            if (column.value !== null) {
                let type = column.type;
                fields.push(column.field)
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
                ].includes(type)) {
                    values.push(`'${column.value}'`)
                } else if (type == 'TimestampTZ' || type == 'Timestamp') {
                    values.push(`'${format(column.value)}'`)
                } else {
                    values.push(`${column.value}`)
                }
            }
        }
    })
    let sql = `INSERT INTO "public"."${props.data.table}" (${fields.join(', ')}) VALUES (${values.join(', ')})`
    loadingStart()
    let res = await update({
        conn: props.conn,
        database: props.data.database,
        sql: sql
    })
    loadingFinish()
    if (!res.is_error) {
        message.success(`Affected Rows: ${res}`)
        showNewRow.value = false
        await handleLoadData()
    }
}

const showNewRow = ref(false)
const handleCreate = async () => {
    showNewRow.value = !showNewRow.value
    if (showNewRow.value) {
        data.value.push([
            {
                "type": "BigInt",
                "field": "id",
                "value": null,
                "old": NaN
            },
            {
                "type": "VarChar",
                "field": "username",
                "value": null,
                "old": NaN
            },
            {
                "type": "VarChar",
                "field": "password",
                "value": null,
                "old": NaN
            },
            {
                "type": "Bool",
                "field": "active",
                "value": null,
                "old": NaN
            },
            {
                "type": "Timestamp",
                "field": "last_login",
                "value": null,
                "old": NaN
            },
            {
                "type": "SmallInt",
                "field": "age",
                "value": null,
                "old": NaN
            }
        ])
    } else {
        data.value.pop()
    }
}
</script>
    
<template>
    <div style="height: 100%; position: relative;">
        <div class="opera-content">
            <div class="left">Count: {{ count }}</div>
            <div class="right">
                <n-button strong secondary size="small" @click="handleUpdate">
                    <template #icon>
                        <n-icon>
                            <checkmark />
                        </n-icon>
                    </template>
                </n-button>&nbsp;
                <n-button strong secondary size="small" @click="handleCreate">
                    <template #icon>
                        <n-icon>
                            <Add />
                        </n-icon>
                    </template>
                </n-button>
            </div>
        </div>
        <n-data-table size="small" :single-line="false" :columns="columns" :data="data" flex-height
            style="position: absolute; top: 32px; bottom: 40px;" :loading="loadingCount > 0" :pagination="pagination"
            :remote="true" :scroll-x="900" @update:sorter="handleUpdateSorter" />
    </div>
</template>
    
<style scoped>
.opera-content {
    display: flex;
    justify-content: space-between;
    padding: 2px 5px;
}

.opera-content .left,
.opera-content .right {
    display: flex;
    align-items: center;
}

.n-data-table :deep(td) {
    margin: 0;
    padding: 0;
    background: none;
}

:deep(.same) {
    min-height: 28px;
}

:deep(.diff) {
    min-height: 28px;
    background: #303033 !important;
}
</style>
    