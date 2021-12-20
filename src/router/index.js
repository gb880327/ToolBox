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
];
const router = new VueRouter({
    mode: "history",
    routes,
});
Vue.use(VueRouter);
export default router;