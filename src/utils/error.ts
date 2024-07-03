import { toast } from 'vue-sonner';

export function handleError(error: unknown) {
  console.error(error);

  toast.error((error as Error).message);
}
