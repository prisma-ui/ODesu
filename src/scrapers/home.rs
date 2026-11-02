use crate::parsing::*;
use crate::scrapers::anime_grid::parse_detpost;
use crate::types::{AnimeCard, HomeResponse};
use scraper::{Html, Selector};

lazy_static::lazy_static! {
    static ref SECTION_SELECTOR: Selector = Selector::parse(".venutama .rseries").unwrap();
    static ref HEADER_SELECTOR: Selector = Selector::parse(".rvad h1").unwrap();
    static ref ITEMS_SELECTOR: Selector = Selector::parse(".venz ul li").unwrap();
}

pub fn parse_home(html: &str) -> HomeResponse {
    let root = Html::parse_document(html);
    let mut ongoing = Vec::new();
    let mut complete = Vec::new();

    for section in root.select(&SECTION_SELECTOR) {
        let header_text = section
            .select(&HEADER_SELECTOR)
            .next()
            .map(|el| clean_text(Some(&el.text().collect::<String>())))
            .unwrap_or_default();

        let items: Vec<AnimeCard> = section
            .select(&ITEMS_SELECTOR)
            .map(parse_detpost)
            .collect();

        if header_text.contains("On-going") || header_text.contains("Ongoing") {
            ongoing = items;
        } else if header_text.contains("Complete") {
            complete = items;
        }
    }

    HomeResponse { ongoing, complete }
}

#[cfg(test)]
mod tests {
    use super::*;
    use insta::assert_yaml_snapshot;

    #[test]
    fn test_parse_home() {
        let html = include_str!("../../tests/fixtures/home.html");
        let result = parse_home(html);
        assert_yaml_snapshot!("home", result);
    }
}