<script setup lang="ts">
import { ref, nextTick } from 'vue'
import { NInputGroup, NInput, NButton } from 'naive-ui'

const props = defineProps<{
    value: string
    isEdit: boolean
    onUpdateValue: (val: string) => void
}>()

const isEdit = ref(props.isEdit)
const inputRef = ref<HTMLInputElement | null>(null)
const inputValue = ref(props.value)
const handleOnClick = () => {
    isEdit.value = true
    nextTick(() => {
        inputRef.value?.focus()
    })
}
const handleChange = () => {
    props.onUpdateValue(inputValue.value)
    isEdit.value = false
}
</script>
    
<template>
    <div @click="handleOnClick">
        <template v-if="isEdit">
            <n-input-group>
                <n-input size="small" ref="inputRef" style="width: 100%" v-model:value="inputValue"
                    @blur="handleChange" />
                <n-button @click="isEdit = false" size="small">取消</n-button>
            </n-input-group>
        </template>
        <template v-else>
            {{ props.value }}
        </template>
    </div>
</template>
    
<style scoped>

</style>
    