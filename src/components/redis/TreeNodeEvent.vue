<script setup lang="ts">
import { NButton, NIcon, NTag } from 'naive-ui'
import { Reload, Trash } from '@vicons/ionicons5'

const props = defineProps<{
    show: boolean
    count: string
    onReload: (e: MouseEvent) => void
    onFlush: (e: MouseEvent) => void
}>()
</script>
    
<template>
    <div class="tree-node-event" v-show="props.show">
        <NTag size="small" v-if="count != '-1'">{{ props.count }}</NTag>
        <NButton size="small" style="margin: 0 2px" tertiary round @click="props.onReload($event)">
            <NIcon size="12">
                <Reload />
            </NIcon>
        </NButton>
        <NButton size="small" tertiary round @click="props.onFlush($event)">
            <NIcon size="12">
                <Trash />
            </NIcon>
        </NButton>
    </div>
</template>
    
<style scoped>
.tree-node-event {
    width: 146px;
    display: flex;
    justify-content: flex-end;
    align-items: center;
}

.n-tag {
    padding: 0 4px;
    height: 14px;
}

.n-button {
    padding: 0;
    width: 20px;
    height: 20px;
}
</style>
    