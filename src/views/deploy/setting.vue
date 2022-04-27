<template>
    <myDialog title="项目部署设置" ref="dialog" @ok="ok" @cancel="cancel" width="800px" btnWidth="48%">
        <el-tabs slot="content" type="border-card" v-model="profile" editable @edit="handleTabsEdit">
            <el-tab-pane v-for="(item,i) of commands" :key="item.profile" :name="item.profile" :label="item.profile">
                <el-form>
                    <el-form-item label="部署路径：(remote_dir)" prop="remote_dir">
                        <el-input v-model="item.remote_dir" size="small" autocomplete=”off”></el-input>
                    </el-form-item>
                    <el-form-item label="部署文件名称：(target_name)" prop="target_name">
                        <el-input v-model="item.target_name" size="small" autocomplete=”off”></el-input>
                    </el-form-item>
                    <el-form-item label="部署前命令：" prop="before">
                        <el-input v-model="item.before" type="textarea" :rows="6" placeholder="文件上传前执行命令，如项目编译、打包等！" autocomplete=”off”></el-input>
                    </el-form-item>
                    <el-form-item label="部署后命令：" prop="after">
                        <el-input v-model="item.after" type="textarea" :rows="6" placeholder="文件上传后执行命令，文件解压、服务重启等！" autocomplete=”off”></el-input>
                    </el-form-item>
                </el-form>
            </el-tab-pane>
        </el-tabs>
    </myDialog>
</template>
<script>

export default {
    name: 'deploySetting',
    data(){
        return {
            projectId: '',
            profile: '',
            commands: []
        }
    },
    methods: {
        getData(){
            this.invoke('DeploySetting', (rep)=>{
                this.commands = rep ? rep : []
                if (this.commands.length == 0){
                    this.commands.push({profile: 'test', remote_dir: '', target_name: '', before: '', after: ''})
                }
                if(rep && rep.length > 0){
                    this.profile = rep[0].profile
                }
            }, {id: this.projectId})
        },
        show(id){
            this.projectId = id
            this.getData()
            this.$refs.dialog.show()
        },
        handleTabsEdit(targetName, action){
            if(action === 'add'){
                this.Swal.fire({
                    input: 'text',
                    showCancelButton: true,
                    inputValidator: (value)=> {
                        return new Promise((resolve)=> {
                            if (!value){
                                resolve('请输入名称！')
                            } else {
                                if(this.commands.findIndex(it=> it.profile === value) >= 0){
                                    resolve(value + ' 已存在!')
                                } else {
                                    resolve()
                                }
                            }
                        })
                    }
                }).then(rep=>{
                    if(!rep.isConfirmed) return
                    this.commands.push({
                        profile: rep.value,
                        remote_dir: '',
                        target_name: '',
                        before: '',
                        after: ''
                    })
                    this.profile = rep.value
                })
            } else if (action === 'remove'){
                if (this.commands.length == 1){ return }
                let tabs = this.commands;
                let activeName = this.profile;
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
                this.profile = activeName;
                this.commands = tabs.filter(tab => tab.profile !== targetName);
            }
        },
        ok(){
            for(let item of this.commands) {
                if(!item.remote_dir || !item.target_name || !item.before || !item.after){
                    this.error(`配置 ${item.profile} 填写不完整！`)
                    return
                }
                item.project_id = this.projectId
            }
            this.invoke('SaveDeploySetting', ()=>{
                this.success('保存成功！')
                this.cancel()
                this.$refs.dialog.close()
            }, {project_id: this.projectId, command: this.commands})

        }, 
        cancel(){
            this.projectId = ''
            this.profile = ''
            this.commands = []
        }
    }
}
</script>
<style scoped>
.tab-title {
    display: inline-block;
}
.tab-title i {
    margin-left: 5px;
}
</style>