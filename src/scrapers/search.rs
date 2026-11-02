use crate::parsing::*;
use crate::types::{GenreLink, SearchResult};
use scraper::{Html, Selector};

lazy_static::lazy_static! {
    static ref ITEM_SELECTOR: Selector = Selector::parse("ul.chivsrc > li").unwrap();
    static ref IMG_SELECTOR: Selector = Selector::parse("img").unwrap();
    static ref TITLE_A_SELECTOR: Selector = Selector::parse("h2 a").unwrap();
    static ref SET_SELECTOR: Selector = Selector::parse(".set").unwrap();
    static ref GENRE_A_SELECTOR: Selector = Selector::parse("a[rel='tag']").unwrap();
}

pub fn parse_search(html: &str) -> Vec<SearchResult> {
    let root = Html::parse_document(html);

    root.select(&ITEM_SELECTOR)
        .map(|li| {
            let img = li.select(&IMG_SELECTOR).next();
            let title_a = li.select(&TITLE_A_SELECTOR).next();

            let genres: Vec<GenreLink> = li
                .select(&GENRE_A_SELECTOR)
                .map(|g| GenreLink {
                    name: clean_text(Some(&g.text().collect::<String>())),
                    slug: slug_from_url(g.value().attr("href")),
                })
                .collect();

            let mut status = None;
            let mut rating = None;

            for set_div in li.select(&SET_SELECTOR) {
                let b = set_div.select(&Selector::parse("b").unwrap()).next();
                if b.is_none() {
                    continue;
                }
                let label = clean_text(Some(&b.unwrap().text().collect::<String>())).to_lowercase();
                let mut value = clean_text(Some(&set_div.text().collect::<String>()));
                value = value.replace(&b.unwrap().text().collect::<String>(), "").trim().to_string();
                value = value.trim_start_matches(':').trim().to_string();

                if label.contains("status") {
                    status = Some(value);
                } else if label.contains("rating") {
                    rating = Some(value);
                }
            }

            SearchResult {
                title: title_a
                    .map(|a| clean_text(Some(&a.text().collect::<String>())))
                    .unwrap_or_default(),
                slug: title_a
                    .map(|a| slug_from_url(a.value().attr("href")))
                    .unwrap_or_default(),
                thumbnail: img.and_then(|el| el.value().attr("src").map(|s| s.to_string())),
                genres,
                status,
                rating,
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use insta::assert_yaml_snapshot;

    #[test]
    fn test_parse_search() {
        let html = include_str!("../../tests/fixtures/search.html");
        let result = parse_search(html);
        assert_yaml_snapshot!("search", result);
    }
}