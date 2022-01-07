<template>
    <div class="content">
        <div class="tool">
            <el-button type="success" icon="el-icon-plus" size="mini" @click="add">新增</el-button>
        </div>
         <el-table :data="data">
            <el-table-column type="index" width="50" label="编号"></el-table-column>
            <el-table-column property="name" label="名称" align="center"></el-table-column>
            <el-table-column property="db_type" label="类型" align="center">
                <span slot-scope="scope">
                    {{config.dbType.find(it=> it.value === scope.row.db_type).label}}
                </span>
            </el-table-column>
            <el-table-column property="host" label="地址" align="center"></el-table-column>
            <el-table-column property="port" label="端口" align="center"></el-table-column>
            <el-table-column property="db_name" label="数据库" align="center"></el-table-column>
            <el-table-column property="prefix" label="数据表前缀" align="center"></el-table-column>
            <el-table-column property="user" label="用户名" align="center"></el-table-column>
            <el-table-column label="密码" align="center">
                <template slot-scope="scope">
                    <el-tooltip class="item" effect="dark" placement="top" :content="scope.row.password">
                        <i class="el-icon-see"></i>
                    </el-tooltip>
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
             <el-form ref="dbForm" slot="content" :model="form" :rules="rules" label-width="100px" size="small">
                 <el-form-item label="名称：" prop="name">
                     <el-input v-model="form.name" placeholder="请输入名称！"></el-input>
                 </el-form-item>
                 <el-form-item label="类型：" prop="db_type">
                     <el-select v-model="form.db_type" style="width: 100%;">
                         <el-option v-for="(item, i) of config.dbType" :key="i" :label="item.label" :value="item.value"></el-option>
                     </el-select>
                 </el-form-item>
                 <el-form-item label="地址：" prop="host">
                     <el-input v-model="form.host" placeholder="127.0.0.1"></el-input>
                 </el-form-item>
                 <el-form-item label="端口：" prop="port">
                     <el-input-number v-model="form.port" placeholder="3306" style="width: 100%;"></el-input-number>
                 </el-form-item>
                 <el-form-item label="数据库：" prop="db_name">
                     <el-input v-model="form.db_name" placeholder="请输入数据库名称！"></el-input>
                 </el-form-item>
                 <el-form-item label="数据表前缀：" prop="prefix">
                     <el-input v-model="form.prefix"></el-input>
                 </el-form-item>
                 <el-form-item label="用户名：" prop="user">
                     <el-input v-model="form.user" placeholder="请输入用户名！"></el-input>
                 </el-form-item>
                 <el-form-item label="密码：" prop="password">
                     <el-input v-model="form.password" placeholder="请输入密码！" :show-password="true"></el-input>
                 </el-form-item>
             </el-form>
         </myDialog>
    </div>
</template>
<script>

export default {  
    data(){
        return {
            data: [],
            title: '',
            total: 0,
            pageNum: 0,
            pageSize: 10,
            form: {
                name: '',
                db_type: 'mysql',
                host: '',
                port: 3306,
                db_name: '',
                prefix: '',
                user: '',
                password: ''
            },
            rules: {
                name: [{ required: true, message: '请输入名称！', trigger: 'blur' }],
                db_type: [{ required: true, message: '请选择数据库类型！', trigger: 'blur' }],
                host: [{ required: true, message: '请输入地址！', trigger: 'blur' }],
                port: [{ required: true, message: '请输入端口！', trigger: 'blur' }],
                db_name: [{ required: true, message: '请输入数据库名称！', trigger: 'blur' }],
                user: [{ required: true, message: '请输入用户名！', trigger: 'blur' }],
                password: [{ required: true, message: '请输入密码！', trigger: 'blur' }]
            }
        }
    },
    created(){
        this.list()
    },
    methods: {
        list(){
            this.invoke('DataSources', (data)=>{
                this.data = data.records
                this.total = data.total
            } , {pageNum: this.pageNum, pageSize: this.pageSize})
        },
        add(){
            this.title = '新增数据源'
            this.$refs.dialog.show()
        },
        pageChange(pageNum, pageSize){
            this.pageNum = pageNum;
            this.pageSize = pageSize;
        },
        ok(){
            this.$refs.dbForm.validate(valid=> {
                if(valid){
                    this.invoke('SaveDataSource', (rep)=>{
                        this.success('保存成功！')
                        this.list()
                        this.$refs.dialog.close()
                        this.cancel()
                    }, { ds: this.form })
                }
            })
        },
        cancel(){
            this.form = {
                name: '',
                db_type: 'mysql',
                host: '',
                port: 3306,
                db_name: '',
                prefix: '',
                user: '',
                password: ''
            }
            this.$refs.dbForm.resetFields()
        },
        modify(row){
            this.title = '修改数据源'
            this.form = row
            this.$refs.dialog.show()
        },
        remove(row){
            this.$confirm("确认删除此数据！").then((result)=>{
                if(result) {
                    this.invoke('RemoveDataSource', (data)=>{
                        this.success('删除成功！')
                        this.list()
                    }, {id: row.id})
                }
            })
        }
    }
}
</script>