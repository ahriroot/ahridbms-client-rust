<script setup lang="ts">
import { h, ref, onBeforeMount } from 'vue'
import { NTree, NIcon, NButton, TreeOption, DropdownOption, NDropdown, useMessage } from 'naive-ui'
import { ServerSharp, ChevronForward, Reload, LayersSharp, ListSharp } from '@vicons/ionicons5'
import { nanoid } from 'nanoid'

import { OpenTabMesagae } from '@/types/Message'
import { Connection } from '@/types/Connection'
import { PgDatabase, PostgresConnect } from '@/types/postgres'
import { getDatabases, getTables, getColumns } from '@/api/postgres'
import { PgColumn, PgTable } from '@/types/postgres/Data'


window.$message = useMessage()

const props = defineProps<{
    conn: Connection<PostgresConnect>
}>()
const emits = defineEmits<{
    (e: 'handleOpenTab', val: OpenTabMesagae<any>): void
    (e: 'handleDeleteConnection', id: string): void
}>()

const databases = ref<PgDatabase[]>([])

onBeforeMount(async () => {
    data.value = [{
        key: `postgres:${props.conn.info.name}`,
        label: props.conn.info.name,
        value: `postgres:${props.conn.info.name}`,
        isLeaf: false,
        children: undefined,
        suffix: () =>
            h(
                NButton,
                {
                    size: 'small',
                    quaternary: true,
                    style: "height: 24px",
                },
                {
                    default: () => h(
                        NIcon,
                        {
                            size: 'small',
                            onClick: async () => {
                                if (data.value.length > 0) {
                                    data.value[0].children = undefined
                                    data.value[0].isLeaf = false
                                    expandedKeys.value = []
                                    expandedKeys.value = [`postgres:${props.conn.info.name}`]
                                }
                            }
                        },
                        {
                            default: () => h(Reload)
                        }
                    )
                }
            ),
        type: 'connection'
    }]
})

const renderSwitcherIcon = () => {
    return h(NIcon, null, { default: () => h(ChevronForward) })
}

const expandedKeys = ref<string[]>([])

const showContextmenu = ref(false)
const optionsContextmenu = ref<DropdownOption[]>([])
const xPos = ref(0)
const yPos = ref(0)
const nodeProps = ({ option }: { option: any }) => {
    return {
        onClick() {
            // if (option.children == undefined || option.children == null) {
            //     emits('handleOpenTab', { id: nanoid(), conn: props.conn, tab_type: option.label })
            // }
        },
        onDblclick() {
        },
        onContextmenu(e: MouseEvent): void {
            e.preventDefault()
            e.stopPropagation()
            switch (option.type) {
                case 'connection':
                    optionsContextmenu.value = [{
                        label: 'Delete',
                        key: 'delete',
                        props: {
                            onClick: () => {
                                emits('handleDeleteConnection', props.conn.id)
                                showContextmenu.value = false
                            }
                        }
                    }, {
                        label: 'Reload',
                        key: 'reload',
                        props: {
                            onClick: async () => {
                                expandedKeys.value = [option.key]
                                option.isLeaf = false
                                option.children = undefined
                                await handleLoad(option)
                                showContextmenu.value = false
                            }
                        }
                    }]
                    break
                case 'database':
                    optionsContextmenu.value = [{
                        label: 'Delete',
                        key: 'delete',
                        props: {
                            onClick: () => {
                                showContextmenu.value = false
                            }
                        }
                    }]
                    break
                case 'tables':
                    optionsContextmenu.value = [{
                        label: 'Reload',
                        key: 'reload',
                        props: {
                            onClick: async () => {
                                handleLoad(option)
                                expandedKeys.value.filter(key => key != option.key)
                                option.isLeaf = false
                                option.children = undefined
                                expandedKeys.value.push(option.key)
                                showContextmenu.value = false
                            }
                        }
                    }]
                    break
                case 'table':
                    optionsContextmenu.value = [{
                        label: 'Open',
                        key: 'open',
                        props: {
                            onClick: async () => {
                                emits('handleOpenTab', {
                                    id: nanoid(), conn: props.conn, tab_type: 'table', data: {
                                        title: `${option.table}@${option.database}`,
                                        database: option.database,
                                        table: option.table
                                    }
                                })
                                showContextmenu.value = false
                            }
                        }
                    }, {
                        label: 'Reload',
                        key: 'reload',
                        props: {
                            onClick: async () => {
                                handleLoad(option)
                                expandedKeys.value.filter(key => key != option.key)
                                option.isLeaf = false
                                option.children = undefined
                                expandedKeys.value.push(option.key)
                                showContextmenu.value = false
                            }
                        }
                    }]
                    break
            }
            showContextmenu.value = true
            xPos.value = e.clientX
            yPos.value = e.clientY
        }
    }
}

const rangeDB = (dbs: PgDatabase[]): TreeOption[] => {
    let tmp: TreeOption[] = []
    dbs.forEach((db: PgDatabase) => {
        tmp.push({
            key: db.datname,
            label: db.datname,
            value: db.datname,
            children: [{
                key: `${db.datname}:tables:${nanoid()}`,
                label: 'Tables',
                value: db.datname,
                isLeaf: false,
                children: undefined,
                type: 'tables',
                database: db.datname
            }],
            prefix: () => h(NIcon, null, { default: () => h(ServerSharp) }),
            type: 'database'
        })
    })
    return tmp
}

const rangeTB = (dbs: PgTable[], database: string): TreeOption[] => {
    let tmp: TreeOption[] = []
    dbs.forEach((tb: PgTable) => {
        tmp.push({
            key: `${database}:${tb.tablename}`,
            label: tb.tablename,
            value: tb.tablename,
            isLeaf: false,
            children: undefined,
            prefix: () => h(NIcon, null, { default: () => h(LayersSharp) }),
            database: database,
            type: 'table',
            table: tb.tablename
        })
    })
    return tmp
}

const rangeCOL = (dbs: PgColumn[], database: string, table: string): TreeOption[] => {
    let tmp: TreeOption[] = []
    dbs.forEach((cln: PgColumn) => {
        tmp.push({
            key: `${database}:${table}:${cln.field}`,
            label: cln.field,
            value: cln.field,
            isLeaf: true,
            children: undefined,
            prefix: () => h(NIcon, null, { default: () => h(ListSharp) }),
            type: 'column',
            database: database,
            table: table
        })
    })
    return tmp
}

const data = ref<TreeOption[]>([])

const handleLoad = async (node: TreeOption) => {
    if (node.type == 'connection') {
        const res = await getDatabases({ conn: props.conn })
        if (!res.is_error) {
            databases.value = []
            res.forEach((db: any) => {
                let database: any = {}
                db.forEach((field: any) => {
                    for (const k in field) {
                        database[field[k].key] = field[k].value
                    }
                })
                databases.value.push(database as PgDatabase)
            })
            node.children = rangeDB(databases.value)
        }
    } else if (node.type == 'tables') {
        const res = await getTables({ conn: props.conn, database: node.value as string })
        if (!res.is_error) {
            let tbs: any = []
            res.forEach((db: any) => {
                let tables: any = {}
                db.forEach((field: any) => {
                    for (const k in field) {
                        tables[field[k].key] = field[k].value
                    }
                })
                tbs.push(tables)
            })
            node.children = rangeTB(tbs, node.database as string)
        }
    } else if (node.type == 'table') {
        const res = await getColumns({ conn: props.conn, database: node.database as string, table: node.table as string })
        if (!res.is_error) {
            let clns: any = []
            res.forEach((db: any) => {
                let columns: any = {}
                db.forEach((field: any) => {
                    for (const k in field) {
                        columns[field[k].key] = field[k].value
                    }
                })
                clns.push(columns)
            })
            node.children = rangeCOL(clns, node.database as string, node.table as string)
        }
    } else {
        node.children = []
    }
}

const handleExpand = (key: string[]) => {
    expandedKeys.value = key
}
</script>

<template>
    <div>
        <n-dropdown trigger="manual" size="small" placement="bottom-start" :show="showContextmenu"
            :options="(optionsContextmenu as any)" :x="xPos" :y="yPos" @clickoutside="showContextmenu = false" />
        <n-tree block-line @update:expanded-keys="handleExpand" :on-load="handleLoad" :data="data" selectable
            :node-props="nodeProps" :render-switcher-icon="renderSwitcherIcon" :expanded-keys="expandedKeys" />
    </div>
</template>

<style scoped>

</style>
