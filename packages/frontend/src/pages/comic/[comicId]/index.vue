<script setup lang="ts">
import { getComic } from '@/api';
import { useQuery } from '@tanstack/vue-query';
import { computed } from 'vue';
import { RouterLink, useRoute } from 'vue-router';

const route = useRoute('/comic/[comicId]/');

const comicId = computed(() => route.params.comicId);

const { data: comic } = useQuery({
  queryKey: [getComic.name, comicId],
  queryFn: () => getComic({ id: comicId.value }),
});
</script>

<template>
  <div class="flex h-full w-full justify-center px-12 py-16">
    <div v-if="comic" class="w-full max-w-[1200px]">
      <div class="flex gap-8">
        <img :src="comic.cover" />
        <div>
          <div class="text-xl font-semibold">{{ comic.name }}</div>
          <div class="mt-2 text-sm">{{ comic.pubDate }}</div>
          <div>{{ comic.author.join(', ') }}</div>
          <div class="mt-8">{{ comic.intro }}</div>
        </div>
      </div>

      <div class="mt-4">
        <div v-for="group in comic.chapterGroups" :key="group.name" class="mt-4">
          <div class="py-1 font-semibold">{{ group.name }}</div>
          <div class="mt-2 grid grid-cols-8 gap-2">
            <RouterLink
              v-for="chapter in group.chapters"
              :key="chapter.id"
              class="cursor-pointer truncate rounded-sm border px-2 py-1"
              :to="`/comic/${comicId}/${chapter.id}`"
            >
              {{ chapter.name }}
            </RouterLink>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>
