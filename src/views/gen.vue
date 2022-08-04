<template>
    <el-form :model="state" size="default" label-position="right" label-width="120px">
        <el-form-item label="数据源：" prop="datasource">
            <el-select v-model="state.datasource" placeholder="请选择数据源！">
                <el-option v-for="(item, i) of dataSources" :key="i" :label="item.name" :value="item.id"></el-option>
            </el-select>
        </el-form-item>
        <el-form-item label="文件目录：" prop="output">
            <el-input v-model="state.output">
                <template #append>
                    <el-button type="primary" plain @click="chooseHandler">选择...</el-button>    
                </template>
            </el-input>
        </el-form-item>
        <el-form-item label="模板：">
            <el-row style="width: 100%;">
                <el-col :span="6">模板</el-col>
                <el-col :span="8">目录 <help-tip /></el-col>
                <el-col :span="8">文件名</el-col>
            </el-row>
            <div style="max-height: 400px; overflow-y: scroll; width: 100%;">
                <el-row class="template-item" v-for="(item,i) of state.template" :key="i" :gutter="5">
                    <el-col :span="6">
                        <el-tree-select v-model="item.template_id" :data="templates" :render-after-expand="false" />
                    </el-col>
                    <el-col :span="8">
                        <el-input v-model="item.file_path"></el-input>
                    </el-col>
                    <el-col :span="8">
                        <el-input v-model="item.file_name"></el-input>
                    </el-col>
                    <el-col :span="2"><el-button type="danger" size="small" :icon="Delete" circle @click="removeTemplate(i)"></el-button></el-col>
                </el-row>
            </div>
            <el-button type="success" class="template-add dashed" size="small" plain :icon="Plus" @click="addTemplate"></el-button>
        </el-form-item>
        <el-form-item style="padding-left: 30%;">
            <el-button type="primary" @click="submitForm" style="width: 30%;">保存</el-button>
        </el-form-item>
    </el-form>
</template>
<script lang="ts" setup>
import { reactive, ref, inject } from 'vue'
import utils from '@/libs/utils'
import { Delete, Plus } from '@element-plus/icons-vue'
import helpTip from '@/components/help.vue'

interface TreeNode {
    value: number,
    label: string
}

const run = inject<Function>('invoke', ()=> {})

const dataSources = ref(new Array<DataSource>())
const templates = ref([{
    value: -1,
    label: '模板列表',
    children: new Array<TreeNode>()
}])
const state = reactive({
    id: -1,
    project_id: -1,
    datasource: '',
    output: '',
    template: new Array<TemplateItem>()
})
const getDs = ()=> {
    run('DataSources', (rep)=>{
        dataSources.value = rep   
    })
}
const getTemplates = ()=> {
    run('CategoryTree', (rep)=>{
        let temp = JSON.stringify(rep).replaceAll('"id"', '"value"')
        templates.value[0].children = JSON.parse(temp)
    })
}
getDs()
getTemplates()
const setData = (projectId)=> {
    state.project_id = projectId
    run('GenSetting', (rep)=> {
        if(rep){
            state.id = rep.id
            state.project_id = rep.project_id
            state.datasource = rep.datasource
            state.output = rep.output
            state.template = JSON.parse(rep.template)
        } else {
            state.datasource = ''
            state.output = ''
            state.template = []
        }
    }, {projectId: projectId})
}
const chooseHandler = async ()=> {
    let path:any = await utils.chooseDir()
    state.output = path || (state.output || '')
}
const removeTemplate = (index)=> {
    state.template.splice(index, 1);
}
const addTemplate = ()=> {
    state.template.push({template_id: -1, file_path: '', file_name: ''})
}
const submitForm = ()=> {
    if(!state.datasource){
        utils.error('请选择数据源！')
        return
    }
    if(!state.output){
        utils.error('请选择文件目录！')
        return
    }
    if(state.template.filter(it=> !it.template_id || !it.file_path || !it.file_name).length > 0){
        utils.error('请填写完整的模板配置信息！')
        return
    }
    let setting = {
        project_id: state.project_id,
        datasource: state.datasource,
        output: state.output,
        template: JSON.stringify(state.template)
    }
    if(state.id > 0) {
        setting['id'] = state.id
    }
    utils.loading()
    run('SaveGenSetting',(rep)=> {
        if(rep) {
            utils.success('保存成功！')
        } else {
            utils.error('保存失败！')
        }
        utils.stopLoading()
    }, {setting})
}
defineExpose({setData})
</script>
<style scoped>
.template-item {
    margin: 4px 0;
    width: 100%;
}
.template-add {
    margin-top: 10px;
    width: 90%;
}
</style>