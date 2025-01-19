import { VueQueryPlugin } from '@tanstack/vue-query';
import { createPinia } from 'pinia';
import { registerSW } from 'virtual:pwa-register';
import { createApp } from 'vue';
import App from './App.vue';
import { router } from './router';
import './style.css';

registerSW({ immediate: true });

const pinia = createPinia();

createApp(App)
  .use(router)
  .use(pinia)
  .use(VueQueryPlugin, {
    enableDevtoolsV6Plugin: true,
  })
  .mount('#app');
