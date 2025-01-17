const EndpointPrefix = '/api';

export enum Endpoints {
  SearchComic = `${EndpointPrefix}/search_comic`,
  GetComic = `${EndpointPrefix}/get_comic`,
  GetChapter = `${EndpointPrefix}/get_chapter`,
  ProxyImage = `${EndpointPrefix}/proxy_image`,
}

export interface ComicBrief {
  id: string;
  name: string;
  cover: string;
  author: string[];
  intro: string;
  pubDate: string;
}

export interface Comic extends ComicBrief {
  chapterGroups: ComicChapterGroup[];
}

export interface ComicChapterBrief {
  id: string;
  comicId: string;
  name: string;
}

export interface ComicChapter extends ComicChapterBrief {
  comicName: string;
  nextId: string;
  prevId: string;
  images: string[];
}

export interface ComicChapterGroup {
  name: string;
  chapters: ComicChapterBrief[];
}

export type SearchComicReq = {
  keyword: string;
};

export type SearchComicResp = ComicBrief[];

export type GetComicReq = {
  id: string;
};

export type GetComicResp = Comic;

export type GetChapterReq = {
  comicId: string;
  chapterId: string;
};

export type GetChapterResp = ComicChapter;
