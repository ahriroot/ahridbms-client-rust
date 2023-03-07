import { Config } from '@/types/store'
import { createPinia, Pinia, defineStore } from 'pinia'

const store: Pinia = createPinia()

export default store

export const useIndexStore = defineStore<
    'index',
    {
        config: Config
    },
    {},
    {
        updateConfig(config: Config, save?: boolean): Promise<void>
    }
>({
    id: 'index',
    state: () => {
        return {
            config: {
                deleteNoConfirm: false,
                showSideBar: true,
                treeWidth: 250,
                pageSize: 20,
                lang: 'zh-CN'
            }
        }
    },
    actions: {
        async updateConfig(config: Config, save: boolean = true) {
            if (config.treeWidth) {
                if (config.treeWidth < 250) {
                    config.treeWidth = 250
                } else if (config.treeWidth > 1300) {
                    config.treeWidth = 1300
                }
            }
            if (!config.pageSize) {
                config.pageSize = 20
            }
            if (!config.lang) {
                config.lang = 'zh-CN'
            }
            if (save) {
                localStorage.setItem('config', JSON.stringify(config))
            }
            this.config = config
        }
    }
})
