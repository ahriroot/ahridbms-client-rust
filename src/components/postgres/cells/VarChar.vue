<script setup lang="ts">
import { ref } from 'vue'
import { NInput } from 'naive-ui'

const props = withDefaults(defineProps<{
    value: string | null
    bg?: string
    onUpdateValue: (val: string | null) => void
}>(), {
    bg: 'none'
})
const emits = defineEmits<{
    (e: 'update:value', val: string | null): void
}>()

const value = ref<string | null>(props.value)
const handleChange = async (val: string | null) => {
    value.value = val
    emits('update:value', val)
    props.onUpdateValue(val)
}
const handleClear = async () => {
    value.value = null
    emits('update:value', null)
    props.onUpdateValue(null)
}
</script>
    
<template>
    <n-input :style="`background: ${props.bg}`" size="small" :value="value" @update:value="handleChange" @clear="handleClear"
        :placeholder="value === null ? 'NULL' : ''" clearable />
</template>
    
<style scoped>

</style>
    