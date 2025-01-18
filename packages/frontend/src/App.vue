<script setup lang="ts">
import { useGlobalStore } from '@/stores/global';
import { useQuery } from '@tanstack/vue-query';
import cookies from 'js-cookie';
import { LoaderCircle } from 'lucide-vue-next';
import { watch } from 'vue';
import { useRouter } from 'vue-router';
import { checkPassword } from './api';

const { password } = useGlobalStore();

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
    router.replace('/search');
    cookies.set('comiya-password', password.value);
  } else {
    router.replace('/password');
  }
});
</script>

<template>
  <div class="h-screen w-screen">
    <div v-if="isPending" class="flex h-full w-full items-center justify-center">
      <LoaderCircle class="animate-spin" :size="36" />
    </div>
    <RouterView v-else />
  </div>
</template>
