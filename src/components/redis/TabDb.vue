<script setup lang="ts">
import { h, ref, onBeforeMount } from 'vue'
import {
    NTable, NLayout, NTag, NButton, NIcon, NModal, SelectRenderLabel, useLoadingBar,
    NSpace, NCard, NSelect, NInput, useMessage, NSpin, NDropdown
} from 'naive-ui'
import { Add, Reload, CloudUploadOutline, Trash, Key, Copy, ArrowUp, ArrowDown } from '@vicons/ionicons5'
import { invoke } from '@tauri-apps/api/tauri'
import useClipboard from "vue-clipboard3"
import { Connection } from '@/types/Connection';
import { setString } from '@/api/redis'

const props = defineProps<{
    conn: Connection
    db: string
}>()
const emits = defineEmits<{
    (e: 'handle', val: null): void
}>()

interface Keyvalue {
    key: string
    key_type: string
    value: string
    size: number
    ttl: number
}
interface Data {
    [key: string]: Keyvalue
}

const { toClipboard } = useClipboard();
const message = useMessage()
const loadingBar = useLoadingBar()
const result = ref<Keyvalue[]>([])
const showAdd = ref<boolean>(false)

const init = async () => {
    loadingStart()
    let res = await invoke<Data[]>('keys', { conn: props })
    console.log(res)
    result.value = []
    res.forEach(item => {
        for (const key in item) {
            result.value.push(item[key])
        }
    })
    loadingFinish()
}

const fieldType = ref<string>('string')
const fieldTypeList = ref([
    {
        label: 'STRING',
        value: 'string'
    },
    {
        label: 'LIST',
        value: 'list'
    },
    {
        label: 'SET',
        value: 'set'
    },
    {
        label: 'ZSET',
        value: 'zset'
    },
    {
        label: 'HASH',
        value: 'hash'
    }
])
const fieldValue = ref<{
    string: {
        key: string
        value: string
        ttl: string
    },
    list: {
        key: string
        value: {
            value: string
        }[]
        ttl: string
    }
}>({
    string: {
        key: '',
        value: '',
        ttl: '-1'
    },
    list: {
        key: '',
        value: [
            {
                value: ''
            }
        ],
        ttl: '-1'
    },
})

const handleListItem = (index: number, opera: 1 | 2 | 3) => {
    switch (opera) {
        case 1:  // 向下
            if (index < fieldValue.value.list.value.length) {
                [fieldValue.value.list.value[index], fieldValue.value.list.value[index + 1]] = [fieldValue.value.list.value[index + 1], fieldValue.value.list.value[index]]
            }
            break
        case 2:  // 向上
            if (index > 0) {
                [fieldValue.value.list.value[index - 1], fieldValue.value.list.value[index]] = [fieldValue.value.list.value[index], fieldValue.value.list.value[index - 1]]
            }
            break
        case 3:  // 删除
            fieldValue.value.list.value.splice(index, 1)
            break
    }
}

const tagList = ref<any>({
    string: {
        tag: 'success',
        text: 'STRING'
    },
    list: {
        tag: 'info',
        text: 'LIST'
    },
    set: {
        tag: 'error',
        text: 'SET'
    },
    zset: {
        tag: 'error',
        text: 'ZSET'
    },
    hash: {
        tag: 'warning',
        text: 'HASH'
    }
})


const renderLabel: SelectRenderLabel = (option) => {
    return h(
        'div',
        {
            style: {
                display: 'flex',
                alignItems: 'center'
            }
        },
        h(NTag, {
            size: "small",
            bordered: false,
            type: tagList.value[option.value as string].tag
        }, {
            default: () => option.label
        })
    )
}

const handleReload = async () => {
    showLoadingTable.value = true
    await init()
    showLoadingTable.value = false
}
const handleReloadKey = async () => {
    loadingStart()
    const res = await invoke<any>('get', { conn: props, key: detailKey.value })
    console.log(res)
    detailTTL.value = res['String'].ttl.toString()
    result.value.forEach(item => {
        if (item.key == res['String'].key) {
            item.ttl = res['String'].ttl
        }
    })
    loadingFinish()
}

const handleRefresh = async () => {
    switch (fieldType.value) {
        case 'string':
            loadingStart()
            let res = await invoke('expire', { conn: props, key: detailKey.value, ttl: Number(detailTTL.value) })
            if (res = "Ok") {
                message.success('Success')
                await handleReload()
            } else {
                message.error('Error')
            }
            loadingFinish()
            break
    }
}

const showLoadingTable = ref<boolean>(false)
const loadingCount = ref<number>(0)
const loadingStart = () => {
    loadingCount.value++
    if (loadingCount.value == 1) {
        loadingBar.start()
    }
}
const loadingFinish = () => {
    loadingCount.value--
    if (loadingCount.value == 0) {
        loadingBar.finish()
    }
}

const handleSubmitAdd = async () => {
    switch (fieldType.value) {
        case 'string':
            loadingStart()
            if (await setString({ conn: props, key: fieldValue.value.string.key, value: fieldValue.value.string.value, ttl: Number(fieldValue.value.string.ttl) }) == "Ok") {
                message.success('Success')
                await handleReload()
            } else {
                message.error('Error')
            }
            loadingFinish()
            break
        case 'list':
            loadingStart()
            let values = fieldValue.value.list.value.map(item => item.value)
            let res = await invoke<string>('rpush', { conn: props, key: fieldValue.value.list.key, value: values, ttl: Number(fieldValue.value.list.ttl) })
            if (res == "Ok") {
                message.success('Success')
                await handleReload()
            } else {
                console.log(res)
                message.error('Error')
            }
            loadingFinish()
            break
    }
    showAdd.value = false
}

const handleCancelAdd = async () => {
    showAdd.value = false
}

const handleDelete = async (val: Keyvalue | null) => {
    let key = ''
    if (val) {
        if (detailKey.value == val.key) {
            detailKey.value = null
        }
        key = val.key
    } else {
        key = detailKey.value || ''
        detailKey.value = null
    }
    loadingStart()
    let res = await invoke('del', { conn: props, key: key })
    if (res = "Ok") {
        message.success('Success')
        await handleReload()
    } else {
        message.error('Error')
    }
    loadingFinish()
}

const handleCopy = async (val: any) => {
    if (val != null && val != undefined) {
        let str;
        try {
            str = JSON.stringify(val)
        } catch (error) {
            str = val
        }
        await toClipboard(str)
        message.success('Cpoied!', { duration: 800 })
    }
}

const detailKey = ref<string | null>(null)
const detailValue = ref<any>('')
const detailTTL = ref<string>('')
const detailKeyType = ref<string>('')
const handleDetail = async (val: Keyvalue) => {
    detailTTL.value = val.ttl.toString()
    detailKey.value = val.key
    detailValue.value = val.value
    detailKeyType.value = val.key_type
}

onBeforeMount(async () => {
    await init()
})

const dropdownList = ref([
    {
        label: 'STRING',
        key: 'string'
    }, {
        label: 'LIST',
        key: 'list'
    }, {
        label: 'SET',
        key: 'set'
    }, {
        label: 'ZSET',
        key: 'zset'
    }, {
        label: 'HASH',
        key: 'hash'
    }
])

const handleSelect = (val: string) => {
    fieldType.value = val
    showAdd.value = true
}
</script>

<template>
    <n-modal v-model:show="showAdd" preset="card" style="width: 600px; height: 80vh" title="新建" size="small">
        <div style="height: 100%; position: relative;">
            <n-select :options="fieldTypeList" :render-label="renderLabel" v-model:value="fieldType" />
            <n-layout position="absolute" style="top: 50px; bottom: 50px; background: #2c2c32;"
                :native-scrollbar="false" content-style="">
                <n-card v-if="fieldType == 'string'">
                    <n-space vertical>
                        <n-input v-model:value="fieldValue.string.key" type="text" placeholder="Key" />
                        <n-input v-model:value="fieldValue.string.value" type="textarea" placeholder="Value" />
                        <n-input v-model:value="fieldValue.string.ttl" type="text" placeholder="TTL" />
                    </n-space>
                </n-card>
                <n-card v-else-if="fieldType == 'list'">
                    <n-space vertical>
                        <n-input v-model:value="fieldValue.list.key" type="text" placeholder="Key" />
                        <n-space vertical>
                            <n-space v-for="(i, index) in fieldValue.list.value">
                                <n-input v-model:value="i.value" type="text" placeholder="Value" size="small" />
                                <n-button strong secondary type="info" size="small" @click="handleListItem(index, 3)">
                                    <template #icon>
                                        <n-icon>
                                            <trash />
                                        </n-icon>
                                    </template>
                                </n-button>
                                <n-button strong secondary type="info" size="small"
                                    :disabled="index == fieldValue.list.value.length - 1"
                                    @click="handleListItem(index, 1)">
                                    <template #icon>
                                        <n-icon>
                                            <arrow-down />
                                        </n-icon>
                                    </template>
                                </n-button>
                                <n-button strong secondary type="info" size="small" :disabled="index == 0"
                                    @click="handleListItem(index, 2)">
                                    <template #icon>
                                        <n-icon>
                                            <arrow-up />
                                        </n-icon>
                                    </template>
                                </n-button>
                            </n-space>
                            <n-button strong secondary type="info" size="small"
                                @click="fieldValue.list.value.push({ value: '' })">
                                Add
                            </n-button>
                        </n-space>
                        <n-input v-model:value="fieldValue.list.ttl" type="text" placeholder="TTL" />
                    </n-space>
                </n-card>
                <n-card v-else>
                </n-card>
            </n-layout>
            <div style="position: absolute; left: 0; bottom: 0;">
                <n-button @click="handleSubmitAdd" style="margin-top: 12px;">Submit</n-button>&nbsp;
                <n-button @click="handleCancelAdd" style="margin-top: 12px;">Cancel</n-button>
            </div>
        </div>
    </n-modal>

    <div class="keys">
        <div class="list">
            <div class="header">
                <div></div>
                <div>
                    <n-dropdown trigger="hover" size="small" placement="bottom-start" :options="dropdownList"
                        @select="handleSelect">
                        <n-button strong secondary circle type="info" size="small" @click="showAdd = true">
                            <template #icon>
                                <n-icon>
                                    <add />
                                </n-icon>
                            </template>
                        </n-button>
                    </n-dropdown>
                    &nbsp;
                    <n-button strong secondary circle type="info" size="small" @click="handleReload">
                        <template #icon>
                            <n-icon>
                                <reload />
                            </n-icon>
                        </template>
                    </n-button>
                </div>
            </div>
            <n-layout position="absolute" style="top: 36px;  color: #fff; height: 100%;" :native-scrollbar="false"
                content-style="">
                <n-spin :show="showLoadingTable">
                    <n-table :bordered="true" :single-line="false" size="small">
                        <tbody>
                            <tr v-for="i in result" style="cursor: pointer;" @click="handleDetail(i)">
                                <td class="td-key">
                                    <n-tag :bordered="false" :type="tagList[i.key_type].tag" size="small">
                                        {{ tagList[i.key_type].text }}
                                    </n-tag>
                                </td>
                                <td style="max-width: 200px; overflow: hidden;">{{ i.key }}</td>
                                <td class="td-size">{{ i.size }} b</td>
                                <td class="td-ttl">{{ i.ttl }} {{ i.ttl > 0 ? 's' : '' }}</td>
                                <td class="td-opera">
                                    <div class="btns">
                                        <n-button strong secondary circle type="info" size="small"
                                            @click.stop="handleCopy(i.key)">
                                            <template #icon>
                                                <n-icon>
                                                    <key />
                                                </n-icon>
                                            </template>
                                        </n-button>
                                        <n-button strong secondary circle type="info" size="small"
                                            @click.stop="handleCopy(i.value)">
                                            <template #icon>
                                                <n-icon>
                                                    <copy />
                                                </n-icon>
                                            </template>
                                        </n-button>
                                        <n-button strong secondary circle type="error" size="small"
                                            @click.stop="handleDelete(i)">
                                            <template #icon>
                                                <n-icon>
                                                    <trash />
                                                </n-icon>
                                            </template>
                                        </n-button>
                                    </div>
                                </td>
                            </tr>
                        </tbody>
                    </n-table>
                </n-spin>
            </n-layout>
        </div>
        <div class="content" v-if="detailKey == null"></div>
        <div class="content" v-else>
            <div class="header">
                <div></div>
                <div style="display: flex; align-items: center;">
                    <n-space>
                        <label style="display: flex; align-items: center;">TTL:&nbsp;
                            <n-input v-model:value="detailTTL" type="text" placeholder="TTL" size="small" />
                        </label>
                    </n-space>
                    &nbsp;
                    <n-button strong secondary circle type="info" size="small" @click="handleRefresh">
                        <template #icon>
                            <n-icon>
                                <cloud-upload-outline />
                            </n-icon>
                        </template>
                    </n-button>
                    &nbsp;
                    <n-button strong secondary circle type="info" size="small" @click.stop="handleCopy(detailKey)">
                        <template #icon>
                            <n-icon>
                                <key />
                            </n-icon>
                        </template>
                    </n-button>
                    &nbsp;
                    <n-button strong secondary circle type="info" size="small" @click.stop="handleCopy(detailValue)">
                        <template #icon>
                            <n-icon>
                                <copy />
                            </n-icon>
                        </template>
                    </n-button>
                    &nbsp;
                    <n-button strong secondary circle type="info" size="small" @click="handleReloadKey">
                        <template #icon>
                            <n-icon>
                                <reload />
                            </n-icon>
                        </template>
                    </n-button>
                    &nbsp;
                    <n-button strong secondary circle type="error" size="small" @click="handleDelete(null)">
                        <template #icon>
                            <n-icon>
                                <trash />
                            </n-icon>
                        </template>
                    </n-button>
                </div>
            </div>
            <n-layout position="absolute" style="background: #282c34; top: 36px;  color: #fff; bottom: 0; padding: 10px"
                :native-scrollbar="false">
                <div v-if="detailKeyType == 'string'">
                    <n-space vertical>
                        <n-input v-model:value="detailKey" :type="`${detailKey.length > 60 ? 'textarea' : 'text'}`"
                            readonly></n-input>
                        <n-input v-model:value="detailValue" type="textarea"></n-input>
                    </n-space>
                </div>
                <div v-if="detailKeyType == 'list'">
                    <n-space vertical>
                        <n-input v-model:value="detailKey" :type="`${detailKey.length > 60 ? 'textarea' : 'text'}`"
                            readonly></n-input>
                        <n-input v-for="i in detailValue" :value="i" type="text" disabled></n-input>
                    </n-space>
                </div>
            </n-layout>
        </div>
    </div>
</template>
<style scoped>
.keys {
    height: 100%;
    display: flex;
}

.keys .list {
    position: relative;
    height: 100%;
    width: 50%;
    border-right: #333842 2px solid;
}

.keys .list .header {
    position: absolute;
    top: 0;
    left: 0;
    width: 100%;
    height: 36px;
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 0 10px;
    border-bottom: #333842 1px solid;
}

.keys .content {
    position: relative;
    height: 100%;
    width: 50%;
    border-left: #333842 2px solid;
}

.keys .content .header {
    position: absolute;
    top: 0;
    left: 0;
    width: 100%;
    height: 36px;
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 0 10px;
    background: #282c34;
    border-bottom: #333842 1px solid;
}

.td-key {
    width: 60px;
}

.td-size {
    width: 80px;
}

.td-ttl {
    width: 100px;
}

.td-opera {
    width: 110px;
}

.td-opera .btns {
    display: flex;
    justify-content: space-between;
}
</style>
