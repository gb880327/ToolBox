<template>
    <el-form ref="formRef" :model="form" :rules="rules" label-width="100px" size="default" status-icon style="width: 600px;">
        <el-form-item label="项目名称：" prop="name">
            <el-input v-model="form.name"></el-input>
        </el-form-item>
        <el-form-item label="项目路径：" prop="path">
            <el-input v-model="form.path">
                <template #append>
                    <el-button @click="chooseHandler">选择...</el-button>    
                </template>
            </el-input>
        </el-form-item>
        <el-form-item>
            <el-button type="primary" @click="submitForm(formRef)" style="width: 30%;">保存</el-button>
            <el-button @click="resetForm(formRef)" style="width: 30%;">重置</el-button>
        </el-form-item>
    </el-form>
</template>
<script lang="ts" setup>
import { reactive, ref, inject } from 'vue'
import type { FormInstance, FormRules } from 'element-plus'
import utils from '@/libs/utils'

const run = inject<Function>('invoke', ()=> {});

const formRef = ref<FormInstance>()
let form = reactive({
    id: -1,
    name: '',
    path: ''
})
const rules = reactive<FormRules>({
    name: [{ required: true, message: '请输入项目名称！', trigger: 'blur' }],
    path: [{ required: true, message: '请选择项目路径！', trigger: 'blur' }]
})

const emit = defineEmits(['update'])
const submitForm = async (formEl: FormInstance | undefined) => {
    if (!formEl) return
    await formEl.validate((valid, fields) => {
        if (valid) {
            utils.loading()
            run('SaveProject', (data)=>{
                if(data){
                    emit('update')
                    utils.success('保存成功！');
                    clear();
                } else {
                    utils.error('保存失败！')
                }
                utils.stopLoading()
            }, {project: { name: form.name, path: form.path}})
        }
    })
}
const resetForm = (formEl: FormInstance | undefined)=> {
    if (!formEl) return
    formEl.resetFields()
}
const clear = ()=> {
    formRef.value?.resetFields()
    form.id = -1,
    form.name = '',
    form.path = ''
}
const chooseHandler = async ()=>{
    let path = await utils.chooseDir(form.path);
    form.path = path ? path?.toString() : (form.path ? form.path : '')
}
defineExpose({form, clear})
</script>
