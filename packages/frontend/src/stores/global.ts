import { LocalStorageKey } from '@/types/const';
import { withRefs } from '@/utils/pinia';
import { defineStore } from 'pinia';

export const useGlobalStore = withRefs(
  defineStore('global', {
    state: () => ({
      password: localStorage.getItem(LocalStorageKey.Password) ?? '',
    }),
    actions: {
      setPassword(password: string) {
        this.password = password;
        localStorage.setItem(LocalStorageKey.Password, password);
      },
    },
  }),
);
