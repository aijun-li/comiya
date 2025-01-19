<script setup lang="ts">
import { onMounted, onUnmounted, ref, useTemplateRef, watchEffect } from 'vue';

const props = withDefaults(
  defineProps<{
    src: string;
    index: number;
    offset?: number;
    activeIndex: number;
  }>(),
  {
    offset: 5,
  },
);

const emit = defineEmits<{
  activated: [];
}>();

const realSrc = ref<string>();

const elRef = useTemplateRef<HTMLImageElement>('image-ref');

watchEffect(() => {
  if (!realSrc.value && Math.abs(props.activeIndex - props.index) <= props.offset) {
    realSrc.value = props.src;
  }
});

const observer = new IntersectionObserver(
  ([entry]) => {
    if (entry.isIntersecting) {
      emit('activated');
    }
  },
  {
    threshold: 0.5,
  },
);

onMounted(() => {
  if (elRef.value) {
    observer.observe(elRef.value);
  }
});

onUnmounted(() => {
  observer.disconnect();
});
</script>

<template>
  <img ref="image-ref" :src="realSrc" :class="{ invisible: !realSrc }" />
</template>
