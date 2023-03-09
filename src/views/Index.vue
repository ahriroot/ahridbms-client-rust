<script setup lang="ts">
import { ref, shallowRef, computed, onMounted } from 'vue'
import { useIndexStore } from '@/store'
import { NIcon, NButton, useMessage } from 'naive-ui'
import { Flash, Settings } from '@vicons/ionicons5'
import { invoke } from '@tauri-apps/api/tauri'
import { emit, listen } from '@tauri-apps/api/event'

import TreeWidget from '@/components/TreeWidget.vue'
import TabWidget from '@/components/TabWidget.vue'
import { uuid } from '@/utils/crypto'


const store = useIndexStore()
window.$message = useMessage()

const side = ref(0)
const oldWidth = ref(store.config.treeWidth)
const width = ref(store.config.treeWidth)
const sizePixel = computed(() => {
    return {
        sideLeft: side.value + 'px',
        sideWidth: side.value + 50 + 'px',
        treeLeft: side.value + 50 + 'px',
        treeWidth: width.value + 'px',
        drawLeft: width.value + 50 + 'px',
        drawWidth: '6px',
        tabLeft: width.value + 56 + 'px',
    }
})
const drawRef = shallowRef<HTMLElement | null>(null)
const resizeable = ref<boolean>(false)
const currentX = ref(0)
const handleDown = (e: MouseEvent) => {
    resizeable.value = true
    currentX.value = e.clientX
}
const handleUp = (e: MouseEvent) => {
    resizeable.value = false
    oldWidth.value = width.value
    store.updateConfig({
        ...store.config,
        treeWidth: width.value
    })
}
onMounted(async () => {
    document.body.addEventListener('mousemove', (ev) => {
        if (resizeable.value) {
            let m = oldWidth.value + ev.clientX - currentX.value
            if (m < 250) {
                width.value = 250
                return
            } else if (m > 1300) {
                width.value = 1300
                return
            } else {
                width.value = oldWidth.value + ev.clientX - currentX.value
            }
        }
    })
    document.body.addEventListener('mouseup', (ev) => {
        if (resizeable.value) {
            oldWidth.value = width.value
            resizeable.value = false
            store.updateConfig({
                ...store.config,
                treeWidth: width.value
            })
        }
    })

    setTimeout(async () => {
        await invoke('close_splashscreen')
    }, 1000)
})

const handleNewConnection = () => {
    let payload = {
        id: 1,
        event: 'create'
    }
    emit('tree-widget', JSON.stringify(payload))
}

const handleSetting = async () => {
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
    let payload = {
        id: 1,
        event: 'open',
        data: message
    }
    emit('tab-widget', JSON.stringify(payload))
}
</script>

<template>
    <div id="index">
        <div id="side" class="nocopy">
            <n-button circle quaternary size="small" @click.stop="handleNewConnection">
                <template #icon>
                    <n-icon>
                        <Flash />
                    </n-icon>
                </template>
            </n-button>
            <n-button circle quaternary size="small" @click.stop="handleSetting">
                <template #icon>
                    <n-icon class="btn-icon-setting">
                        <Settings />
                    </n-icon>
                </template>
            </n-button>
        </div>
        <div id="tree" ref="treeRef" class="nocopy">
            <TreeWidget />
        </div>
        <div id="draw" ref="drawRef" class="nocopy" @mousedown="handleDown($event)" @mouseup="handleUp($event)"></div>
        <div id="tab" ref="contentRef">
            <TabWidget />
        </div>
    </div>
</template>

<style scoped>
#index {
    position: absolute;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
}

#side {
    position: absolute;
    top: 0;
    left: v-bind('sizePixel.sideLeft');
    width: v-bind('sizePixel.sideWidth');
    bottom: 0;
    display: flex;
    flex-direction: column;
    justify-content: space-between;
    align-items: center;
    padding: 10px 0;
}

#tree {
    position: absolute;
    top: 0;
    left: v-bind('sizePixel.treeLeft');
    width: v-bind('sizePixel.treeWidth');
    bottom: 0;
}

#draw {
    position: absolute;
    top: 0;
    left: v-bind('sizePixel.drawLeft');
    width: v-bind('sizePixel.drawWidth');
    bottom: 0;
    cursor: ew-resize;
}

#tab {
    position: absolute;
    top: 0;
    left: v-bind('sizePixel.tabLeft');
    right: 0;
    bottom: 0;
}

.btn-icon-setting {
    transition: 0.3s;
}

.btn-icon-setting:hover {
    transform: rotate(90deg);
}
</style>
