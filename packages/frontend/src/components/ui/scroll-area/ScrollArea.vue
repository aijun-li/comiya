<script setup lang="ts">
import { cn } from '@/utils';
import { ScrollAreaCorner, ScrollAreaRoot, ScrollAreaViewport, type ScrollAreaRootProps } from 'radix-vue';
import { computed, type HTMLAttributes } from 'vue';
import ScrollBar from './ScrollBar.vue';

const props = defineProps<
  ScrollAreaRootProps & { class?: HTMLAttributes['class']; viewportClass?: HTMLAttributes['class'] }
>();

const delegatedProps = computed(() => {
  const { class: _, ...delegated } = props;

  return delegated;
});
</script>

<template>
  <ScrollAreaRoot v-bind="delegatedProps" :class="cn('relative overflow-hidden', props.class)">
    <ScrollAreaViewport :class="cn('h-full w-full rounded-[inherit]', viewportClass)" as-child>
      <slot />
    </ScrollAreaViewport>
    <ScrollBar />
    <ScrollAreaCorner />
  </ScrollAreaRoot>
</template>
