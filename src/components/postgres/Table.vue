<script setup lang="ts">
import { h, ref, shallowRef, onBeforeMount, reactive } from 'vue'
import {
    useLoadingBar, NDataTable, NButton, NIcon, useDialog, useMessage
} from 'naive-ui'
import { executeWithTransaction, select, selectWithStruct, update } from '@/api/postgres'
import { Connection } from '@/types/Connection'
import { PostgresConnect } from '@/types/postgres'
import { Trash, Add, Checkmark, Reload } from '@vicons/ionicons5'
import useClipboard from "vue-clipboard3"

import { useIndexStore } from '@/store'
import { useI18n } from 'vue-i18n'
import { FieldComponents } from '@/data/postgres'


const typeRender = shallowRef<{ [key: string]: any }>(FieldComponents)

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
const loadingStop = () => {
    loadingCount.value = 0
    loadingBar.finish()
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
    if (showNewRow.value) {
        await handleCreate()
    }
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
    loadingStop()
}

const columns = ref<any[]>([])
const struct = ref<any[]>([])
const newData = ref<any[]>([])
const pks = ref<string[]>([])
onBeforeMount(async () => {
    loadingStart()
    pagination.pageSize = store.config.pageSize

    let res = await selectWithStruct({
        conn: props.conn,
        skip: 0,
        limit: 0,
        page: pagination.page,
        size: pagination.pageSize,
        sorts: sorts.value,
        database: props.data.database,
        table: props.data.table
    })

    pks.value = []
    res.table_primary.forEach((fields: any) => {
        fields.forEach((field: any) => {
            for (let k in field) {
                if (field[k].key == 'colname') {
                    pks.value.push(field[k].value)
                }
            }
        })
    })

    struct.value = res.table_struct
    struct.value.forEach((field: any) => {
        newData.value.push({
            type: field.typname,
            name: field.attname,
            value: null
        })
    })

    data.value = res.table_data.data
    pagination.itemCount = res.table_data.count
    count.value = res.table_data.count

    columns.value = []
    struct.value.forEach((column: any) => {
        let t = column.typname
        let comp = typeRender.value[t].component
        minWidth.value += typeRender.value[t].minWidth
        columns.value.push({
            title: column.attname,
            key: column.attname,
            width: typeRender.value[t].width,
            resizable: true,
            minWidth: typeRender.value[t].minWidth,
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
        fixed: 'right',
        render: (row: any, index: number) => {
            if (isNaN(row[0].old)) {
                return h(
                    NButton,
                    {
                        size: 'small',
                        style: 'background: #282c34',
                        onClick: async () => await handleInsert()
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
                        style: 'background: #282c34',
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
    loadingFinish()
})

const where = (values: any[]) => {
    let wheres: string[] = []
    pks.value.forEach((pk, index) => {
        let tmp = values.find((item: any) => item.field == pk)
        let type = tmp.type;
        if (tmp.old === null) {
            wheres.push(`${pk} IS NULL`)
        } else {
            if (['char', 'varchar', 'text', 'date', 'time'].includes(type)) {
                wheres.push(`${pk} = '${tmp.old}'`)
            } else if (['time', 'timetz', 'date', 'timestamptz', 'timestamp', 'interval'].includes(type)) {
                wheres.push(`${pk} = '${format(tmp.old)}'`)
            } else {
                wheres.push(`${pk} = ${tmp.old}`)
            }
        }
    })
    return wheres.join(' AND ')
}

const handleUpdate = async () => {
    if (showNewRow.value) {
        await handleInsert()
        return
    }
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
                    if (['char', 'varchar', 'text', 'date', 'time'].includes(type)) {
                        changeData.push(`${column.field} = '${column.value}'`)
                    } else if (['time', 'timetz', 'date', 'timestamptz', 'timestamp', 'interval'].includes(type)) {
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

const handleInsert = async () => {
    if (!showNewRow.value) {
        return
    }
    let row = data.value[data.value.length - 1]
    let fields: any[] = []
    let values: any[] = []
    row.forEach((column: any) => {
        if (column.value !== column.old) {
            if (isNaN(column.old)) {
                let type = column.type;
                fields.push(column.field)
                if (['char', 'varchar', 'text', 'date', 'time'].includes(type)) {
                    values.push(`'${column.value}'`)
                } else if (['time', 'timetz', 'date', 'timestamptz', 'timestamp', 'interval'].includes(type)) {
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

const handleReload = async () => {
    await handleLoadData()
}

const showNewRow = ref(false)
const handleCreate = async () => {
    showNewRow.value = !showNewRow.value
    if (showNewRow.value) {
        let newLine: any[] = []
        struct.value.forEach((column: any) => {
            newLine.push({
                "type": column.typname,
                "field": column.attname,
                "value": null,
                "old": NaN
            })
        })
        data.value.push(newLine)
    } else {
        data.value.pop()
    }
}
const minWidth = ref(36)
</script>
    
<template>
    <div style="height: 100%; position: relative;">
        <div class="opera-content">
            <div class="left">Count: {{ count }}</div>
            <div class="right">
                <n-button strong secondary size="small" @click="handleReload" :disabled="loadingCount > 0">
                    <template #icon>
                        <n-icon>
                            <reload />
                        </n-icon>
                    </template>
                </n-button>&nbsp;
                <n-button strong secondary size="small" @click="handleCreate" :disabled="loadingCount > 0">
                    <template #icon>
                        <n-icon>
                            <Add />
                        </n-icon>
                    </template>
                </n-button>&nbsp;
                <n-button strong secondary size="small" @click="handleUpdate" :disabled="loadingCount > 0">
                    <template #icon>
                        <n-icon>
                            <checkmark />
                        </n-icon>
                    </template>
                </n-button>
            </div>
        </div>
        <n-data-table size="small" :single-line="false" :columns="columns" :data="data" flex-height
            style="position: absolute; top: 32px; bottom: 40px;" :loading="loadingCount > 0" :pagination="pagination"
            :remote="true" :scroll-x="minWidth" @update:sorter="handleUpdateSorter" />
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
    