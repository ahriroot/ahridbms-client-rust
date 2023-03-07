<script setup lang="ts">
import { ref, onBeforeMount, onMounted } from 'vue'
import { useIndexStore } from '@/store'
import { invoke } from '@tauri-apps/api/tauri'
import {
    NConfigProvider, NGlobalStyle, NLoadingBarProvider, NMessageProvider, NDialogProvider,
    darkTheme, zhCN, enUS
} from 'naive-ui'
import { useI18n } from 'vue-i18n'
import { Config } from './types/store'

import Index from '@/views/Index.vue'


const store = useIndexStore()

onBeforeMount(async () => {
    let config_str = localStorage.getItem('config')
    let config: Config = {
        deleteNoConfirm: false,
        showSideBar: true,
        treeWidth: 300,
        pageSize: 10,
        lang: 'zh-CN'
    }
    if (config_str) {
        try {
            let old_config = JSON.parse(config_str)
            config.deleteNoConfirm = old_config.deleteNoConfirm || false
            config.showSideBar = old_config.showSideBar || true
            config.treeWidth = old_config.treeWidth || 300
            config.pageSize = old_config.pageSize || 10
            config.lang = old_config.lang || 'zh-CN'
        } catch { }
    }
    store.updateConfig(config)
})

onMounted(async () => {
    setTimeout(async () => {
        await invoke('close_splashscreen')
    }, 1000)
})

const languages = ref<{ [x: string]: any }>({
    'zh-CN': zhCN,
    'en-US': enUS
})
const { t, locale } = useI18n()
</script>

<template>
    <n-config-provider :theme="darkTheme" :locale="languages[locale]">
        <n-global-style />
        <n-loading-bar-provider>
            <n-message-provider>
                <n-dialog-provider>
                    <div id="window" class="dark">
                        <div id="title" data-tauri-drag-region></div>
                        <div id="body">
                            <Index />
                        </div>
                    </div>
                </n-dialog-provider>
            </n-message-provider>
        </n-loading-bar-provider>
    </n-config-provider>
</template>

<style scoped>
#window {
    position: fixed;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
}

#title {
    position: absolute;
    top: 0;
    left: 0;
    right: 0;
    height: 30px;
}

#body {
    position: absolute;
    top: 30px;
    left: 0;
    right: 0;
    bottom: 0;
}
</style>
