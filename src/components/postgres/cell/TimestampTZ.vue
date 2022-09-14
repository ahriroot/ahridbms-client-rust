<script setup lang="ts">
import { ref, nextTick, computed } from 'vue'
import { NDatePicker } from 'naive-ui'

const props = defineProps<{
    value: number
    isEdit: boolean
    onUpdateValue: (val: number) => void
}>()

const isEdit = ref(props.isEdit)
const inputRef = ref<any>(null)
const inputValue = ref(props.value)
const handleOnClick = () => {
    isEdit.value = true
    nextTick(() => {
        inputRef.value?.focus()
    })
}
const handleChange = (val: any) => {
    console.log(val)
    inputValue.value = val / 1000
    props.onUpdateValue(inputValue.value)
    isEdit.value = false
}
const format = computed(() => {
    const date = new Date(inputValue.value * 1000)
    const year = date.getUTCFullYear()
    const month = date.getUTCMonth() + 1
    const day = date.getUTCDate()
    const hour = date.getUTCHours()
    const minute = date.getUTCMinutes()
    const second = date.getUTCSeconds()
    return `${year}-${month}-${day} ${hour}:${minute}:${second}+00`
})
</script>

<template>
    <div @click="handleOnClick">
        <template v-if="isEdit">
            <n-date-picker size="small" ref="inputRef" style="width: 100%" :default-value="inputValue * 1000"
                @confirm="handleChange" type="datetime" />
        </template>
        <template v-else>
            {{ format }}
        </template>
    </div>
</template>
    
<style scoped>

</style>
