<script setup lang="ts">
import { h, ref, onBeforeMount } from 'vue'
import {
    NTable, NLayout, NTag, NButton, NIcon, NModal, SelectRenderLabel, useLoadingBar,
    NSpace, NCard, NSelect, NInput, useMessage, NSpin, NDropdown, NInputNumber
} from 'naive-ui'
import { Add, Reload, CloudUploadOutline, Trash, Key, Copy, ArrowUp, ArrowDown } from '@vicons/ionicons5'
import { invoke } from '@tauri-apps/api/tauri'
import useClipboard from "vue-clipboard3"
import { Connection } from '@/types/Connection';
import { keys, setString, rpush, sadd, zadd, hmset } from '@/api/redis'
import { INewFieldValue, Keyvalue } from '@/types/redis'
import { diffDatetime } from '@/utils/datetime'
import { NewFieldValue } from '@/data/redis'

window.$message = useMessage()

const props = defineProps<{
    conn: Connection
    db: string
}>()

const { toClipboard } = useClipboard();
const message = useMessage()
const loadingBar = useLoadingBar()

const result = ref<Keyvalue[]>([])  // 所有 keys
const showAdd = ref<boolean>(false)  // 显示新建 key 模态框

const showLoadingTable = ref<boolean>(false)  // table loading 动画
const loadingCount = ref<number>(0)  // loading 动画计数

const timer = ref<any>(null)  // 计时器
const before = ref<Date>(new Date())
const lastReload = ref<string>('less 1m')

const init = async () => {
    loadingStart()
    try {
        let res = await keys({ conn: props, arg: '*' })
        result.value = []
        res.forEach(item => {
            for (const key in item) {
                result.value.push(item[key])
            }
        })
        before.value = new Date()
        lastReload.value = await diffDatetime(before.value)
    } catch {
    }
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
const fieldValue = ref<INewFieldValue>(NewFieldValue)

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

const handleSetItem = (index: number, opera: 1 | 2 | 3) => {
    switch (opera) {
        case 1:  // 向下
            if (index < fieldValue.value.set.value.length) {
                [fieldValue.value.set.value[index], fieldValue.value.set.value[index + 1]] = [fieldValue.value.set.value[index + 1], fieldValue.value.set.value[index]]
            }
            break
        case 2:  // 向上
            if (index > 0) {
                [fieldValue.value.set.value[index - 1], fieldValue.value.set.value[index]] = [fieldValue.value.set.value[index], fieldValue.value.set.value[index - 1]]
            }
            break
        case 3:  // 删除
            fieldValue.value.set.value.splice(index, 1)
            break
    }
}

const handleZsetItem = (index: number, opera: 1 | 2 | 3) => {
    switch (opera) {
        case 1:  // 向下
            if (index < fieldValue.value.zset.value.length) {
                [fieldValue.value.zset.value[index], fieldValue.value.zset.value[index + 1]] = [fieldValue.value.zset.value[index + 1], fieldValue.value.zset.value[index]]
            }
            break
        case 2:  // 向上
            if (index > 0) {
                [fieldValue.value.zset.value[index - 1], fieldValue.value.zset.value[index]] = [fieldValue.value.zset.value[index], fieldValue.value.zset.value[index - 1]]
            }
            break
        case 3:  // 删除
            fieldValue.value.zset.value.splice(index, 1)
            break
    }
}

const handleHashItem = (index: number, opera: 1 | 2 | 3) => {
    switch (opera) {
        case 1:  // 向下
            if (index < fieldValue.value.hash.value.length) {
                [fieldValue.value.hash.value[index], fieldValue.value.hash.value[index + 1]] = [fieldValue.value.hash.value[index + 1], fieldValue.value.hash.value[index]]
            }
            break
        case 2:  // 向上
            if (index > 0) {
                [fieldValue.value.hash.value[index - 1], fieldValue.value.hash.value[index]] = [fieldValue.value.hash.value[index], fieldValue.value.hash.value[index - 1]]
            }
            break
        case 3:  // 删除
            fieldValue.value.hash.value.splice(index, 1)
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
    let tmp_ket = 'String'
    for (const key in res) {
        tmp_ket = key
    }
    detailTTL.value = res[tmp_ket].ttl.toString()
    result.value.forEach(item => {
        if (item.key == res[tmp_ket].key) {
            item.ttl = res[tmp_ket].ttl
        }
    })
    loadingFinish()
}

const handleRefresh = async () => {
    switch (fieldType.value) {
        case 'string':
            loadingStart()
            let res = await invoke('expire', { conn: props, key: detailKey.value, ttl: Number(detailTTL.value) })
            if (res == "Ok") {
                message.success('Success')
                await handleReload()
            } else {
                message.error('Error')
            }
            loadingFinish()
            break
    }
}

const loadingStart = () => {
    loadingCount.value++
    if (loadingCount.value == 1) {
        loadingBar.start()
    }
}
const loadingFinish = () => {
    if (loadingCount.value > 0) {
        loadingCount.value--
    }
    if (loadingCount.value == 0) {
        loadingBar.finish()
    }
}

const handleSubmitAdd = async () => {
    switch (fieldType.value) {
        case 'string':
            loadingStart()
            let string_res = await setString({ conn: props, key: fieldValue.value.string.key, value: fieldValue.value.string.value, ttl: Number(fieldValue.value.string.ttl) })
            if (string_res == "OK") {
                message.success('Success')
                await handleReload()
            } else {
                message.error('Error')
            }
            loadingFinish()
            break
        case 'list':
            loadingStart()
            let list_values = fieldValue.value.list.value.filter(item => item.value).map(item => item.value)
            let list_res = await rpush({ conn: props, key: fieldValue.value.list.key, value: list_values, ttl: Number(fieldValue.value.list.ttl) })
            if (list_res >= 0) {
                message.success(`Success, ${list_res} items added`)
                await handleReload()
            } else {
                console.log(list_res)
                message.error('Error')
            }
            loadingFinish()
            break
        case 'set':
            loadingStart()
            let set_values = fieldValue.value.set.value.filter(item => item.value).map(item => item.value)
            let set_res = await sadd({ conn: props, key: fieldValue.value.list.key, value: set_values, ttl: Number(fieldValue.value.list.ttl) })
            if (set_res >= 0) {
                message.success(`Success, ${set_res} items added`)
                await handleReload()
            } else {
                console.log(set_res)
                message.error('Error')
            }
            loadingFinish()
            break
        case 'zset':
            loadingStart()
            let zset_res = await zadd({ conn: props, key: fieldValue.value.zset.key, value: fieldValue.value.zset.value, ttl: Number(fieldValue.value.zset.ttl) })
            if (zset_res >= 0) {
                message.success(`Success, ${zset_res} items added`)
                await handleReload()
            } else {
                console.log(zset_res)
                message.error('Error')
            }
            loadingFinish()
            break
        case 'hash':
            loadingStart()
            let hash_res = await hmset({ conn: props, key: fieldValue.value.hash.key, value: fieldValue.value.hash.value, ttl: Number(fieldValue.value.hash.ttl) })
            if (hash_res == 'OK') {
                message.success('Success')
                await handleReload()
            } else {
                console.log(hash_res)
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
        // 判断 val 是否为字符串
        if (typeof val === 'string') {
            str = val;
        } else if (typeof val === 'object') {
            try {
                str = JSON.stringify(val)
            } catch (error) {
                str = val.toString()
            }
        } else {
            str = val.toString()
        }
        await toClipboard(str)
        message.success('Cpoied!', { duration: 800 })
    }
}


const detailKey = ref<string | null>(null)
const detailValue = ref<any>('')
const detailTTL = ref<string>('')
const detailKeyType = ref<string>('')
const zsetToJson = (val: string[]) => {
    let res: any = {}
    let len = val.length / 2
    for (let i = 0; i < len; i++) {
        res[val[i * 2]] = val[i * 2 + 1]
    }
    return res
}
const handleDetail = async (val: Keyvalue) => {
    detailTTL.value = val.ttl.toString()
    detailKey.value = val.key
    detailValue.value = val.value
    detailKeyType.value = val.key_type
}

onBeforeMount(async () => {
    console.log(1)
    const res = await invoke<any>('select', { skip: 0, limit: 10, page: 0, size: 0, table: 'user' })
    console.log(res)
    console.log(2)
    await init()
    if (timer.value) {
        clearInterval(timer.value)
    }
    timer.value = setInterval(async () => {
        lastReload.value = await diffDatetime(before.value)
    }, 5000)
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
                                <n-input style="width: 368px" v-model:value="i.value" type="text" placeholder="Value"
                                    size="small" />
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
                <n-card v-else-if="fieldType == 'set'">
                    <n-space vertical>
                        <n-input v-model:value="fieldValue.set.key" type="text" placeholder="Key" />
                        <n-space vertical>
                            <n-space v-for="(i, index) in fieldValue.set.value">
                                <n-input style="width: 368px" v-model:value="i.value" type="text" placeholder="Value"
                                    size="small" />
                                <n-button strong secondary type="info" size="small" @click="handleSetItem(index, 3)">
                                    <template #icon>
                                        <n-icon>
                                            <trash />
                                        </n-icon>
                                    </template>
                                </n-button>
                                <n-button strong secondary type="info" size="small"
                                    :disabled="index == fieldValue.set.value.length - 1"
                                    @click="handleSetItem(index, 1)">
                                    <template #icon>
                                        <n-icon>
                                            <arrow-down />
                                        </n-icon>
                                    </template>
                                </n-button>
                                <n-button strong secondary type="info" size="small" :disabled="index == 0"
                                    @click="handleSetItem(index, 2)">
                                    <template #icon>
                                        <n-icon>
                                            <arrow-up />
                                        </n-icon>
                                    </template>
                                </n-button>
                            </n-space>
                            <n-button strong secondary type="info" size="small"
                                @click="fieldValue.set.value.push({ value: '' })">
                                Add
                            </n-button>
                        </n-space>
                        <n-input v-model:value="fieldValue.set.ttl" type="text" placeholder="TTL" />
                    </n-space>
                </n-card>
                <n-card v-else-if="fieldType == 'zset'">
                    <n-space vertical>
                        <n-input v-model:value="fieldValue.zset.key" type="text" placeholder="Key" />
                        <n-space vertical>
                            <n-space v-for="(i, index) in fieldValue.zset.value">
                                <n-input-number style="width: 180px" v-model:value="i.score" placeholder="Score"
                                    size="small" />
                                <n-input v-model:value="i.value" type="text" placeholder="Value" size="small" />
                                <n-button strong secondary type="info" size="small" @click="handleZsetItem(index, 3)">
                                    <template #icon>
                                        <n-icon>
                                            <trash />
                                        </n-icon>
                                    </template>
                                </n-button>
                                <n-button strong secondary type="info" size="small"
                                    :disabled="index == fieldValue.zset.value.length - 1"
                                    @click="handleZsetItem(index, 1)">
                                    <template #icon>
                                        <n-icon>
                                            <arrow-down />
                                        </n-icon>
                                    </template>
                                </n-button>
                                <n-button strong secondary type="info" size="small" :disabled="index == 0"
                                    @click="handleZsetItem(index, 2)">
                                    <template #icon>
                                        <n-icon>
                                            <arrow-up />
                                        </n-icon>
                                    </template>
                                </n-button>
                            </n-space>
                            <n-button strong secondary type="info" size="small"
                                @click="fieldValue.zset.value.push({ score: 0, value: '' })">
                                Add
                            </n-button>
                        </n-space>
                        <n-input v-model:value="fieldValue.zset.ttl" type="text" placeholder="TTL" />
                    </n-space>
                </n-card>
                <n-card v-else-if="fieldType == 'hash'">
                    <n-space vertical>
                        <n-input v-model:value="fieldValue.hash.key" type="text" placeholder="Key" />
                        <n-space vertical>
                            <n-space v-for="(i, index) in fieldValue.hash.value">
                                <n-input style="width: 180px" v-model:value="i.key" type="text" placeholder="Key"
                                    size="small" />
                                <n-input v-model:value="i.value" type="text" placeholder="Value" size="small" />
                                <n-button strong secondary type="info" size="small" @click="handleHashItem(index, 3)">
                                    <template #icon>
                                        <n-icon>
                                            <trash />
                                        </n-icon>
                                    </template>
                                </n-button>
                                <n-button strong secondary type="info" size="small"
                                    :disabled="index == fieldValue.hash.value.length - 1"
                                    @click="handleHashItem(index, 1)">
                                    <template #icon>
                                        <n-icon>
                                            <arrow-down />
                                        </n-icon>
                                    </template>
                                </n-button>
                                <n-button strong secondary type="info" size="small" :disabled="index == 0"
                                    @click="handleHashItem(index, 2)">
                                    <template #icon>
                                        <n-icon>
                                            <arrow-up />
                                        </n-icon>
                                    </template>
                                </n-button>
                            </n-space>
                            <n-button strong secondary type="info" size="small"
                                @click="fieldValue.hash.value.push({ key: '', value: '' })">
                                Add
                            </n-button>
                        </n-space>
                        <n-input v-model:value="fieldValue.hash.ttl" type="text" placeholder="TTL" />
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
                    {{ lastReload }}
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
                            readonly placeholder="Key"></n-input>
                        <n-input v-model:value="detailValue" type="textarea"></n-input>
                    </n-space>
                </div>
                <div v-if="detailKeyType == 'list'">
                    <n-space vertical>
                        <n-input v-model:value="detailKey" :type="`${detailKey.length > 60 ? 'textarea' : 'text'}`"
                            readonly placeholder="Key"></n-input>
                        <n-table :bordered="true" :single-line="false" size="small">
                            <tbody>
                                <tr v-for="(v, index) in detailValue">
                                    <td class="list-index">{{ index }}</td>
                                    <td class="list-value">
                                        {{ v }}
                                    </td>
                                </tr>
                            </tbody>
                        </n-table>
                    </n-space>
                </div>
                <div v-if="detailKeyType == 'set'">
                    <n-space vertical>
                        <n-input v-model:value="detailKey" :type="`${detailKey.length > 60 ? 'textarea' : 'text'}`"
                            readonly placeholder="Key"></n-input>
                        <n-table :bordered="true" :single-line="false" size="small">
                            <tbody>
                                <tr v-for="v in detailValue">
                                    <td class="set-key">{{ v }}</td>
                                </tr>
                            </tbody>
                        </n-table>
                    </n-space>
                </div>
                <div v-if="detailKeyType == 'zset'">
                    <n-space vertical>
                        <n-input v-model:value="detailKey" :type="`${detailKey.length > 60 ? 'textarea' : 'text'}`"
                            readonly placeholder="Key"></n-input>
                        <n-table :bordered="true" :single-line="false" size="small">
                            <tbody>
                                <tr v-for="(v, k) in zsetToJson(detailValue)">
                                    <td class="zset-key">{{ k }}</td>
                                    <td class="zset-value">
                                        {{ v }}
                                    </td>
                                </tr>
                            </tbody>
                        </n-table>
                    </n-space>
                </div>
                <div v-if="detailKeyType == 'hash'">
                    <n-space vertical>
                        <n-input v-model:value="detailKey" :type="`${detailKey.length > 60 ? 'textarea' : 'text'}`"
                            readonly placeholder="Key"></n-input>
                        <n-table :bordered="true" :single-line="false" size="small">
                            <tbody>
                                <tr v-for="(v, k) in detailValue">
                                    <td class="hash-key">{{ k }}</td>
                                    <td class="hash-value">
                                        {{ v }}
                                    </td>
                                </tr>
                            </tbody>
                        </n-table>
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
