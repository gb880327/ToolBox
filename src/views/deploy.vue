<template>
    <span class="tips">Tips: 项目目录 - source_dir 部署路径 - remote_dir 部署文件名 - target_name</span>
    <el-tabs v-model="data.currentProfile" type="border-card" editable @edit="handleTabsEdit">
        <el-tab-pane v-for="(item, index) of data.commands" :key="index" :label="item.profile" :name="item.profile">
            <el-form :model="item" label-position="right" label-width="150px" size="default">
                <el-form-item label="是否需要上传文件：">
                    <el-switch v-model="item.need_upload" active-text="需要" inactive-text="不需要" :active-value="1" :inactive-value="0"></el-switch>
                    <span style="padding-left:20px;">(例如docker镜像部署不需要上传jar包)</span>                    
                </el-form-item>
                <el-form-item label="部署路径：" v-if="item.need_upload == 1">
                    <el-input v-model="item.remote_dir" autocomplete=”off” placeholder="remote_dir"></el-input>
                </el-form-item>
                <el-form-item label="部署文件名称：" v-if="item.need_upload == 1">
                    <el-input v-model="item.target_name" autocomplete=”off” placeholder="target_name"></el-input>
                </el-form-item>
                <el-form-item label="部署前命令：" prop="before">
                    <el-input v-model="item.before" type="textarea" :rows="6" placeholder="文件上传前执行命令，如项目编译、打包等！" autocomplete=”off”></el-input>
                </el-form-item>
                <el-form-item label="部署后命令：" prop="after">
                    <el-input v-model="item.after" type="textarea" :rows="6" placeholder="文件上传后执行命令，文件解压、服务重启等！" autocomplete=”off”></el-input>
                </el-form-item>
                <el-form-item>
                    <el-button type="primary" @click="submitForm(item)" class="submit">保存</el-button>
                </el-form-item>
            </el-form>
        </el-tab-pane>
    </el-tabs>
</template>
<script lang="ts" setup>
import { ElMessageBox } from 'element-plus'
import { reactive, inject } from 'vue'
import utils from '@/libs/utils'

const run = inject<Function>('invoke', ()=> {});

let data = reactive({
    currentProfile: '',
    commands: new Array<Profile>(),
    projectId: -1
})

const setData = (projectId)=> {
    data.projectId = projectId
    getData()
}
const getData =()=> {
    run('DeploySetting', (rep)=> {
        data.commands = rep ? rep : []
        if(data.commands.length > 0){
            data.currentProfile = data.commands[0].profile
        }        
    }, {id : data.projectId})
}
const handleTabsEdit = (targetName: string, action: 'remove' | 'add')=> {
    if(action === 'remove') {
        if (data.commands.length == 1){ return }
        utils.confirm('确认删除配置信息？', ()=> {
            let item = data.commands[data.commands.findIndex((it)=> it.profile === targetName)];
            run('RemoveCommand', (rep)=> {
                if(rep) {
                    let tabs = data.commands;
                    let activeName = data.currentProfile;
                    if (activeName === targetName) {
                        tabs.forEach((tab, index) => {
                            if (tab.profile === targetName) {
                                let nextTab = tabs[index + 1] || tabs[index - 1];
                                if (nextTab) {
                                    activeName = nextTab.profile;
                                }
                            }
                        })
                    }
                    data.commands = tabs.filter(tab => tab.profile !== targetName);
                    data.currentProfile = activeName;
                }
            }, {id: item['id']})
        })
    } else if (action === 'add') {
        ElMessageBox.prompt('请输入名称！', '新增配置', {confirmButtonText:'确定', cancelButtonText: '取消'}).then(({value})=>{
            data.commands.push({
                id: -1,
                project_id: data.projectId,
                profile: value,
                need_upload: 1,
                remote_dir:'',
                target_name: '',
                before: '',
                after: ''
            })
            data.currentProfile = value
        }).catch(()=>{})
    }
}
const submitForm = (item)=> {
    if((item.need_upload == 1 && (!item.remote_dir || !item.target_name || !item.before || !item.after)) || 
        (item.need_upload == 0 && (!item.before || !item.after))){
        utils.error(`配置 ${item.profile} 填写不完整！`)
        return
    }
    utils.loading()
    run('SaveDeploySetting', (rep)=> {
        if(rep) {
            utils.success('保存成功！')
        } else {
            utils.error('保存失败！')
        }
        utils.stopLoading()
    }, {command: item})
}

defineExpose({setData})
</script>
<style scoped>
.submit {
    width: 30%;
}
.tips {
    font-size: 14px;
    display: block;
    margin-bottom: 10px;
}
</style>
