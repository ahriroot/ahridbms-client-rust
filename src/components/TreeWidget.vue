<script setup lang="ts">
import { h, ref, shallowRef, onBeforeMount, onMounted } from 'vue'
import {
    NLayout, NIcon, NSpace, NInput, NSelect, NModal,
    NButton, NCard,
    SelectRenderLabel, useMessage
} from 'naive-ui'
import { useI18n } from 'vue-i18n'
import { emit, listen } from '@tauri-apps/api/event'

import { ConnectionComponents, DBType, RedisConnectInit, PostgresConnectInit, MongodbConnectInit } from '@/data/data'

import { test as testRedis } from '@/api/redis'
import { test as testPostgres } from '@/api/postgres'
import { test as testMongodb } from '@/api/mongodb'
import { getConnections, saveConnections, getTabs, saveTabs } from '@/utils/storage'
import { uuid } from '@/utils/crypto'

import { Connection } from '@/types/Connection'
import { IConnectComponents } from '@/types/data'
import { OpenTabMesagae } from '@/types/Message'
import { RedisConnect } from '@/types/redis'
import { PostgresConnect } from '@/types/postgres'
import { MongodbConnect } from '@/types/mongodb'


const { t } = useI18n()
const message = useMessage()
const dbTypeList = shallowRef(DBType)  // 数据库列表
const connList = ref<Connection<RedisConnect>[]>([])
const connComponents = shallowRef<IConnectComponents>(ConnectionComponents)  // 数据库连接组件

onBeforeMount(async () => {
    connList.value = await getConnections()
})

onMounted(async () => {
    const unlisten = await listen<string>('tree-widget', (e) => {
        let payload = JSON.parse(e.payload)
        let event = payload.event
        switch (event) {
            case 'create':
                showConn.value = true
                break
        }
    })
})

// 打开标签页
const handleOpenTab = (message: OpenTabMesagae<any>) => {
    let payload = {
        id: 1,
        event: 'open',
        data: message
    }
    emit('tab-widget', JSON.stringify(payload))
}

// 删除连接
const handleDeleteConnection = async (id: string) => {
    connList.value = connList.value.filter(item => item.id !== id)
    await saveConnections(connList.value)
}

/**
 * 关闭相关标签页, 打开编辑连接的弹窗
 * @param id 连接id
 */
const handleEditConnection = async (id: string) => {
    const conn = connList.value.find(item => item.id == id)
    if (conn) {
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
const showConn = ref(false)
const dbConnType = ref<string>('redis')
const dbRedis = ref<RedisConnect>(RedisConnectInit)  // 默认 redis 数据库连接信息
const dbPostgres = ref<PostgresConnect>(PostgresConnectInit)  // 默认 postgres 数据库连接信息
const dbMongodb = ref<MongodbConnect>(MongodbConnectInit)  // 默认 postgres 数据库连接信息
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
                message.success('OK')
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
                    message.success('OK')
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
                message.success("OK")
            }).finally(() => {
                loadingTest.value = false
            })
            break
    }
}

const editConnId = ref<string>('')
const isEdit = ref(false)
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
                    message.error('Name is required')
                    return
                }
                if (dbRedis.value.host == '') {
                    message.error('Host is required')
                    return
                }
                if (dbRedis.value.port == '') {
                    message.error('Port is required')
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
                    message.error('Name is required')
                    return
                }
                if (dbPostgres.value.host == '') {
                    message.error('Host is required')
                    return
                }
                if (dbPostgres.value.port == '') {
                    message.error('Port is required')
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
                    message.error('Name is required')
                    return
                }
                if (dbMongodb.value.host == '') {
                    message.error('Host is required')
                    return
                }
                if (dbMongodb.value.port == '') {
                    message.error('Port is required')
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
</script>

<template>
    <n-modal v-model:show="showConn" preset="card" style="width: 600px;" :title="t('connections')" size="small"
        :mask-closable="false">
        <n-space vertical>
            <n-select :options="dbTypeList" :render-label="renderLabel" v-model:value="dbConnType" />
            <n-card v-if="dbConnType == 'redis'">
                <n-space vertical>
                    <n-input v-model:value="dbRedis.name" type="text" :placeholder="t('connection.name')" />
                    <n-space>
                        <n-input v-model:value="dbRedis.host" type="text" :placeholder="t('connection.host')" />
                        <n-input v-model:value="dbRedis.port" type="text" :placeholder="t('connection.port')" />
                    </n-space>
                    <n-space>
                        <n-input v-model:value="dbRedis.user" type="text" :placeholder="t('connection.user')" />
                        <n-input v-model:value="dbRedis.pass" type="password" :placeholder="t('connection.pass')" />
                    </n-space>
                    <n-input v-model:value="dbRedis.index" type="text" :placeholder="t('connection.db')" />
                </n-space>
            </n-card>
            <n-card v-else-if="dbConnType == 'postgres'">
                <n-space vertical>
                    <n-input v-model:value="dbPostgres.name" type="text" :placeholder="t('connection.name')" />
                    <n-space>
                        <n-input v-model:value="dbPostgres.host" type="text" :placeholder="t('connection.host')" />
                        <n-input v-model:value="dbPostgres.port" type="text" :placeholder="t('connection.port')" />
                    </n-space>
                    <n-space>
                        <n-input v-model:value="dbPostgres.user" type="text" :placeholder="t('connection.user')" />
                        <n-input v-model:value="dbPostgres.pass" type="password" :placeholder="t('connection.pass')" />
                    </n-space>
                    <n-input v-model:value="dbPostgres.db" type="text" :placeholder="t('connection.db')" />
                </n-space>
            </n-card>
            <n-card v-else-if="dbConnType == 'mongodb'">
                <n-space vertical>
                    <n-input v-model:value="dbMongodb.name" type="text" :placeholder="t('connection.name')" />
                    <n-space>
                        <n-input v-model:value="dbMongodb.host" type="text" :placeholder="t('connection.host')" />
                        <n-input v-model:value="dbMongodb.port" type="text" :placeholder="t('connection.port')" />
                    </n-space>
                    <n-space>
                        <n-input v-model:value="dbMongodb.user" type="text" :placeholder="t('connection.user')" />
                        <n-input v-model:value="dbMongodb.pass" type="password" :placeholder="t('connection.pass')" />
                    </n-space>
                    <n-input v-model:value="dbMongodb.db" type="text" :placeholder="t('connection.db')" />
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
    <div id="tree-widget">
        <n-layout position="absolute" style="top: 0; left: 0; right: 0; bottom: 0; padding: 10px" :native-scrollbar="false">
            <component v-for="i in connList" :key="i.id" :is="connComponents[i.db_type]" :conn="i"
                @handleOpenTab="handleOpenTab" @handleDeleteConnection="handleDeleteConnection"
                @handleEditConnection="handleEditConnection" />
        </n-layout>
    </div>
</template>

<style scoped>
#tree-widget {
    position: absolute;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
}
</style>
