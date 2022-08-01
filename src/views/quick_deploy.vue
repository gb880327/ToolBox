<template>
    <div class="inline-tool">
        <el-button type="primary" @click="addQuick" size="small">新增</el-button>
        <el-button type="danger" size="small" @click="removeQuick" :disabled="state.form.id < 0">删除</el-button>
        <el-button type="success" size="small" :disabled="state.form.id < 0">部署</el-button>
    </div>
    <el-row :gutter="20" class="custom-row">
        <el-col :span="4" class="border-right custom-col">
            <el-scrollbar class="custom-col">
                <el-tree
                    ref="treeRef"
                    :data="state.treeData"
                    :props="state.treeProps"
                    node-key="id"
                    :default-expand-all="true"
                    :expand-on-click-node="false"
                    :highlight-current="true"
                    @node-click="treeNodeClick"
                ></el-tree>
            </el-scrollbar>
        </el-col>
        <el-col :span="10" class="custom-col">
            <el-form ref="formRef" :model="state.form" :rules="state.rules" label-width="120px" size="default">
                <el-form-item label="名称：" prop="name">
                    <el-input v-model="state.form.name" placeholder="请输入名称！"></el-input>
                </el-form-item>
                <el-form-item label="项目：" prop="project">
                    <el-select v-model="state.form.project" @change="projectChange">
                        <el-option v-for="(item, index) of state.projects" :key="index" :label="item.name" :value="item.id"></el-option>
                    </el-select>
                </el-form-item>
                <el-form-item label="部署环境：" prop="profile">
                    <el-select v-model="state.form.profile">
                        <el-option v-for="(item, i) of state.profiles" :key="i" :label="item.profile" :value="item.id"></el-option>
                    </el-select>
                </el-form-item>
                <el-form-item label="部署服务器：" prop="server">
                    <el-select v-model="state.form.server" multiple style="width: 100%;"> 
                        <el-option v-for="(item, index) of state.servers" :key="index" :label="item.name + (item.label ? (' - ' + item.label) : '')" :value="item.id"></el-option>
                    </el-select>
                </el-form-item>
                <el-form-item>
                    <el-button type="primary" @click="submitForm(formRef)" style="width: 30%;">保存</el-button>
                </el-form-item>
            </el-form>
        </el-col>
    </el-row>
</template>
<script lang="ts" setup>
import { ref, reactive, inject, watch, nextTick } from 'vue'
import type { FormInstance } from 'element-plus'
import utils from '@/libs/utils'

interface TreeNode {
    id: number,
    label: string,
    project: number,
    profile: number,
    server: string
}

const treeRef = ref()
const run = inject<Function>('invoke', ()=> {})
const formRef = ref<FormInstance>()
let state = reactive({
    treeData: [{
        id: -1,
        label: "配置列表",
        children: new Array<TreeNode>()
    }],
    treeProps: {
        children: "children",
        label: "label",
        class: "custom-node"
    },
    form: {
        id: -1,
        name: '',
        project: '',
        profile: '',
        server: []
    },
    rules: {
        name: [{ required: true, message: "请输入名称", trigger: "blur" }],
        project: [{ required: true, message: "请选择项目", trigger: "blur" }],
        profile: [{ required: true, message: "请选择部署环境", trigger: "blur" }],
        server: [{ required: true, message: "请选择部署服务器", trigger: "blur" }]
    },
    projects: new Array<Project>(),
    profiles: new Array<Profile>(),
    servers: new Array<Server>()
})

const getServers = ()=> {
    run('Servers', (rep)=> {
        state.servers = rep
    })
}
const getProjects = ()=> {
    run('ProjectList', (rep)=> {
        state.projects = rep
    })
}
getServers()
getProjects()

const getData = ()=> {
    run('QuickDeploys', (rep)=> {
        if(rep){
            state.treeData[0].children = new Array<TreeNode>()
            rep.forEach(item=> {
                state.treeData[0].children.push({id: item.id, label:item.name, project: item.project, profile:item.profile, server: item.server})
            })
        }
    })
}
getData()
const projectChange = (val)=> {
    run('DeploySetting', (rep)=> {
        state.profiles = rep
        if(rep && rep.length > 0) {
            if(!state.form.profile) {
                state.form.profile = rep[0].id
            }
        }
    }, {id : val})
}

const addQuick = ()=> {
    formRef.value?.resetFields()
    treeRef.value?.setCurrentKey(null)
    state.form = { id: -1, name: '', project: '', profile: '', server: [] }
}
const treeNodeClick = async (data: any)=> {
    state.form.id = data.id
    state.form.name = data.label
    state.form.project = data.project
    state.form.profile = data.profile
    projectChange(data.project)
    state.form.server = data.server ? data.server.split('|').map(it=> parseInt(it)) : []
}
const submitForm = async (formEl)=> {
    if (!formEl) return
    await formEl.validate((valid, fields) => {
        if (valid) {
            let params = JSON.parse(JSON.stringify(state.form))
            if(params.id == -1){
                delete params.id
            }
            params.server = params.server.join('|')
            utils.loading()
            run('SaveQuickDeploy', (rep)=>{
                if(rep){
                    utils.success('保存成功！')
                    addQuick()
                    getData()
                } else {
                    utils.error('保存失败！')
                }
                utils.stopLoading()
            }, { quick: params})
        }
    })
}
const removeQuick = ()=> {
    utils.confirm('确定删除该配置信息？', ()=> {
        run('RemoveQuickDeply', (rep)=> {
            if(rep){
                utils.success('删除成功！')
                getData()
                addQuick()
            } else {
                utils.error('删除失败！')
            }
        }, {id: state.form.id})
    })
}
</script>