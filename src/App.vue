<template>
  <div id="app">
    <el-menu :default-active="activeIndex" mode="horizontal" :router="true" background-color="#313D4F" text-color="#A9C7DF" active-text-color="#ffd04b">
      <el-menu-item index="/">项目管理</el-menu-item>
      <el-submenu index="/deploy">
        <template slot="title">项目部署</template>
        <el-menu-item index="/deploy">部署</el-menu-item>
        <el-menu-item index="/server">服务器管理</el-menu-item>
      </el-submenu>
      <el-submenu index="/codegen">
        <template slot="title">代码生成</template>
        <el-menu-item index="2-1">代码生成</el-menu-item>
        <el-menu-item index="2-1">项目配置</el-menu-item>
        <el-menu-item index="2-2">数据源管理</el-menu-item>
        <el-menu-item index="2-3">模板管理</el-menu-item>
      </el-submenu>
    </el-menu>
    <router-view class="router"></router-view>
  </div>
</template>

<script>

export default {
  name: 'App',
  components: {},
  data() {
      return {
        activeIndex: "/"
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
    width: 24px;
    height: 24px;
    background-image: url('./assets/generated.png');
    background-size: 24px;
    background-repeat: no-repeat;
    background-position: center;
    vertical-align: middle;
    cursor: pointer;
  }
</style>
