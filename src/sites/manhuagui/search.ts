import { ComicSearchResult } from '@/types';
import { useQuery } from '@tanstack/vue-query';
import { fetch } from '@tauri-apps/plugin-http';
import { Ref } from 'vue';

export function useSearch(name: Ref<string>) {
  return useQuery({
    queryKey: ['manhuagui', 'search'],
    queryFn: async (): Promise<ComicSearchResult> => {
      const res = await fetch(`https://www.manhuagui.com/s/${encodeURIComponent(name.value)}.html`);
      const text = await res.text();

      const parser = new DOMParser();
      const doc = parser.parseFromString(text, 'text/html');

      const elList = Array.from(doc.querySelector('.book-result')?.querySelectorAll('li.cf') ?? []);

      return elList.map((el) => {
        const titleEl = el.querySelector('.book-detail dl dt > a');
        const title = titleEl?.getAttribute('title') ?? '';
        const path = titleEl?.getAttribute('href') ?? '';
        const url = path ? `https://www.manhuagui.com${path}` : '';

        const coverEl = el.querySelector('.book-cover .bcover img');
        const coverStr = coverEl?.getAttribute('src');
        const cover = coverStr ? `https:${coverStr}` : '';

        return {
          name: title,
          cover,
          url,
        };
      });
    },
    enabled: false,
  });
}
