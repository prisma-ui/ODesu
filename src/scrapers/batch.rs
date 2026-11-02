use crate::parsing::*;
use crate::types::{BatchDetail, DownloadQuality, DownloadLink, GenreLink};
use scraper::{Html, Selector};
use std::collections::HashMap;

lazy_static::lazy_static! {
    static ref TITLE_SELECTOR: Selector = Selector::parse(".jdlrx h1").unwrap();
    static ref THUMB_SELECTOR: Selector = Selector::parse(".imganime img").unwrap();
    static ref DOWNLOAD_LINK_A_SELECTOR: Selector = Selector::parse(".totalepisode h3 a").unwrap();
    static ref BATCHLINK_UL_SELECTOR: Selector = Selector::parse(".batchlink ul").unwrap();
    static ref LI_SELECTOR: Selector = Selector::parse("li").unwrap();
    static ref STRONG_SELECTOR: Selector = Selector::parse("strong").unwrap();
    static ref SIZE_TAG_SELECTOR: Selector = Selector::parse("i").unwrap();
    static ref DOWNLOAD_A_SELECTOR: Selector = Selector::parse("a[href]").unwrap();
    static ref INFOS_SELECTOR: Selector = Selector::parse(".infos").unwrap();
}

pub fn parse_batch(html: &str) -> BatchDetail {
    let root = Html::parse_document(html);

    // Title
    let title = find_text(&root, &TITLE_SELECTOR)
        .unwrap_or_default()
        .replace("[BATCH] Subtitle Indonesia", "")
        .trim()
        .to_string();

    // Thumbnail
    let thumbnail = find_attr(&root, &THUMB_SELECTOR, "src");

    // Parse .infos with <b>Label</b>: value<br/> format
    let (info, genres) = parse_infos_br(&root, &INFOS_SELECTOR);
    let mut info_with_genres: HashMap<String, serde_json::Value> = info
        .into_iter()
        .map(|(k, v)| (k, serde_json::Value::String(v)))
        .collect();
    info_with_genres.insert("genres".to_string(), serde_json::to_value(genres).unwrap());

    // Anime slug from download link
    let anime_slug = find_attr(&root, &DOWNLOAD_LINK_A_SELECTOR, "href")
        .map(|s| slug_from_url(Some(&s)));

    // Downloads
    let mut downloads = Vec::new();
    for ul in root.select(&BATCHLINK_UL_SELECTOR) {
        for li in ul.select(&LI_SELECTOR) {
            let strong = li.select(&STRONG_SELECTOR).next();
            if strong.is_none() {
                continue;
            }
            let label = clean_text(Some(&strong.unwrap().text().collect::<String>()));
            let parts: Vec<&str> = label.split_whitespace().collect();
            let fmt = parts.first().unwrap_or(&"").to_string();
            let quality = parts.get(1).unwrap_or(&"").to_string();

            let size = li
                .select(&SIZE_TAG_SELECTOR)
                .next()
                .map(|el| clean_text(Some(&el.text().collect::<String>())));

            let links: Vec<DownloadLink> = li
                .select(&DOWNLOAD_A_SELECTOR)
                .map(|a| DownloadLink {
                    host: clean_text(Some(&a.text().collect::<String>())),
                    url: a.value().attr("href").unwrap_or("").to_string(),
                })
                .collect();

            if !links.is_empty() {
                downloads.push(DownloadQuality {
                    format: fmt,
                    quality,
                    size,
                    links,
                });
            }
        }
    }

    BatchDetail {
        title,
        thumbnail,
        anime_slug,
        info: info_with_genres,
        downloads,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use insta::assert_yaml_snapshot;

    #[test]
    fn test_parse_batch() {
        let html = include_str!("../../tests/fixtures/batch.html");
        let result = parse_batch(html);
        assert_yaml_snapshot!("batch", result);
    }
}