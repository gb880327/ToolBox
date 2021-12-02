import Vue from "vue";
import router from './router'
import ElementUI from 'element-ui';
// import 'element-ui/lib/theme-chalk/index.css';
import '@/assets/style/theme/index.css'
import App from "./App.vue";
import MyPagination from "@/components/pagination";

// import { listen } from "@tauri-apps/api/event";
// import { invoke } from "@tauri-apps/api/tauri";

Vue.use(ElementUI);
Vue.component('myPagination', MyPagination)
Vue.config.productionTip = false;

// listen("error", (event) => {
//     alert(event.payload);
// });

// let callback = {}

// Vue.prototype.invoke = (method, cb, param) => {
//     callback[method] = cb
//     return invoke("exec", {
//         params: { method, param },
//     });
// };
// listen('response', (event) => {
//     let data = JSON.parse(event.payload);
//     callback[data.method](data);
//     delete callback[data.method]
// })



new Vue({
    render: (h) => h(App),
    router,
}).$mount("#app");