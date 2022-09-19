<script setup lang="ts">
import { ref, watch } from 'vue'
import { NCheckbox } from 'naive-ui'

const props = defineProps<{
    value: boolean | null
    bg?: string
    onUpdateValue: (val: boolean | null) => void
}>()
const emits = defineEmits<{
    (e: 'update:value', val: boolean | null): void
}>()

const value = ref<boolean | null>(props.value === null ? false : props.value)
const nullValue = ref<boolean | null>(props.value)
const handleChange = async (val: boolean | null) => {
    if (nullValue.value === null) {
        value.value = false
        nullValue.value = false
        emits('update:value', false)
        props.onUpdateValue(false)
    } else if (nullValue.value === false) {
        value.value = true
        nullValue.value = true
        emits('update:value', true)
        props.onUpdateValue(true)
    } else if (nullValue.value === true) {
        value.value = false
        nullValue.value = null
        emits('update:value', null)
        props.onUpdateValue(null)
    }
}

watch(() => props.value, (val) => {
    value.value = val === null ? false : val
    nullValue.value = val
})
</script>
    
<template>
    <div class="ckbox">
        <n-checkbox size="small" ref="inputRef" :checked="value || false" @update:checked="handleChange"
            style="transform: translateY(2px);">
            {{ nullValue === null ? 'NULL' : nullValue }}
        </n-checkbox>
    </div>
</template>
    
<style scoped>
.ckbox {
    padding: 0 5px;
    height: 100%;
}

.n-checkbox {
    width: 70px;
}
</style>
    