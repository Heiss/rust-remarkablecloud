import { createApp } from "vue";
import { createPinia } from "pinia";

import App from "./App.vue";
import router from "./router";
import axios from "axios";
import VueAxios from "vue-axios";

import "~/styles/index.scss";
import "uno.css";

import "element-plus/theme-chalk/src/message.scss";
import { merge } from "lodash";

const app = createApp(App);
app.use(createPinia());
app.use(VueAxios, axios);
app.use(router);

axios.defaults = merge(axios.defaults, {
  baseURL:
    import.meta.env.VITE_API_URL !== undefined
      ? import.meta.env.VITE_API_URL
      : "/api",
  withCredentials: true,
  responseType: "text",
  transformResponse: [(v: string) => v],
});

app.mount("#app");
