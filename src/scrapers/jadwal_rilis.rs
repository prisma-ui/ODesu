use crate::parsing::*;
use crate::types::{EpisodeLink, JadwalRilisResponse};
use scraper::{Html, Selector};
use std::collections::HashMap;

lazy_static::lazy_static! {
    static ref BLOCK_SELECTOR: Selector = Selector::parse(".kglist321").unwrap();
    static ref DAY_SELECTOR: Selector = Selector::parse("h2").unwrap();
    static ref LINK_SELECTOR: Selector = Selector::parse("ul li a").unwrap();
}

pub fn parse_jadwal_rilis(html: &str) -> JadwalRilisResponse {
    let root = Html::parse_document(html);
    let mut schedule = HashMap::new();

    for block in root.select(&BLOCK_SELECTOR) {
        let day = block
            .select(&DAY_SELECTOR)
            .next()
            .map(|el| clean_text(Some(&el.text().collect::<String>())))
            .unwrap_or_else(|| "Unknown".to_string());

        let items: Vec<EpisodeLink> = block
            .select(&LINK_SELECTOR)
            .map(|a| EpisodeLink {
                title: clean_text(Some(&a.text().collect::<String>())),
                slug: slug_from_url(a.value().attr("href")),
                date: None,
            })
            .collect();

        schedule.insert(day, items);
    }

    JadwalRilisResponse(schedule)
}

#[cfg(test)]
mod tests {
    use super::*;
    use insta::assert_yaml_snapshot;

    #[test]
    fn test_parse_jadwal_rilis() {
        let html = include_str!("../../tests/fixtures/jadwal-rilis.html");
        let result = parse_jadwal_rilis(html);
        assert_yaml_snapshot!("jadwal_rilis", result);
    }
}