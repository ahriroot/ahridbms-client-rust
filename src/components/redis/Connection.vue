<script setup lang="ts">
import { h, ref } from 'vue'
import { NTree, NIcon, TreeOption, NDropdown, NModal, NCard, NLayout, NSpin } from 'naive-ui'
import { ServerSharp, ChevronForward } from '@vicons/ionicons5'

import { OpenTabMesagae } from '@/types/Message'
import { Connection } from '@/types/Connection'
import { RedisConnect } from '@/types/redis'
import { info } from '@/api/redis'
import iconRedis from '@/components/icon/redis.vue'
import { uuid } from '@/utils/crypto'

const props = defineProps<{
    conn: Connection<RedisConnect>
}>()
const emits = defineEmits<{
    (e: 'handleOpenTab', val: OpenTabMesagae<any>): void
    (e: 'handleDeleteConnection', id: string): void
}>()

const renderSwitcherIcon = () => {
    return h(NIcon, null, { default: () => h(ChevronForward) })
}

const defaultExpandedKeys = ref([])

const showContextmenu = ref(false)
const optionsContextmenu = ref<any[]>([])
const xPos = ref(0)
const yPos = ref(0)
const nodeProps = ({ option }: { option: any }): any => {
    return {
        async onClick() {
            if (option.children == undefined || option.children == null) {
                emits('handleOpenTab', {
                    id: await uuid(), conn: props.conn, tab_type: 'db', data: {
                        title: `${option.label}@${props.conn.info.name}`,
                        table: option.label
                    }
                })
            }
        },
        onContextmenu(e: MouseEvent): void {
            if (option.children != undefined && option.children != null) {
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
                    label: 'Query',
                    key: 'query',
                    props: {
                        onClick: async () => {
                            emits('handleOpenTab', {
                                id: await uuid(), conn: props.conn, tab_type: 'query', data: {
                                    title: `query@${props.conn.info.name}`
                                }
                            })
                            showContextmenu.value = false
                        }
                    }
                }, {
                    label: 'Info',
                    key: 'info',
                    props: {
                        onClick: () => {
                            handleShowInfo()
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

const rangeDB = (): TreeOption[] => {
    let tmp: TreeOption[] = []
    for (let index = 0; index < 16; index++) {
        tmp.push({
            key: `db${index}`,
            label: index.toString(),
            value: index,
            prefix: () => h(NIcon, null, { default: () => h(ServerSharp) })
        })
    }
    return tmp
}

const data = ref<TreeOption[]>([{
    key: `redis:${props.conn.info.name}`,
    label: props.conn.info.name,
    value: `redis:${props.conn.info.name}`,
    prefix: () => h(NIcon, null, { default: () => h(iconRedis) }),
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
        <n-tree block-line :data="data" selectable :node-props="nodeProps" expand-on-click
            :render-switcher-icon="renderSwitcherIcon" :default-expanded-keys="defaultExpandedKeys" />
    </div>
</template>

<style scoped>

</style>
