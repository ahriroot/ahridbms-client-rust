<script setup lang="ts">
import { executeWithTransaction, getPrimaryKeys, select, update } from '@/api/postgres'
import { Connection } from '@/types/Connection'
import { PostgresConnect } from '@/types/postgres'
import { ref, shallowRef, onBeforeMount, getCurrentInstance } from 'vue'
import {
    useLoadingBar, NTable, NButton, NIcon, NPopover, NPagination, useMessage
} from 'naive-ui'
import { Trash, Add } from '@vicons/ionicons5'
import useClipboard from "vue-clipboard3"

import BigIntVue from '@/components/postgres/BigInt.vue'
import IntVue from '@/components/postgres/cell/Int.vue'
import SmallIntVue from '@/components/postgres/cell/SmallInt.vue'
import VarCharVue from '@/components/postgres/cell/VarChar.vue'
import BoolVue from '@/components/postgres/cell/Bool.vue'
import TimestampVue from '@/components/postgres/cell/Timestamp.vue'
import TimestampTZVue from '@/components/postgres/cell/TimestampTZ.vue'


const props = defineProps<{
    conn: Connection<PostgresConnect>
    data: any
}>()
const emits = defineEmits<{
    (e: 'handle', val: null): void
}>()

const { toClipboard } = useClipboard()
const loadingBar = useLoadingBar()
const loadingCount = ref(0)
const message = useMessage()
const page = ref(1)
const size = ref(10)

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

const data = ref<any[]>([])
const handleLoadData = async () => {
    loadingStart()
    select({
        conn: props.conn,
        skip: 0,
        limit: 0,
        page: page.value,
        size: size.value,
        database: props.data.database,
        table: props.data.table
    }).then(res => {
        loadingFinish()
        data.value = res
    }).finally(() => {
        loadingFinish()
    })
}

const value = ref('123')
onBeforeMount(async () => {
    await handleLoadData()
})
</script>
    
<template>
    <BigIntVue v-model:value="value" />{{value}}
    <n-table :single-line="false" size="small">
        <thead>
            <tr>
                <th v-for="column in data[0]">
                    {{ column.field }}
                </th>
            </tr>
        </thead>
        <tbody>
            <tr v-for="row in data">
                <td v-for="column in row">
                    {{ column.value }}
                </td>
            </tr>
        </tbody>
    </n-table>
</template>
    
<style scoped>

</style>
    