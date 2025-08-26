import '@/theme/style.css'
import App from "./App.vue";
import DQC from "./DQC";
import { router } from "./router";

export const app = DQC(App, {
    router,
    modules: []
});
