<script setup lang="ts">
import { h, ref, onBeforeMount } from 'vue'
import { NTree, NIcon, TreeOption, DropdownOption, NDropdown } from 'naive-ui'
import { ServerSharp, ChevronForward } from '@vicons/ionicons5'
import { nanoid } from 'nanoid'

import { OpenTabMesagae } from '@/types/Message'
import { Connection } from '@/types/Connection'
import { Response } from '@/types/redis'
import { PgDatabase, PostgresConnect } from '@/types/postgres'
import { pgGetDatabases } from '@/api/postgres'

const props = defineProps<{
    conn: Connection<PostgresConnect>
}>()
const emits = defineEmits<{
    (e: 'handleOpenTab', val: OpenTabMesagae): void
    (e: 'handleDeleteConnection', id: string): void
}>()

const databases = ref<PgDatabase[]>([])

onBeforeMount(async () => {
    const res = await pgGetDatabases({ conn: props.conn })
    if (!res.is_error) {
        res.forEach((db: any) => {
            let database: any = {};
            db.forEach((field: any) => {
                for (const k in field) {
                    database[field[k].key] = field[k].value
                }

            })
            databases.value.push(database as PgDatabase)
        })

        data.value = [{
            key: `redis:${props.conn.info.name}`,
            label: props.conn.info.name,
            value: `redis:${props.conn.info.name}`,
            children: rangeDB()
        }]
    }
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
    databases.value.forEach((db: PgDatabase) => {
        tmp.push({
            key: db.datname,
            label: db.datname,
            value: db.datname,
            prefix: () => h(NIcon, null, { default: () => h(ServerSharp) })
        })
    })
    return tmp
}

const data = ref<TreeOption[]>([{
    key: `redis:${props.conn.info.name}`,
    label: props.conn.info.name,
    value: `redis:${props.conn.info.name}`,
    children: []
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
