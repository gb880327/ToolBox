<template>
    <div class="inline-tool">
        <el-button type="primary" @click="addEnv" size="small">新增</el-button>
    </div>
    <el-row :gutter="20">
        <el-col :span="12">
            <el-table ref="dataTable" :data="state.data" border highlight-current-row  @current-change="handleCurrentChange" style="width: 100%">
                <el-table-column type="index" label="编号" align="center" width="80" />
                <el-table-column prop="name" label="变量名称" align="center" />
                <el-table-column prop="value" label="变量值" align="center" />
                <el-table-column label="操作" align="center">
                    <template #default="scope">
                        <el-button type="danger" size="small" @click="removeEnv(scope.row.id)">删除</el-button>
                    </template>
                </el-table-column>
            </el-table>
            
            <div style="text-align: right;margin-top: 10px;width: 100%;">
                <el-pagination small background layout="prev, pager, next, sizes" :total="state.total" v-model:pageSize="pageSize" v-model:currentPage="pageNum" />
            </div>
        </el-col>
        <el-col :span="8">
            <el-form ref="envRef" :model="state.form" :rules="state.rules" label-width="100px" size="default">
                <el-form-item label="变量名称：" prop="name">
                    <el-input v-model="state.form.name" placeholder="请输入变量名称！"></el-input>
                </el-form-item>
                <el-form-item label="变量值：" prop="value">
                    <el-input v-model="state.form.value" placeholder="请输入变量值！"></el-input>
                </el-form-item>
                <el-form-item>
                    <el-button type="primary" @click="submitForm(envRef)" style="width: 30%;">保存</el-button>
                </el-form-item>
            </el-form>
        </el-col>
    </el-row>
</template>
<script lang="ts" setup>
import { ref, reactive, inject, watch } from 'vue';
import utils from '@/libs/utils'
import type { FormInstance } from 'element-plus'

const dataTable = ref()
const run = inject<Function>('invoke', ()=> {});
const envRef = ref<FormInstance>()
const pageNum = ref(1)
const pageSize = ref(10)
const state = reactive({
    total: 0,
    form: {
        id: -1,
        name: '',
        value: ''
    },
    rules: {
        name: [{ required: true, message: '请输入变量名称！', trigger: 'blur' }],
        value: [{ required: true, message: '请输入变量值！', trigger: 'blur' }]
    },
    data: [{id: 1, name: 'node', value: 'node'}]
})

const getData = ()=> {
    run('Envs', (rep)=> {
        state.total = rep.total
        state.data = rep.records
    }, {pageNum: pageNum.value, pageSize: pageSize.value})
}
getData()
watch([pageNum, pageSize], ([pn, psize], [prevPageNum, prevPageSize])=> {
    getData()
})
const addEnv = ()=> {
    state.form.id = -1
    envRef.value?.resetFields()
}
const handleCurrentChange = (val)=> {
    if(val){
        state.form.id = val.id
        state.form.name = val.name
        state.form.value = val.value
    }
}
const submitForm = async (formEl)=> {
    if (!formEl) return
    await formEl.validate((valid, fields) => {
        if (valid) {
            utils.loading()
            let params = JSON.parse(JSON.stringify(state.form))
            if(params.id == -1){
                delete params.id
            }
            run('SaveEnv', (rep)=> {
                if(rep){
                    utils.success('新增成功！')
                    pageNum.value = 1
                    envRef.value?.resetFields()
                    getData()
                } else {
                    utils.error('新增失败！')
                }
                utils.stopLoading()
            }, {env: params})
        }
    })
}
const removeEnv =(id)=> {
    utils.confirm('确认是否删除该数据？', ()=> {
        utils.loading()
        run('RemoveEnv', (rep)=>{
            if(rep){
                utils.success('删除成功！')
                pageNum.value = 1
                envRef.value?.resetFields()
                getData()
            } else {
                utils.error('删除失败！')
            }
            utils.stopLoading()
        }, {id})
    })
}
</script>