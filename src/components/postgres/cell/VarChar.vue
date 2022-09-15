<script setup lang="ts">
import { ref, nextTick } from 'vue'
import { NInputGroup, NInput, NButton } from 'naive-ui'


interface Field {
    key: string
    value: string
    type: string
}

const props = defineProps<{
    field: Field
    isEdit: boolean
    handleUpdateValue: (val: Field) => void
}>()

const isEdit = ref(props.isEdit)
const inputRef = ref<HTMLInputElement | null>(null)
const inputValue = ref(props.field.value)
const handleOnClick = () => {
    isEdit.value = true
    nextTick(() => {
        inputRef.value?.focus()
    })
}
const handleChange = () => {
    props.handleUpdateValue({
        ...props.field,
        value: inputValue.value
    })
    isEdit.value = false
}
</script>
    
<template>
    <div @click="handleOnClick">
        <template v-if="isEdit">
            <n-input-group>
                <n-input size="small" ref="inputRef" style="width: 100%" v-model:value="inputValue"
                    @keyup.enter="handleChange" />
                <n-button @click="isEdit = false" size="small">取消</n-button>
            </n-input-group>
        </template>
        <template v-else>
            {{ props.field.value }}
        </template>
    </div>
</template>
    
<style scoped>

</style>
    