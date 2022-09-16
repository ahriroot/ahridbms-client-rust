<script setup lang="ts">
import { ref, nextTick } from 'vue'
import { NInputGroup, NInput, NButton } from 'naive-ui'


interface Field {
    key: string
    value: string | null
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

const isEdit = ref(props.isEdit)
const inputRef = ref<HTMLInputElement | null>(null)
const originValue = ref(props.field.value)
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
}
const handleSetNull = () => {
    inputValue.value = null
    handleChange()
    isEdit.value = false
}
</script>
    
<template>
    <div @click="handleOnClick" :class="inputValue === originValue ? 'same' : 'diff'">
        <template v-if="isEdit || props.newData">
            <n-input-group>
                <n-input size="small" ref="inputRef" style="width: 100%" v-model:value="inputValue"
                    @keyup.enter="handleChange(); isEdit = false" @update:value="handleChange" placeholder="null" />
                <n-button secondary size="small" @click.stop="handleSetNull">Null</n-button>
            </n-input-group>
        </template>
        <template v-else>
            {{ inputValue == null ? 'null' : inputValue }}
        </template>
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
    