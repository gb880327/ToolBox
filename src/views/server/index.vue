<template>
    <div class="content">
        <div class="tool">
            <el-button type="success" icon="el-icon-plus" size="mini" @click="add">新增</el-button>
        </div>
        <el-table :data="data">
            <el-table-column type="index" width="50" label="编号"></el-table-column>
            <el-table-column property="name" label="服务器名称" align="center"></el-table-column>
            <el-table-column property="host" label="服务器地址" align="center"></el-table-column>
            <el-table-column property="port" label="服务器端口" align="center"></el-table-column>
            <el-table-column property="user" label="用户名" align="center"></el-table-column>
            <el-table-column label="认证方式" align="center">
                <template slot-scope="scope">
                    <el-tooltip class="item" effect="dark" placement="top">
                        <template slot="content">
                            <span v-if="scope.row.password">密码：{{scope.row.password}}</span>
                            <span v-else>秘钥：{{scope.row.private_key}}</span>
                        </template>
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

        <myDialog ref="dialog" @ok="ok" @cancel="cancel" width="600px" btnWidth="48%" :title="title">
            <el-form slot="content" :model="form" :rules="rules" ref="serverForm" label-width="150px">
                <el-form-item label="服务器名称：" prop="name">
                    <el-input v-model="form.name"></el-input>
                </el-form-item>
                <el-form-item label="服务器地址：" prop="host">
                    <el-input v-model="form.host"></el-input>
                </el-form-item>
                <el-form-item label="服务器端口：" prop="port">
                    <el-input v-model="form.port" type="number"></el-input>
                </el-form-item>
                <el-form-item label="用户名：" prop="user">
                    <el-input v-model="form.user"></el-input>
                </el-form-item>
                <el-form-item label="认证类型：">
                    <el-select v-model="authType" @change="authChange">
                        <el-option value="0" label="密码"></el-option>
                        <el-option value="1" label="秘钥登陆"></el-option>
                    </el-select>
                </el-form-item>
                <el-form-item ref="password" label="密码：" v-if="authType === '0'" prop="password">
                    <el-input type="password" v-model="form.password" :show-password="true"></el-input>
                </el-form-item>
                <el-form-item ref="privateKey" label="秘钥路径：" v-if="authType === '1'" prop="private_key">
                    <el-input v-model="form.private_key"></el-input>
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
            data: [{name: 'test', host: '127.0.0.1', port: 22, user: 'root', password: '11111'}],
            total: 0,
            authType:'0',
            form: {
                id: 0,
                name: "",
                host: "",
                port: 22,
                user: "",
                password: "",
                private_key: ""
            },
            rules: {
                name: [{ required: true, message: "请输入服务器名称", trigger: "blur" }],
                host: [{ required: true, message: "请输入服务器地址", trigger: "blur" }],
                port: [{ required: true, message: "请输入服务器端口", trigger: "blur" }],
                user: [{ required: true, message: "请输入用户名", trigger: "blur" }],
                password: [{ required: true, message: "请输入密码", trigger: "blur" }]
            }
        }
    },
    methods: {
        add(){
            this.title = '新增服务器'
            this.$refs.dialog.show()
        },
        pageChange(pageNum, pageSize){
            this.pageNum = pageNum;
            this.pageSize = pageSize;
        },
        authChange(){
            this.form.password = "";
            this.form.private_key = "";
            this.form.identity_file = "";
            delete this.rules.private_key;
            delete this.rules.password;
            delete this.rules.identity_file;
            if (this.$refs.password) {
                this.$refs["password"].resetField();
            }
            if (this.$refs.privateKey) {
                this.$refs["privateKey"].resetField();
            }
            if (this.authType == 0) {
                this.rules["password"] = [
                { required: true, message: "请输入密码", trigger: "blur" },
                ];
            } else if (this.authType == 1) {
                this.rules["private_key"] = [
                { required: true, message: "请输入秘钥路径", trigger: "blur" },
                ];
            }
        },
        ok(){
            this.$refs.dialog.close()
            this.cancel()
        },
        cancel(){
            this.authType = '0'
            this.form = {
                id: 0,
                name: "",
                host: "",
                port: 22,
                user: "",
                password: "",
                private_key: ""
            }
            this.$refs.serverForm.resetFields()
        },
        modify(row){
            this.title = '修改服务器'
            this.form = row
            this.$refs.dialog.show()
        },
        remove(row){
            this.confirm("确认删除此数据！").then(()=>{
                
            }).catch(err=>{

            })
        }
    }
}
</script>
<style scoped>
</style>