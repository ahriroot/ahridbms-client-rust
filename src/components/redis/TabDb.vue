<script setup lang="ts">
import { ref, onBeforeMount } from 'vue'
import { invoke } from '@tauri-apps/api/tauri'
import { Connection } from '@/types/Connection';

const props = defineProps<{
    conn: Connection
    db: string
}>()
const emits = defineEmits<{
    (e: 'handle', val: null): void
}>()

interface Keyvalue {
    key: string
    key_type: string
    value: string
}
interface Data {
    [key: string]: Keyvalue
}

const result = ref<Keyvalue[]>([])

onBeforeMount(async () => {
    let res = await invoke<Data[]>('keys', { conn: props })
    console.log(res)
    res.forEach(item => {
        for (const key in item) {
            result.value.push(item[key])
        }
    })
})
</script>

<template>
    <div class="keys">
        <div v-for="i in result">
            <span class="key">{{ i.key }}</span>
        </div>
    </div>
</template>

<style scoped>
.keys {
    display: flex;
    flex-direction: column;
}

.key {
    width: 200px;
    display: inline-block;
    overflow: hidden;
}
</style>
