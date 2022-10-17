<script setup lang="ts">
import { ref, watch } from 'vue'
import { NInputNumber } from 'naive-ui'

const props = withDefaults(defineProps<{
    value: number | null
    bg?: string
    onUpdateValue: (val: number | null) => void
}>(), {
    bg: 'none'
})
const emits = defineEmits<{
    (e: 'update:value', val: number | null): void
}>()

const value = ref(props.value === null ? null : props.value)
const handleChange = async (val: number | null) => {
    value.value = val
    emits('update:value', val)
    props.onUpdateValue(val)
}

watch(() => props.value, (val) => {
    value.value = val === null ? null : val
})
</script>
    
<template>
    <n-input-number size="small" :value="value" @update:value="handleChange" placeholder="NULL" clearable />
</template>
    
<style scoped>
.n-input-number :deep(.n-input) {
    background: v-bind(bg);
}
</style>
    