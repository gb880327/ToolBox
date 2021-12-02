import Vue from "vue";
import VueRouter from "vue-router";
let routes = [{
        path: "/",
        name: "main",
        component: (resolve) => require(["@/views/main.vue"], resolve),
    },
    {
        path: "/project",
        name: "project",
        component: (resolve) => require(["@/views/project/index.vue"], resolve),
    },
    {
        path: "/deploy",
        name: "deploy",
        component: (resolve) => require(["@/views/deploy/index.vue"], resolve),
    },
    {
        path: "/deploy/setting",
        name: "deploySetting",
        component: (resolve) => require(["@/views/deploy/setting.vue"], resolve),
    },
];
const router = new VueRouter({
    mode: "history",
    routes,
});
Vue.use(VueRouter);
export default router;