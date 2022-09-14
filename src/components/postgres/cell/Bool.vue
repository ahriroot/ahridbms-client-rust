<script setup lang="ts">
import { ref } from 'vue'
import { NInputGroup, NCheckbox } from 'naive-ui'

const props = defineProps<{
    value: boolean
    isEdit?: boolean
    onUpdateValue: (val: boolean) => void
}>()

const inputRef = ref<HTMLInputElement | null>(null)
const inputValue = ref(props.value)
const handleChange = () => {
    props.onUpdateValue(inputValue.value)
}
</script>
        
<template>
    <div>
        <n-input-group>
            <n-checkbox size="small" ref="inputRef" style="width: 100%" v-model:checked="inputValue"
                @update:value="inputValue = $event" @blur="handleChange">{{inputValue}}
            </n-checkbox>
        </n-input-group>
    </div>
</template>
        
<style scoped>

</style>
        