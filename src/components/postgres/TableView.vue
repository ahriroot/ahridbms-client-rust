<script setup lang="ts">
import { ref, computed, h } from 'vue'
import { NDataTable, NPopover } from 'naive-ui'

const props = defineProps<{
    data: any
}>()
const emits = defineEmits<{
    (e: 'handle', val: null): void
}>()


const columns = ref<any[]>([])
const data = computed(() => {
    if (props.data) {
        if (props.data.length > 0) {
            props.data[0].forEach((column: any) => {
                for (let i in column) {
                    columns.value.push({
                        title: column[i].key,
                        key: column[i].key,
                        ellipsis: true,
                    })
                }
            })
        }
        let tmp: any[] = []
        props.data.forEach((row: any) => {
            let r: any = {}
            row.forEach((column: any) => {
                for (let i in column) {
                    r[column[i].key] = column[i].value
                }
            })
            tmp.push(r)
        })
        return tmp
    }
    return []
})
</script>
    
<template>
    <n-data-table size="small" :single-line="false" :columns="columns" :data="data" flex-height
        style="position: absolute; top: 0; bottom: 0;" :scroll-x="900" />
</template>
    
<style scoped>
.n-data-table :deep(td) {
    margin: 0;
    padding: 2px 6px;
    background: none;
}
</style>
    