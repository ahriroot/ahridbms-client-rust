<script setup lang="ts">
import { ref } from 'vue'
import { NInput } from 'naive-ui'

const props = defineProps<{
    value: string
}>()
const emits = defineEmits<{
    (e: 'update:value', val: string): void
}>()

const value = ref(props.value)
const handleChange = async (val: string) => {
    value.value = val
    emits('update:value', val)
}
</script>
    
<template>
    <n-input :value="value" @change="handleChange" />
</template>
    
<style scoped>

</style>
    