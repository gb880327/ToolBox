import Vue from "vue";
import App from "./App.vue";
import { listen } from "@tauri-apps/api/event";
import { invoke } from "@tauri-apps/api/tauri";

Vue.config.productionTip = false;

listen("error", (event) => {
    alert(event.payload);
});

let callback = {}

Vue.prototype.invoke = (method, cb, param) => {
    callback[method] = cb
    return invoke("exec", {
        params: { method, param },
    });
};
listen('response', (event) => {
    let data = JSON.parse(event.payload);
    callback[data.method](data);
})



new Vue({
    render: (h) => h(App),
}).$mount("#app");