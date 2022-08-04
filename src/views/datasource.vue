<template>
    <div class="tool">
        <el-button type="primary" @click="addDs" size="small">新增</el-button>
        <el-button type="danger" size="small" :disabled="state.form.id < 0" @click="removeDs">删除</el-button>
    </div>
    <el-row :gutter="20" class="custom-row">
      <el-col :span="4" class="border-right custom-col">
        <el-scrollbar class="custom-col">
          <el-tree ref="treeRef" :data="state.treeData" :props="state.treeProps" node-key="id" :default-expand-all="true" :expand-on-click-node="false" :highlight-current="true"
          @node-click="treeNodeClick"></el-tree>
        </el-scrollbar>
      </el-col>
      <el-col :span="20" class="custom-col">
        <el-form ref="dbRef" :model="state.form" :rules="state.rules" label-position="right" label-width="100px" size="default">
          <el-form-item label="名称：" prop="name">
            <el-input v-model="state.form.name" placeholder="请输入名称！"></el-input>
          </el-form-item>
          <el-form-item label="类型：" prop="db_type">
            <el-select v-model="state.form.db_type" style="width: 100%;">
              <el-option v-for="(item, i) of state.dbType" :key="i" :label="item.label" :value="item.value"></el-option>
            </el-select>
          </el-form-item>
          <el-form-item label="地址：" prop="host">
            <el-input v-model="state.form.host" placeholder="127.0.0.1"></el-input>
          </el-form-item>
          <el-form-item label="端口：" prop="port">
            <el-input-number v-model="state.form.port" placeholder="3306"></el-input-number>
          </el-form-item>
          <el-form-item label="数据库：" prop="db_name">
            <el-input v-model="state.form.db_name" placeholder="请输入数据库名称！"></el-input>
          </el-form-item>
          <el-form-item label="数据表前缀：" prop="prefix">
            <el-input v-model="state.form.prefix"></el-input>
          </el-form-item>
          <el-form-item label="用户名：" prop="user">
            <el-input v-model="state.form.user" placeholder="请输入用户名！"></el-input>
          </el-form-item>
          <el-form-item label="密码：" prop="password">
            <el-input v-model="state.form.password" placeholder="请输入密码！" :show-password="true"></el-input>
          </el-form-item>
          <el-form-item>
            <el-button type="primary" @click="submitForm(dbRef)" style="width: 30%;">保存</el-button>
          </el-form-item>
        </el-form>
      </el-col>
    </el-row>
</template>
<script lang="ts" setup>
import { ref, reactive, inject } from 'vue';
import type { FormInstance } from 'element-plus'
import utils from '@/libs/utils'

interface TreeNode {
  id: number,
  label: string
}

const treeRef = ref()

const run = inject<Function>('invoke', ()=> {})

const dbRef =  ref<FormInstance>()

const state = reactive({
  dbType: [{label: 'MYSQL', value: 'mysql'}],
  form: {
    id: -1,
    name: '',
    db_type: 'mysql',
    host: '127.0.0.1',
    port: 3306,
    db_name: '',
    prefix: '',
    user: '',
    password: ''
  },
  dbs: [],
  rules: {
    name: [{ required: true, message: '请输入名称！', trigger: 'blur' }],
    db_type: [{ required: true, message: '请选择数据库类型！', trigger: 'blur' }],
    host: [{ required: true, message: '请输入地址！', trigger: 'blur' }],
    port: [{ required: true, message: '请输入端口！', trigger: 'blur' }],
    db_name: [{ required: true, message: '请输入数据库名称！', trigger: 'blur' }],
    user: [{ required: true, message: '请输入用户名！', trigger: 'blur' }],
    password: [{ required: true, message: '请输入密码！', trigger: 'blur' }]
  },
  treeData: [{id: -1,
    label: "数据源列表",
    children: new Array<TreeNode>()
  }],
  treeProps: {
    children: "children",
    label: "label",
    class: "custom-node",
  },
})

const getData = ()=> {
  run('DataSources', (rep)=> {
    state.treeData[0].children = new Array<TreeNode>()
    state.dbs = rep
    rep.forEach(item=> {
      state.treeData[0].children.push({id: item.id, label: item.name})
    })
  })
}
getData()
const submitForm = async (formEl: FormInstance | undefined)=> {
    if (!formEl) return
    await formEl.validate((valid, fields) => {
        if (valid) {
            let params = JSON.parse(JSON.stringify(state.form))
            if(params.id == -1){
              delete params.id
            }
            utils.loading()
            run('SaveDataSource', (rep)=> {
              if(rep){
                utils.success('保存成功！')
                getData()
                dbRef.value?.resetFields()
              } else {
                utils.error('保存失败！')
              }
              utils.stopLoading()
            }, {ds:params})
        }
    })
}
const treeNodeClick = (data)=> {
  if(data.id > 0){
    let item: any = state.dbs.find(it=> it['id'] === data.id)
    state.form = JSON.parse(JSON.stringify(item))
  }
}
const addDs = ()=> {
  dbRef.value?.resetFields()
  treeRef.value?.setCurrentKey(null)
}
const removeDs = ()=> {
  utils.confirm('是否确认删除该数据源信息？', ()=> {
    utils.loading()
    run('RemoveDataSource', (rep)=> {
      if(rep) {
        utils.success('删除成功！')
        getData()
        dbRef.value?.resetFields()
        treeRef.value?.setCurrentKey(null)
      } else {
        utils.error('删除失败！')
      }
      utils.stopLoading()
    }, {id: state.form.id})
  })
}
</script>