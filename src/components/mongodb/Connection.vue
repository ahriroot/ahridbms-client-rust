<script setup lang="ts">
import { h, ref, computed, onBeforeMount } from 'vue'
import {
    NTree, NIcon, NButton, TreeOption, NDropdown, NModal,
    NForm, NFormItem, NInput, NInputNumber, NCheckbox, NSelect, NSpin,
    useMessage, useDialog
} from 'naive-ui'
import { ServerSharp, ChevronForward } from '@vicons/ionicons5'

import { OpenTabMesagae } from '@/types/Message'
import { Connection } from '@/types/Connection'
import { MongodbConnect } from '@/types/mongodb'
import { databases, collections, dropDatabase, dropCollection } from '@/api/mongodb'
import { useIndexStore } from '@/store'
import { useI18n } from 'vue-i18n'
import { listen } from '@tauri-apps/api/event'
import iconMongodb from '@/components/icon/mongodb.vue'
import iconCollection from '@/components/icon/collection.vue'
import { uuid } from '@/utils/crypto'


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
        async onClick() {
            // if (option.children == undefined || option.children == null) {
            //     emits('handleOpenTab', { id: await uuid(), conn: props.conn, tab_type: option.label })
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
                                showCreateDatabase.value = true
                                showContextmenu.value = false
                            }
                        }
                    }, {
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
                    }]
                    break
                case 'database':
                    optionsContextmenu.value = [{
                        label: t('query'),
                        key: 'query',
                        props: {
                            onClick: async () => {
                                emits('handleOpenTab', {
                                    id: await uuid(), conn: props.conn, tab_type: 'query', data: {
                                        title: `query@${option.database}`,
                                        database: option.database,
                                        table: option.table
                                    }
                                })
                                showContextmenu.value = false
                            }
                        }
                    }, {
                        label: t('opera'),
                        key: 'opera',
                        props: {
                            onClick: async () => {
                                emits('handleOpenTab', {
                                    id: await uuid(), conn: props.conn, tab_type: 'opera', data: {
                                        title: `opera@${option.database}`,
                                        database: option.database,
                                        table: option.table
                                    }
                                })
                                showContextmenu.value = false
                            }
                        }
                    }, {
                        label: t('delete'),
                        key: 'delete',
                        props: {
                            onClick: async () => {
                                showContextmenu.value = false
                                if (store.config?.deleteNoConfirm) {
                                    if (['admin', 'config', 'local'].includes(option.database)) {
                                        dialog.error({
                                            title: t('delete'),
                                            content: `${t('deleteDatabase')} ${option.database} ?`,
                                            positiveText: t('delete'),
                                            onPositiveClick: async () => {
                                                await dropDatabase({
                                                    conn: props.conn,
                                                    database: option.database
                                                })
                                                await reloadParent(option.key, data.value)
                                            }
                                        })
                                    } else {
                                        await dropDatabase({
                                            conn: props.conn,
                                            database: option.database
                                        })
                                        await reloadParent(option.key, data.value)
                                    }
                                } else {
                                    dialog.warning({
                                        title: t('delete'),
                                        content: `${t('deleteDatabase')} ${option.database} ?`,
                                        positiveText: t('delete'),
                                        onPositiveClick: async () => {
                                            if (['admin', 'config', 'local'].includes(option.database)) {
                                                dialog.error({
                                                    title: t('delete'),
                                                    content: `${t('deleteDatabase')} ${option.database} ?`,
                                                    positiveText: t('delete'),
                                                    onPositiveClick: async () => {
                                                        await dropDatabase({
                                                            conn: props.conn,
                                                            database: option.database
                                                        })
                                                        await reloadParent(option.key, data.value)
                                                    }
                                                })
                                            } else {
                                                await dropDatabase({
                                                    conn: props.conn,
                                                    database: option.database
                                                })
                                                await reloadParent(option.key, data.value)
                                            }
                                        }
                                    })
                                }
                            }
                        }
                    }]
                    break
                case 'collections':
                    optionsContextmenu.value = [{
                        label: t('reload'),
                        key: 'reload',
                        props: {
                            onClick: async () => {
                                showContextmenu.value = false
                                if (!expandedKeys.value.includes(option.key)) {
                                    expandedKeys.value.push(option.key)
                                }
                                option.children = undefined
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
                                showContextmenu.value = false
                                emits('handleOpenTab', {
                                    id: await uuid(), conn: props.conn, tab_type: 'collection', data: {
                                        title: `${option.database}.${option.collection}@${props.conn.info.name}`,
                                        database: option.database,
                                        collection: option.collection
                                    }
                                })
                            }
                        }
                    }, {
                        label: t('reload'),
                        key: 'reload',
                        props: {
                            onClick: async () => {
                                showContextmenu.value = false
                                await reloadParent(option.key, data.value)
                            }
                        }
                    }, {
                        label: t('delete'),
                        key: 'delete',
                        props: {
                            onClick: async () => {
                                showContextmenu.value = false
                                if (store.config?.deleteNoConfirm) {
                                    if (['admin', 'config', 'local'].includes(option.database)) {
                                        dialog.error({
                                            title: t('delete'),
                                            content: `${t('deleteCollection')} ${option.collection} ?`,
                                            positiveText: t('delete'),
                                            onPositiveClick: async () => {
                                                await dropCollection({
                                                    conn: props.conn,
                                                    database: option.database,
                                                    collection: option.collection
                                                })
                                                await reloadParent(option.key, data.value)
                                            }
                                        })
                                    } else {
                                        await dropCollection({
                                            conn: props.conn,
                                            database: option.database,
                                            collection: option.collection
                                        })
                                        await reloadParent(option.key, data.value)
                                    }
                                } else {
                                    dialog.warning({
                                        title: t('delete'),
                                        content: `${t('deleteCollection')} ${option.collection} ?`,
                                        positiveText: t('delete'),
                                        onPositiveClick: async () => {
                                            if (['admin', 'config', 'local'].includes(option.database)) {
                                                dialog.error({
                                                    title: t('delete'),
                                                    content: `${t('deleteCollection')} ${option.collection} ?`,
                                                    positiveText: t('delete'),
                                                    onPositiveClick: async () => {
                                                        await dropCollection({
                                                            conn: props.conn,
                                                            database: option.database,
                                                            collection: option.collection
                                                        })
                                                        await reloadParent(option.key, data.value)
                                                    }
                                                })
                                            } else {
                                                await dropCollection({
                                                    conn: props.conn,
                                                    database: option.database,
                                                    collection: option.collection
                                                })
                                                await reloadParent(option.key, data.value)
                                            }
                                        }
                                    })
                                }
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
            isLeaf: false,
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
</script>

<template>
    <div class="nocopy">
        <n-dropdown trigger="manual" size="small" placement="bottom-start" :show="showContextmenu"
            :options="(optionsContextmenu as any)" :x="xPos" :y="yPos" @clickoutside="showContextmenu = false" />
        <n-tree block-line @update:expanded-keys="handleExpand as any" :on-load="handleLoad" :data="data" selectable
            :node-props="(nodeProps as any)" :render-switcher-icon="renderSwitcherIcon" :expanded-keys="expandedKeys" />
    </div>
</template>

<style scoped></style>
