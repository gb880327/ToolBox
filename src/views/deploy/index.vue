<template>
    <div class="content">
        <el-row>
            <el-col :span="6" class="border-right" :style="{height: height + 'px'}">
                <el-form label-width="100px" :style="{height: (height-40) + 'px', maxHeight: (height-40) + 'px'}" style="padding-right: 10px;">
                    <el-form-item label="部署项目：" class="item">
                        <div class="items">
                            <el-radio-group v-model="form.projectId" @change="projectChange">
                                <el-radio v-for="(item,i) of projects" :key="item.id" :label="item.id">{{item.name}}</el-radio>
                            </el-radio-group>
                        </div>
                    </el-form-item>
                    <el-divider></el-divider>
                    <el-form-item label="部署服务器：" class="item">
                        <div class="items">
                            <el-checkbox-group v-model="form.serverIds" size="small">
                                <el-checkbox v-for="(item,i) of servers" :key="item.id" :label="item.id">{{item.name}}</el-checkbox>
                            </el-checkbox-group>
                        </div>
                    </el-form-item>
                    <el-divider></el-divider>
                    <el-form-item label="部署环境：" class="item">
                        <div class="items">
                            <el-radio-group v-model="form.profile">
                                <el-radio v-for="(item,i) of profiles" :key="item.id" :label="item.id">{{item.profile}}</el-radio>
                            </el-radio-group>
                        </div>
                    </el-form-item>
                </el-form>

                <el-button type="success" round style="width: 70%;margin: 0 10%;" @click="exec" :disabled="$parent.isDeploy || !canRun">部署</el-button>
            </el-col>
            <el-col :span="18">
                <div class="console" :style="{height: (height - 20) + 'px'}">
                    <template v-for="(item, i) of $parent.logs">
                        <span style="margin: 2px 0; display: block;" v-if="item.type == 0">{{item.msg}}</span>
                        <span style="margin: 2px 0; display: block; color: #F56C6C;" v-else="item.type == 1">{{item.msg}}</span>
                    </template>
                    <el-progress v-if="$parent.showProgress" :percentage="$parent.percentage"></el-progress>
                </div>
            </el-col> 
        </el-row>
    </div>
</template>
<script>
export default {
    data(){
        return {
            height: window.innerHeight - 90,
            projects: [],
            servers: [],
            profiles: [],
            form: {
                projectId: '',
                profile: '',
                serverIds: []
            }
        }
    },
    computed: {
        canRun(){
            return this.form.projectId && this.form.profile && this.form.serverIds.length > 0
        }
    },
    created(){
        if(this.$route.query.id) {
            this.form.projectId = parseInt(this.$route.query.id)
            this.projectChange()
        }
        this.projectList()
        this.serverList()
    },
    methods: {
        projectList(){
            this.invoke('ProjectList', (data)=> {
                this.projects = data.records
            }, {pageNum: 1, pageSize: 10000})
        },
        serverList(){
            this.invoke('Servers', (data)=>{
                this.servers = data.records
            },{pageNum: 1, pageSize: 10000})
        },
        projectChange(){
            this.invoke('DeploySetting', (data)=>{
                this.form.profile = ''
                this.profiles = data
            }, {id: this.form.projectId})
        },
        exec(){
            if(!this.form.projectId){
                this.error('请选择需要部署的项目！')
                return
            }
            if(this.form.serverIds.length == 0){
                this.error('请选择需要部署的服务器！')
                return
            }
            if(!this.form.profile){
                this.error('请选择部署环境！')
                return
            }
            let param = {
                project: this.projects.find(it=> it.id === this.form.projectId),
                profile: this.profiles.find(it=> it.id === this.form.profile),
                servers: this.servers.filter(it=> this.form.serverIds.findIndex(i=> i === it.id) >= 0)
            }
            this.$parent.isDeploy = true
            this.invoke('Deploy', (data)=> {},{info: param})
        },
    }
}
</script>
<style scoped>
.item {
    overflow-x: scroll;
}
.border-right {
    border-right: 1px solid #dcdfe6;
}
.items {
    padding-left: 5px;
    max-height: 250px;
    overflow-y: scroll;
}
.el-radio {
    display: block !important;
    margin: 10px 0 !important;
}
.el-checkbox {
    display: block !important;
    line-height: 26px !important;
}
.console {
    margin-left: 10px;
    padding: 10px 10px;
    height: 100%;
    overflow-y: scroll;
    background-color: #171E29;
}
</style>