use crate::parsing::*;
use crate::scrapers::anime_grid::parse_pagination;
use crate::types::{GenreDetailItem, GenreDetailResponse, GenreLink};
use scraper::{Html, Selector};

lazy_static::lazy_static! {
    static ref GENRE_LIST_SELECTOR: Selector = Selector::parse("ul.genres li a").unwrap();
    static ref GENRE_DETAIL_CARD_SELECTOR: Selector = Selector::parse(".col-anime-con .col-anime").unwrap();
    static ref GENRE_TITLE_SELECTOR: Selector = Selector::parse(".col-anime-title a").unwrap();
    static ref GENRE_STUDIO_SELECTOR: Selector = Selector::parse(".col-anime-studio").unwrap();
    static ref GENRE_EPS_SELECTOR: Selector = Selector::parse(".col-anime-eps").unwrap();
    static ref GENRE_RATING_SELECTOR: Selector = Selector::parse(".col-anime-rating").unwrap();
    static ref GENRE_COVER_SELECTOR: Selector = Selector::parse(".col-anime-cover img").unwrap();
    static ref GENRE_SYNOPSIS_SELECTOR: Selector = Selector::parse(".col-synopsis").unwrap();
    static ref GENRE_DATE_SELECTOR: Selector = Selector::parse(".col-anime-date").unwrap();
    static ref GENRE_GENRES_SELECTOR: Selector = Selector::parse(".col-anime-genre a").unwrap();
    static ref GENRE_PAGE_TITLE_SELECTOR: Selector = Selector::parse(".rvad h1").unwrap();
}

pub fn parse_genre_list(html: &str) -> Vec<GenreLink> {
    let root = Html::parse_document(html);

    root.select(&GENRE_LIST_SELECTOR)
        .map(|a| GenreLink {
            name: clean_text(Some(&a.text().collect::<String>())),
            slug: slug_from_url(a.value().attr("href")),
        })
        .collect()
}

pub fn parse_genre_detail(html: &str) -> GenreDetailResponse {
    let root = Html::parse_document(html);

    let items: Vec<GenreDetailItem> = root
        .select(&GENRE_DETAIL_CARD_SELECTOR)
        .map(|card| {
            let title_a = card.select(&GENRE_TITLE_SELECTOR).next();
            let studio = card.select(&GENRE_STUDIO_SELECTOR).next();
            let eps = card.select(&GENRE_EPS_SELECTOR).next();
            let rating = card.select(&GENRE_RATING_SELECTOR).next();
            let cover = card.select(&GENRE_COVER_SELECTOR).next();
            let synopsis_p = card.select(&GENRE_SYNOPSIS_SELECTOR).next();
            let date = card.select(&GENRE_DATE_SELECTOR).next();

            let genres: Vec<GenreLink> = card
                .select(&GENRE_GENRES_SELECTOR)
                .map(|g| GenreLink {
                    name: clean_text(Some(&g.text().collect::<String>())),
                    slug: slug_from_url(g.value().attr("href")),
                })
                .collect();

            GenreDetailItem {
                title: title_a
                    .map(|a| clean_text(Some(&a.text().collect::<String>())))
                    .unwrap_or_default(),
                slug: title_a
                    .map(|a| slug_from_url(a.value().attr("href")))
                    .unwrap_or_default(),
                studio: studio.map(|el| clean_text(Some(&el.text().collect::<String>()))),
                episodes: eps.map(|el| clean_text(Some(&el.text().collect::<String>()))),
                rating: rating.map(|el| clean_text(Some(&el.text().collect::<String>()))),
                genres,
                thumbnail: cover.and_then(|el| el.value().attr("src").map(|s| s.to_string())),
                synopsis: synopsis_p.map(|el| clean_text(Some(&el.text().collect::<String>()))),
                season: date.map(|el| clean_text(Some(&el.text().collect::<String>()))),
            }
        })
        .collect();

    let genre_title = find_text_optional(&root, &GENRE_PAGE_TITLE_SELECTOR).unwrap_or_default();

    GenreDetailResponse {
        genre_title,
        anime_list: items,
        pagination: parse_pagination(&root),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use insta::assert_yaml_snapshot;

    #[test]
    fn test_parse_genre_list() {
        let html = include_str!("../../tests/fixtures/genres.html");
        let result = parse_genre_list(html);
        assert_yaml_snapshot!("genre_list", result);
    }

    #[test]
    fn test_parse_genre_detail() {
        let html = include_str!("../../tests/fixtures/genre-detail.html");
        let result = parse_genre_detail(html);
        assert_yaml_snapshot!("genre_detail", result);
    }
}