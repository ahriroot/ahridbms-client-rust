<script setup lang="ts">
import { ref, shallowRef, onBeforeMount, onMounted } from 'vue'
import { NTabs, NTabPane } from 'naive-ui'
import { emit, listen } from '@tauri-apps/api/event'

import { TabComponents } from '@/data/data'

import { getTab, saveTab, getTabs, saveTabs } from '@/utils/storage'

import { OpenTabMesagae } from '@/types/Message'
import { ITabComponents } from '@/types/data'


const tab = ref<string>('')
const tabs = ref<OpenTabMesagae<any>[]>([])
const tabComponents = shallowRef<ITabComponents>(TabComponents)

onBeforeMount(async () => {
    let ts = await getTabs()
    if (ts) {
        tabs.value = ts
    let t = await getTab()
    if (t) {
        tab.value = t
    }
    }
})

onMounted(async () => {
    const unlisten = await listen<string>('tab-widget', (e) => {
        let payload = JSON.parse(e.payload)
        let event = payload.event
        switch (event) {
            case 'open':
                if (tabs.value.find((item: any) => item.id == payload.data.id)) {
                    tab.value = payload.data.id
                    saveTab(tab.value)
                    return
                }
                tabs.value.push(payload.data)
                tab.value = payload.data.id
                saveTab(tab.value)
                saveTabs(tabs.value)
                break
        }
    })
})

// 关闭标签页
const handleClose = (val: string) => {
    tabs.value = tabs.value.filter((item: any) => item.id !== val)
    if (val == tab.value) {
        tab.value = tabs.value.length > 0 ? tabs.value[0].id : ''
    }
    saveTabs(tabs.value)
}


/**
 * 关闭标签页
 * @param id 连接id
 */
const handleCloseTab = async (id: string) => {
    tabs.value = tabs.value.filter(tab => tab.id != id)
    saveTabs(tabs.value)
    if (tabs.value.length > 0) {
        tab.value = tabs.value[0].id
    } else {
        tab.value = ''
    }
}
</script>

<template>
    <div id="tab-widget">
        <n-tabs v-model:value="tab" @update:value="saveTab" type="card" closable tab-style="min-width: 80px;"
            @close="handleClose" size="small">
            <n-tab-pane display-directive="show" v-for="i in tabs" :key="i.id" :tab="i.data.title" :name="i.id">
                <component :key="i.id" :is="tabComponents[`${i.conn.db_type}:${i.tab_type}`]" :conn="i.conn" :data="i.data"
                    @handleCloseTab="handleCloseTab(i.id)" />
            </n-tab-pane>
        </n-tabs>
    </div>
</template>

<style scoped>
#tab-widget {
    position: absolute;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
}
</style>
