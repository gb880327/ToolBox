<template>
    <div class="content">
        <div class="tool">
            <el-button type="success" icon="el-icon-plus" size="mini" @click="add">新增</el-button>
        </div>
        <el-table :data="data">
            <el-table-column type="index" width="50" label="编号"></el-table-column>
            <el-table-column property="name" label="项目名称" align="center"></el-table-column>
            <el-table-column property="path" label="项目路径" align="center"></el-table-column>
            <el-table-column label="部署设置" align="center">
                <template slot-scope="scope">
                    <el-button type="success" icon="el-icon-setting" size="mini" circle @click="$refs.deploySetting.show(scope.row.id)"></el-button>
                    <el-button type="primary" size="mini" circle @click="goto('/deploy', {id: scope.row.id})"><i class="el-icon-deploy"></i></el-button>
                </template>
            </el-table-column>
            <el-table-column label="代码生成设置" align="center">
                <template slot-scope="scope">
                    <el-button type="success" icon="el-icon-setting" size="mini" circle @click="$refs.codegenSetting.show(scope.row.id, scope.row.path)"></el-button>
                    <el-button type="primary" size="mini" circle @click="goto('/codegen', {id: scope.row.id})"><i class="el-icon-generated"></i></el-button>
                </template>
            </el-table-column>
            <el-table-column label="操作" align="center">
                <template slot-scope="scope">
                    <el-button type="primary" icon="el-icon-edit" size="mini" circle @click="modify(scope.row)"></el-button>
                    <el-button type="danger" icon="el-icon-delete" size="mini" circle @click="remove(scope.row)"></el-button>
                </template>
            </el-table-column>
        </el-table>
        <myPagination :total="total" :pageSize="pageSize" :pageNum="pageNum" @pageChange="pageChange"></myPagination>
        <myDialog ref="dialog" @ok="ok" @cancel="cancel" width="400px" btnWidth="48%" :title="title">
            <el-form slot="content" :model="form" :rules="rules" ref="projectForm" label-width="100px">
                <el-form-item label="项目名称：" prop="name">
                    <el-input v-model="form.name" size="small" placeholder="请输入项目名称"></el-input>
                </el-form-item>
                <el-form-item label="项目路径：" prop="path">
                    <el-input v-model="form.path" size="small" placeholder="请输入项目路径">
                        <el-button slot="suffix" size="mini" type="primary" plain @click="chooseHandler">选择</el-button>
                    </el-input>
                </el-form-item>
            </el-form>
        </myDialog>
        <deploy-setting ref="deploySetting"></deploy-setting>
        <codegen-setting ref="codegenSetting"></codegen-setting>
    </div>
</template>
<script>
import deploySetting from '../deploy/setting.vue'
import codegenSetting from '../codegen/setting.vue'

export default {
    components: {deploySetting, codegenSetting},
    data(){
        return {
            title: '',
            total: 0,
            pageNum: 0,
            pageSize: 10,
            data: [],
            form: {
                name: '',
                path: ''
            },
            rules: {
                name: [{ required: true, message: '请输入项目名称', trigger: 'blur' }],
                path: [{ required: true, message: '请输入项目路径', trigger: 'blur' }]
            }
        }
    },
    created(){
        this.list()
    },
    methods: {
        chooseHandler(){
            this.chooseDir().then(rep=> {
                this.form.path = rep
            })
        },
        list(){
            this.invoke('ProjectList', (data)=> {
                this.data = data.records
                this.total = data.total
            }, {pageNum: this.pageNum, pageSize: this.pageSize})
        },
        add(){
            this.title = '新增项目'
            this.$refs.dialog.show()
        },
        pageChange(pageNum, pageSize){
            this.pageNum = pageNum;
            this.pageSize = pageSize;
            this.list()
        },
        ok(){
            this.$refs.projectForm.validate(valid=> {
                if(valid){
                    this.invoke('SaveProject', (data)=>{
                        this.success('保存成功！')
                        this.list()
                        this.$refs.dialog.close()
                        this.cancel()
                    }, {project: this.form})
                }
            })
        },
        cancel(){
            this.form = {
                name: '',
                path: ''
            }
            this.$refs.projectForm.resetFields()
        },
        modify(row){
            this.title = '修改项目'
            this.form = JSON.parse(JSON.stringify(row))
            this.$refs.dialog.show()
        },
        remove(row){
            this.confirm("确认删除此数据！").then((result)=>{
                if(result){
                    this.invoke('RemoveProject', (data)=>{
                        this.success('删除成功！')
                        this.list()
                    }, {id: row.id})
                }
            })
        }
    }
}
</script>
<style scoped>

</style>