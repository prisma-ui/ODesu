use crate::parsing::*;
use crate::types::{AnimeCard, AnimeGridResponse, Pagination};
use scraper::{ElementRef, Html, Selector};

lazy_static::lazy_static! {
    static ref DETPOST_SELECTOR: Selector = Selector::parse(".venz ul li").unwrap();
    static ref THUMB_A_SELECTOR: Selector = Selector::parse(".thumb a").unwrap();
    static ref THUMB_IMG_SELECTOR: Selector = Selector::parse(".thumb img").unwrap();
    static ref TITLE_SELECTOR: Selector = Selector::parse(".jdlflm").unwrap();
    static ref EPZ_SELECTOR: Selector = Selector::parse(".epz").unwrap();
    static ref EPZTIPE_SELECTOR: Selector = Selector::parse(".epztipe").unwrap();
    static ref NEwnime_SELECTOR: Selector = Selector::parse(".newnime").unwrap();
    static ref PAGINATION_SELECTOR: Selector = Selector::parse(".pagenavix").unwrap();
    static ref CURRENT_PAGE_SELECTOR: Selector = Selector::parse(".page-numbers.current").unwrap();
    static ref PAGE_NUMBERS_SELECTOR: Selector = Selector::parse("a.page-numbers").unwrap();
    static ref NEXT_PAGE_SELECTOR: Selector = Selector::parse("a.next.page-numbers").unwrap();
}

/// Parse a single anime card from .detpost structure (used by home page)
fn parse_detpost(li: ElementRef<'_>) -> AnimeCard {
    let title = find_text_in(&li, &TITLE_SELECTOR);
    let slug = find_attr_in(&li, &THUMB_A_SELECTOR, "href")
        .map(|s| slug_from_url(Some(&s)))
        .unwrap_or_default();
    let thumbnail = find_attr_in(&li, &THUMB_IMG_SELECTOR, "src");
    let episode = find_text_in_optional(&li, &EPZ_SELECTOR);
    let day = find_text_in_optional(&li, &EPZTIPE_SELECTOR);
    let rating = None;
    let date = find_text_in_optional(&li, &NEwnime_SELECTOR);

    AnimeCard {
        title,
        slug,
        thumbnail,
        episode,
        day,
        rating,
        date,
    }
}

/// Parse a single anime card from .detpost structure for grid pages (ongoing/complete)
/// where .epztipe holds a rating, not a day
fn parse_detpost_grid(li: ElementRef<'_>) -> AnimeCard {
    let mut card = parse_detpost(li);
    // For grid pages, epztipe is rating, not day
    card.rating = card.day.clone();
    card.day = None;
    card
}

/// Parse pagination from .pagenavix
pub fn parse_pagination(root: &Html) -> Pagination {
    let nav = match root.select(&PAGINATION_SELECTOR).next() {
        Some(n) => n,
        None => return Pagination::default(),
    };

    let current_page = nav
        .select(&CURRENT_PAGE_SELECTOR)
        .next()
        .and_then(|el| clean_text(Some(&el.text().collect::<String>())).parse().ok())
        .unwrap_or(1);

    let mut page_numbers = Vec::new();
    for a in nav.select(&PAGE_NUMBERS_SELECTOR) {
        let text = clean_text(Some(&a.text().collect::<String>()));
        if let Ok(num) = text.parse::<u32>() {
            page_numbers.push(num);
        }
    }

    let last_page = std::cmp::max(current_page, *page_numbers.iter().max().unwrap_or(&1));
    let has_next = nav.select(&NEXT_PAGE_SELECTOR).next().is_some();

    Pagination {
        current_page,
        last_page,
        has_next,
    }
}

/// Parse anime grid (ongoing-anime, complete-anime)
pub fn parse_anime_grid(html: &str) -> AnimeGridResponse {
    let root = Html::parse_document(html);
    let items = root
        .select(&DETPOST_SELECTOR)
        .map(parse_detpost_grid)
        .collect();
    let pagination = parse_pagination(&root);

    AnimeGridResponse {
        anime_list: items,
        pagination,
    }
}

// Helper functions
fn find_text_in(parent: &ElementRef<'_>, selector: &Selector) -> String {
    parent
        .select(selector)
        .next()
        .map(|el| clean_text(Some(&el.text().collect::<String>())))
        .unwrap_or_default()
}

fn find_text_in_optional(parent: &ElementRef<'_>, selector: &Selector) -> Option<String> {
    parent
        .select(selector)
        .next()
        .map(|el| clean_text(Some(&el.text().collect::<String>())))
        .filter(|s| !s.is_empty())
}

fn find_attr_in(parent: &ElementRef<'_>, selector: &Selector, attr: &str) -> Option<String> {
    parent
        .select(selector)
        .next()
        .and_then(|el| el.value().attr(attr).map(|s| s.to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use insta::assert_yaml_snapshot;

    #[test]
    fn test_parse_anime_grid() {
        let html = include_str!("../../tests/fixtures/ongoing-anime.html");
        let result = parse_anime_grid(html);
        assert_yaml_snapshot!("anime_grid", result);
    }

    #[test]
    fn test_parse_pagination() {
        let html = include_str!("../../tests/fixtures/pagination.html");
        let root = Html::parse_document(html);
        let pagination = parse_pagination(&root);
        assert_yaml_snapshot!("pagination", pagination);
    }
}