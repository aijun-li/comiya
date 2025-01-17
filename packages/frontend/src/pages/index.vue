<script setup lang="ts">
import { searchComic } from '@/api';
import { Input } from '@/components/ui/input';
import { useQuery } from '@tanstack/vue-query';
import { useDebounce } from '@vueuse/core';
import { computed, ref } from 'vue';
import { RouterLink } from 'vue-router';

const keyword = ref('');
const debounced = useDebounce(keyword, 500);

const { data } = useQuery({
  queryKey: [searchComic.name, debounced],
  queryFn: () => searchComic({ keyword: debounced.value }),
  placeholderData: (prev) => prev,
  enabled: () => Boolean(debounced.value.trim()),
});

const comics = computed(() => data.value || []);
</script>

<template>
  <div class="flex h-full w-full flex-col items-center px-12 py-16">
    <div class="transition-all duration-300" :class="[comics.length ? 'h-0' : 'h-1/4']" />
    <div class="flex w-1/2 flex-col items-center gap-8">
      <div class="text-5xl font-semibold">Comiya</div>
      <Input v-model="keyword" autofocus />
    </div>

    <div class="mt-12 grid max-w-[1100px] grid-cols-5 gap-8">
      <RouterLink v-for="comic in comics" :key="comic.id" class="w-full cursor-pointer" :to="`/comic/${comic.id}`">
        <img class="w-full rounded-sm" :src="comic.cover" />
        <div class="mt-2 truncate">{{ comic.name }}</div>
      </RouterLink>
    </div>
  </div>
</template>
