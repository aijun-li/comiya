import { get, post } from './fetch';
import type {
  CheckPasswordReq,
  CheckPasswordResp,
  GetChapterReq,
  GetChapterResp,
  GetComicReq,
  GetComicResp,
  SearchComicReq,
  SearchComicResp,
} from './types';
import { Endpoints } from './types';

export function searchComic(params: SearchComicReq): Promise<SearchComicResp> {
  return get(Endpoints.SearchComic, params);
}

export function getComic(params: GetComicReq): Promise<GetComicResp> {
  return get(Endpoints.GetComic, params);
}

export function getChapter(params: GetChapterReq): Promise<GetChapterResp> {
  return get(Endpoints.GetChapter, params);
}

export function proxyImage(url: string) {
  return `${Endpoints.ProxyImage}?url=${encodeURIComponent(url)}`;
}

export function checkPassword(data: CheckPasswordReq): Promise<CheckPasswordResp> {
  return post(Endpoints.CheckPassword, data);
}
