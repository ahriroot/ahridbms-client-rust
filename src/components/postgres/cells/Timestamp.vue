<script setup lang="ts">
import { ref } from 'vue'
import { NDatePicker } from 'naive-ui'

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

const value = ref(props.value)
const handleChange = async (val: number | null) => {
    if (val === null) {
        emits('update:value', null)
        props.onUpdateValue(null)
    } else {
        value.value = val / 1000
        emits('update:value', val / 1000)
        props.onUpdateValue(val / 1000)
    }
}
const handleClear = async (_: undefined) => {
    emits('update:value', null)
    props.onUpdateValue(null)
}
</script>
    
<template>
    <n-date-picker size="small" ref="inputRef" :default-value="props.value === null ? null : props.value * 1000"
        @confirm="handleChange" @clear="handleClear" @update:value="handleChange" type="datetime" clearable
        placeholder="NULL" />
</template>
    
<style scoped>
.n-date-picker :deep(.n-input) {
    background: v-bind(bg);
    width: 200px;
}
</style>
    