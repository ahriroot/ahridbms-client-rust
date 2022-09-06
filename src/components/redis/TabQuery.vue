<script setup lang="ts">
import { ref, shallowRef, onMounted } from 'vue'
import { Connection } from '@/types/Connection'
import { RedisConnect } from '@/types/redis'
import EditorVue from '@/components/Editor.vue'
import { NButton } from 'naive-ui'

const props = defineProps<{
    conn: Connection<RedisConnect>
    db: string
}>()
const emits = defineEmits<{
    (e: 'handle', val: null): void
}>()

const defaultQuery = ref('SET key value\nGET key')
const editorRef = shallowRef<any>(null)
const handle = async () => {
    if (editorRef.value) {
        await editorRef.value?.getSelectedValue()
        navigator.clipboard
            .readText()
            .then((v) => {
                console.log("获取选中值成功：", v);
            })
    }
}
</script>
    
<template>
    <n-button @click="handle">获取值</n-button>
    <div>
        <EditorVue ref="editorRef" :value="defaultQuery" :type="'redis_query'" />
    </div>
</template>
    
<style scoped>
</style>
    