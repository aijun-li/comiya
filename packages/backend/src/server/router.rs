use std::env;

use axum::{
    body::Body,
    extract::{Query, State},
    middleware::from_fn,
    response::IntoResponse,
    routing::{get, post, MethodRouter},
    Json, Router,
};
use entity::history;
use sea_orm::{prelude::DateTimeUtc, Set};
use tower_http::services::{ServeDir, ServeFile};

use serde::{Deserialize, Serialize};

use crate::{
    db,
    server::types::AppResult,
    site::{Comic, ComicBrief, ComicChapter, Manhuagui, Site},
};

use super::{
    middleware::{password_validate_middleware, wrap_response_middleware},
    AppState,
};

pub fn get_router() -> Router<AppState> {
    let auth_api_router = Router::new()
        .merge(search_comic())
        .merge(get_comic())
        .merge(get_chapter())
        .merge(proxy_image())
        .merge(upsert_history())
        .merge(get_history_list())
        .layer(from_fn(password_validate_middleware));

    let api_router = auth_api_router
        .merge(check_password())
        .layer(from_fn(wrap_response_middleware));

    let serve_dir = ServeDir::new("dist").fallback(ServeFile::new("dist/index.html"));

    Router::new()
        .nest("/api", api_router)
        .fallback_service(serve_dir)
}

fn search_comic() -> Router<AppState> {
    async fn handler(
        Query(SearchComicQuery { keyword }): Query<SearchComicQuery>,
    ) -> AppResult<Json<Vec<ComicBrief>>> {
        let site = Manhuagui;
        let list = site.search_comic(keyword).await?;
        Ok(Json(list))
    }

    route("/search_comic", get(handler))
}

fn get_comic() -> Router<AppState> {
    async fn handler(
        Query(GetComicBriefQuery { id }): Query<GetComicBriefQuery>,
    ) -> AppResult<Json<Comic>> {
        let site = Manhuagui;
        let res = site.get_comic(id).await?;
        Ok(Json(res))
    }

    route("/get_comic", get(handler))
}

fn get_chapter() -> Router<AppState> {
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

fn proxy_image() -> Router<AppState> {
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

fn check_password() -> Router<AppState> {
    async fn handler(
        Json(CheckPasswordData { password }): Json<CheckPasswordData>,
    ) -> AppResult<Json<CheckPasswordResp>> {
        let coconfigured_password = env::var("PASSWORD").unwrap_or("".to_string());
        let configured_password = coconfigured_password.trim();

        Ok(Json(CheckPasswordResp {
            valid: configured_password.is_empty() || password.trim() == configured_password,
        }))
    }

    route("/check_password", post(handler))
}

fn upsert_history() -> Router<AppState> {
    async fn handler(
        State(AppState { db }): State<AppState>,
        Json(UpsertHistoryData {
            comic_id,
            chapter_id,
            comic_name,
            chapter_name,
            page,
            visible,
        }): Json<UpsertHistoryData>,
    ) -> AppResult<Json<()>> {
        db::upsert_history(
            &db,
            history::ActiveModel {
                comic_id: Set(comic_id),
                chapter_id: Set(chapter_id),
                comic_name: Set(comic_name),
                chapter_name: Set(chapter_name),
                page: Set(page),
                visible: Set(visible),
                updated_at: Set(chrono::Utc::now()),
                ..Default::default()
            },
        )
        .await?;

        Ok(Json(()))
    }

    route("/upsert_history", post(handler))
}

fn get_history_list() -> Router<AppState> {
    async fn handler(State(AppState { db }): State<AppState>) -> AppResult<Json<Vec<HistoryItem>>> {
        let list = db::get_history_list(&db).await?;
        let mapped: Vec<HistoryItem> = list
            .into_iter()
            .map(|item| HistoryItem {
                comic_id: item.comic_id,
                chapter_id: item.chapter_id,
                comic_name: item.comic_name,
                chapter_name: item.chapter_name,
                page: item.page,
                visible: item.visible,
                created_at: item.created_at,
                updated_at: item.updated_at,
            })
            .collect();
        Ok(Json(mapped))
    }

    route("/get_history_list", get(handler))
}

fn route(path: &str, method_router: MethodRouter<AppState>) -> Router<AppState> {
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

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct CheckPasswordData {
    password: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct CheckPasswordResp {
    valid: bool,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct UpsertHistoryData {
    comic_id: String,
    chapter_id: String,
    comic_name: String,
    chapter_name: String,
    page: i32,
    visible: bool,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct HistoryItem {
    comic_id: String,
    chapter_id: String,
    comic_name: String,
    chapter_name: String,
    page: i32,
    visible: bool,
    created_at: DateTimeUtc,
    updated_at: DateTimeUtc,
}
