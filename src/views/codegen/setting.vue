<template>
    <myDialog title="代码生成设置" ref="dialog" @ok="ok" @cancel="cancel" width="800px" btnWidth="48%">
        <el-form slot="content" size="small" :model="form" ref="settingForm" label-width="100px">
            <el-form-item label="数据源：" prop="datasource">
                <el-select v-model="form.datasource" placeholder="请选择数据源！">
                    <el-option v-for="(item, i) of dataSources" :key="i" :label="item.name" :value="item.id"></el-option>
                </el-select>
            </el-form-item>
            <el-form-item label="文件目录：" prop="root_path">
                <el-input v-model="form.output"></el-input>
            </el-form-item>
            <el-form-item label="模板：">
                <el-row><el-col :span="6">模板</el-col><el-col :span="8">目录</el-col><el-col :span="8">文件名</el-col></el-row>

                <el-row class="template-item" v-for="(item,i) of form.template" :key="i">
                    <el-col :span="6">
                        <treeselect v-model="item.template_id" :options="templages" style="width: 95%;"/>
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
            dataSources: [],
            templages: [{
                id: 'a',
                label: 'a',
                children: [ {
                    id: 'aa',
                    label: 'aa',
                }, {
                    id: 'ab',
                    label: 'ab',
                } ],
                }, {
                id: 'b',
                label: 'b',
                }, {
                id: 'c',
                label: 'c',
            }],
            form: {
                id: 0,
                datasource: '',
                output: '',
                template: []
            }
        }
    },
    methods: {
        show(id){
            console.log(id)
            this.$refs.dialog.show()
        },
        addTemplate(){
            this.form.template.push({template_id: '', file_path: '', file_name: ''})
        },
        removeTemplate(i){
            this.form.template.splice(i, 1)
        },
        ok(){

            this.cancel()
            this.$refs.dialog.close()
        },
        cancel(){
            this.form = {
                d: 0,
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