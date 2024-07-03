<script lang="ts" setup>
import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';
import { useSearch } from '@/sites/manhuagui';
import { ref } from 'vue';

const keyword = ref('');
const hasSearched = ref(false);

const { isFetching, data: results, refetch } = useSearch(keyword);

function onSearch() {
  hasSearched.value = true;
  refetch();
}
</script>

<template>
  <div class="w-full h-full pt-8 flex items-center flex-col">
    <div
      :class="[
        'flex-center text-6xl font-bold transition-all duration-500 overflow-hidden',
        hasSearched ? 'h-0 opacity-0' : 'h-1/3 opacity-100',
      ]"
    >
      Comiya
    </div>
    <div class="w-full flex-center gap-4 mb-8 px-8">
      <Input class="flex-1" v-model="keyword" />
      <Button :loading="isFetching" @click="onSearch">Search</Button>
    </div>

    <div v-if="results" class="w-full grid grid-cols-5 auto-rows-min gap-4 flex-1 min-h-0 overflow-auto px-8 pb-8">
      <div v-for="result in results" :key="result.url" class="cursor-pointer w-full">
        <img class="w-full aspect-[3/4] object-cover rounded-md border" :src="result.cover" />
        <div class="truncate">{{ result.name }}</div>
      </div>
    </div>
  </div>
</template>
