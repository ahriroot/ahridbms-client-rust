import { createI18n } from 'vue-i18n'

const messages = {
    en: {
        message: {
            hello: 'hello world'
        }
    },
    zh: {
        message: {
            hello: '你好世界'
        }
    }
}
const i18n = createI18n({
    // legacy: false,
    // globalInjection: true,

    locale: 'zh',
    messages
})

export default i18n
