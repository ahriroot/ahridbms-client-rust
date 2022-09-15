<script setup lang="ts">
import { ref } from 'vue'
import { NInputGroup, NCheckbox } from 'naive-ui'


interface Field {
    key: string
    value: boolean
    type: string
}

const props = defineProps<{
    field: Field
    isEdit?: boolean
    handleUpdateValue: (val: Field) => void
}>()

const inputRef = ref<HTMLInputElement | null>(null)
const inputValue = ref(props.field.value)
const handleChange = () => {
    props.handleUpdateValue({
        ...props.field,
        value: inputValue.value
    })
}
</script>
        
<template>
    <div>
        <n-input-group>
            <n-checkbox size="small" ref="inputRef" style="width: 100%" v-model:checked="inputValue"
                @update:checked="handleChange">{{inputValue}}
            </n-checkbox>
        </n-input-group>
    </div>
</template>
        
<style scoped>

</style>
        