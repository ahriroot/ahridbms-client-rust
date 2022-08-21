<script setup lang="ts">
import { h, ref, computed, onBeforeMount } from 'vue'
import { invoke } from '@tauri-apps/api/tauri'
import { NTree, NIcon, TreeOption } from 'naive-ui'
import { ServerSharp, ChevronForward } from '@vicons/ionicons5'
import { nanoid } from 'nanoid'

import { OpenTabMesagae } from '@/types/Message'
import { Connection } from '@/types/Connection'

const props = defineProps<{
    conn: Connection
}>()
const emits = defineEmits<{
    (e: 'handleOpenTab', val: OpenTabMesagae): void
}>()

const currentScope = ref<string>('')
const info = ref<any>({})

onBeforeMount(async () => {
    const res = await invoke<string>('key_space')
    res.split('\n').forEach(item => {
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

const nodeProps = ({ option }: { option: any }) => {
    return {
        onClick() {
            if (option.children == undefined || option.children == null) {
                emits('handleOpenTab', { id: nanoid(), conn: props.conn, tab_type: option.label })
            }
        },
        onContextmenu(e: MouseEvent): void {
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
        <n-tree block-line :data="data" selectable :node-props="nodeProps" expand-on-click
            :render-switcher-icon="renderSwitcherIcon" :default-expanded-keys="defaultExpandedKeys" />
    </div>
</template>

<style scoped>
</style>
