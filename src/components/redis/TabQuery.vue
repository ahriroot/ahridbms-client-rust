<script setup lang="ts">
import { ref, shallowRef, onBeforeMount } from 'vue'
import { Connection } from '@/types/Connection'
import { RedisConnect } from '@/types/redis'
import EditorVue from '@/components/Editor.vue'
import { NButton, NCollapse, NCollapseItem, NLayout, NIcon, useMessage, LayoutSiderInst, NModal, NText, useDialog } from 'naive-ui'
import { exec } from '@/api/redis'
import { ExecType } from '@/data/redis/Data'
import { IExecResult, IExecValueBulk, IExecValueData, IExecValueError, IExecValueInteger, IExecValueStatus } from '@/types/redis/Data'
import { CaretForward, Copy, Trash, Code } from '@vicons/ionicons5'
import useClipboard from "vue-clipboard3"
import DetailJSONVue from './DetailJSON.vue'
import { IRedisQuery } from '@/types/data'
import { useIndexStore } from '@/store'

const props = defineProps<{
    conn: Connection<RedisConnect>
    db: string
}>()

const { toClipboard } = useClipboard()
const message = useMessage()
const dialog = useDialog()
const store = useIndexStore()

interface IResultList {
    datetime: number
    command: string,
    value: string[],
    data?: string | number
    error?: boolean
}

const u8array2string = (value: Uint8Array) => {
    let dataString = "";
    for (var i = 0; i < value.length; i++) {
        dataString += String.fromCharCode(value[i]);
    }
    return dataString
}

const editorRef = shallowRef<any>(null)
const resultRef = ref<LayoutSiderInst | null>(null)
const showDetail = ref(false)

const handle = async () => {
    if (editorRef.value) {
        // 获取选中的文本，需要剪切板权限
        // await editorRef.value?.getSelectedValue()
        // navigator.clipboard
        //     .readText()
        //     .then((v) => {
        //         console.log("获取选中值成功：", v);
        //     })
        let res = await editorRef.value?.getValue()
        // 命令不能以 // 杠开始
        let cmds = res.split('\n').filter((v: string) => v.trim() != '' && !v.trim().startsWith('//'))
        exec({ conn: props.conn, commandLines: cmds, db: '0' }).then(async res => {
            res.forEach((result: IExecResult) => {
                let datetime = new Date().getTime()
                switch (result.type_) {
                    case ExecType.nil:
                        config.value.resultList.unshift({
                            datetime: datetime,
                            command: result.command,
                            value: [`"(nil)"`],
                            data: `(nil)`
                        })
                        break
                    case ExecType.okay:
                        config.value.resultList.unshift({
                            datetime: datetime,
                            command: result.command,
                            value: [`"OK"`],
                            data: `OK`
                        })
                        break
                    case ExecType.data:
                        config.value.resultList.unshift({
                            datetime: datetime,
                            command: result.command,
                            value: [`"${u8array2string((result.value as IExecValueData).Data)}"`],
                            data: u8array2string((result.value as IExecValueData).Data)
                        })
                        break
                    case ExecType.status:
                        console.log((result.value as IExecValueStatus).Status)
                        break
                    case ExecType.integer:
                        let integer = result.value as IExecValueInteger
                        config.value.resultList.unshift({
                            datetime: datetime,
                            command: result.command,
                            value: [`(integer) ${integer.Integer}`],
                            data: integer.Integer
                        })
                        break
                    case ExecType.bulk:
                        let bulk = result.value as IExecValueBulk
                        let tmp: IResultList = {
                            datetime: datetime,
                            command: result.command,
                            value: [],
                            data: ''
                        }
                        bulk.Bulk.forEach((v, index) => {
                            tmp.value.push(`${index + 1}) "${u8array2string(v.Data)}"`)
                        })
                        config.value.resultList.unshift(tmp)
                        break
                    case ExecType.error:
                        config.value.resultList.unshift({
                            datetime: datetime,
                            command: result.command,
                            value: [`"${(result.value as IExecValueError).Error}"`],
                            data: '',
                            error: true
                        })
                        break
                    default:
                        break
                }
                resultRef.value?.scrollTo({ top: 0, behavior: 'smooth' })

                config.value.expanded.push(datetime)
                localStorage.setItem(`redis:query:${props.conn.id}`, JSON.stringify(config.value))
            })
        }).catch((err: any) => {
            console.log(err)
        })
    }
}

const handleDeleteAll = async () => {
    if (store.config?.deleteNoConfirm) {
        config.value.resultList = []
        config.value.expanded = []
        localStorage.setItem(`redis:query:${props.conn.id}`, JSON.stringify(config.value))
    } else {
        dialog.warning({
            title: '删除：',
            content: `确认删除所有操作日志 ?`,
            positiveText: '删除',
            onPositiveClick: async () => {
                config.value.resultList = []
                config.value.expanded = []
                localStorage.setItem(`redis:query:${props.conn.id}`, JSON.stringify(config.value))
            }
        })
    }
}

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

const handleReExec = (command: string) => {

}

const handleDelete = (datetime: number) => {
    if (store.config?.deleteNoConfirm) {
        config.value.resultList = config.value.resultList.filter(v => v.datetime != datetime)
        config.value.expanded = config.value.expanded.filter(v => v != datetime)
        localStorage.setItem(`redis:query:${props.conn.id}`, JSON.stringify(config.value))
    } else {
        dialog.warning({
            title: '删除：',
            content: `确认删除本条操作日志 ?`,
            positiveText: '删除',
            onPositiveClick: async () => {
                config.value.resultList = config.value.resultList.filter(v => v.datetime != datetime)
                config.value.expanded = config.value.expanded.filter(v => v != datetime)
                localStorage.setItem(`redis:query:${props.conn.id}`, JSON.stringify(config.value))
            }
        })
    }
}

const handleExpanded = (index: number[]) => {
    config.value.expanded = index
    localStorage.setItem(`redis:query:${props.conn.id}`, JSON.stringify(config.value))
}

const handleChange = (val: string) => {
    config.value.query = val
    localStorage.setItem(`redis:query:${props.conn.id}`, JSON.stringify(config.value))
}

const config = ref<IRedisQuery>({
    expanded: [],
    resultList: [],
    query: ''
})
onBeforeMount(() => {
    let cfg = localStorage.getItem(`redis:query:${props.conn.id}`)
    if (cfg) {
        config.value = JSON.parse(cfg)
    }
    config.value.expanded = config.value.expanded.filter(v => config.value.resultList.map(v => v.datetime).includes(v))
    localStorage.setItem(`redis:query:${props.conn.id}`, JSON.stringify(config.value))
})

const key = ref('')
const value = ref('')
const command = ref('')
const handleCode = async (val: any) => {
    command.value = val.command
    key.value = val.command.split(' ')[1]
    value.value = val.data
    showDetail.value = true
}
</script>
    
<template>
    <n-modal v-model:show="showDetail" preset="card" style="width: 600px;" :title="command" size="small"
        :bordered="false">
        <DetailJSONVue :k="key" :value="value" />
    </n-modal>
    <div class="query">
        <div class="command">
            <EditorVue ref="editorRef" @change="handleChange" :value="config.query" :type="'redis_query'" />
        </div>
        <div class="result">
            <div
                style="height: 40px; padding: 0 12px; display: flex; justify-content: flex-start; align-items: center;">
                <n-button strong secondary size="small" @click="handle">
                    <template #icon>
                        <n-icon>
                            <caret-forward />
                        </n-icon>
                    </template>
                </n-button>&nbsp;
                <n-button strong secondary size="small" @click="handleDeleteAll">
                    <template #icon>
                        <n-icon>
                            <trash />
                        </n-icon>
                    </template>
                </n-button>
            </div>
            <n-layout position="absolute" ref="resultRef" style="top: 36px; bottom: 40px; background: #282c34;"
                content-style="padding: 12px;" :native-scrollbar="false">
                <n-collapse display-directive="show" :expanded-names="config.expanded"
                    @update:expanded-names="handleExpanded">
                    <n-collapse-item v-for="i in config.resultList" :name="i.datetime">
                        <div class="item">
                            <n-text v-if="i.error" style="white-space: pre-line;" type="error" v-for="j in i.value">
                                {{ j }}
                            </n-text>
                            <span v-else style="white-space: pre-line;" v-for="j in i.value">
                                {{ j }}
                            </span>
                        </div>
                        <template #header>
                            <n-text v-if="i.error" type="error">
                                {{ i.command }}
                            </n-text>
                            <span v-else>
                                {{ i.command }}
                            </span>
                        </template>
                        <template #header-extra>
                            <n-button strong secondary size="small" @click.stop="handleCode(i)"
                                v-if="i.command.startsWith('JSON.GET')">
                                <template #icon>
                                    <n-icon>
                                        <Code />
                                    </n-icon>
                                </template>
                            </n-button>&nbsp;
                            <n-button strong secondary size="small" @click.stop="handleDelete(i.datetime)">
                                <template #icon>
                                    <n-icon>
                                        <trash />
                                    </n-icon>
                                </template>
                            </n-button>&nbsp;
                            <n-button strong secondary size="small" @click.stop="handleCopy(i.command)">
                                <template #icon>
                                    <n-icon>
                                        <copy />
                                    </n-icon>
                                </template>
                            </n-button>&nbsp;
                            <n-button strong secondary size="small" @click.stop="handleReExec(i.command)">
                                <template #icon>
                                    <n-icon>
                                        <caret-forward />
                                    </n-icon>
                                </template>
                            </n-button>
                        </template>
                    </n-collapse-item>
                </n-collapse>
            </n-layout>
        </div>
    </div>
</template>
    
<style scoped>
.query {
    position: absolute;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
}

.command {
    position: absolute;
    width: 50%;
    top: 0;
    left: 0;
    bottom: 0;
}

.result {
    position: absolute;
    top: 0;
    left: 50%;
    right: 0;
    bottom: 0;
}

.item {
    display: flex;
    flex-direction: column;
}

.n-collapse-item {
    border: 1px solid #333842;
    padding: 4px;
}
</style>
    