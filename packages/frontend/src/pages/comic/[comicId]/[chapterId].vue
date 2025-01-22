<script setup lang="ts">
import { getChapter, proxyImage, upsertHistory } from '@/api';
import ChapterGestureMask from '@/components/ChapterGestureMask.vue';
import LazyImage from '@/components/LazyImage.vue';
import { Slider } from '@/components/ui/slider';
import { useActivated } from '@/hooks/activated';
import { LocalStorageKey } from '@/types/const';
import { useQuery, useQueryClient } from '@tanstack/vue-query';
import { onKeyStroke, useDebounceFn, useEventListener, useLocalStorage, useMediaQuery } from '@vueuse/core';
import {
  ArrowLeftToLine,
  ArrowRightToLine,
  BookOpenText,
  ChevronLeft,
  LoaderCircle,
  Pointer,
  Settings,
} from 'lucide-vue-next';
import { computed, nextTick, onMounted, onUnmounted, ref, useTemplateRef, watch } from 'vue';
import { useRoute, useRouter } from 'vue-router';

const isPWA = useMediaQuery('(display-mode: fullscreen), (display-mode: standalone)');

const activated = useActivated();

const route = useRoute('/comic/[comicId]/[chapterId]');
const router = useRouter();

const comicId = computed(() => route.params.comicId);
const chapterId = computed(() => route.params.chapterId);

const { data, isFetching } = useQuery({
  queryKey: [getChapter.name, comicId, chapterId],
  queryFn: () => getChapter({ comicId: comicId.value, chapterId: chapterId.value }),
  staleTime: Infinity,
  gcTime: Infinity,
});

const images = computed(() => data.value?.images || []);
const hasNext = computed(() => Boolean(data.value?.nextId && data.value.nextId !== '0'));
const hasPrev = computed(() => Boolean(data.value?.prevId && data.value.prevId !== '0'));

const activeIndex = ref([0]);
const tempIndex = ref([0]);

watch([comicId, chapterId, route], () => {
  activeIndex.value = [0];
  tempIndex.value = [0];
  scrollToPage(0);
});

watch(data, async () => {
  const page = Number(route.query.page);

  if (data.value && page > 0) {
    await nextTick();
    scrollToPage(page - 1);
  }
});

const listRef = useTemplateRef('list-container');

const pageNavShow = ref(false);

function onSlideChange() {
  pageNavShow.value = true;
}

function onSlideEnd(index: number[]) {
  scrollToPage(index[0]);
  pageNavShow.value = false;
}

const controlMaskShow = ref(false);

const gestureTipShowed = useLocalStorage(LocalStorageKey.GestureTipShowed, false);
const showGestureTip = ref(!gestureTipShowed.value);
const gestureReverse = ref(false);
gestureTipShowed.value = true;

const showNextTip = ref(false);
const resetNextTip = useDebounceFn(() => {
  showNextTip.value = false;
}, 2000);
const showPrevTip = ref(false);
const resetPrevTip = useDebounceFn(() => {
  showPrevTip.value = false;
}, 2000);

function scrollToPage(index: number) {
  listRef.value?.scrollTo({
    top: window.innerHeight * Math.min(images.value.length - 1, Math.max(0, index)),
    behavior: 'instant',
  });
}

function toNextPage() {
  const currentPage = activeIndex.value[0];
  if (currentPage === images.value.length - 1) {
    if (!showNextTip.value) {
      showNextTip.value = true;
      resetNextTip();
    } else if (hasNext.value) {
      toNextChapter();
      showNextTip.value = false;
    }
  } else {
    scrollToPage(activeIndex.value[0] + 1);
  }
}

function toPrevPage() {
  const currentPage = activeIndex.value[0];
  if (currentPage === 0) {
    if (!showPrevTip.value) {
      showPrevTip.value = true;
      resetPrevTip();
    } else if (hasPrev.value) {
      toPreviousChapter();
      showPrevTip.value = false;
    }
  } else {
    scrollToPage(activeIndex.value[0] - 1);
  }
}

function toPreviousChapter() {
  if (hasPrev.value && data.value) {
    router.replace(`/comic/${comicId.value}/${data.value.prevId}`);
  }
}

function toNextChapter() {
  if (hasNext.value && data.value) {
    router.replace(`/comic/${comicId.value}/${data.value.nextId}`);
  }
}

onKeyStroke(['ArrowDown', 'ArrowRight'], (e) => {
  e.preventDefault();
  toNextPage();
});

onKeyStroke(['ArrowUp', 'ArrowLeft'], (e) => {
  e.preventDefault();
  toPrevPage();
});

// pre cache next chapter
const queryClient = useQueryClient();
const toEndDistance = computed(() => {
  if (!data.value || isFetching.value || !hasNext.value) {
    return Infinity;
  }
  return Math.abs(activeIndex.value[0] - (images.value.length - 1));
});
watch(toEndDistance, () => {
  if (toEndDistance.value < 5 && data.value) {
    queryClient.prefetchQuery({
      queryKey: [getChapter.name, comicId.value, data.value.nextId],
      queryFn: () =>
        getChapter({
          comicId: comicId.value,
          chapterId: data.value.nextId,
        }),
      staleTime: Infinity,
      gcTime: Infinity,
    });
  }
});

// update watch history
const historyDirty = ref(false);
const checkTimer = ref(0);
watch([comicId, chapterId, activeIndex], () => {
  if (isFetching.value || !data.value) {
    return;
  }
  historyDirty.value = true;
});
function update() {
  if (historyDirty.value && data.value && comicId.value && chapterId.value && activated.value) {
    upsertHistory({
      comicId: comicId.value,
      chapterId: chapterId.value,
      page: activeIndex.value[0] + 1,
      comicName: data.value.comicName,
      chapterName: data.value.name,
      visible: true,
    }).finally(() => {
      historyDirty.value = false;
    });
  }
}
onMounted(() => {
  checkTimer.value = setInterval(update, 3000);
});
onUnmounted(() => {
  update();
  clearInterval(checkTimer.value);
});
useEventListener(window, 'beforeunload', update);
</script>

<template>
  <div v-if="!data" class="flex h-full w-full items-center justify-center">
    <LoaderCircle class="animate-spin" />
  </div>

  <div
    v-else
    ref="list-container"
    class="scrollbar-hidden relative h-full w-full snap-y snap-mandatory overflow-auto outline-none"
  >
    <div class="flex flex-col" :class="{ 'gap-[env(safe-area-inset-bottom)]': isPWA }">
      <LazyImage
        v-for="(image, index) in images"
        :key="image"
        class="w-dvw snap-start object-contain"
        :class="[isPWA ? 'h-[calc(100dvh_-_env(safe-area-inset-bottom))]' : 'h-dvh']"
        :src="proxyImage(image)"
        :index
        :active-index="activeIndex[0]"
        @activated="
          activeIndex = [index];
          tempIndex = [index];
        "
      />
    </div>

    <div
      v-if="controlMaskShow"
      class="fixed inset-0 z-10 flex flex-col justify-between overscroll-contain bg-black bg-opacity-40"
    >
      <div class="flex h-12 items-center bg-zinc-900 text-white">
        <div class="p-2" @click="router.go(-1)">
          <ChevronLeft :size="28" />
        </div>
        <div class="min-w-0 flex-1 truncate">{{ data.comicName }} - {{ data.name }}</div>
      </div>

      <div
        class="flex-1"
        @click="
          controlMaskShow = false;
          pageNavShow = false;
        "
      />

      <div class="relative flex justify-center p-4 text-white" :class="{ 'pb-[env(safe-area-inset-bottom)]': isPWA }">
        <div class="w-full max-w-[800px] rounded-lg bg-zinc-900 bg-opacity-20 shadow-lg backdrop-blur-md">
          <div class="flex w-full items-center">
            <div
              class="p-4"
              :class="[hasPrev ? 'cursor-pointer' : 'cursor-not-allowed opacity-50']"
              @click="toPreviousChapter"
            >
              <ArrowLeftToLine :size="28" />
            </div>

            <Slider
              v-model="tempIndex"
              :min="0"
              :max="images.length - 1"
              @value-commit="onSlideEnd"
              @update:model-value="onSlideChange"
            />

            <div
              class="p-4"
              :class="[hasNext ? 'cursor-pointer' : 'cursor-not-allowed opacity-50']"
              @click="toNextChapter"
            >
              <ArrowRightToLine :size="28" />
            </div>
          </div>

          <div class="flex w-full items-center justify-between px-12 pb-2 text-sm">
            <div class="operation-btn">
              <Settings />
            </div>
            <div
              class="operation-btn"
              :class="{ '-scale-x-[1]': gestureReverse }"
              @click="
                controlMaskShow = false;
                showGestureTip = true;
                gestureReverse = !gestureReverse;
              "
            >
              <Pointer />
            </div>
            <div
              class="operation-btn"
              @click="
                controlMaskShow = false;
                showGestureTip = true;
              "
            >
              <BookOpenText />
            </div>
          </div>
        </div>

        <div
          v-if="pageNavShow"
          class="absolute -top-4 left-1/2 min-w-20 -translate-x-1/2 rounded bg-zinc-900 bg-opacity-50 px-1 text-center text-white shadow-lg backdrop-blur-sm"
        >
          <span class="text-blue-400">{{ tempIndex[0] + 1 }}</span>
          /
          <span>{{ images.length }}</span>
        </div>
      </div>
    </div>

    <ChapterGestureMask
      v-else
      :show-tip="showGestureTip"
      :reverse="gestureReverse"
      @next-page="toNextPage"
      @prev-page="toPrevPage"
      @open-menu="controlMaskShow = true"
      @click="
        gestureTipShowed = true;
        showGestureTip = false;
      "
    />

    <div
      v-if="showPrevTip || showNextTip"
      class="fixed left-1/2 top-1/2 z-20 -translate-x-1/2 -translate-y-1/2 rounded bg-black bg-opacity-30 px-2 py-1 text-white backdrop-blur-sm"
    >
      {{
        showPrevTip
          ? hasPrev
            ? '👆 再次点击前往上一章'
            : '😭 没有上一章了'
          : hasNext
            ? '👇 再次点击前往下一章'
            : '😭 没有下一章了'
      }}
    </div>
  </div>
</template>

<style scoped>
.operation-btn {
  @apply flex cursor-pointer flex-col items-center gap-1 px-4 py-2;
}
</style>
