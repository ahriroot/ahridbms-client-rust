<script setup lang="ts">
import { h, ref, computed, onBeforeMount } from 'vue'
import {
    NTree, NIcon, NButton, TreeOption, NDropdown, NModal,
    NForm, NFormItem, NInput, NInputNumber, NCheckbox, NSelect, NSpin,
    useMessage, useDialog
} from 'naive-ui'
import { ServerSharp, ChevronForward } from '@vicons/ionicons5'
import { nanoid } from 'nanoid'

import { OpenTabMesagae } from '@/types/Message'
import { Connection } from '@/types/Connection'
import { MongodbConnect } from '@/types/mongodb'
import { databases, collections, documents } from '@/api/mongodb'
import { useIndexStore } from '@/store'
import { useI18n } from 'vue-i18n'
import { listen } from '@tauri-apps/api/event'
import iconMongodb from '@/components/icon/mongodb.vue'
import iconCollection from '@/components/icon/collection.vue'


window.$message = useMessage()

const props = defineProps<{
    conn: Connection<MongodbConnect>
}>()
const emits = defineEmits<{
    (e: 'handleOpenTab', val: OpenTabMesagae<any>): void
    (e: 'handleDeleteConnection', id: string): void
}>()

const { t } = useI18n()
const dialog = useDialog()
const store = useIndexStore()

const initConnection = async () => {
    let k = props.conn.id
    data.value = [{
        key: k,
        label: props.conn.info.name,
        value: `postgres:${props.conn.info.name}`,
        prefix: () => h(NIcon, null, { default: () => h(iconMongodb) }),
        isLeaf: false,
        children: undefined,
        type: 'connection'
    }]
    return k
}

onBeforeMount(async () => {
    await listen<string>('reload', async (event) => {
        let payload = JSON.parse(event.payload)
        if (payload.conn.id === props.conn.id) {
            await reloadTables(`${props.conn.id}:${payload.database}:tables`, data.value)
        }
    })
    await initConnection()
})

const renderSwitcherIcon = () => {
    return h(NIcon, null, { default: () => h(ChevronForward) })
}
const showCreateDatabase = ref(false)
const expandedKeys = ref<string[]>([])

const showContextmenu = ref(false)
const optionsContextmenu = ref<any[]>([])
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
                        label: t('delete'),
                        key: 'delete',
                        props: {
                            onClick: () => {
                                if (store.config?.deleteNoConfirm) {
                                    emits('handleDeleteConnection', props.conn.id)
                                } else {
                                    dialog.warning({
                                        title: t('delete'),
                                        content: `${t('deleteConnection')} ${props.conn.info.name} ?`,
                                        positiveText: t('delete'),
                                        onPositiveClick: async () => {
                                            emits('handleDeleteConnection', props.conn.id)
                                        }
                                    })
                                }
                                showContextmenu.value = false
                            }
                        }
                    }, {
                        label: t('reload'),
                        key: 'reload',
                        props: {
                            onClick: async () => {
                                if (!expandedKeys.value.includes(option.key)) {
                                    expandedKeys.value.push(option.key)
                                }
                                option.children = undefined
                                showContextmenu.value = false
                            }
                        }
                    }, {
                        label: t('create'),
                        key: 'create',
                        props: {
                            onClick: async () => {
                                if (defaultInfo.value === null) {
                                    // let res1 = await executeSelectSql({
                                    //     conn: props.conn,
                                    //     database: props.conn.info.db,
                                    //     sql: 'SELECT * FROM pg_tablespace;'
                                    // })
                                    // res1.forEach((item: any) => {
                                    //     tablespace.value.push({
                                    //         label: item.spcname,
                                    //         value: item.spcname
                                    //     })
                                    // })
                                    // let res2 = await executeSelectSql({
                                    //     conn: props.conn,
                                    //     database: props.conn.info.db,
                                    //     sql: 'select * from pg_roles;'
                                    // })
                                    // res2.forEach((item: any) => {
                                    //     roles.value.push({
                                    //         label: item.rolname,
                                    //         value: item.rolname
                                    //     })
                                    // })
                                }
                                showCreateDatabase.value = true
                                showContextmenu.value = false
                            }
                        }
                    }]
                    break
                case 'database':
                    optionsContextmenu.value = [{
                        label: t('delete'),
                        key: 'delete',
                        props: {
                            onClick: async () => {
                                // if (store.config?.deleteNoConfirm) {
                                //     await update({
                                //         conn: props.conn,
                                //         database: props.conn.info.db,
                                //         sql: `DROP DATABASE ${option.label}`
                                //     })
                                //     expandedKeys.value = []
                                //     expandedKeys.value = [await initConnection()]
                                // } else {
                                //     dialog.warning({
                                //         title: t('delete'),
                                //         content: `${t('deleteDatabase')} ${option.label} ?`,
                                //         positiveText: t('delete'),
                                //         onPositiveClick: async () => {
                                //             await update({
                                //                 conn: props.conn,
                                //                 database: props.conn.info.db,
                                //                 sql: `DROP DATABASE ${option.label}`
                                //             })
                                //             expandedKeys.value = []
                                //             expandedKeys.value = [await initConnection()]
                                //         }
                                //     })
                                // }
                                showContextmenu.value = false
                            }
                        }
                    }, {
                        label: t('query'),
                        key: 'query',
                        props: {
                            onClick: () => {
                                emits('handleOpenTab', {
                                    id: nanoid(), conn: props.conn, tab_type: 'query', data: {
                                        title: `query@${option.database}`,
                                        database: option.database,
                                        table: option.table
                                    }
                                })
                                showContextmenu.value = false
                            }
                        }
                    }]
                    break
                case 'tables':
                    optionsContextmenu.value = [{
                        label: t('reload'),
                        key: 'reload',
                        props: {
                            onClick: async () => {
                                if (!expandedKeys.value.includes(option.key)) {
                                    expandedKeys.value.push(option.key)
                                }
                                option.children = undefined
                                showContextmenu.value = false
                            }
                        }
                    }, {
                        label: t('create'),
                        key: 'create',
                        props: {
                            onClick: async () => {
                                emits('handleOpenTab', {
                                    id: nanoid(), conn: props.conn, tab_type: 'create_table', data: {
                                        title: `#create_table@${option.database}`,
                                        database: option.database,
                                        table: ''
                                    }
                                })
                                showContextmenu.value = false
                            }
                        }
                    }]
                    break
                case 'collection':
                    optionsContextmenu.value = [{
                        label: t('open'),
                        key: 'open',
                        props: {
                            onClick: async () => {
                                emits('handleOpenTab', {
                                    id: nanoid(), conn: props.conn, tab_type: 'collection', data: {
                                        title: `${option.database}.${option.collection}@${props.conn.info.name}`,
                                        database: option.database,
                                        collection: option.collection
                                    }
                                })
                                showContextmenu.value = false
                            }
                        }
                    }, {
                        label: t('reload'),
                        key: 'reload',
                        props: {
                            onClick: async () => {
                                if (!expandedKeys.value.includes(option.key)) {
                                    expandedKeys.value.push(option.key)
                                }
                                option.children = undefined
                                showContextmenu.value = false
                            }
                        }
                    }, {
                        label: t('delete'),
                        key: 'delete',
                        props: {
                            onClick: async () => {
                                if (store.config?.deleteNoConfirm) {
                                    // let res = await update({
                                    //     conn: props.conn,
                                    //     database: option.database,
                                    //     sql: `DROP TABLE "public"."${option.table}"`
                                    // })
                                    // if (!res.is_error) {
                                    //     await reloadParent(option.key, data.value)
                                    // }
                                } else {
                                    dialog.warning({
                                        title: t('delete'),
                                        content: `${t('deleteTable')} ${option.label} ?`,
                                        positiveText: t('delete'),
                                        onPositiveClick: async () => {
                                            // let res = await update({
                                            //     conn: props.conn,
                                            //     database: option.database,
                                            //     sql: `DROP TABLE "public"."${option.table}"`
                                            // })
                                            // if (!res.is_error) {
                                            //     await reloadParent(option.key, data.value)
                                            // }
                                        }
                                    })
                                }
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

const reloadTables = async (key: any, tree: any[]) => {
    for (let index = 0; index < tree.length; index++) {
        if (tree[index].key === key) {
            tree[index].children = undefined
            if (!expandedKeys.value.includes(key)) {
                console.log(key)
                expandedKeys.value.push(key)
            }
            return
        } else {
            if (tree[index].children) {
                await reloadTables(key, tree[index].children)
            }
        }
    }
}

const reloadParent = async (key: any, tree: any[]) => {
    for (let index = 0; index < tree.length; index++) {
        if (tree[index].children && tree[index].children.find((item: any) => item.key === key)) {
            tree[index].children = undefined
            if (!expandedKeys.value.includes(tree[index].key)) {
                expandedKeys.value.push(tree[index].key)
            }
            return
        } else if (tree[index].children) {
            await reloadParent(key, tree[index].children)
        }
    }
}

const rangeDB = (dbs: string[], node: any): TreeOption[] => {
    let tmp: TreeOption[] = []
    dbs.forEach((db: string) => {
        tmp.push({
            key: `${node.key}:${db}`,
            label: db,
            value: db,
            children: [{
                key: `${props.conn.id}:${db}:collections`,
                label: t('collection'),
                value: db,
                isLeaf: false,
                children: undefined,
                type: 'collections',
                database: db
            }],
            prefix: () => h(NIcon, null, { default: () => h(ServerSharp) }),
            type: 'database',
            database: db
        })
    })
    return tmp
}

const rangeTB = (cols: string[], node: any): TreeOption[] => {
    let tmp: TreeOption[] = []
    cols.forEach((col: string) => {
        tmp.push({
            key: `${node.key}:${col}`,
            label: col,
            value: col,
            isLeaf: true,
            children: undefined,
            prefix: () => h(NIcon, null, { default: () => h(iconCollection) }),
            database: node.database,
            type: 'collection',
            collection: col
        })
    })
    return tmp
}

const data = ref<TreeOption[]>([])

const handleLoad = async (node: TreeOption): Promise<void> => {
    if (node.type == 'connection') {
        const res = await databases({ conn: props.conn })
        if (!res.is_error) {
            node.children = rangeDB(res, node)
        } else {
            node.children = []
        }
    } else if (node.type == 'collections') {
        const res = await collections({ conn: props.conn, database: node.value as string })
        if (!res.is_error) {
            node.children = rangeTB(res, node)
        } else {
            node.children = []
        }
    } else {
        node.children = []
    }
}

const handleExpand = (key: string[]) => {
    expandedKeys.value = key
    localStorage.setItem(`expandedKeys:${props.conn.id}`, JSON.stringify(key))
}

const loadingCreateDatabase = ref(false)
const defaultInfo = ref<any | null>(null)
const roles = ref<{
    label: string
    value: string
}[]>([])
const encoding = ref<{
    label: string
    value: string
}[]>([{
    label: 'UTF8',
    value: 'UTF8'
}])
const tablespace = ref<{
    label: string
    value: string
}[]>([])
const newDatabase = ref<any>({
    name: '',
    owner: null,
    template: null,
    encoding: null,
    lcCollate: '',
    lcCtype: '',
    tableSpace: null,
    connectionLimit: -1,
    allowConnections: true,
    isTemplate: false,
    comment: ''
})
const handleSubmitCreateDatabase = async () => {
    // loadingCreateDatabase.value = true
    // let res1 = await update({
    //     conn: props.conn,
    //     database: props.conn.info.db,
    //     sql: sqlCreateDatabase.value
    // })
    // loadingCreateDatabase.value = false
    // if (!res1.is_error) {
    //     expandedKeys.value = []
    //     expandedKeys.value = [await initConnection()]
    //     showCreateDatabase.value = false
    //     if (newDatabase.value.comment.trim() != '') {
    //         let res2 = await update({
    //             conn: props.conn,
    //             database: props.conn.info.db,
    //             sql: `COMMENT ON DATABASE "${newDatabase.value.name}" IS '${newDatabase.value.comment}'`
    //         })
    //         if (!res2.is_error) {
    //             expandedKeys.value = []
    //             expandedKeys.value = [await initConnection()]
    //         }
    //     }
    // }
}
const showPreviewSql = ref(false)
const sqlCreateDatabase = computed(() => {
    let sql = ''
    if (newDatabase.value.name) {
        sql += `CREATE DATABASE "${newDatabase.value.name}"`
    }
    let withes = ''
    if (newDatabase.value.owner) {
        withes += `  OWNER "${newDatabase.value.owner}"\n`
    }
    if (newDatabase.value.template) {
        withes += `  TEMPLATE "${newDatabase.value.template}"\n`
    }
    if (newDatabase.value.encoding) {
        withes += `  ENCODING '${newDatabase.value.encoding}'\n`
    }
    if (newDatabase.value.lcCollate) {
        withes += `  LC_COLLATE '${newDatabase.value.lcCollate}'\n`
    }
    if (newDatabase.value.lcCtype) {
        withes += `  LC_CTYPE '${newDatabase.value.lcCtype}'\n`
    }
    if (newDatabase.value.tableSpace) {
        withes += `  TABLESPACE "${newDatabase.value.tableSpace}"\n`
    }
    if (!newDatabase.value.allowConnections) {
        withes += `  ALLOW_CONNECTIONS FALSE\n`
    }
    if (newDatabase.value.connectionLimit >= 0) {
        withes += `  CONNECTION LIMIT ${newDatabase.value.connectionLimit}\n`
    }
    if (newDatabase.value.isTemplate) {
        withes += `  IS_TEMPLATE TRUE\n`
    }
    if (withes) {
        sql += `\nWITH\n${withes}`
    }
    if (sql) {
        sql += ';'
    }
    return sql
})
</script>

<template>
    <div>
        <n-modal v-model:show="showPreviewSql" preset="card" style="width: 600px;" :title="t('info')" size="small">
            <pre>{{sqlCreateDatabase}}</pre>
        </n-modal>
        <n-modal v-model:show="showCreateDatabase" preset="card" style="width: 600px;" :title="t('info')" size="small">
            <n-spin size="large" :show="loadingCreateDatabase">
                <n-form :model="newDatabase" label-placement="left" label-width="auto"
                    require-mark-placement="right-hanging" size="small" :style="{
                      maxWidth: '640px'
                    }">
                    <n-form-item label="Name" path="name">
                        <n-input v-model:value="newDatabase.name" placeholder="Database Name" />
                    </n-form-item>
                    <n-form-item label="Owner" path="owner">
                        <n-select v-model:value="newDatabase.owner" placeholder="Owner" :options="roles" />
                    </n-form-item>
                    <!-- <n-form-item label="Template" path="template">
                    <n-select v-model:value="newDatabase.template" placeholder="Template" :options="template" />
                </n-form-item> -->
                    <n-form-item label="Encoding" path="encoding">
                        <n-select v-model:value="newDatabase.encoding" placeholder="Encoding" :options="encoding"
                            clearable />
                    </n-form-item>
                    <n-form-item label="LC COLLATE" path="lcCollate">
                        <n-input v-model:value="newDatabase.lcCollate" placeholder="LC COLLATE" />
                    </n-form-item>
                    <n-form-item label="LC CTYPE" path="lcCtype">
                        <n-input v-model:value="newDatabase.lcCtype" placeholder="LC CTYPE" />
                    </n-form-item>
                    <n-form-item label="Table Space" path="tableSpace">
                        <n-select v-model:value="newDatabase.tableSpace" placeholder="Table Space" :options="tablespace"
                            clearable />
                    </n-form-item>
                    <n-form-item label="Connection Limit" path="connectionLimit">
                        <n-input-number v-model:value="newDatabase.connectionLimit" placeholder="Connection Limit" />
                    </n-form-item>
                    <n-form-item label="Allow Connections" path="allConnections">
                        <n-checkbox v-model:checked="newDatabase.allowConnections">Allow Connections</n-checkbox>
                    </n-form-item>
                    <n-form-item label="Is Template" path="isTemplate">
                        <n-checkbox v-model:checked="newDatabase.isTemplate">Is Template</n-checkbox>
                    </n-form-item>
                    <n-form-item label="Comment" path="comment">
                        <n-input v-model:value="newDatabase.comment" placeholder="Comment" />
                    </n-form-item>
                    <div style="display: flex; justify-content: flex-end">
                        <n-button size="small" @click="showPreviewSql = true">{{ t('preview') }}</n-button>
                        &nbsp;
                        <n-button size="small" @click="handleSubmitCreateDatabase">{{ t('create') }}</n-button>
                        &nbsp;
                        <n-button size="small" @click="showCreateDatabase = false">{{ t('cancel') }}</n-button>
                    </div>
                </n-form>
            </n-spin>
        </n-modal>
        <n-dropdown trigger="manual" size="small" placement="bottom-start" :show="showContextmenu"
            :options="(optionsContextmenu as any)" :x="xPos" :y="yPos" @clickoutside="showContextmenu = false" />
        <n-tree block-line @update:expanded-keys="handleExpand" :on-load="handleLoad" :data="data" selectable
            :node-props="(nodeProps as any)" :render-switcher-icon="renderSwitcherIcon" :expanded-keys="expandedKeys" />
    </div>
</template>

<style scoped>

</style>
