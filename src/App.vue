<script setup lang="ts">
import { h, ref, shallowRef, onBeforeMount, onMounted, onBeforeUnmount } from 'vue'
import {
    darkTheme, NConfigProvider, NGlobalStyle, NIcon, NLayout,
    NButton, NModal, NSelect, SelectRenderLabel, NInput, NCard, NSpace,
    NTabs, NTabPane, NLoadingBarProvider, NMessageProvider, NDialogProvider,
    zhCN, enUS
} from 'naive-ui'
import { invoke } from '@tauri-apps/api/tauri'
import { ArrowForward, Add, Settings } from '@vicons/ionicons5'

import { DBType, ConnectionComponents, RedisConnectInit, TabComponents, PostgresConnectInit, InfoComponents, MongodbConnectInit } from '@/data/data'
import { IConnectComponents, IInfoComponents, ITabComponents } from '@/types/data'
import { Connection } from '@/types/Connection'
import { getConnections, saveConnections, getTabs, saveTabs } from '@/utils/storage'
import { OpenTabMesagae } from '@/types/Message'
import { RedisConnect } from '@/types/redis'
import { PostgresConnect } from '@/types/postgres'
import { MongodbConnect } from './types/mongodb'
import { useIndexStore } from '@/store'
import { useI18n } from 'vue-i18n'
import { test as testRedis } from '@/api/redis'
import { test as testPostgres } from '@/api/postgres'
import { test as testMongodb } from '@/api/mongodb'
import { uuid } from '@/utils/crypto'
import { emit, listen } from '@tauri-apps/api/event'

/** ------------------ 变量 Start ------------------ **/
const languages = ref<{ [x: string]: any }>({
    'zh-CN': zhCN,
    'en-US': enUS
})
const showSide = ref<boolean>(true)  // 显示侧边栏
const showConn = ref<boolean>(false)  // 显示编辑连接窗口
const dbTypeList = shallowRef(DBType)  // 数据库列表
const connList = ref<Connection<RedisConnect>[]>([])  // 数据库链接列表
const dbConnType = ref<string>('redis')  // 正在编辑的连接类型
const dbRedis = ref<RedisConnect>(RedisConnectInit)  // 默认 redis 数据库连接信息
const dbPostgres = ref<PostgresConnect>(PostgresConnectInit)  // 默认 postgres 数据库连接信息
const dbMongodb = ref<MongodbConnect>(MongodbConnectInit)  // 默认 postgres 数据库连接信息
const connComponents = shallowRef<IConnectComponents>(ConnectionComponents)  // 数据库连接组件

// tab 组件
const tabComponents = shallowRef<ITabComponents>(TabComponents)

// info 组件
const infoComponents = shallowRef<IInfoComponents>(InfoComponents)

const store = useIndexStore()
const { t, locale } = useI18n()
/** ------------------ 变量 End ------------------ **/

const handleShowSetting = async () => {
    // showSetting.value = true
    // add tab
    const message = {
        id: await uuid(),
        conn: {
            db_type: '',
        },
        tab_type: 'setting',
        data: {
            title: `Setting`,
            table: ''
        }
    }
    
    tabs.value.push(message)
    tab.value = message.id
    handleTabChanged(tab.value)
    saveTabs(tabs.value)
}

onBeforeUnmount(async () => {
    if (unlisten && typeof unlisten == 'function') {
        unlisten()
    }
})

let unlisten: any = null
onBeforeMount(async () => {
    /* Listening Start */
    unlisten = await listen('global', async (event: { payload:string }) => {
        let payload = JSON.parse(event.payload)
        switch(payload.from) {
            case 'redis':
                break
            case 'postgres':
                if (payload.event === "has_tab_opened") {
                    await emit('postgres', JSON.stringify({
                        event: 'has_tab_opened',
                        data: {
                            id: payload.data.id,
                            has: tabs.value.some((item) =>item.conn.id === payload.data.id)
                        }
                    }))
                } else if (payload.event === "edit_connection") {
                    await handleEditConnection(payload.data.id)
                } else if (payload.event === "delete_connection") {
                    await handleDeleteConnection(payload.data.id)
                }
                break
            case 'mongodb':
                break
        }
    })
    /* Listening End */
    try {
        let config = localStorage.getItem('config')
        if (config) {
            store.updateConfig(JSON.parse(config))
        } else {
            store.updateConfig({
                deleteNoConfirm: false,
                showSideBar: true,
                sideBarWidth: 250,
                pageSize: 20,
                lang: 'zh-CN'
            }, false)
        }
        locale.value = store.config.lang

        width.value = store.config.sideBarWidth
        oldWidth.value = width.value

        connList.value = await getConnections()
        let connIds = connList.value.map(item => item.id)  // 所有链接 id
        let tmp = await getTabs()  // 所有标签页
        let current_tab = localStorage.getItem('current_tab')
        tabs.value = tmp.filter((item: OpenTabMesagae<any>) => {
            if (item.id == current_tab) {
                tab.value = current_tab
            }
            return connIds.includes(item.conn.id) || item.tab_type == 'setting'
        })  // 存在连接的标签页
        saveTabs(tabs.value)
        if (tabs.value.length > 0 && !tab.value) {
            // 切换到第一个标签页
            tab.value = tabs.value[0].id
        }
    } catch { }
})

const sidebarRef = shallowRef<HTMLElement | null>(null)
const contentRef = shallowRef<HTMLElement | null>(null)
const resizeable = ref<boolean>(false)
const width = ref(store.config.sideBarWidth)
const oldWidth = ref(250)
const cursor = ref('default')
const currentMoveX = ref(0)
onMounted(async () => {
    if (sidebarRef.value && contentRef.value) {
        sidebarRef.value.addEventListener('mousedown', (ev) => {
            if (cursor.value == 'ew-resize') {
                resizeable.value = true
                currentMoveX.value = ev.clientX
                oldWidth.value = width.value
            }
        })
        contentRef.value.addEventListener('mousedown', (ev) => {
            if (cursor.value == 'ew-resize') {
                resizeable.value = true
                currentMoveX.value = ev.clientX
                oldWidth.value = width.value
            }
        })
        document.body.addEventListener('mousemove', (ev) => {
            if (sidebarRef.value && (width.value - 4 < ev.offsetX && ev.offsetX < width.value + 4)) {
                cursor.value = 'ew-resize'
            } else {
                cursor.value = 'default'
            }
            if (resizeable.value) {
                const tmp = oldWidth.value + ev.clientX - currentMoveX.value
                if (tmp < 150) {
                    width.value = 150
                } else if (tmp > 1000) {
                    width.value = 1000
                } else {
                    width.value = tmp
                }
            }
        })
        document.body.addEventListener('mouseup', (ev) => {
            resizeable.value = false
            store.updateConfig({
                ...store.config,
                sideBarWidth: width.value
            })
        })
    }

    setTimeout(async () => {
        await invoke('close_splashscreen')
    }, 1000)
})

// 新建连接时不同类型的数据库图标
const renderLabel: SelectRenderLabel = (option) => {
    return h(
        'div', { style: { display: 'flex', alignItems: 'center' } },
        [
            h(NIcon, { size: 20 }, { default: () => h(option.icon as any) }),
            h('div', { style: { marginLeft: '12px', padding: '4px 0' } }, h('div', null, [option.label as string]))
        ]
    )
}

// 测试连接
const loadingTest = ref<boolean>(false)
const handleTestConn = async () => {
    switch (dbConnType.value) {
        case 'redis':
            loadingTest.value = true
            testRedis({
                conn: {
                    id: '',
                    db_type: 'redis',
                    info: JSON.parse(JSON.stringify(dbRedis.value))
                },
                db: '0'
            }).then(() => {
                window.$message.success('OK')
            }).finally(() => {
                loadingTest.value = false
            })
            break
        case 'postgres':
            loadingTest.value = true
            testPostgres({
                conn: {
                    id: '',
                    db_type: 'postgres',
                    info: JSON.parse(JSON.stringify(dbPostgres.value))
                },
                database: dbPostgres.value.db
            }).then((res) => {
                if (!res.is_error) {
                    window.$message.success('OK')
                }
            })
            .finally(() => {
                loadingTest.value = false
            })
            break
        case 'mongodb':
            loadingTest.value = true
            testMongodb({
                conn: {
                    id: '',
                    db_type: 'mongodb',
                    info: JSON.parse(JSON.stringify(dbMongodb.value))
                },
                database: dbMongodb.value.db
            }).then((res) => {
                window.$message.success("OK")
            }).finally(() => {
                loadingTest.value = false
            })
            break
    }
}

const handleSubmitConn = async () => {
    if (isEdit.value) {
        // 编辑连接
        let conn = connList.value.find(item => item.id == editConnId.value)
        if (conn) {
            switch (conn.db_type) {
                case 'redis':
                    conn.info = JSON.parse(JSON.stringify(dbRedis.value))
                    break
                case 'postgres':
                    conn.info = JSON.parse(JSON.stringify(dbPostgres.value))
                    await emit('postgres', JSON.stringify({
                        event: 'edit_connection',
                        data: {
                            id: conn.id
                        }
                    }))
                    break
                case 'mongodb':
                    conn.info = JSON.parse(JSON.stringify(dbMongodb.value))
                    break
            }
            await saveConnections(connList.value)
            isEdit.value = false
            showConn.value = false
        }
    } else {
        // 添加连接
        switch (dbConnType.value) {
            case 'redis':
                if (dbRedis.value.name == '') {
                    window.$message.error('Name is required')
                    return
                }
                if (dbRedis.value.host == '') {
                    window.$message.error('Host is required')
                    return
                }
                if (dbRedis.value.port == '') {
                    window.$message.error('Port is required')
                    return
                }
                connList.value.push({
                    id: await uuid(),
                    db_type: 'redis',
                    info: JSON.parse(JSON.stringify(dbRedis.value))
                })
                break
            case 'postgres':
                if (dbPostgres.value.name == '') {
                    window.$message.error('Name is required')
                    return
                }
                if (dbPostgres.value.host == '') {
                    window.$message.error('Host is required')
                    return
                }
                if (dbPostgres.value.port == '') {
                    window.$message.error('Port is required')
                    return
                }
                connList.value.push({
                    id: await uuid(),
                    db_type: 'postgres',
                    info: JSON.parse(JSON.stringify(dbPostgres.value))
                })
                break
            case 'mongodb':
                if (dbMongodb.value.name == '') {
                    window.$message.error('Name is required')
                    return
                }
                if (dbMongodb.value.host == '') {
                    window.$message.error('Host is required')
                    return
                }
                if (dbMongodb.value.port == '') {
                    window.$message.error('Port is required')
                    return
                }
                connList.value.push({
                    id: await uuid(),
                    db_type: 'mongodb',
                    info: JSON.parse(JSON.stringify(dbMongodb.value))
                })
                break
        }
        await saveConnections(connList.value)
        isEdit.value = false
        showConn.value = false
    }
}

// 取消添加连接
const handleCancelConn = () => {
    isEdit.value = false
    showConn.value = false
    dbRedis.value = RedisConnectInit
}

// ----------------------------------
const tab = ref<string>('')
const tabs = ref<OpenTabMesagae<any>[]>([])

// 打开标签页
const handleOpenTab = (message: OpenTabMesagae<any>) => {
    if (message) {
        switch (message.conn.db_type) {
            case 'redis':
                tabs.value.push(message)
                tab.value = message.id
                handleTabChanged(tab.value)
                saveTabs(tabs.value)
                break
            case 'postgres':
                tabs.value.push(message)
                tab.value = message.id
                handleTabChanged(tab.value)
                saveTabs(tabs.value)
                break
            case 'mongodb':
                tabs.value.push(message)
                tab.value = message.id
                handleTabChanged(tab.value)
                saveTabs(tabs.value)
                break
            default:
                break
        }
    }
}

const handleCloseTabs = (id: string) => {
    tabs.value = tabs.value.filter(item => item.conn.id !== id)
    if (!tabs.value.find(item => item.conn.id === id)) {
        tab.value = tabs.value.length > 0 ? tabs.value[0].id : ''
    }
    saveTabs(tabs.value)
}

// 删除连接
const handleDeleteConnection = async (id: string) => {
    connList.value = connList.value.filter(item => item.id !== id)
    await saveConnections(connList.value)
    await handleCloseTabs(id)
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

const handleShowSide = async () => {
    showSide.value = !showSide.value
    store.updateConfig({
        ...store.config,
        showSideBar: showSide.value
    })
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

const isEdit = ref(false)  // 是否是编辑连接
const editConnId = ref<string>('')  // 正在编辑的连接id

/**
 * 关闭相关标签页, 打开编辑连接的弹窗
 * @param id 连接id
 */
const handleEditConnection = async (id: string) => {
    const conn = connList.value.find(item => item.id == id)
    if (conn) {
        await handleCloseTabs(id)
        isEdit.value = true  // 编辑连接而不是新建连接
        showConn.value = true  // 打开连接弹窗
        editConnId.value = id  // 编辑的连接id
        switch (conn.db_type) {
            case 'redis':
                dbConnType.value = 'redis'
                dbRedis.value = {
                    ...RedisConnectInit,
                    ...conn.info
                }
                break
            case 'postgres':
                dbConnType.value = 'postgres'
                dbPostgres.value = {
                    ...PostgresConnectInit,
                    ...conn.info
                }
                break
            case 'mongodb':
                dbConnType.value = 'mongodb'
                dbMongodb.value = {
                    ...MongodbConnectInit,
                    ...conn.info
                }
                break
        }
    }
}
</script>

<template>
    <n-config-provider :theme="darkTheme" :locale="languages[locale]">
        <n-global-style />
        <n-loading-bar-provider>
            <n-message-provider>
                <n-dialog-provider>
                    <n-modal v-model:show="showConn" preset="card" style="width: 600px;" :title="t('connections')"
                        size="small">
                        <n-space vertical>
                            <n-select :options="dbTypeList" :render-label="renderLabel" v-model:value="dbConnType" />
                            <n-card v-if="dbConnType == 'redis'">
                                <n-space vertical>
                                    <n-input v-model:value="dbRedis.name" type="text"
                                        :placeholder="t('connection.name')" />
                                    <n-space>
                                        <n-input v-model:value="dbRedis.host" type="text"
                                            :placeholder="t('connection.host')" />
                                        <n-input v-model:value="dbRedis.port" type="text"
                                            :placeholder="t('connection.port')" />
                                    </n-space>
                                    <n-space>
                                        <n-input v-model:value="dbRedis.user" type="text"
                                            :placeholder="t('connection.user')" />
                                        <n-input v-model:value="dbRedis.pass" type="password"
                                            :placeholder="t('connection.pass')" />
                                    </n-space>
                                    <n-input v-model:value="dbRedis.index" type="text"
                                        :placeholder="t('connection.db')" />
                                </n-space>
                            </n-card>
                            <n-card v-else-if="dbConnType == 'postgres'">
                                <n-space vertical>
                                    <n-input v-model:value="dbPostgres.name" type="text"
                                        :placeholder="t('connection.name')" />
                                    <n-space>
                                        <n-input v-model:value="dbPostgres.host" type="text"
                                            :placeholder="t('connection.host')" />
                                        <n-input v-model:value="dbPostgres.port" type="text"
                                            :placeholder="t('connection.port')" />
                                    </n-space>
                                    <n-space>
                                        <n-input v-model:value="dbPostgres.user" type="text"
                                            :placeholder="t('connection.user')" />
                                        <n-input v-model:value="dbPostgres.pass" type="password"
                                            :placeholder="t('connection.pass')" />
                                    </n-space>
                                    <n-input v-model:value="dbPostgres.db" type="text"
                                        :placeholder="t('connection.db')" />
                                </n-space>
                            </n-card>
                            <n-card v-else-if="dbConnType == 'mongodb'">
                                <n-space vertical>
                                    <n-input v-model:value="dbMongodb.name" type="text"
                                        :placeholder="t('connection.name')" />
                                    <n-space>
                                        <n-input v-model:value="dbMongodb.host" type="text"
                                            :placeholder="t('connection.host')" />
                                        <n-input v-model:value="dbMongodb.port" type="text"
                                            :placeholder="t('connection.port')" />
                                    </n-space>
                                    <n-space>
                                        <n-input v-model:value="dbMongodb.user" type="text"
                                            :placeholder="t('connection.user')" />
                                        <n-input v-model:value="dbMongodb.pass" type="password"
                                            :placeholder="t('connection.pass')" />
                                    </n-space>
                                    <n-input v-model:value="dbMongodb.db" type="text"
                                        :placeholder="t('connection.db')" />
                                </n-space>
                            </n-card>
                            <n-card v-else>
                            </n-card>
                            <n-space>
                                <n-button @click="handleTestConn" :loading="loadingTest" style="margin-top: 12px;">
                                    {{ t('test') }}
                                </n-button>
                                <n-button @click="handleSubmitConn" style="margin-top: 12px;">
                                    {{ t('confirm') }}
                                </n-button>
                                <n-button @click="handleCancelConn" style="margin-top: 12px;">
                                    {{ t('cancel') }}
                                </n-button>
                            </n-space>
                        </n-space>
                    </n-modal>

                    <!-- <div id="main" class="nocopy"> -->
                    <div id="main">
                        <aside class="side nocopy" :class="store.config?.showSideBar ? '' : 'show'">
                            <div class="sidebar">
                                <n-button circle quaternary size="small" @click.stop="handleShowSetting">
                                    <template #icon>
                                        <n-icon class="btn-icon-setting">
                                            <Settings />
                                        </n-icon>
                                    </template>
                                </n-button>
                            </div>
                        </aside>
                        <main class="main" :class="store.config?.showSideBar ? '' : 'show'"
                            :style="`cursor: ${resizeable ? 'ew-resize' : cursor}`">
                            <div class="connection" ref="sidebarRef" :style="`width: ${width}px`">
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
                                            @handleDeleteConnection="handleDeleteConnection"
                                            @handleEditConnection="handleEditConnection" />
                                    </n-layout>
                                </div>
                                <div class="btn-side">
                                    <n-icon size="20" @click="handleShowSide">
                                        <arrow-forward class="icon" :class="store.config?.showSideBar ? 'show' : ''" />
                                    </n-icon>
                                </div>
                            </div>
                            <div class="content" ref="contentRef" :style="`left: ${width}px`">
                                <header data-tauri-drag-region class="header">
                                    <div v-for="i in connList" v-show="i.id == tabs.find(t => t.id == tab)?.conn.id">
                                        <component :key="i.id" :is="infoComponents[i.db_type]" :conn="i" />
                                    </div>
                                </header>
                                <section class="workspace">
                                    <n-tabs v-model:value="tab" @update:value="handleTabChanged" type="card" closable
                                        tab-style="min-width: 80px;" @close="handleClose" size="small">
                                        <n-tab-pane display-directive="show" v-for="i in tabs" :key="i.id"
                                            :tab="i.data.title" :name="i.id">
                                            <component :key="i.id"
                                                :is="tabComponents[`${i.conn.db_type}:${i.tab_type}`]" :conn="i.conn"
                                                :data="i.data" @handleCloseTab="handleCloseTab(i.id)" />
                                        </n-tab-pane>
                                    </n-tabs>
                                </section>
                            </div>
                        </main>
                    </div>
                </n-dialog-provider>
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
.btn-icon-setting {
    transition: 0.3s;
}

.btn-icon-setting:hover {
    transform: rotate(90deg);
}

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

#main .side .sidebar {
    padding: 10px 0;
    height: 100%;
    width: 100%;
    display: flex;
    flex-direction: column;
    justify-content: flex-end;
    align-items: center;
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
    right: 0;
    bottom: 0;
}

#main .main .content .header {
    position: absolute;
    top: 0;
    left: 0;
    right: 0;
    height: 30px;
    display: flex;
    justify-content: flex-end;
    align-items: center;
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

.n-tab-pane {
    position: relative;
}
</style>
