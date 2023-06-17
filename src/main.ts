import devtools from "@vue/devtools";
import "virtual:svg-icons-register";
import {createApp} from "vue";
import App from "./App.vue";
import "./assets/main.postcss";
import router from "./router";

if (process.env.NODE_ENV === "development") {
    devtools.connect("http://localhost", 8098);
}

createApp(App).use(router).mount("#app");
