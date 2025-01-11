use anyhow::Result;
use dom_query::{Document, Selection};

#[derive(Debug)]
pub struct ComicBrief {
    pub name: String,
    pub cover: String,
    pub author: Vec<String>,
    pub intro: String,
    pub pub_date: String,
}

#[trait_variant::make(Send)]
pub trait Site {
    async fn get_comic_brief(&self, id: String) -> Result<ComicBrief>;
    async fn search_comic(&self, keyword: String) -> Result<Vec<ComicBrief>>;
}

pub struct Manhuagui;

impl Site for Manhuagui {
    async fn get_comic_brief(&self, id: String) -> Result<ComicBrief> {
        let body = reqwest::get(format!("https://www.manhuagui.com/comic/{id}"))
            .await?
            .text()
            .await?;

        let doc = Document::from(body);

        let brief = parse_brief(
            &doc.select_single("body"),
            &BriefSelectors {
                cover: ".book-cover>.hcover>img",
                name: ".book-detail>.book-title>h1",
                author: ".book-detail>.detail-list>li:nth-child(2)>span:nth-child(2)>a",
                pub_date: ".book-detail>.detail-list>li:first-child>span:first-child>a",
                intro: "#intro-cut",
            },
        );

        Ok(brief)
    }

    async fn search_comic(&self, keyword: String) -> Result<Vec<ComicBrief>> {
        let encoded: String = url::form_urlencoded::byte_serialize(keyword.as_bytes()).collect();
        let body = reqwest::get(format!("https://www.manhuagui.com/s/{}.html", encoded))
            .await?
            .text()
            .await?;

        let doc = Document::from(body);

        let selectors = BriefSelectors {
            cover: ".book-cover>.bcover>img",
            name: ".book-detail dt>a",
            author: ".book-detail dd.tags:nth-of-type(3)>span>a",
            pub_date: ".book-detail dd.tags:nth-of-type(2)>span:first-child>a",
            intro: ".book-detail dd.intro>span",
        };

        let list = doc
            .select(".book-result>ul>li.cf")
            .iter()
            .map(|item| parse_brief(&item, &selectors))
            .collect();

        Ok(list)
    }
}

struct BriefSelectors<'a> {
    cover: &'a str,
    name: &'a str,
    author: &'a str,
    pub_date: &'a str,
    intro: &'a str,
}

fn parse_brief(node: &Selection, selectors: &BriefSelectors) -> ComicBrief {
    let name = node
        .select_single(&selectors.name)
        .text()
        .trim()
        .to_string();

    let mut cover = node
        .select_single(&selectors.cover)
        .text()
        .trim()
        .to_string();
    if cover.starts_with("//") {
        cover = format!("https:{cover}");
    }

    let author: Vec<String> = node
        .select(&selectors.author)
        .iter()
        .map(|item| item.text().trim().to_string())
        .collect();

    let pub_date = node
        .select_single(&selectors.pub_date)
        .text()
        .trim()
        .to_string();

    let intro = node
        .select_single(&selectors.intro)
        .text()
        .trim()
        .to_string();

    ComicBrief {
        name,
        cover,
        author,
        pub_date,
        intro,
    }
}
