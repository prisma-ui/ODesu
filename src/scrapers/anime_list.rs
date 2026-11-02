use crate::parsing::*;
use crate::types::{AnimeListGroups, EpisodeLink};
use scraper::{Html, Selector};
use std::collections::HashMap;

lazy_static::lazy_static! {
    static ref ROW_SELECTOR: Selector = Selector::parse(".bariskelom").unwrap();
    static ref LETTER_SELECTOR: Selector = Selector::parse(".barispenz a").unwrap();
    static ref ITEM_SELECTOR: Selector = Selector::parse(".jdlbar a.hodebgst").unwrap();
}

pub fn parse_anime_list(html: &str) -> AnimeListGroups {
    let root = Html::parse_document(html);
    let mut groups = HashMap::new();

    for row in root.select(&ROW_SELECTOR) {
        let letter = row
            .select(&LETTER_SELECTOR)
            .next()
            .map(|a| clean_text(Some(&a.text().collect::<String>())))
            .unwrap_or_else(|| "#".to_string());

        let items: Vec<EpisodeLink> = row
            .select(&ITEM_SELECTOR)
            .map(|a| EpisodeLink {
                title: clean_text(Some(&a.text().collect::<String>())),
                slug: slug_from_url(a.value().attr("href")),
                date: None,
            })
            .collect();

        if !items.is_empty() {
            groups.insert(letter, items);
        }
    }

    AnimeListGroups(groups)
}

#[cfg(test)]
mod tests {
    use super::*;
    use insta::assert_yaml_snapshot;

    #[test]
    fn test_parse_anime_list() {
        let html = include_str!("../../tests/fixtures/anime-list.html");
        let result = parse_anime_list(html);
        assert_yaml_snapshot!("anime_list", result);
    }
}