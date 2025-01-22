import { onActivated, onDeactivated, ref } from 'vue';

export function useActivated() {
  const activated = ref(true);

  onActivated(() => {
    activated.value = true;
  });
  onDeactivated(() => {
    activated.value = false;
  });

  return activated;
}
