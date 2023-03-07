<script setup lang="ts">
import { Connection } from '@/types/Connection'
import { PostgresConnect } from '@/types/postgres'
import { computed, ref } from 'vue'
import {
    NIcon, NButton, NInput, NInputNumber, NCheckbox, NSelect, NSpin, NModal,
    NTable, NTabs, NTabPane, NPopover, NInputGroup, useMessage
} from 'naive-ui'
import { Add, ArrowDown, ArrowUp, Trash, EllipsisHorizontal, Checkmark } from '@vicons/ionicons5'
import { useI18n } from 'vue-i18n'
import { executeWithTransaction, update } from '@/api/postgres'
import { emit } from '@tauri-apps/api/event'

const props = defineProps<{
    conn: Connection<PostgresConnect>
    data: any
}>()
const emits = defineEmits<{
    (e: 'handleCloseTab'): void
}>()

const { t } = useI18n()
const message = useMessage()

/** ------------- column ------------- **/
const tablename = ref('')
const loadingCreateTable = ref(false)
const types = ref([
    { label: 'smallserial', value: 'smallserial' },
    { label: 'bigserial', value: 'bigserial' },
    { label: 'serial', value: 'serial' },
    { label: 'serial2', value: 'serial2' },
    { label: 'serial4', value: 'serial4' },
    { label: 'serial8', value: 'serial8' },
    { label: 'bit', value: 'bit' },
    { label: 'bool', value: 'bool' },
    { label: 'box', value: 'box' },
    { label: 'bytea', value: 'bytea' },
    { label: 'char', value: 'char' },
    { label: 'cidr', value: 'cidr' },
    { label: 'circle', value: 'circle' },
    { label: 'date', value: 'date' },
    { label: 'decimal', value: 'decimal' },
    { label: 'float4', value: 'float4' },
    { label: 'float8', value: 'float8' },
    { label: 'inet', value: 'inet' },
    { label: 'int2', value: 'int2' },
    { label: 'int4', value: 'int4' },
    { label: 'int8', value: 'int8' },
    { label: 'interval', value: 'interval' },
    { label: 'json', value: 'json' },
    { label: 'jsonb', value: 'jsonb' },
    { label: 'line', value: 'line' },
    { label: 'lseg', value: 'lseg' },
    { label: 'macaddr', value: 'macaddr' },
    { label: 'money', value: 'money' },
    { label: 'numeric', value: 'numeric' },
    { label: 'path', value: 'path' },
    { label: 'point', value: 'point' },
    { label: 'polygon', value: 'polygon' },
    { label: 'text', value: 'text' },
    { label: 'time', value: 'time' },
    { label: 'timestamp', value: 'timestamp' },
    { label: 'timestamptz', value: 'timestamptz' },
    { label: 'timetz', value: 'timetz' },
    { label: 'tsquery', value: 'tsquery' },
    { label: 'tsvector', value: 'tsvector' },
    { label: 'txid_snapshot', value: 'txid_snapshot' },
    { label: 'uuid', value: 'uuid' },
    { label: 'varbit', value: 'varbit' },
    { label: 'varchar', value: 'varchar' },
    { label: 'xml', value: 'xml' },
    { label: '(Domain)', value: '(Domain)' },
    { label: '(Type)', value: '(Type)' },
])
const table = ref<any>({
    name: '',
    columns: [{
        index: new Date().getTime(),
        name: 'id',
        type: 'int4',
        length: null,
        decimalDigits: null,
        notNull: true,
        isPrimary: true,
        comment: '',
        default: '',
        dimension: 0,
        collation1: '',
        collation2: '',
        virtualType: '',
        increment: 0,
        minvalue: 0,
        maxvalue: 0,
        start: 0,
        cache: 0,
        cycle: false,
        expression: ''
    }]
})
const showColumnItemDetail = ref(false)
const showColumnItemIndex = ref(-1)
const handleColumnItemDetail = (index: number) => {
    showColumnItemIndex.value = index
    showColumnItemDetail.value = true
}
const handleColumnItem = (index: number, opera: 1 | 2 | 3) => {
    switch (opera) {
        case 1:  // 向下
            if (index < table.value.columns.length) {
                [table.value.columns[index], table.value.columns[index + 1]] = [table.value.columns[index + 1], table.value.columns[index]]
            }
            break
        case 2:  // 向上
            if (index > 0) {
                [table.value.columns[index - 1], table.value.columns[index]] = [table.value.columns[index], table.value.columns[index - 1]]
            }
            break
        case 3:  // 删除
            table.value.columns.splice(index, 1)
            break
    }
}
const addColumn = async () => {
    table.value.columns.push({
        index: new Date().getTime(),
        name: '',
        type: '',
        length: null,
        decimalDigits: null,
        notNull: false,
        isPrimary: false,
        comment: '',
        default: '',
        dimension: 0,
        collation1: '',
        collation2: '',
        virtualType: '',
        increment: 0,
        minvalue: 0,
        maxvalue: 0,
        start: 0,
        cache: 0,
        cycle: false,
        expression: ''
    })
}
const sqlCreateTable = computed(() => {
    let sql = `CREATE TABLE "public"."${tablename.value ? tablename.value : 'Untitled'}" (\n`
    let columns_sql: string[] = []
    let primaries: string[] = []
    let comments: string[] = []
    table.value.columns.forEach((item: any) => {
        if (item.name && item.type) {
            let column_sql = `  "${item.name}"`
            if (item.type != '(Domain)' && item.type != '(Type)') {
                column_sql += ` ${item.type}`
            }
            if (item.length && ['varchar', 'time', 'timestamp', 'timestamptz', 'timetz'].includes(item.type)) {
                column_sql += `(${item.length}`
                if (item.decimalDigits && ['numeric', 'decimal'].includes(item.type)) {
                    column_sql += `,${item.decimalDigits}`
                }
                column_sql += ')'
            }
            if (item.dimension) {
                for (let i = 0; i < item.dimension; i++) {
                    column_sql += '[]'
                }
            }
            if (item.collation1 && item.collation2) {
                column_sql += ` COLLATE "${item.collation1}"."${item.collation2}"`
            }
            if (item.notNull) {
                column_sql += ' NOT NULL'
            }
            if (item.default) {
                column_sql += ` DEFAULT ${item.default}`
            }
            if (item.virtualType) {
                if (item.virtualType == 'GENERATED ALWAYS AS IDENTITY') {
                    let virtualTypeList1: string[] = []
                    if (item.increment) {
                        virtualTypeList1.push(`    INCREMENT ${item.increment}`)
                    }
                    if (item.minvalue) {
                        virtualTypeList1.push(`    MINVALUE ${item.minvalue}`)
                    }
                    if (item.maxvalue) {
                        virtualTypeList1.push(`    MAXVALUE ${item.maxvalue}`)
                    }
                    if (item.start) {
                        virtualTypeList1.push(`    START ${item.start}`)
                    }
                    if (item.cache) {
                        virtualTypeList1.push(`    CACHE ${item.cache}`)
                    }
                    if (item.cycle) {
                        virtualTypeList1.push(`    CYCLE`)
                    }
                    if (virtualTypeList1.length) {
                        column_sql += ` GENERATED ALWAYS AS IDENTITY (\n${virtualTypeList1.join('\n')}\n  )`
                    } else {
                        column_sql += ` GENERATED ALWAYS AS IDENTITY`
                    }
                } else if (item.virtualType == 'GENERATED BY DEFAULT AS IDENTITY') {
                    let virtualTypeList2: string[] = []
                    if (item.increment) {
                        virtualTypeList2.push(`    INCREMENT ${item.increment}`)
                    }
                    if (item.minvalue) {
                        virtualTypeList2.push(`    MINVALUE ${item.minvalue}`)
                    }
                    if (item.maxvalue) {
                        virtualTypeList2.push(`    MAXVALUE ${item.maxvalue}`)
                    }
                    if (item.start) {
                        virtualTypeList2.push(`    START ${item.start}`)
                    }
                    if (item.cache) {
                        virtualTypeList2.push(`    CACHE ${item.cache}`)
                    }
                    if (item.cycle) {
                        virtualTypeList2.push(`    CYCLE`)
                    }
                    if (virtualTypeList2.length) {
                        column_sql += ` GENERATED BY DEFAULT AS IDENTITY (\n${virtualTypeList2.join('\n')}\n  )`
                    } else {
                        column_sql += ` GENERATED BY DEFAULT AS IDENTITY`
                    }
                } else if (item.virtualType == 'GENERATED ALWAYS AS') {
                    column_sql += ` GENERATED ALWAYS AS (${item.expression})`
                } else if (item.virtualType == 'GENERATED BY DEFAULT AS') {
                    column_sql += ` GENERATED BY DEFAULT AS (${item.expression})`
                } else if (item.virtualType == 'GENERATED ALWAYS AS STORED') {
                    column_sql += ` GENERATED ALWAYS AS (\n    ${item.expression}\n  ) STORED`
                }
            }
            if (item.isPrimary) {
                primaries.push(item.name)
            }
            if (item.comment) {
                comments.push(`COMMENT ON COLUMN "public"."${tablename.value ? tablename.value : 'Untitled'}"."${item.name}" IS '${item.comment}';`)
            }
            columns_sql.push(column_sql)
        }
    })
    if (columns_sql.length > 0) {
        sql += columns_sql.join(',\n')
    }
    if (primaries.length > 0) {
        sql += ',\n  PRIMARY KEY ("' + primaries.join('", "') + '")'
    }
    if (sql) {
        sql += '\n);'
    }
    return {
        sql,
        comments
    }
})

/** ------------- index ------------- **/
const functions = ref([
    { label: 'btree', value: 'btree' },
    { label: 'hash', value: 'hash' },
    { label: 'gist', value: 'gist' },
    { label: 'gin', value: 'gin' },
    { label: 'spgist', value: 'spgist' },
    { label: 'brin', value: 'brin' },
    { label: 'gin', value: 'gin' },
    { label: 'gin', value: 'gin' },
])
const index = ref<any[]>([{
    index: new Date().getTime(),
    name: '',
    fields: [],
    function: null,
    uniqueKey: false,
    concurrently: false,
    comment: ''
}])
const handleIndexItem = (i: number, opera: 1 | 2 | 3) => {
    switch (opera) {
        case 1:  // 向下
            if (i < index.value.length) {
                [index.value[i], index.value[i + 1]] = [index.value[i + 1], index.value[i]]
            }
            break
        case 2:  // 向上
            if (i > 0) {
                [index.value[i - 1], index.value[i]] = [index.value[i], index.value[i - 1]]
            }
            break
        case 3:  // 删除
            index.value.splice(i, 1)
            break
    }
}
const addIndex = async () => {
    index.value.push({
        index: new Date().getTime(),
        name: '',
        fields: [],
        function: null,
        uniqueKey: false,
        concurrently: false,
        comment: ''
    })
}
const sqlIndex = computed(() => {
    let sqls: string[] = []
    let comments: string[] = []
    index.value.forEach((item: any) => {
        if (item.name) {
            let fields: string[] = []
            let index_sql = `CREATE ${item.uniqueKey ? 'UNIQUE ' : ''}INDEX ${item.concurrently ? 'CONCURRENTLY ' : ''}"${item.name}" ON "public"."${tablename.value ? tablename.value : 'Untitled'}"${item.function ? ' USING ' + item.function : ''} (\n`
            if (item.comment) {
                comments.push(`COMMENT ON INDEX "${'public'}"."${item.name}" IS '${item.comment}';`)
            }
            item.fields.forEach((field: any) => {
                if (field.name) {
                    let tmp = `  "${field.name}"`
                    if (field.collation) {
                        tmp += ` COLLATE `
                        if (field.collationPattern) {
                            tmp += `"${field.collationPattern}".`
                        }
                        tmp += `"${field.collation}"`
                    }
                    if (field.operator) {
                        tmp += ` `
                        if (field.operatorPattern) {
                            tmp += `"${field.operatorPattern}".`
                        }
                        tmp += `"${field.operator}"`
                    }
                    if (field.sortOrder) {
                        tmp += ` ${field.sortOrder}`
                    }
                    if (field.nullsSort) {
                        tmp += ` ${field.nullsSort}`
                    }
                    fields.push(tmp)
                }
            })
            if (fields.length > 0) {
                index_sql += fields.join(',\n')
            }
            if (index_sql) {
                index_sql += '\n);'
                sqls.push(index_sql)
            }
        }
    })
    return {
        sqls,
        comments
    }
})
const showIndexFields = ref(false)
const showIndexFieldsIndex = ref(-1)
const handleEditFields = (i: number) => {
    showIndexFields.value = true
    showIndexFieldsIndex.value = i
}
const handleIndexFieldsItem = (i: number, opera: 1 | 2 | 3) => {
    switch (opera) {
        case 1:  // 向下
            if (i < index.value[showIndexFieldsIndex.value].fields.length) {
                [index.value[showIndexFieldsIndex.value].fields[i], index.value[showIndexFieldsIndex.value].fields[i + 1]] = [index.value[showIndexFieldsIndex.value].fields[i + 1], index.value[showIndexFieldsIndex.value].fields[i]]
            }
            break
        case 2:  // 向上
            if (i > 0) {
                [index.value[showIndexFieldsIndex.value].fields[i - 1], index.value[showIndexFieldsIndex.value].fields[i]] = [index.value[showIndexFieldsIndex.value].fields[i], index.value[showIndexFieldsIndex.value].fields[i - 1]]
            }
            break
        case 3:  // 删除
            index.value[showIndexFieldsIndex.value].fields.splice(i, 1)
            break
    }
}
const addIndexFields = () => {
    if (showIndexFieldsIndex.value > -1 && showIndexFieldsIndex.value < index.value.length) {
        index.value[showIndexFieldsIndex.value].fields.push({
            name: '',
            collationPattern: '',
            collation: '',
            operatorPattern: '',
            operator: '',
            sortOrder: '',
            nullsSort: ''
        })
    }
}

const formatIndexFields = (fields: any[]) => {
    let fields_list: any[] = []
    fields.forEach((field: any) => {
        if (field.name) {
            let tmp = `  "${field.name}"`
            if (field.collation) {
                tmp += ` COLLATE `
                if (field.collationPattern) {
                    tmp += `"${field.collationPattern}".`
                }
                tmp += `"${field.collation}"`
            }
            if (field.operator) {
                tmp += ` `
                if (field.operatorPattern) {
                    tmp += `"${field.operatorPattern}".`
                }
                tmp += `"${field.operator}"`
            }
            if (field.sortOrder) {
                tmp += ` ${field.sortOrder}`
            }
            if (field.nullsSort) {
                tmp += ` ${field.nullsSort}`
            }
            fields_list.push(tmp)
        }
    })
    return fields_list
}

const handleSubmitCreateTable = async () => {
    try {
        loadingCreateTable.value = true
        let sqls: string[] = [
            sqlCreateTable.value.sql
        ]
        // let resCreateTable = await update({
        //     conn: props.conn,
        //     database: props.data.database,
        //     sql: sqlCreateTable.value.sql
        // })
        // if (resCreateTable.is_error) {
        //     loadingCreateTable.value = false
        //     return
        // }
        for (let index = 0; index < sqlCreateTable.value.comments.length; index++) {
            sqls.push(sqlCreateTable.value.comments[index])
            // let resCreateTableComment = await update({
            //     conn: props.conn,
            //     database: props.data.database,
            //     sql: sqlCreateTable.value.comments[index]
            // })
            // if (resCreateTableComment.is_error) {
            //     loadingCreateTable.value = false
            //     return
            // }
        }
        for (let index = 0; index < sqlIndex.value.sqls.length; index++) {
            sqls.push(sqlIndex.value.sqls[index])
            // let resIndex = await update({
            //     conn: props.conn,
            //     database: props.data.database,
            //     sql: sqlIndex.value.sqls[index]
            // })
            // if (resIndex.is_error) {
            //     loadingCreateTable.value = false
            //     return
            // }
        }
        for (let index = 0; index < sqlIndex.value.comments.length; index++) {
            sqls.push(sqlIndex.value.comments[index])
            // let resIndexComment = await update({
            //     conn: props.conn,
            //     database: props.data.database,
            //     sql: sqlIndex.value.comments[index]
            // })
            // if (resIndexComment.is_error) {
            //     loadingCreateTable.value = false
            //     return
            // }
        }
        let res = await executeWithTransaction({
            conn: props.conn,
            database: props.data.database,
            sqls: sqls
        })
        if (!res.is_error) {
            message.success(res)
        }
        // await emit('reload', { conn: props.conn, database: props.data.database, table: tablename.value || 'Untitled' })
        // await emits('handleCloseTab')
    } catch (e) {
        message.error(e as string)
    } finally {
        loadingCreateTable.value = false
    }
}
</script>
    
<template>
    <div class="create-table" style="padding: 14px">
        <n-modal v-model:show="showColumnItemDetail" class="nocopy" preset="card" style="width: 574px;" :title="t('info')"
            size="small">
            <n-table size="small" :single-line="true">
                <tbody>
                    <tr>
                        <td>Default: </td>
                        <td>
                            <n-select size="small" filterable tag v-model:value="table.columns[showColumnItemIndex].default"
                                :options="[
                                    { label: 'NULL', value: 'NULL' },
                                    { label: 'Empty String', value: '\'\'' },
                                ]" clearable />
                        </td>
                    </tr>
                    <tr>
                        <td>Collation: </td>
                        <td style="display: flex;">
                            <n-select size="small" filterable tag
                                v-model:value="table.columns[showColumnItemIndex].collation1" :options="[
                                    { label: 'information_schema', value: 'information_schema' },
                                    { label: 'pg_catalog', value: 'pg_catalog' },
                                    { label: 'pg_toast', value: 'pg_toast' },
                                    { label: 'public', value: 'public' }
                                ]" clearable />
                            &nbsp;
                            <n-select size="small" filterable tag
                                v-model:value="table.columns[showColumnItemIndex].collation2" :options="[]" clearable />
                        </td>
                    </tr>
                    <tr>
                        <td>Dimension: </td>
                        <td>
                            <n-input-number size="small" v-model:value="table.columns[showColumnItemIndex].dimension" />
                        </td>
                    </tr>
                    <tr>
                        <td>Virtual Type: </td>
                        <td>
                            <n-select size="small" v-model:value="table.columns[showColumnItemIndex].virtualType" :options="[
                                { label: 'GENERATED ALWAYS AS IDENTITY', value: 'GENERATED ALWAYS AS IDENTITY' },
                                { label: 'GENERATED BY DEFAULT AS IDENTITY', value: 'GENERATED BY DEFAULT AS IDENTITY' },
                                { label: 'GENERATED ALWAYS AS STORED', value: 'GENERATED ALWAYS AS STORED' }
                            ]" clearable />
                        </td>
                    </tr>
                    <tr v-show="
                        table.columns[showColumnItemIndex].virtualType === 'GENERATED ALWAYS AS IDENTITY' ||
                        table.columns[showColumnItemIndex].virtualType === 'GENERATED BY DEFAULT AS IDENTITY'
                    ">
                        <td>Increment: </td>
                        <td>
                            <n-input-number size="small" v-model:value="table.columns[showColumnItemIndex].increment" />
                        </td>
                    </tr>
                    <tr v-show="
                        table.columns[showColumnItemIndex].virtualType === 'GENERATED ALWAYS AS IDENTITY' ||
                        table.columns[showColumnItemIndex].virtualType === 'GENERATED BY DEFAULT AS IDENTITY'
                    ">
                        <td>Minvalue: </td>
                        <td>
                            <n-input-number size="small" v-model:value="table.columns[showColumnItemIndex].minvalue" />
                        </td>
                    </tr>
                    <tr v-show="
                        table.columns[showColumnItemIndex].virtualType === 'GENERATED ALWAYS AS IDENTITY' ||
                        table.columns[showColumnItemIndex].virtualType === 'GENERATED BY DEFAULT AS IDENTITY'
                    ">
                        <td>Maxvalue: </td>
                        <td>
                            <n-input-number size="small" v-model:value="table.columns[showColumnItemIndex].maxvalue" />
                        </td>
                    </tr>
                    <tr v-show="
                        table.columns[showColumnItemIndex].virtualType === 'GENERATED ALWAYS AS IDENTITY' ||
                        table.columns[showColumnItemIndex].virtualType === 'GENERATED BY DEFAULT AS IDENTITY'
                    ">
                        <td>Start: </td>
                        <td>
                            <n-input-number size="small" v-model:value="table.columns[showColumnItemIndex].start" />
                        </td>
                    </tr>
                    <tr v-show="
                        table.columns[showColumnItemIndex].virtualType === 'GENERATED ALWAYS AS IDENTITY' ||
                        table.columns[showColumnItemIndex].virtualType === 'GENERATED BY DEFAULT AS IDENTITY'
                    ">
                        <td>Cache: </td>
                        <td>
                            <n-input-number size="small" v-model:value="table.columns[showColumnItemIndex].cache" />
                        </td>
                    </tr>
                    <tr v-show="
                        table.columns[showColumnItemIndex].virtualType === 'GENERATED ALWAYS AS IDENTITY' ||
                        table.columns[showColumnItemIndex].virtualType === 'GENERATED BY DEFAULT AS IDENTITY'
                    ">
                        <td>Cycle: </td>
                        <td>
                            <n-checkbox size="small" v-model:checked="table.columns[showColumnItemIndex].cycle">
                                Cycle
                            </n-checkbox>
                        </td>
                    </tr>
                    <tr v-show="table.columns[showColumnItemIndex].virtualType === 'GENERATED ALWAYS AS STORED'">
                        <td>Expression: </td>
                        <td>
                            <n-input size="small" v-model:value="table.columns[showColumnItemIndex].expression"
                                placeholder="Expression" />
                        </td>
                    </tr>
                </tbody>
            </n-table>
        </n-modal>
        <n-modal v-model:show="showIndexFields" class="nocopy" preset="card" style="width: 1200px;" :title="t('info')"
            size="small">
            <n-table size="small" :single-line="false">
                <thead>
                    <tr>
                        <th>Name</th>
                        <th style="width: 140px">Collation Pattern</th>
                        <th style="width: 140px">Collation</th>
                        <th style="width: 140px">Operator Pattern</th>
                        <th style="width: 140px">Operator</th>
                        <th style="width: 140px">Sort Order</th>
                        <th style="width: 140px">Nulls Sort</th>
                        <th style="width: 140px">
                            <n-button strong secondary size="small" @click="addIndexFields">
                                <template #icon>
                                    <n-icon>
                                        <add />
                                    </n-icon>
                                </template>
                            </n-button>
                        </th>
                    </tr>
                </thead>
                <tbody>
                    <tr v-for="(i, j) in index[showIndexFieldsIndex].fields">
                        <td>
                            <n-select size="small" filterable tag v-model:value="i.name" placeholder="Name"
                                :options="table.columns.map((column: any) => ({ label: column.name, value: column.name }))"
                                clearable />
                        </td>
                        <td>
                            <n-select size="small" filterable tag v-model:value="i.collationPattern" placeholder=""
                                :options="[
                                    { label: 'information_schema', value: 'information_schema' },
                                    { label: 'pg_catalog', value: 'pg_catalog' },
                                    { label: 'pg_toast', value: 'pg_toast' },
                                    { label: 'public', value: 'public' }
                                ]" clearable />
                        </td>
                        <td>
                            <n-select size="small" filterable tag v-model:value="i.collation" placeholder="" :options="[]"
                                clearable />
                        </td>
                        <td>
                            <n-select size="small" filterable tag v-model:value="i.operatorPattern" placeholder="" :options="[
                                { label: 'information_schema', value: 'information_schema' },
                                { label: 'pg_catalog', value: 'pg_catalog' },
                                { label: 'pg_toast', value: 'pg_toast' },
                                { label: 'public', value: 'public' }
                            ]" clearable />
                        </td>
                        <td>
                            <n-select size="small" filterable tag v-model:value="i.operator" placeholder="" :options="[]"
                                clearable />
                        </td>
                        <td>
                            <n-select size="small" v-model:value="i.sortOrder" placeholder=""
                                :options="[{ label: 'ASC', value: 'ASC' }, { label: 'DESC', value: 'DESC' }]" clearable />
                        </td>
                        <td>
                            <n-select size="small" v-model:value="i.nullsSort" placeholder=""
                                :options="[{ label: 'NULLS FIRST', value: 'NULLS FIRST' }, { label: 'NULLS LAST', value: 'NULLS LAST' }]"
                                clearable />
                        </td>
                        <td style="display: flex; justify-content: space-between;">
                            <n-button strong secondary type="info" size="small" @click="handleIndexFieldsItem(j, 3)">
                                <template #icon>
                                    <n-icon>
                                        <trash />
                                    </n-icon>
                                </template>
                            </n-button>
                            <n-button strong secondary type="info" size="small" :disabled="j == table.columns.length - 1"
                                @click="handleIndexFieldsItem(j, 1)">
                                <template #icon>
                                    <n-icon>
                                        <arrow-down />
                                    </n-icon>
                                </template>
                            </n-button>
                            <n-button strong secondary type="info" size="small" :disabled="j == 0"
                                @click="handleIndexFieldsItem(j, 2)">
                                <template #icon>
                                    <n-icon>
                                        <arrow-up />
                                    </n-icon>
                                </template>
                            </n-button>
                        </td>
                    </tr>
                </tbody>
            </n-table>
        </n-modal>
        <n-spin size="large" :show="loadingCreateTable">
            <n-input-group>
                <n-input v-model:value="tablename" placeholder="Table Name" clearable />
                <n-button ghost @click="handleSubmitCreateTable" :loading="loadingCreateTable">
                    <template #icon>
                        <n-icon>
                            <checkmark />
                        </n-icon>
                    </template>
                </n-button>
            </n-input-group>
            <n-tabs class="nocopy" type="card" size="small">
                <n-tab-pane display-directive="show" key="column" name="column" :tab="t('column')">
                    <n-table size="small" :single-line="false">
                        <thead>
                            <tr>
                                <th style="width: 200px">Name</th>
                                <th style="width: 154px">Type</th>
                                <th style="width: 130px">Length</th>
                                <th style="width: 120px">Decimal Digits</th>
                                <th style="width: 74px">Not Null</th>
                                <th style="width: 98px">Is Primary</th>
                                <th>Comment</th>
                                <th style="width: 180px">
                                    <n-button strong secondary size="small" @click="addColumn">
                                        <template #icon>
                                            <n-icon>
                                                <add />
                                            </n-icon>
                                        </template>
                                    </n-button>
                                </th>
                            </tr>
                        </thead>
                        <tbody>
                            <tr v-for="(i, index) in table.columns">
                                <td>
                                    <n-input size="small" v-model:value="i.name" placeholder="Name" clearable />
                                </td>
                                <td>
                                    <n-select size="small" filterable v-model:value="i.type" placeholder="Type"
                                        :options="types" />
                                </td>
                                <td>
                                    <n-input-number size="small" v-model:value="i.length" placeholder="Length" clearable />
                                </td>
                                <td>
                                    <n-input-number size="small" v-model:value="i.decimalDigits"
                                        placeholder="Decimal Digits" clearable />
                                </td>
                                <td>
                                    <n-checkbox size="small" v-model:checked="i.notNull">Null</n-checkbox>
                                </td>
                                <td>
                                    <n-checkbox size="small" v-model:checked="i.isPrimary">Primary</n-checkbox>
                                </td>
                                <td>
                                    <n-input size="small" v-model:value="i.comment" placeholder="Comment" clearable />
                                </td>
                                <td style="display: flex; justify-content: space-between;">
                                    <n-button strong secondary type="info" size="small"
                                        @click="handleColumnItemDetail(index)">
                                        <template #icon>
                                            <n-icon>
                                                <EllipsisHorizontal />
                                            </n-icon>
                                        </template>
                                    </n-button>
                                    <n-button strong secondary type="info" size="small" @click="handleColumnItem(index, 3)">
                                        <template #icon>
                                            <n-icon>
                                                <trash />
                                            </n-icon>
                                        </template>
                                    </n-button>
                                    <n-button strong secondary type="info" size="small"
                                        :disabled="index == table.columns.length - 1" @click="handleColumnItem(index, 1)">
                                        <template #icon>
                                            <n-icon>
                                                <arrow-down />
                                            </n-icon>
                                        </template>
                                    </n-button>
                                    <n-button strong secondary type="info" size="small" :disabled="index == 0"
                                        @click="handleColumnItem(index, 2)">
                                        <template #icon>
                                            <n-icon>
                                                <arrow-up />
                                            </n-icon>
                                        </template>
                                    </n-button>
                                </td>
                            </tr>
                        </tbody>
                    </n-table>
                </n-tab-pane>
                <n-tab-pane display-directive="show" key="index" name="index" :tab="t('index')">
                    <n-table size="small" :single-line="false">
                        <thead>
                            <tr>
                                <th style="width: 200px">Name</th>
                                <th style="width: 46px">Fields</th>
                                <th style="width: 140px">Function</th>
                                <th style="width: 140px">Unique Key</th>
                                <th style="width: 74px">Concurrently</th>
                                <th>Comment</th>
                                <th style="width: 140px">
                                    <n-button strong secondary size="small" @click="addIndex">
                                        <template #icon>
                                            <n-icon>
                                                <add />
                                            </n-icon>
                                        </template>
                                    </n-button>
                                </th>
                            </tr>
                        </thead>
                        <tbody>
                            <tr v-for="(i, j) in index">
                                <td>
                                    <n-input size="small" v-model:value="i.name" placeholder="Name" clearable />
                                </td>
                                <td>
                                    <n-popover trigger="hover">
                                        <template #trigger>
                                            <n-button size="small" strong secondary @click="handleEditFields(j)">Edit
                                            </n-button>
                                        </template>
                                        <pre>{{ formatIndexFields(i.fields).join('\n') }}</pre>
                                    </n-popover>
                                </td>
                                <td>
                                    <n-select size="small" filterable v-model:value="i.function" placeholder="Type"
                                        :options="functions" clearable />
                                </td>
                                <td>
                                    <n-checkbox size="small" v-model:checked="i.uniqueKey"></n-checkbox>
                                </td>
                                <td>
                                    <n-checkbox size="small" v-model:checked="i.concurrently"></n-checkbox>
                                </td>
                                <td>
                                    <n-input size="small" v-model:value="i.comment" placeholder="Comment" clearable />
                                </td>
                                <td style="display: flex; justify-content: space-between;">
                                    <n-button strong secondary type="info" size="small" @click="handleIndexItem(j, 3)">
                                        <template #icon>
                                            <n-icon>
                                                <trash />
                                            </n-icon>
                                        </template>
                                    </n-button>
                                    <n-button strong secondary type="info" size="small"
                                        :disabled="j == table.columns.length - 1" @click="handleIndexItem(j, 1)">
                                        <template #icon>
                                            <n-icon>
                                                <arrow-down />
                                            </n-icon>
                                        </template>
                                    </n-button>
                                    <n-button strong secondary type="info" size="small" :disabled="j == 0"
                                        @click="handleIndexItem(j, 2)">
                                        <template #icon>
                                            <n-icon>
                                                <arrow-up />
                                            </n-icon>
                                        </template>
                                    </n-button>
                                </td>
                            </tr>
                        </tbody>
                    </n-table>
                </n-tab-pane>
                <n-tab-pane display-directive="show" key="preview" name="preview" :tab="t('preview')">
                    <pre>
{{ sqlCreateTable.sql }}
{{ sqlIndex.sqls.join('\n') }}
{{ sqlIndex.comments.join('\n') }}
{{ sqlCreateTable.comments.join('\n') }}
</pre>
                </n-tab-pane>
            </n-tabs>
        </n-spin>
    </div>
</template>
    
<style scoped>
.create-table {
    position: absolute;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
}

.n-tabs {
    position: absolute;
    top: 40px;
    left: 0;
    right: 0;
    bottom: 0;
}
</style>
    