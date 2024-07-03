import { QueryCache, VueQueryPlugin } from '@tanstack/vue-query';
import { createPinia } from 'pinia';
import { createApp } from 'vue';
import App from './App.vue';
import { router } from './router';
import './styles/index.scss';
import { handleError } from './utils/error';

const app = createApp(App);

app.use(VueQueryPlugin, {
  enableDevtoolsV6Plugin: true,
  queryClientConfig: {
    queryCache: new QueryCache({
      onError: handleError,
    }),
  },
});

const pinia = createPinia();
app.use(pinia);

app.use(router);

app.mount('#app');
