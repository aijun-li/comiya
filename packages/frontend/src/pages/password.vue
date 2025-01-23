<script setup lang="ts">
import { checkPassword } from '@/api';
import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';
import { Label } from '@/components/ui/label';
import { useGlobalStore } from '@/stores/global';
import { useQuery } from '@tanstack/vue-query';
import { useMediaQuery } from '@vueuse/core';
import { ref, watch } from 'vue';
import { useRouter } from 'vue-router';

const input = ref('');

const isPWA = useMediaQuery('(display-mode: fullscreen), (display-mode: standalone)');

const router = useRouter();

const {
  data,
  isFetching,
  refetch: check,
} = useQuery({
  queryKey: [checkPassword.name, 'input'],
  queryFn: () => checkPassword({ password: input.value }),
  enabled: false,
});

const { setPassword } = useGlobalStore();

const invalid = ref(false);

watch(data, () => {
  if (data.value?.valid) {
    setPassword(input.value);
    router.replace('/search');
  } else if (data.value?.valid === false) {
    invalid.value = true;
  }
});
</script>

<template>
  <div class="flex h-full w-full items-center justify-center" :class="{ 'pb-[4rem]': !isPWA }">
    <div class="flex w-4/5 max-w-[500px] flex-col gap-2 rounded px-6 pb-8 pt-6 shadow">
      <Label class="font-semibold" :class="{ 'text-red-500': invalid }">Password</Label>
      <div class="flex items-center gap-2">
        <Input
          v-model="input"
          class="text-base"
          type="password"
          autofocus
          :disabled="isFetching"
          @update:model-value="invalid = false"
        />
        <Button class="min-w-16" :disabled="isFetching" @click="() => check()">GO</Button>
      </div>
    </div>
  </div>
</template>
