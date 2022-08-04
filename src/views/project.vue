<template>
  <div class="tool">
    <el-button type="primary" @click="addProject" size="small">新增</el-button>
    <el-button type="danger" size="small" @click="removeProject" :disabled="state.selectNode.id < 0">删除</el-button>
    <el-button type="info" size="small" :disabled="state.selectNode.id < 0" @click="exec">生成代码</el-button>
  </div>
  <el-row :gutter="20" class="custom-row">
    <el-col :span="4" class="border-right custom-col">
      <el-scrollbar class="custom-col">
        <el-tree
          ref="treeRef"
          :data="treeData"
          :props="treeProps"
          node-key="id"
          :default-expand-all="true"
          :expand-on-click-node="false"
          :highlight-current="true"
          @node-click="treeNodeClick"
        ></el-tree>
      </el-scrollbar>
    </el-col>
    <el-col :span="20" class="custom-col">
      <el-tabs v-model="state.activeName" v-if="state.selectNode.id >= 0 || state.isAdd" @tab-change="tabChange">
        <el-tab-pane label="基础信息" name="info"><info ref="infoRef" @update="getData"></info></el-tab-pane>
        <el-tab-pane label="部署配置" name="deploy" v-if="state.selectNode.id >= 0"><deploy ref="deployRef"></deploy></el-tab-pane>
        <el-tab-pane label="代码生成配置" name="gen" v-if="state.selectNode.id >= 0"><gen ref="genRef"></gen></el-tab-pane>
      </el-tabs>
    </el-col>
  </el-row>
  <gen-exec ref="genExecRef"></gen-exec>
</template>
<script lang="ts" setup>
import { ref, reactive, inject, nextTick } from "vue";
import info from "./info.vue";
import deploy from "./deploy.vue";
import gen from "./gen.vue";
import GenExec from "@/components/gen_exec.vue";
import utils from '@/libs/utils'

interface NodeProp {
  id: number,
  label: string,
  name: string,
  path: string
}

const treeRef = ref()
const infoRef = ref()
const deployRef = ref()
const genRef = ref()
const genExecRef = ref()

const state = reactive({
  selectNode: {id: -1, name: '', path: ''},
  isAdd: true,
  activeName: 'info'
})
let treeData = ref([{
  id: -1,
  label: "项目列表",
  children: new Array<NodeProp>(),
}]);

const treeProps = reactive({
  children: "children",
  label: "label",
  class: 'custom-node'
});

const run = inject<Function>('invoke', ()=> {});

const getData = ()=> {
  run('ProjectList', (data)=> {
      treeData.value[0].children = []
      data.forEach(item=> treeData.value[0].children.push({ id: item.id, label: item.name, name: item.name, path: item.path }))
  })
}

getData()

const treeNodeClick = async (data: any) => {
  if (data.id == -1) {
    return;
  }
  state.selectNode = data;
  state.isAdd = false
  await nextTick()
  initTabsData(state.activeName)
};

const tabChange = async (name)=> {
  await nextTick()
  initTabsData(name)
}

const initTabsData = (name)=> {
  if(name === 'info') {
    infoRef.value?.clear();
    let form: any = infoRef.value['form'];
    form.id = state.selectNode.id
    form.name = state.selectNode.name
    form.path = state.selectNode.path
  } else if (name === 'deploy'){
    deployRef.value?.setData(state.selectNode.id)
  } else if (name === 'gen') {
    genRef.value?.setData(state.selectNode.id)
  }
}

const addProject = ()=> {
  state.isAdd = true
  state.selectNode = {id: -1, name: '', path: ''}
  treeRef.value?.setCurrentKey(null)
  infoRef.value?.clear()
}

const removeProject = ()=> {
  utils.confirm('确认是否删除该项目？', ()=> {
    run('RemoveProject', (data)=>{
      if(data){
        utils.success('删除成功！')
        getData()
      } else {
        utils.error('删除失败！')
      }
    }, {id: state.selectNode.id})
  })
}

const exec = ()=> {
  genExecRef.value?.show(state.selectNode.id, state.selectNode.name)
}
defineExpose([getData])
</script>
