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
        updateConfig(config: Config): Promise<void>
    }
>({
    id: 'index',
    state: () => {
        return {
            config: {
                deleteNoConfirm: false,
                showSideBar: true,
                sideBarWidth: 250
            }
        }
    },
    actions: {
        async updateConfig(config: Config) {
            if (config.sideBarWidth) {
                if (config.sideBarWidth < 150) {
                    config.sideBarWidth = 150
                } else if (config.sideBarWidth > 1000) {
                    config.sideBarWidth = 1000
                }
            }
            localStorage.setItem('config', JSON.stringify(config))
            this.config = config
        }
    }
})
