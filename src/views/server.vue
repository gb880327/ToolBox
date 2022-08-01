<template>
    <div class="tool">
        <el-button type="primary" @click="addServer" size="small">新增</el-button>
        <el-button type="danger" size="small" :disabled="state.form.id < 0" @click="removeHandler">删除</el-button>
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
        <el-col :span="20" class="custom-col">
            <el-form :model="state.form" :rules="state.rules" ref="serverForm" size="default" label-width="150px">
                <el-form-item label="服务器名称：" prop="name">
                    <el-input v-model="state.form.name"></el-input>
                </el-form-item>
                <el-form-item label="服务器标签：" prop="label">
                    <el-input v-model="state.form.label"></el-input>
                </el-form-item>
                <el-form-item label="服务器地址：" prop="host">
                    <el-input v-model="state.form.host"></el-input>
                </el-form-item>
                <el-form-item label="服务器端口：" prop="port">
                    <el-input-number v-model="state.form.port"></el-input-number>
                </el-form-item>
                <el-form-item label="用户名：" prop="user">
                    <el-input v-model="state.form.user"></el-input>
                </el-form-item>
                <el-form-item label="认证类型：">
                    <el-select v-model="state.form.auth_type" >
                        <el-option :value="0" label="密码"></el-option>
                        <el-option :value="1" label="秘钥登陆"></el-option>
                    </el-select>
                </el-form-item>
                <el-form-item ref="password" label="密码：" v-if="state.form.auth_type === 0" prop="password">
                    <el-input type="password" v-model="state.form.password" :show-password="true"></el-input>
                </el-form-item>
                <el-form-item ref="privateKey" label="秘钥路径：" v-if="state.form.auth_type === 1" prop="private_key">
                    <el-input v-model="state.form.private_key">
                        <template #append>
                            <el-button @click="chooseHandler">选择...</el-button>    
                        </template>
                    </el-input>
                </el-form-item>
                <el-form-item label="登陆后执行命令：" prop="command">
                    <el-input v-model="state.form.command"></el-input>
                </el-form-item>

                <el-form-item>
                    <el-button type="primary" @click="submitForm(serverForm)" style="width: 30%;">保存</el-button>
                </el-form-item>
            </el-form>
        </el-col>
    </el-row>
</template>
<script lang="ts" setup>
import { ref, reactive, inject } from 'vue'
import type { FormInstance } from 'element-plus'
import utils from '@/libs/utils'

interface TreeNode {
    id: number,
    label: string
}
const treeRef = ref()
const run = inject<Function>('invoke', ()=> {})

const serverForm = ref<FormInstance>()
const state = reactive({
    form: { 
        id: -1,
        name: '',
        label: '',
        host: '',
        port: 22,
        user: '',
        auth_type: 0,
        password: '',
        private_key: '',
        command: ''
    },
    treeData: [{id: -1,
        label: "服务器列表",
        children: new Array<TreeNode>()
    }],
    treeProps: {
        children: "children",
        label: "label",
        class: "custom-node",
    },
    rules: {
        name: [{ required: true, message: "请输入服务器名称", trigger: "blur" }],
        host: [{ required: true, message: "请输入服务器地址", trigger: "blur" }],
        port: [{ required: true, message: "请输入服务器端口", trigger: "blur" }],
        user: [{ required: true, message: "请输入用户名", trigger: "blur" }],
        password: [{ required: true, message: "请输入密码", trigger: "blur" }],
        private_key: [{ required: true, message: "请输入秘钥路径", trigger: "blur" }]
    },
    servers: []
})
const getData = ()=> {
    run('Servers', (rep)=> {
        state.servers = rep
        state.treeData[0].children = new Array<TreeNode>()
        rep.forEach(it=> {
            state.treeData[0].children.push({id: it.id, label: `${it.name}` + (it.label ? `(${it.label})` : '')})
        })
    })
}

getData()

const treeNodeClick = (data: any)=> {
    serverForm.value?.resetFields()
    state.form.name = data.name
    if(data.id > 0){
        let item: any = state.servers.find(it=> it['id'] === data.id)
        state.form = item
    }
}
const addServer = ()=> {
    state.form = { id: -1, name: '', label: '', host: '', port: 22, user: '', auth_type: 0, password: '', private_key: '', command: '' }
    serverForm.value?.resetFields()
    treeRef.value?.setCurrentKey(null)
}
const chooseHandler = async ()=> {
    let path = await utils.chooseFile()
    state.form.private_key = path ? path.toString() : (state.form.private_key ? state.form.private_key : '')
}
const submitForm = async (formEl: FormInstance | undefined)=> {
    if (!formEl) return
    await formEl.validate((valid, fields) => {
        if (valid) {
            utils.loading()
            let params = JSON.parse(JSON.stringify(state.form))
            if(params.id == -1){
                delete params.id
            }
            run('SaveServer', (rep)=> {
                if(rep){
                    utils.success('保存成功！')
                    getData()
                    addServer()
                } else (
                    utils.error('保存失败！')
                )
                utils.stopLoading()
            }, {server: params})
        }
    })
}
const removeHandler = ()=> {
    utils.confirm('确认是否删除该服务器配置？', ()=> {
        utils.loading()
        run('RemoveServer', (rep)=> {
            if(rep){
                utils.success('删除成功！')
                getData()
                addServer()
            } else {
                utils.error('删除失败！')
            }
            utils.stopLoading()
        }, {id: state.form.id})
    })
}
</script>
<style >

</style>