import {createRouter, createWebHashHistory, Router, RouteRecordRaw, RouterOptions} from "vue-router";

const routes: RouteRecordRaw[] = [
    {path: "/", redirect: "/offline"},
    {path: "/offline", name: "OfflineParser", component: () => import("../views/OfflineParser.vue")},
    {path: "/online", name: "OnlineParser", component: () => import("../views/OnlineParser.vue")},
    {path: "/bind", name: "BindInfo", component: () => import("../views/BindInfo.vue")},
];

const options: RouterOptions = {
    history: createWebHashHistory(),
    routes,
};

const router: Router = createRouter(options);

export default router;
