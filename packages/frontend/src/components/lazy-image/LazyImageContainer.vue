<script setup lang="ts">
import { provide, readonly, ref } from 'vue';
import { injectionKey } from './types';

const activeIndex = ref(0);

function updateActiveIndex(index: number) {
  activeIndex.value = index;
}

provide(injectionKey, {
  activeIndex: readonly(activeIndex),
  updateActiveIndex,
});
</script>

<template>
  <div>
    <slot />
  </div>
</template>
