<script setup lang="ts">
import { ref } from 'vue'
import { NButton, NSelect, NCheckbox, NModal } from 'naive-ui'
import { useI18n } from 'vue-i18n'
import { useIndexStore } from '@/store'
import { checkUpdate, installUpdate } from '@tauri-apps/api/updater'
import { relaunch } from '@tauri-apps/api/process'
import tauriConfig from '../../src-tauri/tauri.conf.json'

const props = defineProps<{
    conn: any
    data: any
}>()
const emits = defineEmits<{
    (e: 'handle', val: null): void
}>()

const store = useIndexStore()
const { t, locale } = useI18n()
const langs = ref([
    { label: '简体中文', value: 'zh-CN' },
    { label: 'English', value: 'en-US' }
])
const handleUpdateLang = async (_: string) => {
    store.updateConfig({
        ...store.config,
        lang: locale.value
    })
}
const handleCheckedChange = async (val: boolean) => {
    store.updateConfig({
        ...store.config,
        deleteNoConfirm: val
    })
}
const loadingClear = ref(false)
const handleClearAllData = async () => {
    loadingClear.value = true
    localStorage.clear()
    setTimeout(() => {
        loadingClear.value = false
    }, 1000)
}

// ------------- Update Start -------------
const showUpdateInfo = ref(false)
const updateStatus = ref<any>({})
const updateLoading = ref(false)
const handleUpdate = async () => {
    try {
        const { shouldUpdate, manifest } = await checkUpdate()
        if (shouldUpdate) {
            updateStatus.value = manifest
            showUpdateInfo.value = true
        } else {
            alert('当前已是最新版本')
        }
    } catch (error) {
        alert(error)
    }
}
const handleStartUpdate = async () => {
    updateLoading.value = true
    await installUpdate()
    await relaunch()
    updateLoading.value = false
}
const handleCancelUpdate = () => {
    showUpdateInfo.value = false
}
// ------------- Update End -------------
</script>

<template>
    <div class="setting">
        <n-modal v-model:show="showUpdateInfo" preset="card" style="width: 600px;" :title="t('info')"
            size="small">
            <h1>Version: {{updateStatus.version}}</h1>
            <br>
            <p>Info: {{updateStatus.body}}</p>
            <br>
            <p>Publish Date: {{updateStatus.date}}</p>
            <br>
            <n-button size="small" @click="handleStartUpdate" :loading="updateLoading">Install</n-button>
            &nbsp;
            <n-button size="small" @click="handleCancelUpdate">Cancel</n-button>
        </n-modal>
        <div class="space-setting">
            <n-select size="small" v-model:value="locale" :options="langs"
                @update:value="handleUpdateLang" />
            <br>
            <n-checkbox :checked="store.config?.deleteNoConfirm" @update:checked="handleCheckedChange">
                {{ t('noConfirmationIsRequiredForDeletion') }}
            </n-checkbox>
            <br>
            <br>
            <n-button :loading="loadingClear" size="small" @click="handleClearAllData">
                {{ t('clearAllData') }}
            </n-button>
            <br>
            <br>
            <n-button :loading="updateLoading" size="small" @click="handleUpdate">
                {{ t('checkForUpdates') }}
            </n-button>
            <br>
            <br>
            <div>Version: {{tauriConfig.package.version}}</div>
        </div>
    </div>
</template>

<style scoped>
.setting {
    position: absolute;
    top: 0;
    left: 0;
    right: 0;
    bottom: 36px;
    display: flex;
    flex-direction: column;
}

.setting .space-setting {
    padding: 10px 0;
    min-width: 400px;
    max-width: 800px;
    margin: 0 auto;
}
</style>
