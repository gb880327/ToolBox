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
                    <el-button type="success" icon="el-icon-setting" size="mini" circle @click="$refs.deploySetting.show()"></el-button>
                    <el-button type="primary" size="mini" circle @click="goto('/deploy', {id: scope.row.id})"><i class="el-icon-deploy"></i></el-button>
                </template>
            </el-table-column>
            <el-table-column label="代码生成设置" align="center"></el-table-column>
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
                    <el-input v-model="form.path" size="small" placeholder="请输入项目路径"></el-input>
                </el-form-item>
            </el-form>
        </myDialog>
        <deploy-setting ref="deploySetting"></deploy-setting>
    </div>
</template>
<script>
import deploySetting from '../deploy/setting.vue'

export default {
    components: {deploySetting},
    data(){
        return {
            title: '',
            total: 0,
            pageNum: 0,
            pageSize: 10,
            data: [{name: 'test', path: 'test'}],
            total: 0,
            form: {
                id: '',
                name: '',
                path: ''
            },
            rules: {
                name: [{ required: true, message: '请输入项目名称', trigger: 'blur' }],
                path: [{ required: true, message: '请输入项目路径', trigger: 'blur' }]
            }
        }
    },
    methods: {
        add(){
            this.title = '新增项目'
            this.$refs.dialog.show()
        },
        pageChange(pageNum, pageSize){
            this.pageNum = pageNum;
            this.pageSize = pageSize;
        },
        ok(){
            this.data.push(JSON.parse(JSON.stringify(this.form)))
            this.$refs.dialog.close()
            this.cancel()
        },
        cancel(){
            this.form = {
                id: '',
                name: ''
            }
            this.$refs.projectForm.resetFields()
        },
        modify(row){
            this.title = '修改项目'
            this.form = row
            this.$refs.dialog.show()
        },
        remove(row){
            this.$confirm("确认删除此数据！").then(()=>{
                
            }).catch(err=>{

            })
        }
    }
}
</script>
<style scoped>

</style>