import { createRouter, createWebHistory, RouteRecordRaw } from 'vue-router'
import Project from '../views/project.vue'
import Server from '../views/server.vue'
import Datasource from '../views/datasource.vue'
import Template from '../views/template.vue'
import QuickDeploy from '../views/quick_deploy.vue'
import Setting from '../views/setting.vue'

const routes: Array<RouteRecordRaw> = [
  { path: '/', name: 'project', component: Project },
  { path: '/server', name: 'server', component: Server },
  { path: '/datasource', name: 'datasource', component: Datasource },
  { path: '/template', name: 'template', component: Template },
  { path: '/quickDeploy', name: 'quickDeploy', component: QuickDeploy},
  { path: '/setting', name: 'setting', component: Setting }
]

const router = createRouter({
  history: createWebHistory(process.env.BASE_URL),
  routes
})

export default router
