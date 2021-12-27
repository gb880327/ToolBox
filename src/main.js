import Vue from "vue";
import router from './router'
import ElementUI from 'element-ui';
// import 'element-ui/lib/theme-chalk/index.css';
import '@/assets/style/theme/index.css'
import Swal from 'sweetalert2'
import App from "./App.vue";
import mixin from './mixin'
import MyPagination from "@/components/pagination";
import MyDialog from '@/components/MyDialog'
import Treeselect from '@riophae/vue-treeselect'
import '@riophae/vue-treeselect/dist/vue-treeselect.css'
import config from '@/libs/config'
import { listen } from "@tauri-apps/api/event";
import { invoke } from "@tauri-apps/api/tauri";

Vue.use(ElementUI);
Vue.component('Treeselect', Treeselect)
Vue.component('myPagination', MyPagination)
Vue.component('myDialog', MyDialog)
Vue.mixin(mixin)
Vue.config.productionTip = false;
Vue.prototype.config = config

let callback = {}

Vue.prototype.invoke = (method, cb, param) => {
    callback[method] = cb
    return invoke("exec", {
        params: { method, param },
    });
};
listen('response', (event) => {
    let data = JSON.parse(event.payload);
    callback[data.method](data.data);
    delete callback[data.method]
})

const Toast = Swal.mixin({
    toast: true,
    position: 'top-end',
    showConfirmButton: false,
    timer: 3000,
    timerProgressBar: true,
    didOpen: (toast) => {
        toast.addEventListener('mouseenter', Swal.stopTimer)
        toast.addEventListener('mouseleave', Swal.resumeTimer)
    }
})

Vue.prototype.success = (msg) => {
    Toast.fire({
        icon: 'success',
        title: msg
    })
}

Vue.prototype.error = (msg) => {
    Toast.fire({
        title: msg,
        icon: 'error',
    })
}

Vue.prototype.confirm = (msg) => {
    return new Promise((resolve, reject) => {
        Swal.fire({
            title: msg ? msg : '确定要删除该信息?',
            icon: 'warning',
            showCancelButton: true,
            confirmButtonColor: '#3085d6',
            cancelButtonColor: '#d33',
            cancelButtonText: '取消',
            confirmButtonText: '确认!'
        }).then(result => {
            resolve(result.isConfirmed)
        })
    })
}

Vue.prototype.Swal = Swal

Vue.prototype.goto = (page, params) => {
    router.push({ path: page, query: params })
}


new Vue({
    render: (h) => h(App),
    router,
}).$mount("#app");