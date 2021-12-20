<template>
  <div id="app">
    <el-menu :default-active="activeIndex" mode="horizontal" :router="true" background-color="#313D4F" text-color="#A9C7DF" active-text-color="#ffd04b">
      <el-menu-item index="/">项目管理</el-menu-item>
      <el-submenu index="/deploy">
        <template slot="title">{{titleMap['/deploy']}}</template>
        <el-menu-item index="/deploy">项目部署</el-menu-item>
        <el-menu-item index="/server">服务器管理</el-menu-item>
      </el-submenu>
      <el-submenu index="/codegen">
        <template slot="title">{{titleMap['/codegen']}}</template>
        <el-menu-item index="/codegen">代码生成</el-menu-item>
        <el-menu-item index="/datasource">数据源管理</el-menu-item>
        <el-menu-item index="/template">模板管理</el-menu-item>
      </el-submenu>
    </el-menu>
    <router-view class="router"></router-view>
  </div>
</template>

<script>

export default {
  name: 'App',
  data() {
      return {
        activeIndex: "/",
        titleMap: {'/deploy': '项目部署', '/codegen': '代码生成'}
      }
  },
  created(){
    this.$router.afterEach((to, _from) => {
      this.activeIndex = to.path
    })

    // this.$router.push({path: this.activeIndex})
    // this.invoke('ProjectList',this.getProjects)
    // this.invoke('DeployProject', this.getProjects, {projectId: 1})
  },
}
</script>

<style>
  body {
    margin: 0;
    padding: 0;
    background: #222C3C;    
    color: #FFFFFF;
  }
  .router {
    padding: 10px 10px;
  }
  .swal2-container{
    z-index: 9999;
  }
  .tool{
    margin-bottom: 10px;
  }
  .el-icon-see {
    width: 24px;
    height: 24px;
    background-image: url('./assets/eyes.png');
    background-size: 24px;
    background-repeat: no-repeat;
    background-position: center;
    vertical-align: middle;
    cursor: pointer;
  }
  .el-icon-deploy {
    width: 12px;
    height: 12px;
    background-image: url('./assets/deploy.png');
    background-size: 12px;
    background-repeat: no-repeat;
    background-position: center;
    vertical-align: middle;
    cursor: pointer;
  }
  .el-icon-generated {
    width: 12px;
    height: 12px;
    background-image: url('./assets/generated.png');
    background-size: 12px;
    background-repeat: no-repeat;
    background-position: center;
    vertical-align: middle;
    cursor: pointer;
  }
</style>
