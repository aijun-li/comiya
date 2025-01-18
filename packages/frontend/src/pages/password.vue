<script setup lang="ts">
import { checkPassword } from '@/api';
import { Input } from '@/components/ui/input';
import { Label } from '@/components/ui/label';
import { useGlobalStore } from '@/stores/global';
import { useQuery } from '@tanstack/vue-query';
import { useDebounce } from '@vueuse/core';
import { ref, watchEffect } from 'vue';

const input = ref('');
const debounced = useDebounce(input, 800);

const { data, isFetching } = useQuery({
  queryKey: [checkPassword.name, 'input', debounced],
  queryFn: () => checkPassword({ password: debounced.value }),
  enabled: () => Boolean(debounced.value),
});

const { setPassword } = useGlobalStore();

const invalid = ref(false);

watchEffect(() => {
  if (data.value?.valid) {
    setPassword(debounced.value);
  } else if (data.value?.valid === false) {
    invalid.value = true;
  }
});
</script>

<template>
  <div class="flex h-full w-full items-center justify-center">
    <div class="flex w-4/5 max-w-[500px] flex-col gap-2 rounded px-12 pb-8 pt-6 shadow-md">
      <Label class="font-semibold" :class="{ 'text-red-500': invalid }">Password</Label>
      <Input v-model="input" autofocus :disabled="isFetching" @update:model-value="invalid = false" />
    </div>
  </div>
</template>
