<script setup lang="ts">
import { getLibrary } from '@/api';
import { Button } from '@/components/ui/button';
import { useQuery } from '@tanstack/vue-query';
import { ChevronLeft } from 'lucide-vue-next';
import { computed, onActivated } from 'vue';
import { RouterLink, useRouter } from 'vue-router';

const router = useRouter();

const { data, refetch } = useQuery({
  queryKey: [getLibrary.name],
  queryFn: () => getLibrary(),
});

const comics = computed(() => data.value || []);

onActivated(refetch);
</script>

<template>
  <div class="flex h-full w-full flex-col gap-4 px-6 py-14 lg:px-12 lg:py-16">
    <Button class="fixed left-2 top-2" variant="outline" size="icon" @click="router.go(-1)">
      <ChevronLeft />
    </Button>

    <div class="grid grid-cols-3 gap-4 sm:grid-cols-4 sm:gap-6 md:grid-cols-5 md:gap-8 lg:grid-cols-6">
      <RouterLink v-for="comic in comics" :key="comic.id" class="w-full" :to="`/comic/${comic.id}`">
        <img class="aspect-[3/4] w-full rounded-sm shadow-lg" :src="comic.cover" />
        <div class="mt-2 truncate">{{ comic.name }}</div>
      </RouterLink>
    </div>
  </div>
</template>
