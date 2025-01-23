<script setup lang="ts">
import { useGlobalStore } from '@/stores/global';
import { useQuery } from '@tanstack/vue-query';
import cookies from 'js-cookie';
import { LoaderCircle } from 'lucide-vue-next';
import { watch } from 'vue';
import { useRoute, useRouter } from 'vue-router';
import { checkPassword } from './api';
import { ScrollArea } from './components/ui/scroll-area';

const { password } = useGlobalStore();

const route = useRoute();
const router = useRouter();

const { data, isPending } = useQuery({
  queryKey: [checkPassword.name, password],
  queryFn: () => checkPassword({ password: password.value }),
});

watch(data, () => {
  if (!data.value) {
    return;
  }
  if (data.value.valid) {
    cookies.set('comiya-password', password.value);
    if (route.name === '/password') {
      router.replace('/search');
    }
  } else {
    router.replace('/password');
  }
});
</script>

<template>
  <ScrollArea class="h-dvh w-dvw" type="scroll" viewport-class="outline-none">
    <div v-if="isPending" class="flex h-full w-full items-center justify-center">
      <LoaderCircle class="animate-spin" />
    </div>
    <RouterView v-else v-slot="{ Component }">
      <KeepAlive>
        <component :is="Component" />
      </KeepAlive>
    </RouterView>
  </ScrollArea>
</template>
