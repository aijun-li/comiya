import type { InjectionKey, Ref } from 'vue';

export const injectionKey = Symbol() as InjectionKey<{
  activeIndex: Readonly<Ref<number>>;
  updateActiveIndex: (index: number) => void;
}>;
