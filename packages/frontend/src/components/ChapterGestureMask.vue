<script setup lang="ts">
import { useDebounceFn } from '@vueuse/core';
import { onMounted, onUnmounted, useTemplateRef } from 'vue';

const props = defineProps<{
  showTip: boolean;
  reverse: boolean;
}>();

const emit = defineEmits<{
  nextPage: [];
  prevPage: [];
  openMenu: [];
  click: [];
}>();

const containerRef = useTemplateRef('container');

function toPrevPage() {
  if (props.showTip) {
    return;
  }
  emit('prevPage');
}

function toNextPage() {
  if (props.showTip) {
    return;
  }
  emit('nextPage');
}

function openMenu() {
  if (props.showTip) {
    return;
  }
  emit('openMenu');
}

function inBox({ x, y }: { x: number; y: number }, el?: Element) {
  if (!el) {
    return false;
  }
  const { left, top, width, height } = el.getBoundingClientRect();
  const right = left + width;
  const bottom = top + height;
  return x >= left && x <= right && y >= top && y <= bottom;
}

const listener = useDebounceFn((event: MouseEvent) => {
  if (!containerRef.value) {
    return;
  }

  const { x, y } = event;

  const nextEls = Array.from(containerRef.value.querySelectorAll('.click-bg-next'));
  const prevEls = Array.from(containerRef.value.querySelectorAll('.click-bg-prev'));
  const menuEls = Array.from(containerRef.value.querySelectorAll('.click-bg-menu'));

  if (nextEls.some((el) => inBox({ x, y }, el))) {
    toNextPage();
  } else if (prevEls.some((el) => inBox({ x, y }, el))) {
    toPrevPage();
  } else if (menuEls.some((el) => inBox({ x, y }, el))) {
    openMenu();
  }
  emit('click');
}, 100);

onMounted(() => {
  setTimeout(() => {
    document.addEventListener('click', listener);
  });
});

onUnmounted(() => {
  document.removeEventListener('click', listener);
});
</script>

<template>
  <div
    ref="container"
    class="pointer-events-none fixed inset-0 z-10 flex flex-col outline-none"
    :class="{ 'opacity-0': !showTip }"
  >
    <div class="flex h-1/6" :class="{ 'flex-row-reverse': reverse }">
      <div class="click-bg-prev flex-grow-[2]"></div>
      <div class="click-bg-next flex-grow-[1]"></div>
    </div>
    <div class="flex flex-1" :class="{ 'flex-row-reverse': reverse }">
      <div class="click-tip click-bg-prev">上一页</div>
      <div class="click-tip click-bg-menu">菜单</div>
      <div class="click-tip click-bg-next">下一页</div>
    </div>
    <div class="flex h-1/6" :class="{ 'flex-row-reverse': reverse }">
      <div class="click-bg-prev flex-grow-[1]"></div>
      <div class="click-bg-next flex-grow-[2]"></div>
    </div>
  </div>
</template>

<style scoped>
.click-tip {
  @apply flex flex-1 select-none items-center justify-center text-2xl font-semibold text-white [writing-mode:vertical-lr];
}

.click-bg-menu {
  @apply bg-cyan-400 bg-opacity-80;
}

.click-bg-next {
  @apply bg-orange-400 bg-opacity-80;
}

.click-bg-prev {
  @apply bg-yellow-400 bg-opacity-80;
}
</style>
