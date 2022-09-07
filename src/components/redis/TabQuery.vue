<script setup lang="ts">
import { ref, shallowRef, onBeforeMount } from 'vue'
import { Connection } from '@/types/Connection'
import { RedisConnect } from '@/types/redis'
import EditorVue from '@/components/Editor.vue'
import { NButton, NCollapse, NCollapseItem, NLayout, NIcon, useMessage, LayoutSiderInst } from 'naive-ui'
import { exec } from '@/api/redis'
import { ExecType } from '@/data/redis/Data'
import { IExecResult, IExecValueBulk, IExecValueData, IExecValueError, IExecValueInteger, IExecValueStatus } from '@/types/redis/Data'
import { CaretForward, Copy, Trash } from '@vicons/ionicons5'
import useClipboard from "vue-clipboard3"

const props = defineProps<{
    conn: Connection<RedisConnect>
    db: string
}>()
const emits = defineEmits<{
    (e: 'handle', val: null): void
}>()

const { toClipboard } = useClipboard()
const message = useMessage()

interface IResultList {
    datetime: number
    command: string,
    value: string[]
}

const u8array2string = (value: Uint8Array) => {
    let dataString = "";
    for (var i = 0; i < value.length; i++) {
        dataString += String.fromCharCode(value[i]);
    }
    return dataString
}

const defaultQuery = ref(``)
const resultList = ref<IResultList[]>([])
const editorRef = shallowRef<any>(null)
const expanded = ref<number[]>([])
const resultRef = ref<LayoutSiderInst | null>(null)

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
                        resultList.value.unshift({
                            datetime: datetime,
                            command: result.command,
                            value: [`"(nil)"`]
                        })
                        break
                    case ExecType.okay:
                        resultList.value.unshift({
                            datetime: datetime,
                            command: result.command,
                            value: [`"OK"`]
                        })
                        break
                    case ExecType.data:
                        resultList.value.unshift({
                            datetime: datetime,
                            command: result.command,
                            value: [`"${u8array2string((result.value as IExecValueData).Data)}"`]
                        })
                        break
                    case ExecType.status:
                        console.log((result.value as IExecValueStatus).Status)
                        break
                    case ExecType.integer:
                        let integer = result.value as IExecValueInteger
                        resultList.value.unshift({
                            datetime: datetime,
                            command: result.command,
                            value: [`(integer) ${integer.Integer}`]
                        })
                        break
                    case ExecType.bulk:
                        let bulk = result.value as IExecValueBulk
                        let tmp: IResultList = {
                            datetime: datetime,
                            command: result.command,
                            value: []
                        }
                        bulk.Bulk.forEach((v, index) => {
                            tmp.value.push(`${index + 1}) "${u8array2string(v.Data)}"`)
                        })
                        resultList.value.unshift(tmp)
                        break
                    case ExecType.error:
                        console.log((result.value as IExecValueError).Error)
                        break
                    default:
                        break
                }
                resultRef.value?.scrollTo({ top: 0, behavior: 'smooth' })

                expanded.value.push(datetime)
                localStorage.setItem(`redis:expanded:${props.conn.id}`, JSON.stringify(expanded.value))
                localStorage.setItem(`redis:result:${props.conn.id}`, JSON.stringify(resultList.value))
            })
        }).catch((err: any) => {
            console.log(err)
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
    resultList.value = resultList.value.filter(v => v.datetime != datetime)
    expanded.value = expanded.value.filter(v => v != datetime)
    localStorage.setItem(`redis:expanded:${props.conn.id}`, JSON.stringify(expanded.value))
    localStorage.setItem(`redis:result:${props.conn.id}`, JSON.stringify(resultList.value))
}

const handleExpanded = (index: number[]) => {
    expanded.value = index
    localStorage.setItem(`redis:expanded:${props.conn.id}`, JSON.stringify(expanded.value))
}

const handleChange = (val: string) => {
    localStorage.setItem(`redis:query:${props.conn.id}`, val)
}

onBeforeMount(() => {
    if (localStorage.getItem(`redis:query:${props.conn.id}`)) {
        let query = localStorage.getItem(`redis:query:${props.conn.id}`)
        if (query) {
            defaultQuery.value = query || ''
        }
    }
    let resultStr = localStorage.getItem(`redis:result:${props.conn.id}`)
    resultList.value = resultStr ? JSON.parse(resultStr) : []
    let expandedStr = localStorage.getItem(`redis:expanded:${props.conn.id}`)
    let resultIds = resultList.value.map(v => v.datetime)
    let expandedArr: number[] = []
    if (expandedStr) {
        expandedArr = JSON.parse(expandedStr)
        expandedArr = expandedArr.filter(v => resultIds.includes(v))
    }
    expanded.value = expandedArr
})

</script>
    
<template>
    <div class="query">
        <div class="command">
            <EditorVue ref="editorRef" @change="handleChange" :value="defaultQuery" :type="'redis_query'" />
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
                </n-button>
            </div>
            <n-layout position="absolute" ref="resultRef" style="top: 36px; bottom: 40px; background: #282c34;"
                content-style="padding: 12px;" :native-scrollbar="false">
                <n-collapse display-directive="show" :expanded-names="expanded" @update:expanded-names="handleExpanded">
                    <n-collapse-item v-for="i in resultList" :name="i.datetime">
                        <div class="item">
                            <span style="white-space: pre-line;" v-for="j in i.value">
                                {{ j }}
                            </span>
                        </div>
                        <template #header>
                            {{ i.command }}
                        </template>
                        <template #header-extra>
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
    