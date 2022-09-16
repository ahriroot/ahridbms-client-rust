<script setup lang="ts">
import { ref } from 'vue'
import { NCheckbox, NButton } from 'naive-ui'


interface Field {
    key: string
    value: boolean | null
    type: string
}

const props = withDefaults(defineProps<{
    field: Field
    isEdit: boolean
    newData?: boolean
    handleUpdateValue: (val: Field) => void
}>(), {
    newData: false
})

const inputRef = ref<HTMLInputElement | null>(null)
const originValue = ref(props.field.value)
const nullValue = ref<boolean | null>(props.field.value)
const inputValue = ref(props.field.value || false)
const handleChange = () => {
    nullValue.value = inputValue.value
    props.handleUpdateValue({
        ...props.field,
        value: inputValue.value
    })
}
const handleSetNull = () => {
    nullValue.value = null
    inputValue.value = false
    props.handleUpdateValue({
        ...props.field,
        value: null
    })
}
</script>
        
<template>
    <div :class="!props.newData && (inputValue !== originValue || nullValue !== originValue) ? 'diff' : 'same'"
        style="display: flex;justify-content: space-between">
        <n-checkbox size="small" ref="inputRef" v-model:checked="inputValue" @update:checked="handleChange"
            style="transform: translateY(2px);">
            {{ nullValue == null ? 'null' : inputValue }}
        </n-checkbox>
        <n-button secondary size="small" @click.stop="handleSetNull">Null</n-button>
    </div>
</template>
        
<style scoped>
.same {
    padding: 2px 4px;
    border-radius: 2px;
    background-color: none;
}

.diff {
    padding: 2px 4px;
    border-radius: 2px;
    background-color: #2e2e35;
}
</style>
        