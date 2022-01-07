<template>
    <div class="content">
        <el-row>
            <el-col :span="4" class="border-right" :style="{height: height + 'px'}">
                <ul class="list">
                    <el-radio-group v-model="form.projectId" @change="projectChange">
                        <el-radio v-for="(item,i) of projects" :key="item.id" :label="item.id">{{item.name}}</el-radio>
                    </el-radio-group>
                </ul>
            </el-col>
            <el-col :span="20" style="padding: 0 10px;">
                <el-button type="success" round size="small" style="width: 120px;" @click="exec" :disabled="!canRun">执行 <i class="el-icon-caret-right"></i></el-button>
                <el-divider></el-divider>
                <el-row>
                    <el-col :span="11">
                        <el-card class="box-card">
                            <div slot="header">
                                <el-checkbox v-model="allDbChecked" style="display: inline !important;" @change="allTableChange"></el-checkbox>
                                <span style="padding-left:10px;">数据表</span>
                            </div>
                            <el-checkbox-group v-model="form.tables" size="small" style="overflow-y: scroll;" :style="{maxHeight: (height - 140) + 'px'}">
                                <el-checkbox v-for="(item,i) of tables" :key="i" :label="item.org_name">{{item.org_name}} - {{item.comment}}</el-checkbox>
                            </el-checkbox-group>
                        </el-card>
                    </el-col>
                    <el-col :span="1">&nbsp;</el-col>
                    <el-col :span="12">
                        <el-card class="box-card">
                            <div slot="header">
                                <el-checkbox v-model="allTemplateChecked" style="display: inline !important;" @change="allTemplateChange"></el-checkbox>
                                <span style="padding-left:10px;">项目模板</span>
                            </div>
                            <el-checkbox-group v-model="form.templates" size="small" style="overflow-y: scroll;" :style="{maxHeight: (height - 140) + 'px'}">
                                <el-checkbox v-for="(item,i) of templates" :key="i" :label="item.template_id">{{item.file_path}}/{{item.file_name}}</el-checkbox>
                            </el-checkbox-group>
                        </el-card>
                    </el-col>
                </el-row>
            </el-col>
        </el-row>
        <el-dialog :visible.sync="showLog" :show-close="false" :close-on-click-modal="false" :close-on-press-escape="false" title="处理结果">
            <div class="console">
                <template v-for="(item, i) of $parent.logs">
                    <span style="margin: 2px 0; display: block;" v-if="item.type == 0">{{item.msg}}</span>
                    <span style="margin: 2px 0; display: block; color: #F56C6C;" v-else="item.type == 1">{{item.msg}}</span>
                </template>
            </div>
            <br/>
            <div style="text-align: center;"><el-button type="success" size="mini" :disabled="$parent.isDeploy" @click="close">关闭</el-button></div>
        </el-dialog>
    </div>
</template>
<script>
export default {
    data(){
        return {
            height: window.innerHeight - 90,
            projects: [],
            allDbChecked: false,
            allTemplateChecked: false,
            projectId: '',
            tables: [],
            templates: [],
            form: {
                projectId: '',
                tables: [],
                templates: []
            },
            showLog: false
        }
    },
    created(){
        if(this.$route.query.id) {
            this.form.projectId = parseInt(this.$route.query.id)
            this.projectChange()
        }
        this.projectList()
        this.$watch('$parent.isDeploy', (n, o)=> {
            if(this.$parent.isDeploy){
                this.showLog = true
            }
        })
    },
    computed:{
        canRun(){
            return this.form.projectId && this.form.tables.length > 0 && this.form.templates.length > 0
        }
    },
    methods: {
        projectList(){
            this.invoke('ProjectList', (data)=> {
                this.projects = data.records
            }, {pageNum: 1, pageSize: 10000})
        },
        projectChange(){
            this.invoke('TableAndTemplate', (data)=> {
                this.tables = data.table
                this.templates = data.template
            }, {projectId: this.form.projectId})
        },
        allTableChange(val){
            this.form.tables = []
            if(val){
                this.tables.forEach(it=> this.form.tables.push(it.org_name))
            }
        },
        allTemplateChange(val){
            this.form.templates = []
            if(val){
                this.templates.forEach(it=> this.form.templates.push(it.template_id))
            }
        },
        exec(){
            this.$parent.isDeploy = true
            let tables = this.tables.filter(it=> this.form.tables.findIndex(i=> i === it.org_name) >= 0)
            let templates = this.templates.filter(it=> this.form.templates.findIndex(i=> i === it.template_id) >= 0)
            let data = {project_id: this.form.projectId, tables, templates}
            this.invoke('GenTemplate', (rep)=>{}, {genInfo: data})
        },
        close(){
            this.form = {
                projectId: '',
                tables: [],
                templates: []
            }
            this.showLog = false
        }
    }
}
</script>
<style scoped>
.border-right {
    border-right: 1px solid #dcdfe6;
}
.list{
  list-style: none;
  margin: 0;
  padding: 0;
  width: 95%;
}
.list li{
  list-style: none;
  margin: 8px 0;
  padding: 0 10px;
  border-radius: 5px;
  border: 1px solid #303133;
}
.list li span{
  vertical-align: middle;
  line-height: 32px;
  font-size: 12px;
  display: inline-block;
  width: calc(100% - 50px);
  cursor: pointer;
}
.selected {
  background-color: #25374F;
  border: 1px solid #FFFFFF !important;
}
.el-checkbox {
    display: block !important;
    line-height: 26px !important;
    color: #FFFFFF;
}
.box-card {
    height: 100%;
}
.console {
    padding: 10px 0;
    max-height: 400px;
    overflow: scroll;
    color: black;
}
.console span {
    display: block;
    margin: 5px 0;
}
</style>