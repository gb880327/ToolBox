import Vue from "vue";
import VueRouter from "vue-router";
let routes = [{
        path: "/",
        name: "project",
        component: (resolve) => require(["@/views/project/index.vue"], resolve),
        meta: {
            title: '项目管理'
        }
    },
    {
        path: "/deploy",
        name: "deploy",
        component: (resolve) => require(["@/views/deploy/index.vue"], resolve),
        meta: {
            key: '/deploy',
            title: '项目部署'
        }
    },
    {
        path: "/server",
        name: "server",
        component: (resolve) => require(["@/views/server/index.vue"], resolve),
        meta: {
            key: '/deploy',
            title: '服务器管理'
        }
    },
    {
        path: "/codegen",
        name: "codegen",
        component: (resolve) => require(["@/views/codegen/index.vue"], resolve),
        meta: {
            key: '/codegen',
            title: '代码生成'
        }
    },
    {
        path: "/datasource",
        name: "datasource",
        component: (resolve) => require(["@/views/codegen/datasource.vue"], resolve),
        meta: {
            key: '/codegen',
            title: '数据源管理'
        }
    },
    {
        path: "/template",
        name: "template",
        component: (resolve) => require(["@/views/codegen/templates.vue"], resolve),
        meta: {
            key: '/codegen',
            title: '模板管理'
        }
    }
];
const router = new VueRouter({
    mode: "history",
    routes,
});
Vue.use(VueRouter);
export default router;