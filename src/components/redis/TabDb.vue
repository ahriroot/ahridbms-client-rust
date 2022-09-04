<script setup lang="ts">
import { h, shallowRef, ref, onBeforeMount, onMounted, computed, nextTick, watch } from 'vue'
import {
    NTable, NLayout, NTag, NButton, NIcon, NModal, SelectRenderLabel, useLoadingBar,
    NSpace, NCard, NSelect, NInput, useMessage, NSpin, NDropdown, NInputNumber, NTooltip
} from 'naive-ui'
import { Add, Reload, Trash, Key, Copy, ArrowUp, ArrowDown, Pencil, Checkmark, CaretBack, CaretForward, TimeOutline } from '@vicons/ionicons5'
import useClipboard from "vue-clipboard3"
import { Connection } from '@/types/Connection'
import { keys, setString, rpush, sadd, zadd, hmset, srem, del, get, expire, resetString, lset } from '@/api/redis'
import { INewFieldValue, Keyvalue, RedisConnect } from '@/types/redis'
import { diffDatetime } from '@/utils/datetime'
import { NewFieldValue } from '@/data/redis'
import EditorVue from '@/components/Editor.vue'
import DetailListVue from '@/components/redis/DetailList.vue'

window.$message = useMessage()

const props = defineProps<{
    conn: Connection<RedisConnect>
    db: string
}>()

const { toClipboard } = useClipboard()
const message = useMessage()
const loadingBar = useLoadingBar()

const result = ref<Keyvalue[]>([])  // 所有 keys
const showAdd = ref<boolean>(false)  // 显示新建 key 模态框

const showLoadingTable = ref<boolean>(false)  // table loading 动画
const loadingCount = ref<number>(0)  // loading 动画计数

const timer = ref<any>(null)  // 计时器
const before = ref<Date>(new Date())  // 上次刷新时间
const lastReload = ref<string>('less 1m')  // 距离上次刷新时间

const editorNewRef = shallowRef<any>(undefined)  // 新建 json 数据编辑器

const sidebarRef = shallowRef<HTMLElement | null>(null)  // 左边 html 元素
const resizeable = ref<boolean>(false)  // 正在调整大小
const width = ref(500)  // 左边实时宽度
const oldWidth = ref(500)  // 左边开始宽度
const cursor = ref('default')  // 默认鼠标显示
const currentMoveX = ref(0)  // 鼠标移动距离
const editorRef = shallowRef<any>(undefined)  // 显示 json 数据编辑器
onMounted(() => {
    if (sidebarRef.value) {
        sidebarRef.value.addEventListener('mousedown', (ev) => {
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
                if (tmp < 300) {
                    width.value = 300
                } else if (tmp > 1000) {
                    width.value = 1000
                } else {
                    width.value = tmp
                    localStorage.setItem('redis-sidebar-width', width.value.toString())
                }
            }
        })
        document.body.addEventListener('mouseup', (_) => {
            resizeable.value = false
        })
    }
})

const init = async () => {
    loadingStart()
    keys({ conn: props.conn, arg: '*', db: props.db }).then(async (res) => {
        result.value = []
        res.forEach(item => {
            for (const key in item) {
                result.value.push(item[key])
            }
        })
        before.value = new Date()
        lastReload.value = await diffDatetime(before.value)
    }).finally(() => {
        loadingFinish()
    })
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
    },
    {
        label: 'JSON',
        value: 'ReJSON-RL'
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
    'ReJSON-RL': {
        tag: '',
        text: 'JSON'
    },
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
    if (!detailKey.value) {
        return
    }
    get({ conn: props.conn, key: detailKey.value, db: props.db }).then((res: any) => {
        let tmp_key = 'String'
        for (const key in res) {
            tmp_key = key
        }
        detailTTL.value = res[tmp_key].ttl.toString()
        detailValue.value = res[tmp_key].value
        result.value.forEach(async item => {
            if (item.key == res[tmp_key].key) {
                if (item.key_type == 'ReJSON-RL') {
                    if (!editorRef.value) {
                        await nextTick()
                    }
                    editorRef.value?.setValue(res[tmp_key].value)
                }
                item.ttl = res[tmp_key].ttl
                item.size = res[tmp_key].size
                item.value = res[tmp_key].value
            }
        })
    }).finally(() => {
        loadingFinish()
    })
}

const handleRefresh = async () => {
    loadingStart()
    if (!detailKey.value) {
        return
    }
    expire({ conn: props.conn, key: detailKey.value, ttl: Number(detailTTL.value), db: props.db }).then(async (res: string) => {
        message.success(res)
        await handleReload()
    }).finally(() => {
        loadingFinish()
    })
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

/**
 * 确定添加数据
 */
const handleSubmitAdd = async () => {
    switch (fieldType.value) {
        case 'string':
            loadingStart()
            setString({ conn: props.conn, key: fieldValue.value.string.key, value: fieldValue.value.string.value, ttl: Number(fieldValue.value.string.ttl), db: props.db }).then(async res => {
                message.success(res)
                await handleReload()
            }).finally(() => {
                loadingFinish()
            })
            break
        case 'list':
            loadingStart()
            let list_values = fieldValue.value.list.value.filter(item => item.value).map(item => item.value)
            rpush({ conn: props.conn, key: fieldValue.value.list.key, value: list_values, ttl: Number(fieldValue.value.list.ttl), db: props.db }).then(async res => {
                message.success(`Success, ${res} items added`)
                await handleReload()
            }).finally(() => {
                loadingFinish()
            })
            break
        case 'set':
            loadingStart()
            let set_values = fieldValue.value.set.value.filter(item => item.value).map(item => item.value)
            sadd({ conn: props.conn, key: fieldValue.value.set.key, value: set_values, ttl: Number(fieldValue.value.set.ttl), db: props.db }).then(async res => {
                message.success(`Success, ${res} items added`)
                await handleReload()
            }).finally(() => {
                loadingFinish()
            })
            break
        case 'zset':
            loadingStart()
            zadd({ conn: props.conn, key: fieldValue.value.zset.key, value: fieldValue.value.zset.value, ttl: Number(fieldValue.value.zset.ttl), db: props.db }).then(async res => {
                message.success(`Success, ${res} items added`)
                await handleReload()
            }).finally(() => {
                loadingFinish()
            })
            break
        case 'hash':
            loadingStart()
            hmset({ conn: props.conn, key: fieldValue.value.hash.key, value: fieldValue.value.hash.value, ttl: Number(fieldValue.value.hash.ttl), db: props.db }).then(async res => {
                message.success(res)
                await handleReload()
            }).finally(() => {
                loadingFinish()
            })
            break
        case 'ReJSON-RL':
            if (!editorNewRef.value) {
                await nextTick()
            }
            console.log(await editorNewRef.value?.getValue())
            // let json_res = await setJson({ conn: props, key: fieldValue.value.json.key, value: fieldValue.value.json.value, ttl: Number(fieldValue.value.json.ttl) })
            // if (json_res == 'OK') {
            //     message.success('Success')
            //     await handleReload()
            // } else {
            //     console.log(json_res)
            //     message.error('Error')
            // }
            break
    }
    showAdd.value = false
}

const handleCancelAdd = async () => {
    showAdd.value = false
}

/**
 * 删除数据
 */
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
    del({ conn: props.conn, key: key, db: props.db }).then(async res => {
        message.success(res)
        await handleReload()
    }).finally(() => {
        loadingFinish()
    })
}

/**
 * 复制数据到剪切板
 * @param val 要复制的数据
 */
const handleCopy = async (val: any) => {
    if (val != null && val != undefined) {
        let str
        // 判断 val 是否为字符串
        if (typeof val === 'string') {
            str = val
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

const detailFilter = ref<string>('')  // 过滤显示 detailValue 内容
const detailKey = ref<string | null>(null)  // 当前显示数据的 key
const detailValue = ref<any>('')  // 当前显示数据的 value
const detailTTL = ref<string>('')  // 当前显示数据的 ttl
const detailKeyType = ref<string>('')  // 当前显示数据的 keyType
const zsetToJson = computed(() => {  // zset 转 [json]
    let res: any = []
    if (detailKeyType.value == 'zset') {
        let len = detailValue.value.length / 2
        for (let i = 0; i < len; i++) {
            res.push({
                score: detailValue.value[i],
                member: detailValue.value[len + i]
            })
        }
    }
    return res
})
const listToJson = computed(() => {  // hash 转 [json]
    let res: any = []
    if (detailKeyType.value == 'list') {
        detailValue.value.forEach((item: any, index: number) => {
            res.push({
                index: index,
                value: item
            })
        })
    }
    return res
})
const hashToJson = computed(() => {  // hash 转 [json]
    let res: any = []
    if (detailKeyType.value == 'hash') {
        for (let k in detailValue.value) {
            res.push({
                field: k,
                value: detailValue.value[k]
            })
        }
    }
    return res
})

/**
 * 显示详细数据
 */
const handleDetail = async (val: Keyvalue) => {
    detailFilter.value = ''
    detailTTL.value = val.ttl.toString()
    detailKey.value = val.key
    detailValue.value = val.value
    detailKeyType.value = val.key_type
    if (val.key_type == 'ReJSON-RL') {
        if (!editorRef.value) {
            await nextTick()
        }
        editorRef.value?.setValue(val.value)
    }
}

onBeforeMount(async () => {
    width.value = Number(localStorage.getItem('redis-sidebar-width')) || 500
    oldWidth.value = width.value
    await init()
    if (timer.value) {
        clearInterval(timer.value)
    }
    timer.value = setInterval(async () => {
        lastReload.value = await diffDatetime(before.value)
    }, 5000)
})

// 新建数据快速选择数据类型
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
    }, {
        label: 'JSON',
        key: 'ReJSON-RL'
    }
])

/**
 * 快速创建数据
 * @param val 数据类型
 */
const handleSelect = (val: string) => {
    fieldType.value = val
    showAdd.value = true  // 显示新建数据弹窗
}

watch(() => detailFilter.value, async () => {
    editListItem.value.index = -1
    editZsetItem.value.key = null
})

// 正在编辑的 list 中的数据
const editListItem = ref({
    index: -1,
    value: ''
})
const handleEditListItem = async () => {
    if(detailKey.value && editListItem.value.index >= 0){
        loadingStart()
        lset({
            conn: props.conn,
            key: detailKey.value,
            index: editListItem.value.index,
            value: editListItem.value.value,
            db: props.db
        }).then(async res => {
            message.success(res)
            await handleReloadKey()
            editListItem.value.index = -1
        }).finally(() => {
            loadingFinish()
        })
    }
}

// 正在编辑的 zset 中的数据
const editZsetItem = ref<{
    key: string | null
    value: string
}>({
    key: null,
    value: ''
})

// String Value Opera
const handleStringReset = async () => {
    if (detailKey.value) {
        loadingStart()
        resetString({ conn: props.conn, key: detailKey.value, value: detailValue.value, db: props.db }).then(async res => {
            message.success(res)
            await handleReloadKey()
        }).finally(() => {
            loadingFinish()
        })
    }
}

// Set Value Opera
/**
 * 删除 set 类型数据的 某一项
 * @param v 要删除的数据 
 */
const handleDeleteSetValue = async (v: string) => {
    if (detailKey.value) {
        loadingStart()
        srem({ conn: props.conn, key: detailKey.value, value: [v], db: props.db }).then(async res => {
            message.success(`Success, ${res} items deleted`)
            await handleReloadKey()
        }).finally(() => {
            loadingFinish()
        })
    }
}
</script>

<template>
    <n-modal v-model:show="showAdd" preset="card" style="width: 600px; height: 80vh" title="新建" size="small">
        <div style="height: 100%; position: relative;">
            <n-select :options="fieldTypeList" :render-label="renderLabel" v-model:value="fieldType" />
            <n-layout position="absolute" style="top: 50px; bottom: 50px; background: #2c2c32;"
                :native-scrollbar="false">
                <n-card v-show="fieldType == 'ReJSON-RL'">
                    <n-space vertical>
                        <n-input v-model:value="fieldValue.string.key" type="text" placeholder="Key" />
                        <n-input v-model:value="fieldValue.string.value" type="textarea" placeholder="Value" />
                        <EditorVue ref="editorNewRef" value="" type="json" />
                        <n-input v-model:value="fieldValue.string.ttl" type="text" placeholder="TTL" />
                    </n-space>
                </n-card>
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

    <div class="keys" :style="`cursor: ${resizeable ? 'ew-resize' : cursor}`">
        <div class="list" ref="sidebarRef" :style="`width: ${width}px`">
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
            <n-layout position="absolute" style="top: 36px;  color: #fff; height: 100%;" :native-scrollbar="false">
                <n-spin :show="showLoadingTable" class="copy">
                    <n-table :bordered="true" :single-line="false" size="small">
                        <tbody>
                            <tr v-for="i in result" style="cursor: pointer;" @click="handleDetail(i)">
                                <td class="td-key">
                                    <n-tag :bordered="false" :type="tagList[i.key_type].tag" size="small">
                                        {{ tagList[i.key_type].text }}
                                    </n-tag>
                                </td>
                                <td style="max-width: 200px; overflow: hidden;">{{ i.key }}</td>
                                <td class="td-size">{{ i.size }} B</td>
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
        <div class="content" :style="`left: ${width}px`" v-if="detailKey == null"></div>
        <div class="content" :style="`left: ${width}px`" v-else>
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
                                <time-outline />
                            </n-icon>
                        </template>
                    </n-button>
                    &nbsp;
                    <n-tooltip trigger="hover">
                        <template #trigger>
                            <n-button strong secondary circle type="info" size="small"
                                @click.stop="handleCopy(detailKey)">
                                <template #icon>
                                    <n-icon>
                                        <key />
                                    </n-icon>
                                </template>
                            </n-button>
                        </template>
                        复制 Key
                    </n-tooltip>
                    &nbsp;
                    <n-tooltip trigger="hover">
                        <template #trigger>
                            <n-button strong secondary circle type="info" size="small"
                                @click.stop="handleCopy(detailValue)">
                                <template #icon>
                                    <n-icon>
                                        <copy />
                                    </n-icon>
                                </template>
                            </n-button>
                        </template>
                        复制 Value
                    </n-tooltip>
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
            <div class="value" style="padding: 4px">
                <n-input v-model:value="detailKey" type="textarea" readonly placeholder="Key" :rows="3"></n-input>
                <div v-if="detailKeyType == 'string'" style="padding: 4px 0; display: flex; justify-content: flex-end;">
                    <n-tooltip trigger="hover">
                        <template #trigger>
                            <n-button strong secondary size="small" @click="handleStringReset">
                                <template #icon>
                                    <n-icon>
                                        <checkmark />
                                    </n-icon>
                                </template>
                            </n-button>
                        </template>
                        保存
                    </n-tooltip>
                    &nbsp;
                    <n-tooltip trigger="hover">
                        <template #trigger>
                            <n-button strong secondary size="small" @click="handleReloadKey">
                                <template #icon>
                                    <n-icon>
                                        <reload />
                                    </n-icon>
                                </template>
                            </n-button>
                        </template>
                        刷新
                    </n-tooltip>
                </div>
                <div v-else-if="detailKeyType == 'list'"
                    style="padding: 4px 0; display: flex; justify-content: flex-end;">
                    <n-input size="small" v-model:value="detailFilter" type="text" placeholder="Filter" />&nbsp;
                    <n-tooltip trigger="hover">
                        <template #trigger>
                            <n-button strong secondary size="small">
                                <template #icon>
                                    <n-icon>
                                        <caret-back />
                                    </n-icon>
                                </template>
                            </n-button>

                        </template>
                        LPOP
                    </n-tooltip>&nbsp;
                    <n-tooltip trigger="hover">
                        <template #trigger>
                            <n-button strong secondary size="small">
                                <template #icon>
                                    <n-icon>
                                        <caret-forward />
                                    </n-icon>
                                </template>
                            </n-button>

                        </template>
                        RPOP
                    </n-tooltip>&nbsp;
                    <n-input size="small" type="text" placeholder="Push" />&nbsp;
                    <n-tooltip trigger="hover">
                        <template #trigger>
                            <n-button strong secondary size="small">
                                <template #icon>
                                    <n-icon>
                                        <caret-forward />
                                    </n-icon>
                                </template>
                            </n-button>

                        </template>
                        LPUSH
                    </n-tooltip>&nbsp;
                    <n-tooltip trigger="hover">
                        <template #trigger>
                            <n-button strong secondary size="small">
                                <template #icon>
                                    <n-icon>
                                        <caret-back />
                                    </n-icon>
                                </template>
                            </n-button>

                        </template>
                        RPUSH
                    </n-tooltip>&nbsp;
                    <n-tooltip trigger="hover">
                        <template #trigger>
                            <n-button strong secondary size="small" @click="handleReloadKey">
                                <template #icon>
                                    <n-icon>
                                        <reload />
                                    </n-icon>
                                </template>
                            </n-button>
                        </template>
                        刷新
                    </n-tooltip>
                </div>
                <div v-else-if="detailKeyType == 'set'"
                    style="padding: 4px 0; display: flex; justify-content: flex-end;">
                    <n-input size="small" v-model:value="detailFilter" type="text" placeholder="Filter" />&nbsp;
                    <n-input size="small" type="text" placeholder="New Member" />&nbsp;
                    <n-button strong secondary size="small" @click="handleReloadKey">
                        <template #icon>
                            <n-icon>
                                <add />
                            </n-icon>
                        </template>
                    </n-button>&nbsp;
                    <n-tooltip trigger="hover">
                        <template #trigger>
                            <n-button strong secondary size="small" @click="handleReloadKey">
                                <template #icon>
                                    <n-icon>
                                        <reload />
                                    </n-icon>
                                </template>
                            </n-button>
                        </template>
                        刷新
                    </n-tooltip>
                </div>
                <div v-else-if="detailKeyType == 'zset'"
                    style="padding: 4px 0; display: flex; justify-content: flex-end;">
                    <n-input size="small" v-model:value="detailFilter" type="text" placeholder="Filter" />&nbsp;
                    <n-input size="small" type="text" placeholder="Memeber" />&nbsp;
                    <n-input size="small" type="text" placeholder="Score" />&nbsp;
                    <n-tooltip trigger="hover">
                        <template #trigger>
                            <n-button strong secondary size="small" @click="handleStringReset">
                                <template #icon>
                                    <n-icon>
                                        <checkmark />
                                    </n-icon>
                                </template>
                            </n-button>
                        </template>
                        保存
                    </n-tooltip>&nbsp;
                    <n-tooltip trigger="hover">
                        <template #trigger>
                            <n-button strong secondary size="small" @click="handleReloadKey">
                                <template #icon>
                                    <n-icon>
                                        <reload />
                                    </n-icon>
                                </template>
                            </n-button>
                        </template>
                        刷新
                    </n-tooltip>
                </div>
                <div v-else-if="detailKeyType == 'hash'"
                    style="padding: 4px 0; display: flex; justify-content: flex-end;">
                    <n-input size="small" v-model:value="detailFilter" type="text" placeholder="Filter" />&nbsp;
                    <n-button strong secondary size="small" @click="handleReloadKey">
                        <template #icon>
                            <n-icon>
                                <add />
                            </n-icon>
                        </template>
                    </n-button>&nbsp;
                    <n-tooltip trigger="hover">
                        <template #trigger>
                            <n-button strong secondary size="small" @click="handleReloadKey">
                                <template #icon>
                                    <n-icon>
                                        <reload />
                                    </n-icon>
                                </template>
                            </n-button>
                        </template>
                        刷新
                    </n-tooltip>
                </div>
                <div v-else-if="detailKeyType == 'ReJSON-RL'"
                    style="padding: 4px 0; display: flex; justify-content: flex-end;">
                    <n-tooltip trigger="hover">
                        <template #trigger>
                            <n-button strong secondary size="small" @click="handleStringReset">
                                <template #icon>
                                    <n-icon>
                                        <checkmark />
                                    </n-icon>
                                </template>
                            </n-button>
                        </template>
                        保存
                    </n-tooltip>&nbsp;
                    <n-tooltip trigger="hover">
                        <template #trigger>
                            <n-button strong secondary size="small" @click="handleReloadKey">
                                <template #icon>
                                    <n-icon>
                                        <reload />
                                    </n-icon>
                                </template>
                            </n-button>
                        </template>
                        刷新
                    </n-tooltip>
                </div>
                <div style="position: absolute; top: 120px; left: 4px; right: 4px; bottom: 36px;">
                    <n-layout position="absolute"
                        style="background: #282c34; top: 0; left: 0; right: 0; bottom: 0; color: #fff;"
                        :native-scrollbar="false" content-style="position: relative; top: 0; bottom: 0">
                        <div v-show="detailKeyType == 'ReJSON-RL'">
                            <EditorVue ref="editorRef" :value="detailValue" type="json" />
                        </div>
                        <div v-if="detailKeyType == 'string'" style="height: 100%">
                            <n-input style="height: 100%" v-model:value="detailValue" type="textarea"
                                :autosize="{ minRows: 10 }"></n-input>
                        </div>
                        <div v-else-if="detailKeyType == 'list'">
                            <n-table :bordered="true" :single-line="false" size="small">
                                <tbody>
                                    <tr
                                        v-for="i in listToJson.filter((item: any) => !detailFilter || item.index == parseInt(detailFilter))">
                                        <td class="list-index" style="width: 120px">{{ i.index }}</td>
                                        <td class="list-value">
                                            <div v-show="editListItem.index != i.index"
                                                @click="editListItem.index = i.index; editListItem.value = i.value">{{
                                                i.value }}
                                            </div>
                                            <n-input v-show="editListItem.index == i.index"
                                                v-model:value="editListItem.value" size="small"
                                                @keyup.enter.native="handleEditListItem"></n-input>
                                        </td>
                                        <td class="list-opera">
                                            <n-button v-show="editListItem.index != i.index" strong secondary circle
                                                type="info" size="small"
                                                @click="editListItem.index = i.index; editListItem.value = i.value">
                                                <template #icon>
                                                    <n-icon>
                                                        <pencil />
                                                    </n-icon>
                                                </template>
                                            </n-button>
                                            <n-button v-show="editListItem.index == i.index" strong secondary circle
                                                type="info" size="small" @click="handleEditListItem">
                                                <template #icon>
                                                    <n-icon>
                                                        <checkmark />
                                                    </n-icon>
                                                </template>
                                            </n-button>
                                        </td>
                                    </tr>
                                </tbody>
                            </n-table>
                        </div>
                        <div v-else-if="detailKeyType == 'set'">
                            <n-table :bordered="true" :single-line="false" size="small">
                                <tbody>
                                    <tr
                                        v-for="v in detailValue.filter((item: string) => !detailFilter || item.includes(detailFilter))">
                                        <td class="set-key">{{ v }}</td>
                                        <td class="set-opera">
                                            <n-button strong secondary circle type="error" size="small"
                                                @click="handleDeleteSetValue(v)">
                                                <template #icon>
                                                    <n-icon>
                                                        <trash />
                                                    </n-icon>
                                                </template>
                                            </n-button>
                                        </td>
                                    </tr>
                                </tbody>
                            </n-table>
                        </div>
                        <div v-else-if="detailKeyType == 'zset'">
                            <n-table :bordered="true" :single-line="false" size="small">
                                <tbody>
                                    <tr
                                        v-for="i in zsetToJson.filter((item: any) => !detailFilter || item.member.includes(detailFilter))">
                                        <td class="zset-member">{{ i.member }}</td>
                                        <td class="zset-score" style="width: 120px">
                                            <n-input v-show="editZsetItem.key == i.member"
                                                v-model:value="editZsetItem.value" size="small"></n-input>
                                            <div v-show="editZsetItem.key != i.member"
                                                @click="editZsetItem.key = i.member; editZsetItem.value = i.score">{{
                                                i.score
                                                }}
                                            </div>
                                        </td>
                                        <td class="zset-opera">
                                            <n-button v-show="editZsetItem.key != i.member" strong secondary circle
                                                type="info" size="small"
                                                @click="editZsetItem.key = i.member; editZsetItem.value = i.score">
                                                <template #icon>
                                                    <n-icon>
                                                        <pencil />
                                                    </n-icon>
                                                </template>
                                            </n-button>
                                            <n-button v-show="editZsetItem.key == i.member" strong secondary circle
                                                type="info" size="small" @click="editZsetItem.key = null">
                                                <template #icon>
                                                    <n-icon>
                                                        <checkmark />
                                                    </n-icon>
                                                </template>
                                            </n-button>
                                        </td>
                                    </tr>
                                </tbody>
                            </n-table>
                        </div>
                        <div v-else-if="detailKeyType == 'hash'">
                            <n-table :bordered="true" :single-line="false" size="small">
                                <tbody>
                                    <tr
                                        v-for="i in hashToJson.filter((item: any) => !detailFilter || item.field.includes(detailFilter))">
                                        <td class="hash-key">{{ i.field }}</td>
                                        <td class="hash-value">
                                            {{ i.value }}
                                        </td>
                                    </tr>
                                </tbody>
                            </n-table>
                        </div>
                    </n-layout>
                </div>
            </div>
        </div>
    </div>
</template>
<style scoped>
.keys {
    height: 100%;
    position: relative;
}

.keys .list {
    position: absolute;
    top: 0;
    left: 0;
    bottom: 0;
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
    position: absolute;
    top: 0;
    right: 0;
    bottom: 0;
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

.keys .content .value {
    position: absolute;
    top: 36px;
    left: 0;
    bottom: 0;
    width: 100%;
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

.set-opera,
.zset-opera,
.list-opera {
    width: 42px;
}
</style>
