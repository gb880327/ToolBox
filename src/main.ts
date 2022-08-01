import { createApp, provide } from "vue";
import App from "./App.vue";
import router from "./router";
import ElementPlus from "element-plus";
import "element-plus/dist/index.css";
import "element-plus/theme-chalk/dark/css-vars.css";

import { Event, listen } from "@tauri-apps/api/event";
import { invoke } from "@tauri-apps/api";

const app = createApp(App);

let callback = {};
app.provide("invoke", (method: string, cb: Function, param) => {
  callback[method] = cb;
  return invoke("exec", {
    params: { method, param },
  });
});

listen("response", (event: Event<string>) => {
  let data = JSON.parse(event.payload);
  callback[data.method](data.data);
  delete callback[data.method];
});

app.use(ElementPlus, { size: "default" }).use(router).mount("#app");
