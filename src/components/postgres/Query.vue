<script setup lang="ts">
import { Connection } from '@/types/Connection';
import { PostgresConnect } from '@/types/postgres';
import { ref } from 'vue'

const props = defineProps<{
    conn: Connection<PostgresConnect>
    data: any
}>()
const emits = defineEmits<{
    (e: 'handle', val: null): void
}>()
</script>
    
<template>
    Query
</template>
    
<style scoped>

</style>
    