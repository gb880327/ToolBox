<template>
    <div class="content">
        <el-row>
            <el-col :span="6">
                <el-tree :data="treeData" node-key="id" default-expand-all :expand-on-click-node="false" @node-click="treeSelect" :style="{fontSize: '12px', height: height + 'px'}">
                    <template slot-scope="{ node, data }">
                        <span class="custom-tree-node" v-if="data.id >=0">
                            <span>{{ node.label }}</span>
                            <span>
                                <i class="el-icon-circle-plus-outline" style="color:#67C23A; margin-right: 5px;" @click="addCategory(data)"></i>
                                <i class="el-icon-delete" style="color: #F56C6C" @click="removeCategory(node, data)"></i>
                            </span>
                        </span>
                        <span class="custom-tree-node" v-else>
                            <span>{{ node.label }}</span>
                            <span>
                                <i class="el-icon-circle-plus-outline" style="color:#67C23A; margin-right: 5px;" @click="addCategory(data)"></i>
                                <span style="width:14px;height:14px;display: inline-block"></span>
                            </span>
                        </span>
                    </template>
                </el-tree>
            </el-col>
            <el-col :span="18" style="padding-left: 10px;">
                <div class="tool">
                    <el-button type="success" icon="el-icon-plus" size="mini" @click="add">新增</el-button>
                    <span style="font-size:12px; padding-left: 10px;">当前分类：{{currentName}}</span>
                </div>
                <el-table :data="data">
                    <el-table-column type="index" width="50" label="编号"></el-table-column>
                    <el-table-column property="name" label="名称" align="center"></el-table-column>
                    <el-table-column label="分类" align="center">
                        <template slot-scope="scope">
                            {{categoryList.find(it=> it.id === scope.row.category_id).name}}
                        </template>
                    </el-table-column>
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
        <editor ref="editor" @update="getTemplates"></editor>
    </div>
</template>
<script>
import editor from './editor.vue'

export default {
    components: {editor},
    data(){
        return {
            height: window.innerHeight - 90,
            categoryList: [],
            treeData: [{
                id: -1,
                label: '模板分类',
                children: []
            }],
            data: [],
            total: 0,
            pageNum: 0,
            pageSize: 10,
            current: 0,
            currentName: ''
        }
    },
    created(){
        this.categorys()
        this.getTemplates()
    },
    methods: {
        categorys(){
            this.invoke('Categorys', (data)=> {
                this.categoryList = data
                this.treeData[0].children = this.getSubItem(data, 0)
            })
        },
        getSubItem(data, parentId) {
            let parents = data.filter(it=> it.parent_id === parentId)
            let item = []
            if(parents && parents.length > 0) {
                parents.forEach(it=> {
                    item.push({ id: it.id, label: it.name, children: this.getSubItem(data, it.id) })
                })
            }
            return item
        },
        addCategory(data){
            this.Swal.fire({
                input: 'text',
                showCancelButton: true,
                inputValidator: (value)=> {
                    return new Promise((resolve)=> {
                        if (!value){
                            resolve('请输入分类名称！')
                        } else {
                            resolve()
                        }
                    })
                }
            }).then(rep=>{
                if(!rep.isConfirmed) return
                this.invoke('SaveCategory',(rep)=>{
                    this.categorys()
                }, {category: {name: rep.value, parent_id: data.id == -1 ? 0 : data.id}})
            })
        },
        removeCategory(node, data){
            this.$confirm("确认删除此数据！").then((result)=>{
                this.invoke('RemoveCategory', (data)=>{
                    this.success('删除成功！')
                    this.categorys()
                }, {id: data.id})
            }).catch(err=> {})
        },
        getTemplates(category){
            this.current = category
            this.invoke('Templates', (data)=> {
                this.data = data.records
                this.total = data.total
            }, { categoryId: category })
        },
        treeSelect(data, node, tree){
            this.current = data.id
            if(this.current == -1){
                this.getTemplates()
                this.currentName = ''
            } else {
                this.currentName = this.categoryList.find(it=> it.id === data.id).name
                this.getTemplates(this.current)
            }
        },
        add(){
            if(!this.current || this.current == -1){
                this.error('请先选择分类！')
                return
            }
            this.$refs.editor.show(this.current)
        },
        modify(row){
            this.$refs.editor.show(this.current, row)
        },
        remove(row){
            this.confirm("确认删除此数据！").then((result)=>{
                if(result){
                    this.invoke('RemoveTemplate', (data)=>{
                        this.success('删除成功！')
                        this.getTemplates(this.current)
                    }, {id: row.id})
                }
            }).catch(err=>{})
        },
        pageChange(pageNum, pageSize){
            this.pageNum = pageNum;
            this.pageSize = pageSize;
            this.getTemplates(this.current)
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
    color: black;
}
.category-title {
    font-size: 12px;
    margin-bottom: 10px;
    display: inline-block;
}
</style>