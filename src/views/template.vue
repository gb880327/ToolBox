<template>
  <el-row :gutter="20" class="custom-row">
    <el-col :span="4" class="border-right custom-col">
      <el-scrollbar class="custom-col">
        <el-tree ref="treeRef" :data="state.treeData" :props="state.treeProps" node-key="id" :default-expand-all="true" :expand-on-click-node="false" :highlight-current="true"
          @node-click="treeNodeClick">
            <template #default="{ node, data }">
              <span class="custom-tree-node">
                <span>{{ node.label }}</span>
                <el-space wrap>
                  <el-icon color="green" @click="addCategory(data)" v-if="!data.is_template"><CirclePlus/></el-icon>
                  <el-icon color="red" @click="removeCategory(data)" v-if="data.id >= 0"><Delete/></el-icon>
                </el-space>
              </span>
            </template>
          </el-tree>
        </el-scrollbar>
    </el-col>
    <el-col :span="20" class="custom-col">
      <el-form ref="templateRef" :model="state.form" :rules="state.rules" label-width="100px" size="default">
        <el-form-item label="名称：" prop="name">
          <el-input v-model="state.form.name" placeholder="请输入模板名称！"></el-input>
        </el-form-item>
        <el-form-item label="模板语言：" prop="language">
          <el-select v-model="state.form.language" @change="langChange">
            <el-option v-for="(item,i) of state.langs" :key="i" :label="item.label" :value="item.value"></el-option>
          </el-select>
          <help-tip></help-tip>
        </el-form-item>
        <el-form-item label="模板：" prop="content">
          <v-ace-editor v-model:value="state.form.content" :lang="state.form.language" theme="dracula" :options="{'fontSize': '14px', 'tabSize': 4}"
           :style="{'height': height +'px', 'width': '100%'}"/>
        </el-form-item>
        <el-form-item>
          <el-button type="primary" @click="submitForm(templateRef)" style="width: 30%;">保存</el-button>
        </el-form-item>
      </el-form>
    </el-col>
  </el-row>
</template>
<script lang="ts" setup>
import { CirclePlus, Delete } from '@element-plus/icons-vue'
import { ref, reactive, inject, nextTick } from 'vue';
import { VAceEditor } from "vue3-ace-editor";
import "ace-builds/webpack-resolver";
import { FormInstance, ElMessageBox } from 'element-plus'
import utils from '@/libs/utils'
import helpTip from '@/components/help.vue'

const run = inject<Function>('invoke', ()=> {});

const height = ref(window.innerHeight - 300)
const templateRef = ref<FormInstance>();
const state = reactive({
  isNew: false,
  form: {
    id: -1,
    name: '',
    language: 'text',
    category_id: -1,
    content: ''
  },
  rules: {
    name: [{ required: true, message: '请输入名称！', trigger: 'blur' }],
    content: [{ required: true, message: '请填写模板内容！', trigger: 'blur' }],
    language: [{ required: true, message: '请选择模板语言！', trigger: 'blur' }]
  }, 
  categoryId: -1,
  langs: [{label: 'TEXT', value: 'text'},{label: 'JAVA', value: 'java'}],
  treeData: [{
      id: -1,
      label: "模板列表",
      children: []
    }
  ],
  treeProps: {
    children: "children",
    label: "label",
    class: "custom-node",
  }
})

const getData =()=> {
  run('CategoryTree', (rep)=> {
    state.treeData[0].children = rep
  })
}

getData()

const addTemplate = ()=> {
  state.isNew = true
  nextTick(()=> {
    state.form = { id: -1, category_id: -1, name: '', language: 'text', content: '' }
    templateRef.value?.resetFields()
  })
}
const treeNodeClick = (data)=> {
  state.isNew = false
  if(!data.is_template){
    state.form.id = -1
    state.isNew = true
    state.categoryId = data.id
    templateRef.value?.resetFields()
  } else {
    state.categoryId = -1
    state.form.id = data.id
    state.form.name = data.label
    state.form.language = data.language
    state.form.content = data.content
    state.form.category_id = data.category_id
  }
}
const langChange = (val)=> {
  state.form.language = val
}
const addCategory = (data)=> {
  ElMessageBox.prompt('请输入分类名称', '新增模板分类', {
    confirmButtonText: '确定',
    cancelButtonText: '取消'
  }).then(({value})=> {
    run('SaveCategory', (rep)=> {
      if(rep){
        getData()
        utils.success('新增成功！')
      } else {
        utils.error('新增失败！')
      }
    }, {category: {name: value, parent_id: state.categoryId == -1 ? 0 : state.categoryId}})
  }).catch(()=> {})
}
const removeCategory = (data)=> {
  if(!data.is_template && data.children.length > 0){
    utils.error('该分类下已有模板，不能删除！')
    return
  }
  utils.confirm(data.is_template ? '确定是否删除当前模板？' : '确定是否删除当前分类？', ()=> {
      run(data.is_template ? 'RemoveTemplate' : 'RemoveCategory', (rep)=> {
        if(rep){
          utils.success('删除成功！')
          getData()
          if(data.is_template){
            templateRef.value?.resetFields()
          }
        } else {
          utils.error('删除失败！')
        }
      }, data.is_template ? {templateId: data.id} : {id: data.id})
  })
}
const submitForm = async (formEl: FormInstance | undefined)=> {
    if (!formEl) return
    await formEl.validate((valid, fields) => {
        if (valid) {
            let params = JSON.parse(JSON.stringify(state.form))
            if(params.id == -1){
              delete params.id
              params.category_id = state.categoryId
            }
            utils.loading()
            run('SaveTemplate', (rep)=> {
              if(rep) {
                utils.success('保存成功！')
                getData()
                templateRef.value?.resetFields()
              } else {
                utils.error('保存失败！')
              }
              utils.stopLoading()
            }, {template: params})
        }
    })
}
</script>
<style scoped>
.editor {
  overflow-y: scroll;
  height: 500px;
}
</style>