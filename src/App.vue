<template>
  <el-menu :default-active="activeIndex" mode="horizontal" :ellipsis="false" @select="handleSelect">
    <el-menu-item index="/">
      <template #title>
        <el-icon>
          <Platform />
        </el-icon>
        <span>项目管理</span>
      </template>
    </el-menu-item>
    <el-menu-item index="/server">
      <template #title>
        <el-icon>
          <Grid />
        </el-icon>
        <span>服务器管理</span>
      </template>
    </el-menu-item>
    <el-menu-item index="/datasource">
      <template #title>
        <el-icon>
          <Refrigerator />
        </el-icon>
        <span>数据源管理</span>
      </template>
    </el-menu-item>
    <el-menu-item index="/template">
      <template #title>
        <el-icon>
          <Document />
        </el-icon>
        <span>模板管理</span>
      </template>
    </el-menu-item>
    <el-menu-item index="/quickDeploy">
      <template #title>
        <el-icon>
          <CaretRight />
        </el-icon>
        <span>快速部署配置</span>
      </template>
    </el-menu-item>
    <el-menu-item index="/setting">
      <template #title>
        <el-icon>
          <Setting />
        </el-icon>
        <span>设置</span>
      </template>
    </el-menu-item>
  </el-menu>
  <div class="content" :style="{'height': state.height +'px'}">
    <router-view></router-view>
  </div>
</template>

<script lang="ts" setup>
import { ref, onMounted, onUnmounted, reactive } from 'vue'
import { Platform, Setting, Document, Grid, Refrigerator, CaretRight } from '@element-plus/icons-vue'
import { useRouter } from 'vue-router'
import { Event, listen } from "@tauri-apps/api/event";
import utils from '@/libs/utils'

listen('error', (event: Event<string>)=> {
    utils.error(event.payload)
})

const state = reactive({
  height: window.innerHeight - 100
});
const router = useRouter();
const activeIndex = ref('/')

const handleSelect = (key: string, keyPath: string[]) => {
  router.push({path: key})
}

const resetHeight = (e)=> {
  state.height = window.innerHeight - 100
}
onMounted(()=> {
  window.addEventListener('resize', resetHeight)
})
onUnmounted(()=> {
  window.removeEventListener('resize', resetHeight)
})

</script>
<style>
.el-tree-node.is-current > .el-tree-node__content {
  background-color: #313D4F !important;
}

.content {
  margin: 10px;
  overflow-y: scroll;
}

.tool {
  margin: 20px 0 0 0;
  padding-bottom: 20px;
}
.inline-tool {
  margin: 5px 0 0 0;
  padding-bottom: 20px;
}
.custom-tree-node {
  flex: 1;
  display: flex;
  align-items: center;
  justify-content: space-between;
  font-size: 14px;
  padding-right: 8px;
}
.custom-node {
  margin: 5px 0 !important;
}
.border-right {
  border-right: 1px solid #414243;
}

.custom-row {
  height: calc(100% - 70px);
  margin-left: 0 !important;
  margin-right: 0 !important;
}
.custom-col {
  height: calc(100%);
}
</style>
