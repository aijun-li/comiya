<script setup lang="ts">
import { deleteHistory, getHistory } from '@/api';
import { Button } from '@/components/ui/button';
import { handleError } from '@/utils/error';
import { useQuery } from '@tanstack/vue-query';
import dayjs from 'dayjs';
import { ChevronLeft, Trash2 } from 'lucide-vue-next';
import { computed, onActivated } from 'vue';
import { RouterLink, useRouter } from 'vue-router';

const router = useRouter();

const { data, refetch } = useQuery({
  queryKey: [getHistory.name],
  queryFn: () => getHistory(),
});

const items = computed(() => (data.value ?? []).filter((item) => item.visible));

async function onDelete(comicId: string) {
  try {
    await deleteHistory({ comicId });
    refetch();
  } catch (error) {
    handleError(error);
  }
}

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
      class="flex cursor-pointer gap-1 rounded border p-4 shadow"
      :to="`/comic/${item.comicId}/${item.chapterId}?page=${item.page}`"
    >
      <div class="min-w-0 flex-1">
        <div class="flex flex-col gap-1 sm:flex-row sm:gap-4">
          <div class="truncate font-semibold">{{ item.comicName }}</div>

          <div class="truncate">[{{ item.chapterName }}] (P{{ item.page }})</div>
        </div>
        <div class="mt-2 truncate text-sm text-gray-400">
          上次观看: {{ dayjs(item.updatedAt).format('YYYY/MM/DD HH:mm') }}
        </div>
      </div>

      <Button
        class="-translate-y-1.5 translate-x-2.5"
        variant="ghost"
        size="icon"
        @click.stop.prevent="onDelete(item.comicId)"
      >
        <Trash2 />
      </Button>
    </RouterLink>
  </div>
</template>
