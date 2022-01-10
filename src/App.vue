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
    <keep-alive>
      <router-view class="router"></router-view>
    </keep-alive>
  </div>
</template>

<script>
import { invoke } from "@tauri-apps/api/tauri";
import { listen } from "@tauri-apps/api/event";

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
        titleMap: {'/deploy': '项目部署', '/codegen': '代码生成'},
        logs: [],
        showProgress: false,
        percentage: 0,
        isDeploy: false,
        hasError: false
      }
  },
  created(){
    listen("error", (event) => {
        this.error(event.payload)
    });
    listen("console", (event) => {
      if(event.payload === 'over') {
        this.isDeploy = false
        return
      } 
      this.logs.push({msg: event.payload, type: 0})
      this.$nextTick(()=> {
        this.goToEnd()
      })
    })
    listen('console_error', (event)=>{
      this.hasError = true
      this.logs.push({msg: event.payload, type: 1})
      this.$nextTick(()=> {
        this.goToEnd()
      })
    })
    listen('console_progress', (event)=> {
      if(!this.showProgress){
        this.showProgress = true
      }
      this.percentage = parseInt(event.payload)
      if(this.percentage >= 100){
        this.percentage = 0
        this.showProgress = false
      }
      this.$nextTick(()=> {
        this.goToEnd()
      })
    })
    this.$router.beforeEach((to, from, next)=> {
      if(this.isDeploy){
        return
      }
      next()
    })
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
  mounted(){
    window.addEventListener('keydown', this.keyDownHander, false)
  },
  destroyed(){
    window.removeEventListener("keydown", this.keyDownHander, false)
  },
  methods: {
    goToEnd(){
      let div = document.getElementsByClassName('console')[0]
      div.scrollTop = div.scrollHeight + 20
    },
    keyDownHander(e){
        if(e.keyCode === 67 && e.metaKey){
          document.execCommand('copy');
        } else if (e.keyCode === 86 && e.metaKey){
          document.execCommand('paste')
        } else if (e.keyCode === 81 && e.metaKey){
          invoke('exit')
        }
    }
  }
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
    z-index: 99999 !important;
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
