<template>
    <el-dialog v-model="showModel" :title="title + ' - 项目部署'" width="90%" @close="clearData" :close-on-click-modal="false" :close-on-press-escape="false" :show-close="isOver" top="2vh">
        <div class="console" :style="{height: height + 'px'}">
            <template v-for="(item, i) of logs">
                <span style="margin: 2px 0; display: block;" v-if="item.type == 0">{{item.msg}}</span>
                <span style="margin: 2px 0; display: block; color: #F56C6C;" v-else="item.type == 1">{{item.msg}}</span>
            </template>
            <el-progress v-if="showProgress" :percentage="percentage"></el-progress>
        </div>
    </el-dialog>
</template>
<script lang="ts" setup>
import { ref, reactive, inject, nextTick } from 'vue'
import { Event, listen } from "@tauri-apps/api/event";


const run = inject<Function>('invoke', ()=> {})
const showModel = ref(false)
const isOver = ref(true)
const hasError = ref(false)
const showProgress = ref(false)
const percentage = ref(0)
const title = ref("")
const height = ref(window.innerHeight - 220)
const logs = reactive(new Array<MsgNode>())

listen("console", async (event: Event<string>)=> {
    if(event.payload === 'over') {
        isOver.value = true
        return
    }
    logs.push({msg: event.payload, type: 0})
    await nextTick()
    goToEnd()
})
listen("console_error", async (event: Event<string>)=> {
    hasError.value = true
    logs.push({msg: event.payload, type: 1})
    await nextTick()
    goToEnd()
})
listen("console_progress", async (event: Event<string>)=> {
    if(!showProgress.value){
        showProgress.value = true
    }
    percentage.value = parseInt(event.payload)
    if(percentage.value >= 100){
        percentage.value = 0
        showProgress.value = false
    }
    await nextTick()
    goToEnd()
})

const goToEnd = ()=> {
    let div = document.getElementsByClassName('console')[0]
    div.scrollTop = div.scrollHeight + 20
}
const show = (name, params)=> {
    showModel.value = true
    title.value = name
    isOver.value = false
    run('Deploy', (rep)=> {}, params)
}
const clearData = ()=> {
    logs.splice(0)
}
defineExpose({show})
</script>
<style scoped>
.console {
    margin-left: 10px;
    padding: 10px 10px;
    height: 100%;
    overflow-y: scroll;
}
</style>