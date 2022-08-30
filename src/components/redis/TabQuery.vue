<script setup lang="ts">
import { ref } from 'vue'
import { Connection } from '@/types/Connection'
import { RedisConnect } from '@/types/redis'
import EditorVue from '@/components/Editor.vue'

const props = defineProps<{
    conn: Connection<RedisConnect>
    db: string
}>()
const emits = defineEmits<{
    (e: 'handle', val: null): void
}>()
</script>
    
<template>
    <EditorVue :value="''" :type="'redis_query'" />
</template>
    
<style scoped>
</style>
    