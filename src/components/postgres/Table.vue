<script setup lang="ts">
import { select } from '@/api/postgres';
import { Connection } from '@/types/Connection';
import { PostgresConnect } from '@/types/postgres';
import { h, ref, shallowRef, onBeforeMount } from 'vue'
import {
    useLoadingBar, NDataTable, NButton, NCheckbox
} from 'naive-ui'
import IntVue from '@/components/postgres/cell/Int.vue'
import VarCharVue from '@/components/postgres/cell/VarChar.vue'
import BoolVue from '@/components/postgres/cell/Bool.vue'
import TimestampTZVue from '@/components/postgres/cell/TimestampTZ.vue'


const props = defineProps<{
    conn: Connection<PostgresConnect>
    data: any
}>()
const emits = defineEmits<{
    (e: 'handle', val: null): void
}>()

const loadingBar = useLoadingBar()
const skip = ref(0)
const limit = ref(10)
const loadingCount = ref(0)

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

const typeRender = shallowRef<{ [key: string]: any; }>({
    Int: {
        width: 160,
        component: IntVue
    },
    VarChar: {
        width: 200,
        component: VarCharVue
    },
    Bool: {
        width: 200,
        component: BoolVue
    },
    TimestampTZ: {
        width: 200,
        component: TimestampTZVue
    }
})
const data = ref([])
const columns = ref<any>([])
const pagination = ref(false as const)
const edits = ref<any>({})
onBeforeMount(async () => {
    loadingStart()
    let res = await select({
        conn: props.conn,
        skip: skip.value,
        limit: limit.value,
        page: 0,
        size: 0,
        database: props.data.database,
        table: props.data.table
    })
    if (res.length > 0) {
        let first = res[0]
        console.log(first)
        columns.value = []
        first.forEach((field: any) => {
            for (const k in field) {
                let fieldType = typeRender.value[k]
                let tmp: any = {
                    title: field[k].key,
                    key: field[k].key,
                }
                if (fieldType) {
                    tmp.width = fieldType.width
                    tmp.render = (row: any, index: number) => {
                        return h(fieldType.component, {
                            value: row[field[k].key],
                            isEdit: false,
                            onUpdateValue(v: never) {
                                let c = []
                                for (let s in row) {
                                    // 
                                }
                                console.log(v)
                                data.value[index][field[k].key] = v
                                // edits.value[row.id] = true
                            }
                        })
                    }
                } else {
                    tmp.render = (row: any) => {
                        return h('span', row[field[k].key])
                    }
                }
                columns.value.push(tmp)
            }
        })
        columns.value.push({
            title: 'Action',
            key: 'actions',
            fixed: 'right',
            render(row: any) {
                return h(
                    NButton,
                    {
                        strong: true,
                        tertiary: true,
                        size: 'small',
                        onClick: () => {

                            console.log(row)
                        }
                    },
                    { default: () => 'Edit' }
                )
            }
        })

        let tbs: any = []
        res.forEach((data: any) => {
            let table: any = {}
            data.forEach((field: any) => {
                for (const k in field) {
                    table[field[k].key] = field[k].value
                }
            })
            tbs.push(table)
        })
        data.value = tbs
        loadingFinish()
    }
})
</script>
    
<template>
    <n-data-table :columns="columns" :data="data" :pagination="pagination" :single-line="false" size="small" />
    <div>{{edits}}</div>
</template>
    
<style scoped>

</style>
    