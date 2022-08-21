<script setup lang="ts">
import { ref } from 'vue'

const props = defineProps<{
    key: string
}>()
const emits = defineEmits<{
    (e: 'handle', val: null): void
}>()
</script>

<template>
</template>

<style scoped>
</style>
