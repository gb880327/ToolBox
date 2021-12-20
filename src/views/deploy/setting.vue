<template>
    <myDialog title="项目部署设置" ref="dialog" @ok="ok" @cancel="cancel" width="800px" btnWidth="48%">
        <el-tabs slot="content" type="border-card" v-model="profile" editable @edit="handleTabsEdit">
            <el-tab-pane v-for="(item,i) of commands" :key="item.profile" :name="item.profile" :label="item.profile">
                <el-form>
                    <el-form-item label="部署路径：(remote_dir)" prop="remote_dir">
                        <el-input v-model="item.remote_dir" size="small"></el-input>
                    </el-form-item>
                    <el-form-item label="部署文件名称：(target_name)" prop="target_name">
                        <el-input v-model="item.target_name" size="small"></el-input>
                    </el-form-item>
                    <el-form-item label="部署前命令：" prop="before">
                        <el-input v-model="item.before" type="textarea" :rows="6" placeholder="文件上传前执行命令，如项目编译、打包等！"></el-input>
                    </el-form-item>
                    <el-form-item label="部署后命令：" prop="after">
                        <el-input v-model="item.after" type="textarea" :rows="6" placeholder="文件上传后执行命令，文件解压、服务重启等！"></el-input>
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
            profile: 'test',
            commands: [{
                profile: 'test',
                remote_dir: '',
                target_name: '',
                before: '',
                after: ''
            }]
        }
    },
    methods: {
        show(){
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

        }, 
        cancel(){

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