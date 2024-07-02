import Home from '@/views/Home.vue';
import { RouteRecordRaw, createRouter, createWebHistory } from 'vue-router';

export const routes: RouteRecordRaw[] = [
  {
    path: '/',
    name: 'login',
    component: Home,
  },
];

export const router = createRouter({
  history: createWebHistory(),
  routes,
});
