import { Config } from '@/types/store'
import { createPinia, Pinia, defineStore } from 'pinia'

const store: Pinia = createPinia()

export default store

export const useIndexStore = defineStore<
    'index',
    {
        config: Config | null
    },
    {},
    {
        updateConfig(config: Config): Promise<void>
    }
>({
    id: 'index',
    state: () => {
        return {
            config: null
        }
    },
    actions: {
        async updateConfig(config: Config) {
            localStorage.setItem('config', JSON.stringify(config))
            this.config = config
        }
    }
})
