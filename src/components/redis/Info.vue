<script setup lang="ts">
import { ref } from 'vue'
import { info } from '@/api/redis'
import { NModal, NButton, NCard, NLayout, NSpin, NSpace, NTable, NCollapse, NCollapseItem } from 'naive-ui'
import { Connection } from '@/types/Connection'
import { RedisConnect } from '@/types/redis'

const props = defineProps<{
    conn: Connection<RedisConnect>
}>()
const emits = defineEmits<{
    (e: 'handle', val: null): void
}>()

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
        console.log(infoData.value)
        loading.value = false
    }
}
</script>
    
<template>
    <n-modal v-model:show="showInfo">
        <n-card style="width: 750px; height: 400px; position: relative; background: #282c34;" title="Info"
            :bordered="false" aria-modal="true">
            <n-spin :show="loading" style="position: relative; height: 100%; background: #282c34;">
                <n-layout position="absolute" style="background: #282c34; color: #fff;" :native-scrollbar="false"
                    content-style="background: #282c34;">
                    <n-collapse display-directive="show">
                        <n-collapse-item v-for="(value, key) in infoData" :name="key">
                            <n-table :single-line="false" size="small">
                                <tbody>
                                    <tr v-for="(v, k) in value" v-show="k && v">
                                        <td>{{ k }}</td>
                                        <td>{{ v }}</td>
                                    </tr>
                                </tbody>
                            </n-table>
                            <template #header>
                                {{ key }}
                            </template>
                        </n-collapse-item>
                    </n-collapse>
                </n-layout>
            </n-spin>
        </n-card>
    </n-modal>
    <n-space>
        <n-button secondary @click="handleShowInfo" size="small">Info</n-button>
    </n-space>
</template>
    
<style scoped>
td {
    background: #282c34 !important;
}
</style>
    