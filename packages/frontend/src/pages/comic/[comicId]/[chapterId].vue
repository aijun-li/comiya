<script setup lang="ts">
import { getChapter, proxyImage } from '@/api';
import { useQuery } from '@tanstack/vue-query';
import { computed } from 'vue';
import { useRoute } from 'vue-router';

const route = useRoute('/comic/[comicId]/[chapterId]');

const comicId = computed(() => route.params.comicId);
const chapterId = computed(() => route.params.chapterId);

const { data } = useQuery({
  queryKey: [getChapter.name, comicId, chapterId],
  queryFn: () => getChapter({ comicId: comicId.value, chapterId: chapterId.value }),
});

const images = computed(() => data.value?.images || []);
</script>

<template>
  <div class="flex h-full w-full snap-y snap-mandatory justify-center overflow-auto">
    <div>
      <img v-for="image in images" :key="image" class="h-screen snap-start" :src="proxyImage(image)" />
    </div>
  </div>
</template>
