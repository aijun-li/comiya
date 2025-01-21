<script setup lang="ts">
import { getHistoryList } from '@/api';
import { Button } from '@/components/ui/button';
import { useQuery } from '@tanstack/vue-query';
import dayjs from 'dayjs';
import { ChevronLeft } from 'lucide-vue-next';
import { computed, onActivated } from 'vue';
import { RouterLink, useRouter } from 'vue-router';

const router = useRouter();

const { data, refetch } = useQuery({
  queryKey: [getHistoryList.name],
  queryFn: () => getHistoryList(),
});

const items = computed(() => data.value ?? []);

onActivated(refetch);
</script>

<template>
  <div class="flex h-full w-full flex-col gap-4 px-6 py-14 lg:px-12 lg:py-16">
    <Button class="fixed left-2 top-2" variant="outline" size="icon" @click="router.go(-1)">
      <ChevronLeft />
    </Button>

    <RouterLink
      v-for="item in items"
      :key="item.comicId"
      class="cursor-pointer rounded border p-4 shadow"
      :to="`/comic/${item.comicId}/${item.chapterId}?page=${item.page}`"
    >
      <div class="flex flex-col gap-1 sm:flex-row sm:gap-4">
        <div class="font-semibold">{{ item.comicName }}</div>

        <div>[{{ item.chapterName }}] (P{{ item.page }})</div>
      </div>
      <div class="mt-2 text-sm text-gray-400">上次观看: {{ dayjs(item.updatedAt).format('YYYY/MM/DD HH:mm') }}</div>
    </RouterLink>
  </div>
</template>
