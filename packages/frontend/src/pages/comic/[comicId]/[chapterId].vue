<script setup lang="ts">
import { getChapter, proxyImage } from '@/api';
import LazyImage from '@/components/lazy-image/LazyImage.vue';
import LazyImageContainer from '@/components/lazy-image/LazyImageContainer.vue';
import { ScrollArea } from '@/components/ui/scroll-area';
import { Slider } from '@/components/ui/slider';
import { useQuery } from '@tanstack/vue-query';
import { ArrowLeftToLine, ArrowRightToLine, ChevronLeft, LoaderCircle } from 'lucide-vue-next';
import { computed, ref, useTemplateRef } from 'vue';
import { useRoute, useRouter } from 'vue-router';

const route = useRoute('/comic/[comicId]/[chapterId]');
const router = useRouter();

const comicId = computed(() => route.params.comicId);
const chapterId = computed(() => route.params.chapterId);

const { data } = useQuery({
  queryKey: [getChapter.name, comicId, chapterId],
  queryFn: () => getChapter({ comicId: comicId.value, chapterId: chapterId.value }),
});

const images = computed(() => data.value?.images || []);

const activeIndex = ref([0]);

const listRef = useTemplateRef('list-container');

const pageShow = ref(false);

function onSlideChange() {
  pageShow.value = true;
}

function onSlideEnd(index: number[]) {
  (listRef.value?.$el as HTMLDivElement).firstElementChild?.scrollTo({
    top: window.innerHeight * index[0],
    behavior: 'instant',
  });
  pageShow.value = false;
}

const maskShow = ref(false);

function toPreviousChapter() {
  if (data.value?.prevId) {
    router.replace(`/comic/${comicId.value}/${data.value.prevId}`);
  }
}

function toNextChapter() {
  if (data.value?.nextId) {
    router.replace(`/comic/${comicId.value}/${data.value.nextId}`);
  }
}
</script>

<template>
  <div v-if="!data" class="flex h-full w-full items-center justify-center">
    <LoaderCircle class="animate-spin" />
  </div>

  <ScrollArea
    v-else
    ref="list-container"
    class="h-full w-full"
    viewport-class="snap-y snap-mandatory relative"
    type="scroll"
  >
    <LazyImageContainer @click="maskShow = true">
      <LazyImage
        v-for="(image, index) in images"
        :key="image"
        class="h-screen w-screen snap-start object-contain"
        :src="proxyImage(image)"
        :index
        @activated="activeIndex = [index]"
      />
    </LazyImageContainer>

    <div v-if="data && maskShow" class="fixed inset-0 z-10 flex flex-col justify-between bg-black bg-opacity-40">
      <div class="flex h-12 items-center bg-zinc-900 text-white">
        <div class="p-2" @click="router.go(-1)">
          <ChevronLeft :size="28" />
        </div>
        <div class="min-w-0 flex-1 truncate">{{ data.comicName }} - {{ data.name }}</div>
      </div>

      <div
        class="flex-1"
        @click="
          maskShow = false;
          pageShow = false;
        "
      />

      <div class="relative p-4">
        <div class="flex h-16 w-full items-center rounded-lg bg-zinc-900 bg-opacity-20 shadow-lg backdrop-blur-md">
          <div class="p-4 text-white" @click="toPreviousChapter">
            <ArrowLeftToLine :size="28" />
          </div>

          <Slider
            v-model="activeIndex"
            :min="0"
            :max="images.length - 1"
            @value-commit="onSlideEnd"
            @update:model-value="onSlideChange"
          />

          <div class="p-4 text-white" @click="toNextChapter">
            <ArrowRightToLine :size="28" />
          </div>
        </div>

        <div
          v-if="pageShow"
          class="absolute -top-4 left-1/2 min-w-20 -translate-x-1/2 rounded bg-zinc-900 bg-opacity-50 px-1 text-center text-white shadow-lg backdrop-blur-sm"
        >
          <span class="text-blue-400">{{ activeIndex[0] + 1 }}</span>
          /
          <span>{{ images.length }}</span>
        </div>
      </div>
    </div>
  </ScrollArea>
</template>
