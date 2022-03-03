<template>
    <div class="content">
        <el-row>
            <el-col :span="6" class="border-right" :style="{height: height + 'px'}">
                <div :style="{height: leftHeight + 'px', maxHeight: leftHeight + 'px'}" style="padding-right: 10px;">
                    <div class="plane">
                        <span class="title">部署项目</span>
                        <el-select v-model="form.projectId" placeholder="请选择" style="width: 100%;" @change="projectChange">
                            <el-option v-for="(item,i) of projects" :key="item.id" :value="item.id" :label="item.name"></el-option>
                        </el-select>
                    </div>
                    <el-divider></el-divider>
                    <div class="plane">
                        <span class="title">部署服务器</span>
                        <el-select v-model="form.serverIds" multiple placeholder="请选择" style="width: 100%;">
                            <el-option v-for="(item,i) of servers" :key="item.id" :value="item.id" :label="item.name"></el-option>
                        </el-select>
                    </div>
                    <el-divider></el-divider>
                    <div class="plane">
                        <span class="title">部署环境</span>
                        <el-select v-model="form.profile" placeholder="请选择" style="width: 100%;">
                            <el-option v-for="(item,i) of profiles" :key="item.id" :value="item.id" :label="item.profile"></el-option>
                        </el-select>
                    </div>
                </div>
                <el-button type="success" round style="width: 80%;margin: 0 10%;" @click="exec" :disabled="$parent.isDeploy || !canRun">部署</el-button>
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
            leftHeight: window.innerHeight - 150,
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
.title {
    color: #FFFFFF;
    font-size: 14px;
    display: block;
    border-bottom: 1px solid black;
    padding-bottom: 10px;
    margin-bottom: 15px;
}
.plane {
    overflow-x: scroll;
    margin: 15px 0;
}
.border-right {
    border-right: 1px solid #dcdfe6;
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