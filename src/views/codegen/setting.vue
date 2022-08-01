<template>
    <myDialog title="代码生成设置" ref="dialog" @ok="ok" @cancel="cancel" width="800px" btnWidth="48%">
        <el-form slot="content" size="small" :model="form" ref="settingForm" label-width="100px">
            <el-form-item label="数据源：" prop="datasource">
                <el-select v-model="form.datasource" placeholder="请选择数据源！">
                    <el-option v-for="(item, i) of dataSources" :key="i" :label="item.name" :value="item.id"></el-option>
                </el-select>
            </el-form-item>
            <el-form-item label="文件目录：" prop="root_path">
                <el-input v-model="form.output">
                    <el-button slot="suffix" size="mini" type="primary" plain @click="chooseHandler">选择</el-button>
                </el-input>
            </el-form-item>
            <el-form-item label="模板：">
                <el-row><el-col :span="6">模板</el-col><el-col :span="8">目录</el-col><el-col :span="8">文件名</el-col></el-row>

                <el-row class="template-item" v-for="(item,i) of form.template" :key="i">
                    <el-col :span="6">
                        <treeselect v-model="item.template_id" :options="templages" style="width: 95%;" :disable-branch-nodes="true" :show-count="true"/>
                    </el-col>
                    <el-col :span="8">
                        <el-input v-model="item.file_path" style="width: 95%;"></el-input>
                    </el-col>
                    <el-col :span="8">
                        <el-input v-model="item.file_name" style="width: 95%;"></el-input>
                    </el-col>
                    <el-col :span="2"><el-button size="mini" type="danger" icon="el-icon-delete" circle @click="removeTemplate(i)"></el-button></el-col>
                </el-row>

                <el-button type="success" class="template-add" size="mini" plain icon="el-icon-plus" @click="addTemplate">新增</el-button>
            </el-form-item>
        </el-form>
    </myDialog>
</template>
<script>
export default {
    name: 'codegenSetting',
    data(){
        return {
            projectId: '',
            projectPath: '',
            dataSources: [],
            templages: [],
            form: {
                datasource: '',
                output: '',
                template: []
            }
        }
    },
    created(){
        this.getDataSouce()
        this.getTemplates()
    },
    methods: {
        chooseHandler(){
            this.chooseDir(this.projectPath).then(path=> {
                if(path){
                    this.form.output = path.replace(this.projectPath, '')
                }
            })
        },
        getDataSouce(){
            this.invoke('DataSources', (data)=>{
                this.dataSources = data.records
            } , {pageNum: 1, pageSize: 1000})
        },
        getTemplates(){
            this.invoke('Categorys', (data)=> {
                this.invoke('Templates', (rep)=> {
                    this.templages = this.getSubItem(data, 0, rep.records)
                },{ categoryId: undefined })
            })
        },
        getSubItem(data, parentId, templates) {
            let parents = data.filter(it=> it.parent_id === parentId)
            let item = []
            if(parents && parents.length > 0) {
                parents.forEach(it=> {
                    let children = this.getSubItem(data, it.id, templates)
                    let temps = templates.filter(t=> t.category_id === it.id)
                    if(temps && temps.length > 0){
                        temps.forEach(t=> {
                            children.push({id: t.id, label: t.name})
                        })
                    }
                    item.push({ id: it.id, label: it.name, children: children})
                })
            }
            return item
        },
        show(id, path){
            this.projectPath = path
            this.projectId = id
            this.invoke('GenSetting', (data)=> {
                if(data){
                    this.form.id = data.id
                    this.form.project_id = data.project_id
                    this.form.datasource = data.datasource
                    this.form.output = data.output
                    this.form.template = JSON.parse(data.template)
                }
                this.$refs.dialog.show()
            }, {projectId: this.projectId})
        },
        addTemplate(){
            this.form.template.push({template_id: '', file_path: '', file_name: ''})
        },
        removeTemplate(i){
            this.form.template.splice(i, 1)
        },
        ok(){
            if(!this.form.datasource){
                this.error('请选择数据源！')
                return
            }
            if(!this.form.output){
                this.error('请选择文件目录！')
                return
            }
            if(this.form.template.filter(it=> !it.template_id || !it.file_path || !it.file_name).length > 0){
                this.error('请填写完整的模板配置信息！')
                return
            }
            let data = {
                id: this.form.id,
                project_id: this.projectId,
                datasource: this.form.datasource,
                output: this.form.output,
                template: JSON.stringify(this.form.template)
            }
            this.invoke('SaveGenSetting', (rep)=> {
                this.success('保存成功！')
                this.cancel()
                this.$refs.dialog.close()
            }, {setting: data})
        },
        cancel(){
            this.form = {
                datasource: '',
                output: '',
                template: []
            }
        }
    }
}
</script>
<style scoped>
.template-item {
    margin: 2px 0;
}
.template-add {
    border: 1px dotted;
    width: 90%;
}
</style>