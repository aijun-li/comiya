use anyhow::Result;
use dom_query::Document;

#[derive(Debug)]
pub struct ComicBrief {
    pub name: String,
    pub cover: String,
    pub author: Vec<String>,
    pub intro: String,
    pub pub_date: String,
}

pub trait Site {
    async fn get_comic_brief(&self, id: String) -> Result<ComicBrief>;
}

pub struct Manhuagui;

impl Site for Manhuagui {
    async fn get_comic_brief(&self, id: String) -> Result<ComicBrief> {
        let body = reqwest::get(format!("https://www.manhuagui.com/comic/{id}"))
            .await?
            .text()
            .await?;

        let doc = Document::from(body);
        let name = doc
            .select_single(".book-detail>.book-title>h1")
            .text()
            .to_string();
        let intro = doc.select_single("#intro-cut").text().to_string();

        Ok(ComicBrief {
            name,
            cover: "".to_string(),
            author: vec![],
            intro,
            pub_date: "".to_string(),
        })
    }
}
