import Search from '@/views/Search.vue';
import { RouteRecordRaw, createRouter, createWebHistory } from 'vue-router';

export const routes: RouteRecordRaw[] = [
  {
    path: '/',
    name: 'login',
    component: Search,
  },
];

export const router = createRouter({
  history: createWebHistory(),
  routes,
});
