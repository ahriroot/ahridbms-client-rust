import { createApp } from 'vue'
import './style.css'
import App from './App.vue'
import store from './store'
import i18n from './locales/i18n'


createApp(App).use(store).use(i18n).mount('#app')
