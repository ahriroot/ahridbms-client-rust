<script setup lang="ts">
import { ref } from 'vue'
import {NTable,NButton,NInput,NIcon} from 'naive-ui'
import { Pencil, Checkmark } from '@vicons/ionicons5'

const props = defineProps<{
    data: any
}>()
const emits = defineEmits<{
    (e: 'handle', val: null): void
}>()

const detailFilter = ref('')
const editListItem = ref({
    index: -1,
    value: ''
})
const handleEditListItem = async () => {
    editListItem.value.index = -1
}
</script>

<template>
    {{data}}
    <n-table :bordered="true" :single-line="false" size="small">
        <tbody>
            <tr
                v-for="(v, index) in props.data.filter((_: string, index: number) => !detailFilter || index == parseInt(detailFilter))">
                <td class="list-index" style="width: 120px">{{ index }}</td>
                <td class="list-value">
                    <div v-show="editListItem.index != index"
                        @click="editListItem.index = index; editListItem.value = v">{{ v }}
                    </div>
                    <n-input v-show="editListItem.index == index" v-model:value="editListItem.value" size="small">
                    </n-input>
                </td>
                <td class="list-opera">
                    <n-button v-show="editListItem.index != index" strong secondary circle type="info" size="small"
                        @click="editListItem.index = index; editListItem.value = v">
                        <template #icon>
                            <n-icon>
                                <pencil />
                            </n-icon>
                        </template>
                    </n-button>
                    <n-button v-show="editListItem.index == index" strong secondary circle type="info" size="small"
                        @click="handleEditListItem">
                        <template #icon>
                            <n-icon>
                                <checkmark />
                            </n-icon>
                        </template>
                    </n-button>
                </td>
            </tr>
        </tbody>
    </n-table>
</template>

<style scoped>
</style>
    