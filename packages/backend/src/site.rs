use anyhow::Result;
use dom_query::{Document, Selection};
use regex::Regex;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ComicBrief {
    pub id: String,
    pub name: String,
    pub cover: String,
    pub author: Vec<String>,
    pub intro: String,
    pub pub_date: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ComicChapterBrief {
    pub id: String,
    pub comic_id: String,
    pub name: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ComicChapterGroup {
    pub name: String,
    pub chapters: Vec<ComicChapterBrief>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Comic {
    pub id: String,
    pub name: String,
    pub cover: String,
    pub author: Vec<String>,
    pub intro: String,
    pub pub_date: String,
    pub chapter_groups: Vec<ComicChapterGroup>,
    pub first_chapter_id: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ComicChapter {
    pub id: String,
    pub comic_id: String,
    pub name: String,
    pub comic_name: String,
    pub next_id: String,
    pub prev_id: String,
    pub images: Vec<String>,
}

#[trait_variant::make(Send)]
pub trait Site {
    async fn search_comic(&self, keyword: String) -> Result<Vec<ComicBrief>>;
    async fn get_comic(&self, id: String) -> Result<Comic>;
    async fn get_chapter(&self, comic_id: String, chapter_id: String) -> Result<ComicChapter>;
}

pub struct Manhuagui;

impl Site for Manhuagui {
    async fn get_comic(&self, id: String) -> Result<Comic> {
        let body = reqwest::get(format!("https://www.manhuagui.com/comic/{id}"))
            .await?
            .text()
            .await?;

        let mut doc = Document::from(body);

        let brief = parse_brief(
            &doc.select_single("body"),
            &BriefSelectors {
                cover: ".book-cover>.hcover>img",
                name: ".book-detail>.book-title>h1",
                author: ".book-detail>.detail-list>li:nth-child(2)>span:nth-child(2)>a",
                pub_date: ".book-detail>.detail-list>li:first-child>span:first-child>a",
                intro: "#intro-cut",
            },
            id.clone(),
        );

        let first_chapter_id = doc
            .select_single(".book-btn a")
            .attr_or("href", "")
            .trim()
            .trim_start_matches(&format!("/comic/{}/", id))
            .trim_end_matches(".html")
            .to_string();

        // check if there are hidden content due to audition
        let hidden_input = doc.select(r#"#erroraudit_show + input[type="hidden"]"#);
        if hidden_input.exists() {
            let encoded = hidden_input.attr_or("value", "");
            let decoded = lz_str::decompress_from_base64(&encoded)
                .ok_or(0)
                .map(|s| String::from_utf16(&s).unwrap_or("".to_string()))
                .unwrap_or("".to_string());

            if !decoded.is_empty() {
                doc = Document::fragment(decoded);
            }
        }

        let chapter_groups = doc
            .select("h4:has(~ .chapter-list")
            .iter()
            .enumerate()
            .map(|(index, node)| {
                let group_name = node.select_single("span").text().to_string();

                let chapters = doc
                    .select(&format!("h4:nth-of-type({}) ~ .chapter-list ul", index + 1))
                    .iter()
                    .map(|ul_node| {
                        ul_node
                            .select("li a")
                            .iter()
                            .map(|a_node| {
                                let chapter_id = a_node
                                    .attr_or("href", "")
                                    .trim()
                                    .trim_start_matches(&format!("/comic/{}/", id))
                                    .trim_end_matches(".html")
                                    .to_string();
                                // let url = format!(
                                //     "https://www.manhuagui.com{}",
                                //     a_node.attr_or("href", "")
                                // );
                                let name = a_node.attr_or("title", "").to_string();
                                ComicChapterBrief {
                                    id: chapter_id,
                                    comic_id: id.clone(),
                                    name,
                                }
                            })
                            .rev()
                    })
                    .flatten()
                    .collect();

                ComicChapterGroup {
                    name: group_name,
                    chapters,
                }
            })
            .collect();

        Ok(Comic {
            id: brief.id,
            name: brief.name,
            cover: brief.cover,
            author: brief.author,
            pub_date: brief.pub_date,
            intro: brief.intro,
            chapter_groups,
            first_chapter_id,
        })
    }

    async fn search_comic(&self, keyword: String) -> Result<Vec<ComicBrief>> {
        let encoded = urlencoding::encode(&keyword);
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
            .map(|item| {
                let id = item
                    .select_single("a.bcover")
                    .attr_or("href", "")
                    .trim()
                    .trim_start_matches("/comic/")
                    .trim_end_matches("/")
                    .to_string();
                let mut brief = parse_brief(&item, &selectors, id);
                brief.intro = brief
                    .intro
                    .trim_start_matches("简介：")
                    .trim_end_matches("[详情]")
                    .to_string();
                brief
            })
            .collect();

        Ok(list)
    }

    async fn get_chapter(&self, comic_id: String, chapter_id: String) -> Result<ComicChapter> {
        let body = reqwest::get(format!(
            "https://www.manhuagui.com/comic/{comic_id}/{chapter_id}.html"
        ))
        .await?
        .text()
        .await?;

        let doc = Document::from(body);

        let name = doc
            .select_single(".title .fr + div h2")
            .text()
            .trim()
            .to_string();

        let comic_name = doc
            .select_single(".title .fr + div h1 a")
            .text()
            .trim()
            .to_string();

        let mut list = vec![];
        let mut next_id = 0_usize;
        let mut prev_id = 0_usize;

        doc.select("body script:last-of-type").iter().any(|node| {
            let script = node.text();
            let script = script.trim();
            if !script.starts_with(r#"window["\x65\x76\x61\x6c"]"#) {
                return false;
            }

            let re = Regex::new(r"'([^,]+)'\['\\x73\\x70\\x6c\\x69\\x63'\]\('\\x7c'\)").unwrap();
            let Some(caps) = re.captures(script) else {
                return true;
            };
            let cap = caps.get(1).unwrap().as_str();
            let Some(decoded_vec) = lz_str::decompress_from_base64(cap) else {
                return true;
            };
            let Ok(decoded_str) = String::from_utf16(&decoded_vec) else {
                return true;
            };

            let script = re.replace(
                script,
                format!(
                    "[{}]",
                    decoded_str
                        .split('|')
                        .map(|part| format!("'{}'", part))
                        .collect::<Vec<String>>()
                        .join(",")
                ),
            );
            let script = script.trim_start_matches(r#"window["\x65\x76\x61\x6c"]"#);

            let Ok(rt) = rquickjs::Runtime::new() else {
                return true;
            };
            let Ok(context) = rquickjs::Context::full(&rt) else {
                return true;
            };

            let Ok(eval_result) = context.with(|ctx| ctx.eval::<String, &str>(script)) else {
                return true;
            };
            let eval_result = eval_result
                .trim()
                .trim_start_matches("SMH.imgData(")
                .trim_end_matches(").preInit();");

            #[derive(Deserialize, Debug)]
            struct ParseSl {
                e: usize,
                m: String,
            }

            #[derive(Deserialize, Debug)]
            #[serde(rename_all = "camelCase")]
            struct ParseData {
                files: Vec<String>,
                path: String,
                next_id: usize,
                prev_id: usize,
                sl: ParseSl,
            }

            let Ok(parsed) = serde_json::from_str::<ParseData>(eval_result) else {
                return true;
            };

            list = parsed
                .files
                .iter()
                .map(|file| {
                    format!(
                        "https://i.hamreus.com{}{}?e={}&m={}",
                        parsed.path, file, parsed.sl.e, parsed.sl.m
                    )
                })
                .collect();

            prev_id = parsed.prev_id;
            next_id = parsed.next_id;

            true
        });

        Ok(ComicChapter {
            id: chapter_id,
            comic_id,
            name,
            comic_name,
            prev_id: prev_id.to_string(),
            next_id: next_id.to_string(),
            images: list,
        })
    }
}

struct BriefSelectors<'a> {
    cover: &'a str,
    name: &'a str,
    author: &'a str,
    pub_date: &'a str,
    intro: &'a str,
}

fn parse_brief(node: &Selection, selectors: &BriefSelectors, id: String) -> ComicBrief {
    let name = node
        .select_single(&selectors.name)
        .text()
        .trim()
        .to_string();

    let mut cover = node
        .select_single(&selectors.cover)
        .attr_or("src", "")
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
        .trim_end_matches("年")
        .to_string();

    let intro = node
        .select_single(&selectors.intro)
        .text()
        .trim()
        .to_string();

    ComicBrief {
        id,
        name,
        cover,
        author,
        pub_date,
        intro,
    }
}
