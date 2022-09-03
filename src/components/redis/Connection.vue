<script setup lang="ts">
import { h, ref, onBeforeMount } from 'vue'
import { invoke } from '@tauri-apps/api/tauri'
import { NTree, NIcon, TreeOption, DropdownOption, NDropdown } from 'naive-ui'
import { ServerSharp, ChevronForward } from '@vicons/ionicons5'
import { nanoid } from 'nanoid'

import { OpenTabMesagae } from '@/types/Message'
import { Connection } from '@/types/Connection'
import { RedisConnect, Response } from '@/types/redis'

const props = defineProps<{
    conn: Connection<RedisConnect>
}>()
const emits = defineEmits<{
    (e: 'handleOpenTab', val: OpenTabMesagae): void
    (e: 'handleDeleteConnection', id: string): void
}>()

const currentScope = ref<string>('')
const info = ref<any>({})

onBeforeMount(async () => {
    const res = await invoke<Response<string>>('plugin:redis|key_space', { conn: { ...props, db: "0" } })
    res.data.split('\n').forEach(item => {
        item = item.trim()
        if (item.length > 0) {
            if (item.startsWith('#')) {
                currentScope.value = item
                info.value[currentScope.value] = []
            } else {
                if (info.value[currentScope.value]) {
                    info.value[currentScope.value].push(item)
                }
            }
        }
    })
})

const renderSwitcherIcon = () => {
    return h(NIcon, null, { default: () => h(ChevronForward) })
}

const defaultExpandedKeys = ref([])

const showContextmenu = ref(false)
const optionsContextmenu = ref<DropdownOption[]>([])
const xPos = ref(0)
const yPos = ref(0)
const nodeProps = ({ option }: { option: any }) => {
    return {
        onClick() {
            if (option.children == undefined || option.children == null) {
                emits('handleOpenTab', { id: nanoid(), conn: props.conn, tab_type: option.label })
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
                        onClick: () => {
                            emits('handleOpenTab', { id: nanoid(), conn: props.conn, tab_type: 'query' })
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
    children: rangeDB()
}])
</script>

<template>
    <div>
        <n-dropdown trigger="manual" size="small" placement="bottom-start" :show="showContextmenu"
            :options="(optionsContextmenu as any)" :x="xPos" :y="yPos" @clickoutside="showContextmenu = false" />
        <n-tree block-line :data="data" selectable :node-props="nodeProps" expand-on-click
            :render-switcher-icon="renderSwitcherIcon" :default-expanded-keys="defaultExpandedKeys" />
    </div>
</template>

<style scoped>
</style>
