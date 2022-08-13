import { createApp } from "vue";
import { createPinia } from "pinia";

import App from "./App.vue";

import router from "./router";

import axios from "axios";
import VueAxios from "vue-axios";


// import "~/styles/element/index.scss";

// import ElementPlus from "element-plus";
// import all element css, uncommented next line
// import "element-plus/dist/index.css";

// or use cdn, uncomment cdn link in `index.html`

import "~/styles/index.scss";
import 'uno.css'

// If you want to use ElMessage, import it.
import "element-plus/theme-chalk/src/message.scss"

const app = createApp(App);
app.use(createPinia());
app.use(VueAxios, axios);
app.use(router);

axios.defaults.baseURL =
  import.meta.env.VITE_API_URL !== undefined
    ? import.meta.env.VITE_API_URL
        : "/api";
    
// app.use(ElementPlus);
app.mount("#app");
