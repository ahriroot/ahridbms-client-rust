<script setup lang="ts">
import { documents } from '@/api/mongodb'
import { Connection } from '@/types/Connection'
import { MongodbConnect } from '@/types/mongodb'
import { h, onBeforeMount, shallowRef, ref, reactive } from 'vue'
import EditorVue from '@/components/Editor.vue'
import { NSpace, NRadioGroup, NRadio, NDataTable, NButton, NIcon } from 'naive-ui'
import { Reload } from '@vicons/ionicons5'


const symbol = Symbol('mongodb')

const props = defineProps<{
    conn: Connection<MongodbConnect>
    data: any
}>()
const emits = defineEmits<{
    (e: 'handleCloseTab'): void
}>()

const pagination = reactive({
    page: 1,
    pageSize: 5,
    showSizePicker: true,
    itemCount: 0,
    showQuickJumper: true,
    pageSizes: [5, 10, 20, 50, 100, 500, 1000],
    onChange: async (page: number) => {
        pagination.page = page
        await handleLoadData()
    },
    onUpdatePageSize: async (pageSize: number) => {
        pagination.pageSize = pageSize
        pagination.page = 1
        await handleLoadData()
    }
})
const sorts = ref<{
    field: string
    order: string
}[]>([])

const handleChange = (val: string) => {
}
const config = ref({
    expanded: [],
    resultList: [],
    query: '',
    readonly: true
})
const history = ref<any[]>([])
const minWidth = ref(36)
const columns = ref<any[]>([])
const data = ref<any[]>([])
const loadingCount = ref(0)
const editorRef = shallowRef<any>(undefined)  // 显示 json 数据编辑器
const showType = ref('table')  // 显示类型
const showTypes = ref([
    {
        label: 'JSON',
        value: 'json'
    }, {
        label: 'TABLE',
        value: 'table'
    }
])

const analyData = (key: string, res: any) => {
    history.value = []
    data.value = []
    columns.value = []
    minWidth.value = 36
    let fields: string[] = []
    res.forEach((document: any) => {
        Object.keys(document).forEach((key: string) => {
            if (!fields.includes(key)) {
                fields.push(key)
                minWidth.value += 200
                columns.value.push({
                    title: key,
                    key: key,
                    sorter: {
                        multiple: 3
                    },
                    render(row: any, index: number) {
                        return h('div',
                            {},
                            {
                                default: () => {
                                    if (typeof row[key] === 'object') {
                                        if (Array.isArray(row[key])) {
                                            return [
                                                h(NButton, {
                                                    size: 'small',
                                                    secondary: true,
                                                    onClick: () => {
                                                        let tmp: any[] = []
                                                        row[key].forEach((item: any, index: number) => {
                                                            tmp.push({
                                                                [key]: item
                                                            })
                                                        })
                                                        analyData(key, tmp)
                                                    }
                                                }, {
                                                    default: () => {
                                                        return `(Array) ${row[key].length} Elements`
                                                    }
                                                })
                                            ]
                                        } else {
                                            let keys = Object.keys(row[key])
                                            if (keys.length == 1 && keys[0] === '$oid') {
                                                return row[key].$oid
                                            } else {
                                                return [
                                                    h(NButton, {
                                                        size: 'small',
                                                        secondary: true,
                                                        onClick: () => {
                                                            analyData(key, [row[key]])
                                                        }
                                                    }, {
                                                        default: () => {
                                                            return `(Document) ${keys.length} Fields`
                                                        }
                                                    })
                                                ]
                                            }
                                        }
                                    } else {
                                        if (row[key] == symbol) {
                                            return '(N/A)'
                                        } else {
                                            return row[key]
                                        }
                                    }
                                }
                            }
                        )
                    }
                })
            }
        })
    })
    res.forEach((document: any) => {
        let keys = Object.keys(document)
        let tmp: { [x: string]: any } = {}
        fields.forEach((field: string) => {
            if (keys.includes(field)) {
                tmp[field] = document[field]
            } else {
                tmp[field] = symbol
            }
        })
        data.value.push(tmp)
    })
    history.value.push({
        key: key,
        columns: columns.value,
        data: data.value
    })
}

const handleLoadData = async () => {
    const res = await documents({
        conn: props.conn,
        database: props.data.database,
        collection: props.data.collection,
        skip: 0,
        limit: 0,
        page: pagination.page,
        size: pagination.pageSize,
        sorts: sorts.value
    })
    editorRef.value?.setValue(JSON.stringify(res, null, 4))
    pagination.itemCount = res.count
    analyData('ROOT', res.documents)
}
const handleGoBack = async (key: number) => {
    let tmp = history.value.slice(0, key + 1)
    history.value = tmp
    columns.value = tmp[key].columns
    data.value = tmp[key].data
}
const handleUpdateSorter = async (sorter: any) => {
    sorter.value = sorter
    sorts.value = []
    sorter.forEach((item: any) => {
        if (item.order === "ascend" || item.order === "descend") {
            sorts.value.push({
                field: item.columnKey,
                order: item.order === "ascend" ? "ASC" : "DESC"
            })
        }
    })
    await handleLoadData()
}

onBeforeMount(async () => {
    await handleLoadData()
})
</script>

<template>
    <div class="data">
        <div class="option">
            <div>
                <span>
                    <n-button text size="small" @click="handleGoBack(0)">ROOT</n-button>
                </span>
                <span v-for="(i, index) in history" v-show="index > 0">
                    &gt; <n-button text size="small" @click="handleGoBack(index)">{{ i.key }}</n-button>
                </span>
            </div>
            <div class="right">
                <span>
                    <n-button strong secondary circle type="info" size="small" @click="handleLoadData">
                        <template #icon>
                            <n-icon>
                                <reload />
                            </n-icon>
                        </template>
                    </n-button>
                </span>
                <n-radio-group v-model:value="showType" name="radiogroup">
                    <n-space>
                        <n-radio v-for="showType in showTypes" :key="showType.value" :value="showType.value">
                            {{ showType.label }}
                        </n-radio>
                    </n-space>
                </n-radio-group>
            </div>
        </div>
        <div class="result">
            <EditorVue v-show="showType == 'json'" ref="editorRef" @change="handleChange" :value="config.query"
                :type="'json'" :read-only="true" />
            <div class="table-view">
                <n-data-table v-show="showType == 'table'" size="small" :single-line="false" :columns="columns" :data="data"
                    flex-height style="position: absolute; top: 0; left: 0; right: 0; bottom: 0;"
                    :loading="loadingCount > 0" :pagination="pagination" :remote="true" :scroll-x="minWidth"
                    @update:sorter="handleUpdateSorter" />
            </div>
        </div>
    </div>
</template>

<style scoped>
.n-data-table :deep(td) {
    margin: 0;
    padding: 2px 6px;
    background: none;
}

.data {
    position: absolute;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
}

.data .option {
    position: absolute;
    top: 0;
    left: 0;
    right: 0;
    height: 40px;
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding-left: 6px;
}

.data .option .right {
    width: 190px;
    display: flex;
    justify-content: space-between;
    align-items: center;
}

.data .result {
    position: absolute;
    top: 40px;
    left: 0;
    right: 0;
    bottom: 0;
}

.data .result .table-view {
    position: absolute;
    top: 0;
    left: 4px;
    right: 4px;
    bottom: 4px;
}
</style>
