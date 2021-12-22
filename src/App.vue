<template>
  <div id="app">
    <el-menu :default-active="activeIndex" mode="horizontal" :router="true" background-color="#313D4F" text-color="#A9C7DF" active-text-color="#ffd04b">
      <template v-for="menu of menus">
        <el-menu-item v-if="!menu.subItems" :index="menu.path">{{menu.title}}</el-menu-item>
        <el-submenu v-else :index="menu.path">
          <template slot="title">{{titleMap[menu.path]}}</template>
          <el-menu-item v-for="(sub, k) of menu.subItems" :key="k" :index="sub.path">{{sub.title}}</el-menu-item>
        </el-submenu>
      </template>
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
        menus: [
          { path: '/', title: '项目管理' },
          { path: '/deploy', title: '项目部署', subItems: [ { path: '/deploy', title: '项目部署' }, { path: '/server', title: '服务器管理' } ] },
          { path: '/codegen', title: '代码生成', subItems: [ { path: '/codegen', title: '代码生成' }, { path: '/datasource', title: '数据源管理' }, { path: '/template', title: '模板管理' } ] }
        ],
        titleMap: {'/deploy': '项目部署', '/codegen': '代码生成'}
      }
  },
  created(){
    this.$router.afterEach((to, _from) => {
      this.activeIndex = to.path
      this.titleMap[to.meta.key] = to.meta.title
      this.menus.forEach(item=> {
        if(item.path !== to.meta.key){
          this.titleMap[item.path] = item.title
        }
      })
    })
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
