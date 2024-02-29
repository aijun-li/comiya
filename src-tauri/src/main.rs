// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use scraper::{Html, Selector};

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
async fn greet() -> String {
    let html = reqwest::get("https://www.manhuagui.com/s/怪兽.html")
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    let document = Html::parse_document(&html);
    let selector = Selector::parse(".book-result ul > .cf .book-detail > dl > dt > a").unwrap();

    let title = document
        .select(&selector)
        .next()
        .unwrap()
        .attr("title")
        .unwrap();

    title.to_string()
}

#[tokio::main]
async fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
