<script setup lang="ts">
const props = defineProps<{
  showTip: boolean;
  reverse: boolean;
}>();

const emit = defineEmits<{
  nextPage: [];
  prevPage: [];
  openMenu: [];
}>();

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
</script>

<template>
  <div class="fixed inset-0 z-10 flex flex-col" :class="{ 'opacity-0': !showTip }">
    <div class="flex h-1/6" :class="{ 'flex-row-reverse': reverse }">
      <div class="click-bg-prev flex-grow-[2]" @click="toPrevPage"></div>
      <div class="click-bg-next flex-grow-[1]" @click="toNextPage"></div>
    </div>
    <div class="flex flex-1" :class="{ 'flex-row-reverse': reverse }">
      <div class="click-tip click-bg-prev" @click="toPrevPage">上一页</div>
      <div class="click-tip click-bg-menu" @click="openMenu">菜单</div>
      <div class="click-tip click-bg-next" @click="toNextPage">下一页</div>
    </div>
    <div class="flex h-1/6" :class="{ 'flex-row-reverse': reverse }">
      <div class="click-bg-prev flex-grow-[1]" @click="toPrevPage"></div>
      <div class="click-bg-next flex-grow-[2]" @click="toNextPage"></div>
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
