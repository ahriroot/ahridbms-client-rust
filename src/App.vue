<script setup lang="ts">
import { h, ref, shallowRef, onBeforeMount } from 'vue'
import {
    darkTheme, NConfigProvider, NGlobalStyle, NIcon, NLayout,
    NButton, NModal, NSelect, SelectRenderLabel, NInput, NCard, NSpace,
    NTabs, NTabPane, NLoadingBarProvider, NMessageProvider
} from 'naive-ui'
import { ArrowForward, ServerSharp, Add } from '@vicons/ionicons5'
import { nanoid } from 'nanoid'  // 唯一 id 生成器

import { DBType, ConnectionComponents, RedisConnectInit, TabComponents, PostgresConnectInit } from '@/data/data'
import { IConnectComponents, ITabComponents } from '@/types/data'
import { Connection } from '@/types/Connection'
import { getConnections, saveConnections, getTabs, saveTabs } from '@/utils/storage'
import { OpenTabMesagae } from '@/types/Message'
import { RedisConnect } from '@/types/redis'
import { PostgresConnect } from '@/types/postgres'
import { useIndexStore } from '@/store'

/** ------------------ 变量 Start ------------------ **/
const showSide = ref<boolean>(true)  // 显示侧边栏
const showConn = ref<boolean>(false)  // 显示编辑连接窗口
const dbTypeList = ref(DBType)  // 数据库列表
const connList = ref<Connection<RedisConnect>[]>([])  // 数据库链接列表
const dbConn = ref<string>('redis')  // 正在编辑的连接类型
const dbRedis = ref<RedisConnect>(RedisConnectInit)  // 默认 redis 数据库连接信息
const dbPostgres = ref<PostgresConnect>(PostgresConnectInit)  // 默认 postgres 数据库连接信息
const connComponents = shallowRef<IConnectComponents>(ConnectionComponents)  // 数据库连接组件

// tab 组件
const tabComponents = shallowRef<ITabComponents>(TabComponents)

const store = useIndexStore()
/** ------------------ 变量 End ------------------ **/

onBeforeMount(async () => {
    let config = localStorage.getItem('config')
    store.updateConfig(config ? JSON.parse(config) : { deleteNoConfirm: false })
    connList.value = await getConnections()
    let connIds = connList.value.map(item => item.id)  // 所有链接 id
    let tmp = await getTabs()  // 所有标签页
    let current_tab = localStorage.getItem('current_tab')
    tabs.value = tmp.filter((item: OpenTabMesagae) => {
        if (item.id == current_tab) {
            tab.value = current_tab
        }
        return connIds.includes(item.conn.id)
    })  // 存在连接的标签页
    saveTabs(tabs.value)
    if (tabs.value.length > 0 && !tab.value) {
        // 切换到第一个标签页
        tab.value = tabs.value[0].id
    }
})

// 新建连接时不同类型的数据库图标
const renderLabel: SelectRenderLabel = (option) => {
    return h(
        'div', { style: { display: 'flex', alignItems: 'center' } },
        [
            h(NIcon, { size: 20 }, { default: () => h(ServerSharp) }),
            h('div', { style: { marginLeft: '12px', padding: '4px 0' } }, h('div', null, [option.label as string]))
        ]
    )
}

// 添加连接
const handleSubmitConn = async () => {
    switch (dbConn.value) {
        case 'redis':
            connList.value.push({
                id: nanoid(),
                db_type: 'redis',
                info: JSON.parse(JSON.stringify(dbRedis.value))
            })
            break
        case 'postgres':
            connList.value.push({
                id: nanoid(),
                db_type: 'postgres',
                info: JSON.parse(JSON.stringify(dbPostgres.value))
            })
            break
    }
    await saveConnections(connList.value)
    showConn.value = false
}

// 取消添加连接
const handleCancelConn = () => {
    showConn.value = false
    dbRedis.value = RedisConnectInit
}

// ----------------------------------
const tab = ref<string>('')
const tabs = ref<OpenTabMesagae[]>([])

// 打开标签页
const handleOpenTab = (message: OpenTabMesagae) => {
    if (message) {
        if (message.conn.db_type === 'redis') {
            if (message.tab_type === 'query') {
                tabs.value.push(message)
                tab.value = message.id
                saveTabs(tabs.value)
            } else {
                tabs.value.push(message)
                tab.value = message.id
                saveTabs(tabs.value)
            }
        }
    }

}

// 删除连接
const handleDeleteConnection = async (id: string) => {
    connList.value = connList.value.filter(item => item.id !== id)
    await saveConnections(connList.value)
    tabs.value = tabs.value.filter(item => item.conn.id !== id)
    if (!tabs.value.find(item => item.conn.id === id)) {
        tab.value = tabs.value.length > 0 ? tabs.value[0].id : ''
    }
    saveTabs(tabs.value)
}

const handleTabChanged = (val: string) => {
    localStorage.setItem('current_tab', val)
}

// 关闭标签页
const handleClose = (val: string) => {
    tabs.value = tabs.value.filter(item => item.id !== val)
    if (val == tab.value) {
        tab.value = tabs.value.length > 0 ? tabs.value[0].id : ''
    }
    saveTabs(tabs.value)
}
</script>

<template>
    <n-config-provider :theme="darkTheme">
        <n-global-style />
        <n-loading-bar-provider>
            <n-message-provider>

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
                        <n-card v-else-if="dbConn == 'postgres'">
                            <n-space vertical>
                                <n-input v-model:value="dbPostgres.name" type="text" placeholder="Name" />
                                <n-space>
                                    <n-input v-model:value="dbPostgres.host" type="text" placeholder="Host" />
                                    <n-input v-model:value="dbPostgres.port" type="text" placeholder="Port" />
                                </n-space>
                                <n-space>
                                    <n-input v-model:value="dbPostgres.user" type="text" placeholder="User" />
                                    <n-input v-model:value="dbPostgres.pass" type="text" placeholder="Pass" />
                                </n-space>
                                <n-input v-model:value="dbPostgres.db" type="text" placeholder="DB Name" />
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
                    <main class="main" :class="showSide ? '' : 'show'">
                        <div class="connection">
                            <div class="header">
                                <n-button strong secondary size="small" @click.stop="showConn = true">
                                    <template #icon>
                                        <n-icon>
                                            <add />
                                        </n-icon>
                                    </template>
                                </n-button>
                            </div>
                            <div class="conn">
                                <n-layout position="absolute" style="background: #21252b; color: #fff"
                                    :native-scrollbar="false" content-style="padding: 10px;">
                                    <component v-for="i in connList" :key="i.id" :is="connComponents[i.db_type]"
                                        :conn="i" @handleOpenTab="handleOpenTab"
                                        @handleDeleteConnection="handleDeleteConnection" />
                                </n-layout>
                            </div>
                            <div class="btn-side">
                                <n-icon size="20" @click="showSide = !showSide">
                                    <arrow-forward class="icon" :class="showSide ? 'show' : ''" />
                                </n-icon>
                            </div>
                        </div>
                        <div class="content">
                            <header class="header"></header>
                            <section class="workspace">
                                <n-tabs v-model:value="tab" @update:value="handleTabChanged" type="card" closable
                                    tab-style="min-width: 80px;" @close="handleClose" size="small">
                                    <n-tab-pane display-directive="show" v-for="i in tabs" :key="i.id"
                                        :tab="`${i.tab_type}@${i.conn.info.name}`" :name="i.id">
                                        <component :key="i.id"
                                            :is="tabComponents[`${i.conn.db_type}:${i.tab_type == 'query' ? 'query' : 'db'}`]"
                                            :conn="i.conn" :db="i.tab_type" />
                                    </n-tab-pane>
                                </n-tabs>
                            </section>
                        </div>
                    </main>
                </div>
            </n-message-provider>
        </n-loading-bar-provider>
    </n-config-provider>
</template>

<style>
* {
    outline: none;
    padding: 0;
    margin: 0;
    box-sizing: border-box;
}

.n-tabs {
    height: 100%;
}

.n-tab-pane {
    height: 100%;
    padding: 0 !important;
}
</style>

<style scoped>
#main {
    position: fixed;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;

}

#main .side {
    width: 50px;
    background: #333842;
    overflow: hidden;
    transition: .3s;
    position: absolute;
    top: 0;
    left: 0;
    bottom: 0;
}

#main .side.show {
    left: -50px;
}

#main .main {
    background: #282c34;
    transition: .3s;
    position: absolute;
    top: 0;
    left: 50px;
    right: 0;
    bottom: 0;
}

#main .main.show {
    left: 0;
}

#main .main .connection {
    width: 250px;
    background: #21252b;
    position: absolute;
    top: 0;
    left: 0;
    bottom: 0;
}

#main .main .connection .header {
    height: 32px;
    padding: 0 2px;
    display: flex;
    /* justify-content: center; */
    align-items: center;
}

#main .main .connection .conn {
    overflow: hidden;
    position: absolute;
    top: 32px;
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
    background: #282c34;
    position: absolute;
    top: 0;
    left: 250px;
    right: 0;
    bottom: 0;
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
