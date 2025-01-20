<script setup lang="ts">
import { getComic } from '@/api';
import type { ComicChapterGroup } from '@/api/types';
import { Button } from '@/components/ui/button';
import { useQuery } from '@tanstack/vue-query';
import { ArrowDown01, ArrowDown10, ChevronLeft, LoaderCircle } from 'lucide-vue-next';
import { computed, ref, watch } from 'vue';
import { RouterLink, useRoute, useRouter } from 'vue-router';

const route = useRoute('/comic/[comicId]/');
const router = useRouter();

const comicId = computed(() => route.params.comicId);

const { data: comic } = useQuery({
  queryKey: [getComic.name, comicId],
  queryFn: () => getComic({ id: comicId.value }),
});

const chapterGroups = ref<(ComicChapterGroup & { reverse: boolean })[]>([]);

watch(
  comic,
  () => {
    if (!comic.value) {
      return;
    }
    chapterGroups.value = comic.value.chapterGroups.map((group) => ({
      ...group,
      reverse: true,
    }));
  },
  { immediate: true },
);
</script>

<template>
  <div v-if="!comic" class="flex h-full w-full items-center justify-center">
    <LoaderCircle class="animate-spin" />
  </div>

  <div v-else class="flex h-full w-full justify-center px-6 py-8 lg:px-12 lg:py-16">
    <Button class="fixed left-2 top-2" variant="outline" size="icon" @click="router.go(-1)">
      <ChevronLeft />
    </Button>

    <div class="max-w-[1200px]">
      <div class="flex flex-col gap-4 sm:flex-row">
        <img :src="comic.cover" class="aspect-[3/4] max-w-[50%] self-start object-contain" />
        <div>
          <div class="text-xl font-semibold">{{ comic.name }}</div>
          <div class="mt-2 text-sm">{{ comic.pubDate }}</div>
          <div>{{ comic.author.join(', ') }}</div>
          <div class="mt-2">{{ comic.intro }}</div>
        </div>
      </div>

      <div class="mt-4 pb-4">
        <div v-for="group in chapterGroups" :key="group.name" class="mt-4">
          <div class="flex items-center justify-between py-1 font-semibold">
            {{ group.name }}
            <Button size="sm" variant="ghost" @click="group.reverse = !group.reverse">
              <ArrowDown10 v-if="group.reverse" :size="40" />
              <ArrowDown01 v-else />
            </Button>
          </div>
          <div class="mt-2 grid grid-cols-4 gap-1 sm:grid-cols-5 md:grid-cols-6 md:gap-2 lg:grid-cols-7 xl:grid-cols-8">
            <RouterLink
              v-for="chapter in group.reverse ? [...group.chapters].reverse() : group.chapters"
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
