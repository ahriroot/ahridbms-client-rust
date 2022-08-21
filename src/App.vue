<script setup lang="ts">
import { h, ref, shallowRef, onBeforeMount } from 'vue'
import {
    darkTheme, NConfigProvider, NGlobalStyle, NIcon, NLayout,
    NButton, NModal, NSelect, SelectRenderLabel, NInput, NCard, NSpace,
    NTabs, NTabPane
} from 'naive-ui'
import { invoke } from '@tauri-apps/api/tauri'
import { ArrowBack, ServerSharp } from '@vicons/ionicons5'
import { nanoid } from 'nanoid'

import { DBType, ConnectionComponents, RedisConnectInit } from '@/data/data'
import { IConnectComponents } from '@/types/data'
import { Connection, RedisConnect } from '@/types/Connection'
import { getConnections, saveConnections } from '@/utils/storage'
import { OpenTabMesagae } from '@/types/Message'
import TabDbVue from '@/components/redis/TabDb.vue'

/** ------------------ 变量 Start ------------------ **/
const showSide = ref<boolean>(true)  // 显示侧边栏
const showConn = ref<boolean>(false)  // 显示编辑连接窗口
const dbTypeList = ref(DBType)  // 数据库列表
const connList = ref<Connection[]>([])  // 数据库链接列表
const dbConn = ref<string>('redis')  // 正在编辑的连接类型
const dbRedis = ref<RedisConnect>(RedisConnectInit)  // 默认 redis 数据库连接信息
const connComponents = shallowRef<IConnectComponents>(ConnectionComponents)  // 数据库连接组件
/** ------------------ 变量 End ------------------ **/

onBeforeMount(async () => {
    connList.value = await getConnections()
    let tabs_str = localStorage.getItem('tabs')
    tabs.value = tabs_str ? JSON.parse(tabs_str) : []
    if (tabs.value.length > 0) {
        tab.value = tabs.value[0].id
    }
})

interface Keyvalue {
    key: string
    value: string
}
interface Data {
    [key: string]: Keyvalue
}

const result = ref<any>('')
const value = ref<string>('')

const handleKeys = async () => {
    const res = await invoke<Data[]>('keys')
    console.log(res)
    result.value = res
}

const handleGet = async () => {
    const res = await invoke<any>('get', { key: value.value })
    console.log(res)
    result.value = res
}

const renderLabel: SelectRenderLabel = (option) => {
    return h(
        'div',
        {
            style: {
                display: 'flex',
                alignItems: 'center'
            }
        },
        [
            h(NIcon, {
                size: 20
            }, {
                default: () => h(ServerSharp)
            }),
            h(
                'div',
                {
                    style: {
                        marginLeft: '12px',
                        padding: '4px 0'
                    }
                },
                h('div', null, [option.label as string])
            )
        ]
    )
}

const handleSubmitConn = async () => {
    switch (dbConn.value) {
        case 'redis':
            connList.value.push({
                id: nanoid(),
                db_type: 'redis',
                info: dbRedis.value
            })
            break
    }
    await saveConnections(connList.value)
    showConn.value = false
}

const handleCancelConn = () => {
    showConn.value = false
}

// ----------------------------------
const tab = ref<string>('')
const tabs = ref<OpenTabMesagae[]>([])
const handleOpenTab = (message: OpenTabMesagae) => {
    if (message) {
        if (message.conn.db_type === 'redis') {
            if (message.tab_type === 'query') {
                console.log('redis query')
            } else {
                tabs.value.push(message)
                localStorage.setItem('tabs', JSON.stringify(tabs.value))
                if (tabs.value.length === 1) {
                    tab.value = tabs.value[0].id
                }
            }
        }
    }

}
const handleClose = () => {

}

const tabComponents = shallowRef<any>({
    'redis:db': TabDbVue
})
</script>

<template>
    <n-config-provider :theme="darkTheme">
        <n-global-style />

        <n-modal v-model:show="showConn" preset="card" style="width: 600px;" title="连接" size="small">
            <n-space vertical>
                <n-select :options="dbTypeList" :render-label="renderLabel" v-model:value="dbConn" />
                <n-card v-if="dbConn == 'redis'">
                    <n-space vertical>
                        <n-input v-model:value="dbRedis.name" type="text" placeholder="Name" />
                        <n-space>
                            <n-input v-model:value="dbRedis.host" type="text" placeholder="Host" />
                            <n-input v-model:value="dbRedis.port" type="text" placeholder="Port" />
                        </n-space>
                        <n-space>
                            <n-input v-model:value="dbRedis.user" type="text" placeholder="User" />
                            <n-input v-model:value="dbRedis.pass" type="text" placeholder="Pass" />
                        </n-space>
                        <n-input v-model:value="dbRedis.index" type="text" placeholder="DB Index" />
                    </n-space>
                </n-card>
                <n-card v-else>
                </n-card>
                <n-space>
                    <n-button @click="handleSubmitConn" style="margin-top: 12px;">Submit</n-button>
                    <n-button @click="handleCancelConn" style="margin-top: 12px;">Cancel</n-button>
                </n-space>
            </n-space>
        </n-modal>

        <div id="main">
            <aside class="side" :class="showSide ? '' : 'show'"></aside>
            <main class="main">
                <div class="connection">
                    <n-button strong secondary size="small" @click="showConn = true">
                        New Conn
                    </n-button>
                    <div class="conn">
                        <n-layout position="absolute" style="background: #21252b; color: #fff" :native-scrollbar="false"
                            content-style="padding: 10px;">
                            <component v-for="i in connList" :is="connComponents[i.db_type]" :conn="i"
                                @handleOpenTab="handleOpenTab" />
                        </n-layout>
                    </div>
                    <div class="btn-side">
                        <n-icon size="20" @click="showSide = !showSide">
                            <arrow-back class="icon" :class="showSide ? 'show' : ''" />
                        </n-icon>
                    </div>
                </div>
                <div class="content">
                    <header class="header"></header>
                    <section class="workspace">
                        <n-layout position="absolute" style="background: #282c34; color: #fff" :native-scrollbar="false"
                            content-style="padding: 10px 0;">
                            <n-tabs v-model:value="tab" type="card" closable tab-style="min-width: 80px;"
                                @close="handleClose">
                                <n-tab-pane v-for="i in tabs" :key="i.id" :tab="i.conn.info.name" :name="i.id">
                                    <component
                                        :is="tabComponents[`${i.conn.db_type}:${i.tab_type == 'query' ? 'query' : 'db'}`]"
                                        :conn="i.conn" :db="i.tab_type" />
                                </n-tab-pane>
                            </n-tabs>
                        </n-layout>
                    </section>
                </div>
            </main>
        </div>
    </n-config-provider>
</template>

<style>
* {
    outline: none;
    padding: 0;
    margin: 0;
    box-sizing: border-box;
}
</style>

<style scoped>
#main {
    position: fixed;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    /* height: 100vh;
    width: 100vw; */
    display: flex;

}

#main .side {
    width: 50px;
    background: #333842;
    overflow: hidden;
    transition: .3s;
}

#main .side.show {
    width: 0;
}

#main .main {
    flex: 1;
    background: #282c34;
    display: flex;
}

#main .main .connection {
    width: 250px;
    background: #21252b;
    position: relative;
}

#main .main .connection .conn {
    overflow: hidden;
    position: absolute;
    top: 30px;
    left: 0;
    right: 0;
    bottom: 30px;
    border-top: #333842 1px solid;
    border-bottom: #333842 1px solid;
}

#main .main .connection .btn-side {
    position: absolute;
    left: 8px;
    bottom: 0;
    cursor: pointer;
    color: #333842;
}

#main .main .connection .btn-side:hover {
    color: #3278c4;
}

#main .main .connection .btn-side .icon {
    transition: .3s;
}

#main .main .connection .btn-side .icon.show {
    transform: rotate(-180deg);
}

#main .main .content {
    flex: 1;
    background: #282c34;
    position: relative;
}

#main .main .content .header {
    position: absolute;
    top: 0;
    left: 0;
    right: 0;
    height: 30px;
}

#main .main .content .workspace {
    background: #282c34;
    position: absolute;
    top: 30px;
    left: 0;
    right: 0;
    bottom: 0;
    border-top: #333842 1px solid;
}
</style>
