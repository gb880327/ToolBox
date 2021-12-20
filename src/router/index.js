import Vue from "vue";
import VueRouter from "vue-router";
let routes = [{
        path: "/",
        name: "project",
        component: (resolve) => require(["@/views/project/index.vue"], resolve),
    },
    {
        path: "/deploy",
        name: "deploy",
        component: (resolve) => require(["@/views/deploy/index.vue"], resolve),
    },
    {
        path: "/server",
        name: "server",
        component: (resolve) => require(["@/views/server/index.vue"], resolve),
    },
    {
        path: "/codegen",
        name: "codegen",
        component: (resolve) => require(["@/views/codegen/index.vue"], resolve)
    },
    {
        path: "/datasource",
        name: "datasource",
        component: (resolve) => require(["@/views/codegen/datasource.vue"], resolve)
    },
    {
        path: "/template",
        name: "template",
        component: (resolve) => require(["@/views/codegen/template.vue"], resolve)
    }
];
const router = new VueRouter({
    mode: "history",
    routes,
});
Vue.use(VueRouter);
export default router;