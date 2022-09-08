<script setup lang="ts">
import EditorVue from '../Editor.vue';

const props = defineProps<{
    k: string
    value: string
}>()
</script>

<template>
    <div style="height: 60vh">
        <EditorVue ref="editorRef" :value="props.value" type="json" />
    </div>
</template>

<style scoped>

</style>
    