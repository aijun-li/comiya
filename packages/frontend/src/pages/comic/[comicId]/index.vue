<script setup lang="ts">
import { addToLibrary, checkInLibrary, getComic, getComicHistory, removeFromLibrary } from '@/api';
import type { ComicChapterGroup } from '@/api/types';
import { Button } from '@/components/ui/button';
import { useActivated } from '@/hooks/activated';
import { handleError } from '@/utils/error';
import { useQuery } from '@tanstack/vue-query';
import { ArrowDown01, ArrowDown10, ChevronLeft, LoaderCircle, Star } from 'lucide-vue-next';
import { computed, ref, watch } from 'vue';
import { RouterLink, useRoute, useRouter } from 'vue-router';

const route = useRoute('/comic/[comicId]/');
const router = useRouter();

const comicId = computed(() => route.params.comicId);

const activated = useActivated();

const { data: comic } = useQuery({
  queryKey: [getComic.name, comicId],
  queryFn: () => getComic({ id: comicId.value }),
  refetchOnWindowFocus: false,
  staleTime: 5 * 60 * 1000,
  enabled: () => Boolean(activated.value && comicId.value),
});

const chapterGroups = ref<(ComicChapterGroup & { reverse: boolean })[]>([]);

const { data: checkData, refetch: check } = useQuery({
  queryKey: [checkInLibrary.name, comicId],
  queryFn: () => checkInLibrary({ id: comicId.value }),
  enabled: () => Boolean(activated.value && comicId.value),
});

const inLibrary = computed(() => checkData.value?.inLibrary ?? false);

const { data: historyData } = useQuery({
  queryKey: [getComicHistory.name, comicId],
  queryFn: () => getComicHistory({ id: comicId.value }),
  enabled: () => Boolean(activated.value && comicId.value),
});

const history = computed(() => historyData.value?.history);

function toRead() {
  const chapterId = history.value ? `${history.value.chapterId}?page=${history.value.page}` : comic.value?.firstChapterId ?? '';
  if (chapterId) {
    router.push(`/comic/${comicId.value}/${chapterId}`);
  }
}

async function addComic() {
  if (!comic.value) {
    return;
  }
  try {
    await addToLibrary({
      id: comic.value.id,
      name: comic.value.name,
      cover: comic.value.cover,
    });
    await check();
  } catch (error) {
    handleError(error);
  }
}

async function removeComic() {
  if (!comic.value) {
    return;
  }
  try {
    await removeFromLibrary({
      id: comic.value.id,
    });
    await check();
  } catch (error) {
    handleError(error);
  }
}

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

  <div v-else class="flex h-full w-full justify-center px-6 py-14 lg:px-12 lg:py-16">
    <Button class="fixed left-2 top-2" variant="outline" size="icon" @click="router.go(-1)">
      <ChevronLeft />
    </Button>

    <div class="max-w-[1200px]">
      <div class="flex flex-col gap-4 sm:flex-row">
        <img :src="comic.cover" class="aspect-[3/4] max-w-[50%] self-start object-contain" />
        <div>
          <div class="flex items-center text-xl font-semibold">
            {{ comic.name }}
            <Button class="ml-1" variant="ghost" size="icon" @click="inLibrary ? removeComic() : addComic()">
              <Star v-if="!inLibrary" class="!h-5 !w-5" />
              <Star v-else fill="#facc15" stroke-width="0" class="!h-5 !w-5" />
            </Button>
          </div>
          <div class="mt-2 text-sm">{{ comic.pubDate }}</div>
          <div>{{ comic.author.join(', ') }}</div>
          <div class="mt-2">{{ comic.intro }}</div>
        </div>
      </div>

      <Button class="mt-4" @click="toRead">
        <template v-if="history"> 继续阅读 [{{ history.chapterName }}] </template>
        <template v-else> 开始阅读 </template>
      </Button>

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
