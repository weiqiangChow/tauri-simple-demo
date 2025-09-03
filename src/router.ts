import { createRouter, createWebHistory, RouteRecordRaw } from "vue-router";
import Home from "./views/Home.vue";
import Great from "./views/Great.vue";
import ElementTest from "./views/ElementTest.vue";
import ConfigSetting from "./views/ConfigSetting.vue";

const routes: Array<RouteRecordRaw> = [
  {
    path: "/",
    name: "Home",
    component: Home,
  },
  {
    path: "/great",
    name: "Great",
    component: Great,
  },
  {
    path: "/element-test",
    name: "ElementTest",
    component: ElementTest,
  },
  {
    path: "/config-setting",
    name: "ConfigSetting",
    component: ConfigSetting,
  },
];

const router = createRouter({
  history: createWebHistory(),
  routes,
});

export default router;
