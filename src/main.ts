import { VueQueryPlugin } from '@tanstack/vue-query';
import { createPinia } from 'pinia';
import { createApp } from 'vue';
import App from './App.vue';
import { router } from './router';
import './styles/index.scss';

const app = createApp(App);

app.use(VueQueryPlugin);

const pinia = createPinia();
app.use(pinia);

app.use(router);

app.mount('#app');
