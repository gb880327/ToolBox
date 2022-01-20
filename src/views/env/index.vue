<template>
    <div class="content">
        <div class="tool">
            <el-button type="success" icon="el-icon-plus" size="mini" @click="add">新增</el-button>
        </div>
        <el-table :data="data">
            <el-table-column type="index" width="50" label="编号"></el-table-column>
            <el-table-column property="name" label="变量名称" align="center"></el-table-column>
            <el-table-column property="value" label="变量值" align="center"></el-table-column>
            <el-table-column label="操作" align="center">
                <template slot-scope="scope">
                    <el-button type="primary" icon="el-icon-edit" size="mini" circle @click="modify(scope.row)"></el-button>
                    <el-button type="danger" icon="el-icon-delete" size="mini" circle @click="remove(scope.row)"></el-button>
                </template>
            </el-table-column>
        </el-table>
        <myPagination :total="total" :pageSize="pageSize" :pageNum="pageNum" @pageChange="pageChange"></myPagination>
        <myDialog ref="dialog" @ok="ok" @cancel="cancel" width="400px" btnWidth="48%" :title="title">
            <el-form slot="content" :model="form" :rules="rules" ref="envForm" label-width="100px">
                <el-form-item label="变量名称：" prop="name">
                    <el-input v-model="form.name" size="small" placeholder="请输入变量名称"></el-input>
                </el-form-item>
                <el-form-item label="变量值：" prop="value">
                    <el-input v-model="form.value" size="small" placeholder="请输入变量值"></el-input>
                </el-form-item>
            </el-form>
        </myDialog>
    </div>
</template>
<script>
export default {
    data(){
        return {
            title: '',
            total: 0,
            pageNum: 0,
            pageSize: 10,
            data: [],
            form: {
                name: '',
                value: ''
            },
            rules: {
                name: [{ required: true, message: '请输入变量名称', trigger: 'blur' }],
                value: [{ required: true, message: '请输入变量值', trigger: 'blur' }]
            }
        }
    },
    created(){
        this.list()
    },
    methods: {
        add(){
            this.title = '新增环境变量'
            this.$refs.dialog.show()
        },
        pageChange(pageNum, pageSize){
            this.pageNum = pageNum;
            this.pageSize = pageSize;
            this.list()
        },
        list(){
            this.invoke('Envs', (data)=> {
                this.data = data.records
                this.total = data.total
            }, {pageNum: this.pageNum, pageSize: this.pageSize})
        },
        ok(){
            this.$refs.envForm.validate(valid=> {
                if(valid){
                    this.invoke('SaveEnv', (data)=>{
                        this.success('保存成功！')
                        this.list()
                        this.$refs.dialog.close()
                        this.cancel()
                    }, {env: this.form})
                }
            })
        },
        cancel(){
            this.form = {
                name: '',
                value: ''
            }
            this.$refs.envForm.resetFields()
        },
        modify(row){
            this.title = '修改环境变量'
            this.form = JSON.parse(JSON.stringify(row))
            this.$refs.dialog.show()
        },
        remove(row){
            this.confirm("确认删除此数据！").then((result)=>{
                if(result){
                    this.invoke('RemoveEnv', (data)=>{
                        this.success('删除成功！')
                        this.list()
                    }, {id: row.id})
                }
            })
        }
    }
}
</script>