<template>
    <el-dialog v-model="showModel" :title="title + ' - 代码生成'" width="90%" @close="clearData" :close-on-click-modal="false" :close-on-press-escape="false" :show-close="isOver" top="2vh">
        <el-button type="primary" @click="exec" size="small" style="margin-bottom:5px;" :disabled="!isOver">执行</el-button>
        <el-row :gutter="20">
            <el-col :span="6">
                <el-card class="box-card">
                    <template #header>
                        <el-checkbox v-model="allDbChecked" @change="allTableChange" label="数据表" size="large" style="display: inline !important;"></el-checkbox>
                    </template>
                    <el-checkbox-group v-model="state.tables" size="large" style="overflow-y: scroll;" :style="{maxHeight: height + 'px'}">
                        <el-checkbox v-for="(item,i) of tables" :key="i" :label="item.org_name">{{item.org_name}} - {{item.comment}}</el-checkbox>
                    </el-checkbox-group>
                </el-card>
            </el-col>
            <el-col :span="6">
                <el-card class="box-card">
                    <template #header>
                        <el-checkbox v-model="allTemplateChecked" @change="allTemplateChange" label="项目模板" size="large" style="display: inline !important;"></el-checkbox>
                    </template>
                    <el-checkbox-group v-model="state.templates" size="large" style="overflow-y: scroll;" :style="{maxHeight: height + 'px'}">
                        <el-checkbox v-for="(item,i) of templates" :key="i" :label="item.template_id">{{item.file_path}}/{{item.file_name}}</el-checkbox>
                    </el-checkbox-group>
                </el-card>
            </el-col>
            <el-col :span="12">
                <el-card class="box-card" header="日志">
                    <div class="console">
                        <template v-for="(item, i) of state.logs">
                            <span style="margin: 2px 0; display: block;" v-if="item.type == 0">{{item.msg}}</span>
                            <span style="margin: 2px 0; display: block; color: #F56C6C;" v-else="item.type == 1">{{item.msg}}</span>
                        </template>
                    </div>
                </el-card>
            </el-col>
        </el-row>
    </el-dialog>
</template>
<script lang="ts" setup>
import { ref, reactive, inject, nextTick } from 'vue'
import utils from '@/libs/utils'
import { Event, listen } from "@tauri-apps/api/event";

const height = ref(window.innerHeight - 100)
const run = inject<Function>('invoke', ()=> {})
const showModel = ref(false)
const title = ref("")
const isOver = ref(true)
const tables = ref(new Array<TableInfo>())
const templates = ref(new Array<TemplateItem>())
const allDbChecked = ref(false)
const allTemplateChecked = ref(false)

listen("console", async (event: Event<string>)=> {
    if(event.payload === 'over') {
        isOver.value = true
        state.logs.push({msg: "任务完成！", type: 0})
        return
    }
    state.logs.push({msg: event.payload, type: 0})
    await nextTick()
    goToEnd()
})
listen("console_error", async (event: Event<string>)=> {
    state.logs.push({msg: event.payload, type: 1})
    await nextTick()
    goToEnd()
})

const state = reactive({
    projectId: 0,
    tables: new Array<string>(),
    templates: new Array<number>(),
    logs: new Array<MsgNode>()
})
const show = (id, name)=> {
    showModel.value = true;
    title.value = name;
    state.projectId = id
    run('TableAndTemplate', (rep)=> {
        if(rep){
            tables.value = rep.table
            templates.value = rep.template
        }
    }, { projectId: id })
}
const allTableChange = (val)=> {
    state.tables = []
    if(val){
        tables.value?.forEach(it=> state.tables.push(it.org_name))
    }
}
const allTemplateChange = (val)=> {
    state.templates = []
    if(val){
        templates.value?.forEach(it=> state.templates.push(it.template_id))
    }
}
const clearData = ()=> {
    state.projectId = 0
    state.tables = []
    state.templates = []
    state.logs = []
}
const exec = ()=> {
    let table = tables.value?.filter(it=> state.tables.findIndex(i=> i === it.org_name) >= 0)
    if(table.length == 0) {
        utils.error('请选择数据表！')
        return
    }
    let template = templates.value?.filter(it=> state.templates.findIndex(i=> i === it.template_id) >= 0)
    if(template.length == 0) {
        utils.error('请选择模板！')
        return
    }
    isOver.value = false
    let data = { project_id: state.projectId, tables: table, templates: template }
    run('GenTemplate', (rep)=> {}, {genInfo: data})
}
const goToEnd = ()=> {
    let div = document.getElementsByClassName('console')[0]
    div.scrollTop = div.scrollHeight + 20
}
defineExpose({show})
</script>

<style scoped>
.box-card {
    height: 100%;
}
.el-checkbox-group label {
    display: block !important;
}
.console {
    padding: 10px 0;
    max-height: 400px;
    overflow: scroll;
}
.console span {
    display: block;
    margin: 5px 0;
}
</style>