<script setup lang="ts">
import { ref, nextTick, computed } from 'vue'
import { NInputGroup, NDatePicker, NButton } from 'naive-ui'


interface Field {
    key: string
    value: number | null
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
const inputRef = ref<any>(null)
const inputValue = ref(props.field.value)
const handleOnClick = () => {
    isEdit.value = true
    nextTick(() => {
        inputRef.value?.focus()
    })
}
const handleChange = (val: any) => {
    console.log(val)
    if (val != null) {
        inputValue.value = val / 1000
    } else {
        inputValue.value = null
    }
    props.handleUpdateValue({
        ...props.field,
        value: inputValue.value
    })
    isEdit.value = false
}
const format = computed(() => {
    if (inputValue.value) {
        const date = new Date(inputValue.value * 1000)
        const year = date.getUTCFullYear()
        const month = date.getUTCMonth() + 1
        const day = date.getUTCDate()
        const hour = date.getUTCHours()
        const minute = date.getUTCMinutes()
        const second = date.getUTCSeconds()
        return `${year}-${month}-${day} ${hour}:${minute}:${second}`
    }
    return ''
})
const handleSetNull = () => {
    inputValue.value = null
    handleChange(null)
    isEdit.value = false
}
</script>

<template>
    <div @click="handleOnClick" :class="inputValue === props.field.value? 'same' : 'diff'">
        <template v-if="isEdit || props.newData">
            <n-input-group>
                <n-date-picker size="small" ref="inputRef" style="width: 100%" :default-value="inputValue"
                    @confirm="handleChange" type="datetime" placeholder="null" />
                <n-button secondary size="small" @click.stop="handleSetNull">Null</n-button>
            </n-input-group>
        </template>
        <template v-else>
            {{ inputValue == null ? 'null' : format }}
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
