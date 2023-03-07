<script setup lang="ts">
import { h, ref, onBeforeMount } from 'vue'
import { NTree, NIcon, TreeOption, NDropdown, NModal, NCard, NLayout, NSpin } from 'naive-ui'
import { ServerSharp, ChevronForward } from '@vicons/ionicons5'

import { OpenTabMesagae } from '@/types/Message'
import { Connection } from '@/types/Connection'
import { RedisConnect } from '@/types/redis'
import { info } from '@/api/redis'
import iconRedis from '@/components/icon/redis.vue'
import { uuid } from '@/utils/crypto'
import { useI18n } from 'vue-i18n'
import treeNodeEvent from '@/components/redis/TreeNodeEvent.vue'

const props = defineProps<{
    conn: Connection<RedisConnect>
}>()
const emits = defineEmits<{
    (e: 'handleOpenTab', val: OpenTabMesagae<any>): void
    (e: 'handleEditConnection', id: string): void
    (e: 'handleDeleteConnection', id: string): void
}>()

const renderSwitcherIcon = () => {
    return h(NIcon, null, { default: () => h(ChevronForward) })
}

const { t } = useI18n()

const expandedKeys = ref<string[]>([])
const handleExpandKeys = (keys: string[]) => {
    expandedKeys.value = keys
}

const showContextmenu = ref(false)
const optionsContextmenu = ref<any[]>([])
const xPos = ref(0)
const yPos = ref(0)
const nodeProps = ({ option }: { option: any }): any => {
    return {
        async onClick() {
        },
        async onDblclick() {
            if (option.is_db) {
                emits('handleOpenTab', {
                    id: `${option.label}@${props.conn.id}`, conn: props.conn, tab_type: 'db', data: {
                        title: `${option.label}@${props.conn.info.name}`,
                        table: option.label
                    }
                })
            }
        },
        onContextmenu(e: MouseEvent): void {
            if (option.children != undefined && option.children != null) {
                optionsContextmenu.value = [{
                    label: t('query'),
                    key: 'query',
                    props: {
                        onClick: async () => {
                            let id = `query@${props.conn.info.name}`
                            emits('handleOpenTab', {
                                id: await uuid(), conn: props.conn, tab_type: 'query', data: {
                                    title: `query@${props.conn.info.name}`
                                }
                            })
                            showContextmenu.value = false
                        }
                    }
                }, {
                    label: t('info'),
                    key: 'info',
                    props: {
                        onClick: () => {
                            handleShowInfo()
                            showContextmenu.value = false
                        }
                    }
                }, {
                    label: t('edit'),
                    key: 'edit',
                    props: {
                        onClick: () => {
                            emits('handleEditConnection', props.conn.id)
                            showContextmenu.value = false
                        }
                    }
                }, {
                    label: t('delete'),
                    key: 'delete',
                    props: {
                        onClick: () => {
                            emits('handleDeleteConnection', props.conn.id)
                            showContextmenu.value = false
                        }
                    }
                }]
                showContextmenu.value = true
                xPos.value = e.clientX
                yPos.value = e.clientY
                e.preventDefault()
            } else if (option.is_db) {
                optionsContextmenu.value = [{
                    label: t('open'),
                    key: 'open',
                    props: {
                        onClick: async () => {
                            emits('handleOpenTab', {
                                id: `${option.label}@${props.conn.id}`, conn: props.conn, tab_type: 'db', data: {
                                    title: `${option.label}@${props.conn.info.name}`,
                                    table: option.label
                                }
                            })
                            showContextmenu.value = false
                        }
                    }
                }]
                showContextmenu.value = true
                xPos.value = e.clientX
                yPos.value = e.clientY
                e.preventDefault()
            }
        }
    }
}

onBeforeMount(async () => {
    await handleGetInfo()
})

const dbKeyCount = ref<{ [k: string]: string }>({
    'db0': '0',
    'db1': '0',
    'db2': '0',
    'db3': '0',
    'db4': '0',
    'db5': '0',
    'db6': '0',
    'db7': '0',
    'db8': '0',
    'db9': '0',
    'db10': '0',
    'db11': '0',
    'db12': '0',
    'db13': '0',
    'db14': '0',
    'db15': '0',
})

const handleGetInfo = async (index: number = -1) => {
    let result = await info({ conn: props.conn, db: '0' })
    if (result) {
        let results = result.split('\n')
        let handle_key_space = false
        for (let c = 0; c < results.length; c++) {
            let line = results[c]
            if (handle_key_space && line && line.startsWith('db')) {
                let [db, count] = line.split(':')
                if (index == -1) {
                    dbKeyCount.value[db] = count.split(',')[0].split('=')[1]
                } else {
                    if (db == `db${index}`) {
                        dbKeyCount.value[db] = count.split(',')[0].split('=')[1]
                        break
                    }
                }
            } else if (line && line.startsWith('#') && line.includes('Keyspace')) {
                handle_key_space = true
                continue
            }
        }
    }
}

const rangeDB = (): TreeOption[] => {
    let tmp: TreeOption[] = []
    for (let index = 0; index < 16; index++) {
        tmp.push({
            key: `db${index}`,
            label: index.toString(),
            value: index,
            is_db: true,
            prefix: () => h(NIcon, null, { default: () => h(ServerSharp) }),
            suffix: () => h(
                treeNodeEvent,
                {
                    show: true,
                    count: dbKeyCount.value[`db${index}`],
                    onReload: async (e: MouseEvent) => {
                        e.stopPropagation()
                        await handleGetInfo(index)
                    },
                    onFlush: async (e: MouseEvent) => {
                        e.stopPropagation()
                    }
                },
                {
                }
            )
        })
    }
    return tmp
}

const data = ref<TreeOption[]>([{
    key: `redis:${props.conn.info.name}`,
    label: props.conn.info.name,
    value: `redis:${props.conn.info.name}`,
    is_db: false,
    prefix: () => h(NIcon, null, { default: () => h(iconRedis) }),
    suffix: () => h(
        treeNodeEvent,
        {
            show: expandedKeys.value.includes(`redis:${props.conn.info.name}`),
            count: '-1',
            onReload: async (e: MouseEvent) => {
                e.stopPropagation()
                await handleGetInfo()
            },
            onFlush: async (e: MouseEvent) => {
                e.stopPropagation()
            }
        },
        {
        }
    ),
    children: rangeDB()
}])

const loading = ref(false)
const showInfo = ref(false)
const infoData = ref<any>({})
const handleShowInfo = async () => {
    showInfo.value = true
    if (props.conn) {
        loading.value = true
        let res = await info({ conn: props.conn, db: '0' })
        if (res) {
            let current_module = ''
            res.split('\n').forEach((line: string) => {
                if (line) {
                    if (line.startsWith('#')) {
                        current_module = line.split(' ')[1]
                        infoData.value[current_module] = {}
                    } else {
                        let [key, value] = line.split(':')
                        infoData.value[current_module][key] = value
                    }
                }
            })
        }
        loading.value = false
    }
}
</script>

<template>
    <div>
        <n-modal v-model:show="showInfo">
            <n-card style="width: 750px; height: 400px; position: relative; background: #282c34;" title="Info"
                :bordered="false" aria-modal="true">
                <n-spin :show="loading" style="position: relative; height: 100%; background: #282c34;">
                    <n-layout position="absolute" style="background: #282c34; color: #fff;" :native-scrollbar="false"
                        content-style="background: #282c34;">
                        {{ infoData }}
                    </n-layout>
                </n-spin>
            </n-card>
        </n-modal>
        <n-dropdown trigger="manual" size="small" placement="bottom-start" :show="showContextmenu"
            :options="(optionsContextmenu as any)" :x="xPos" :y="yPos" @clickoutside="showContextmenu = false" />
        <n-tree block-line :data="data" :node-props="nodeProps" expand-on-click :render-switcher-icon="renderSwitcherIcon"
            :expanded-keys="expandedKeys" :on-update:expanded-keys="(handleExpandKeys as any)" />
    </div>
</template>

<style scoped></style>
