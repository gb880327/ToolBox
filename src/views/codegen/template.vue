<template>
    <div class="content">
        <el-row>
            <el-col :span="6">
                <span class="category-title">模板分类：</span>
                <el-tree :data="treeData" node-key="id" default-expand-all :expand-on-click-node="false" :style="{fontSize: '12px', height: height + 'px'}">
                    <span class="custom-tree-node" slot-scope="{ node, data }">
                        <span>{{ node.label }}</span>
                        <span>
                            <i class="el-icon-circle-plus-outline" style="color:#67C23A; margin-right: 5px;" @click="addCategory(data)"></i>
                            <i class="el-icon-delete" style="color: #F56C6C" @click="removeCategory(node, data)"></i>
                        </span>
                    </span>
                </el-tree>
            </el-col>
            <el-col :span="18" style="padding-left: 10px;">
                <div class="tool">
                    <el-button type="success" icon="el-icon-plus" size="mini" @click="add">新增</el-button>
                </div>
                <el-table :data="data">
                    <el-table-column type="index" width="50" label="编号"></el-table-column>
                    <el-table-column property="name" label="名称" align="center"></el-table-column>
                    <el-table-column property="category_id" label="模板分类" align="center"></el-table-column>
                    <el-table-column property="language" label="模板语言" align="center"></el-table-column>
                    <el-table-column label="操作" align="center">
                        <template slot-scope="scope">
                            <el-button type="primary" icon="el-icon-edit" size="mini" circle @click="modify(scope.row)"></el-button>
                            <el-button type="danger" icon="el-icon-delete" size="mini" circle @click="remove(scope.row)"></el-button>
                        </template>
                    </el-table-column>
                </el-table>
                <myPagination :total="total" :pageSize="pageSize" :pageNum="pageNum" @pageChange="pageChange"></myPagination>
            </el-col>
        </el-row>
        <editor ref="editor"></editor>
    </div>
</template>
<script>
import editor from './editor.vue'

export default {
    components: {editor},
    data(){
        return {
            height: window.innerHeight - 115,
            treeData: [{
                id: 1,
                label: '一级 1',
                children: [{
                    id: 4,
                    label: '二级 1-1'
                }]
            }],
            data: [{id: 1, name: 'Demo', category_id: 1, language: 'JAVA'}],
            total: 0,
            pageNum: 0,
            pageSize: 10,
        }
    },
    created(){
        
    },
    methods: {
        addCategory(data){
            console.log(data)
        },
        removeCategory(node, data){
            console.log(node, data)
        },
        add(){
            this.$refs.editor.show()
        },
        modify(row){
            this.$refs.editor.show(row)
        },
        remove(row){
            this.confirm("确认删除此数据！").then(()=>{
                
            }).catch(err=>{

            })
        },
        pageChange(pageNum, pageSize){
            this.pageNum = pageNum;
            this.pageSize = pageSize;
        }
    }
}
</script>
<style scoped>
.custom-tree-node {
    flex: 1;
    display: flex;
    align-items: center;
    justify-content: space-between;
    font-size: 14px;
    padding-right: 8px;
}
.category-title {
    font-size: 12px;
    margin-bottom: 10px;
    display: inline-block;
}
</style>