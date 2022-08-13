import { createApp } from "vue";
import { createPinia } from "pinia";

import App from "./App.vue";
import router from "./router";

import "./assets/main.css";

import axios from "axios";
import VueAxios from "vue-axios";

import ElementPlus from "element-plus";
import "element-plus/dist/index.css";

const app = createApp(App);

app.use(createPinia());
app.use(ElementPlus);
app.use(VueAxios, axios);
app.use(router);

console.log(import.meta.env);

axios.defaults.baseURL =
  import.meta.env.VITE_API_URL !== undefined
    ? import.meta.env.VITE_API_URL
    : "/api";
app.mount("#app");
