use axum::{
    body::Body,
    extract::Query,
    middleware::from_fn,
    response::IntoResponse,
    routing::{get, MethodRouter},
    Json, Router,
};
use tower_http::services::{ServeDir, ServeFile};

use serde::Deserialize;

use crate::{
    server::types::AppResult,
    site::{Comic, ComicBrief, ComicChapter, Manhuagui, Site},
};

use super::middleware::wrap_response_middleware;

pub fn get_router() -> Router {
    let api_router = Router::new()
        .merge(search_comic())
        .merge(get_comic())
        .merge(get_chapter())
        .merge(proxy_image())
        .layer(from_fn(wrap_response_middleware));

    let serve_dir = ServeDir::new("dist").fallback(ServeFile::new("dist/index.html"));

    Router::new()
        .nest("/api", api_router)
        .fallback_service(serve_dir)
}

fn search_comic() -> Router {
    async fn handler(
        Query(SearchComicQuery { keyword }): Query<SearchComicQuery>,
    ) -> AppResult<Json<Vec<ComicBrief>>> {
        let site = Manhuagui;
        let list = site.search_comic(keyword).await?;
        Ok(Json(list))
    }

    route("/search_comic", get(handler))
}

fn get_comic() -> Router {
    async fn handler(
        Query(GetComicBriefQuery { id }): Query<GetComicBriefQuery>,
    ) -> AppResult<Json<Comic>> {
        let site = Manhuagui;
        let res = site.get_comic(id).await?;
        Ok(Json(res))
    }

    route("/get_comic", get(handler))
}

fn get_chapter() -> Router {
    async fn handler(
        Query(GetChapterImagesQuery {
            comic_id,
            chapter_id,
        }): Query<GetChapterImagesQuery>,
    ) -> AppResult<Json<ComicChapter>> {
        let site = Manhuagui;
        let chapter = site.get_chapter(comic_id, chapter_id).await?;
        Ok(Json(chapter))
    }

    route("/get_chapter", get(handler))
}

fn proxy_image() -> Router {
    async fn handler(
        Query(ProxyImageQuery { url }): Query<ProxyImageQuery>,
    ) -> AppResult<impl IntoResponse> {
        let decoded_url = urlencoding::decode(&url)?;
        let resp = reqwest::Client::new()
            .get(decoded_url.as_ref())
            .header("Referer", "https://www.manhuagui.com/")
            .send()
            .await?;
        let headers = resp.headers().to_owned();
        let stream = resp.bytes_stream();

        Ok((headers, Body::from_stream(stream)))
    }

    route("/proxy_image", get(handler))
}

fn route(path: &str, method_router: MethodRouter<()>) -> Router {
    Router::new().route(path, method_router)
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct SearchComicQuery {
    keyword: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct GetComicBriefQuery {
    id: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct GetChapterImagesQuery {
    comic_id: String,
    chapter_id: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct ProxyImageQuery {
    url: String,
}