use crate::parsing::*;
use crate::types::{AnimeDetail, EpisodeLink, GenreLink, RecommendationItem};
use scraper::{ElementRef, Html, Selector};

lazy_static::lazy_static! {
    static ref TITLE_SELECTOR: Selector = Selector::parse(".jdlrx h1").unwrap();
    static ref THUMB_SELECTOR: Selector = Selector::parse(".fotoanime img").unwrap();
    static ref INFOZINGLE_SELECTOR: Selector = Selector::parse(".infozingle p").unwrap();
    static ref SINOPSIS_SELECTOR: Selector = Selector::parse(".sinopc").unwrap();
    static ref EPISODELIST_SELECTOR: Selector = Selector::parse(".episodelist").unwrap();
    static ref MONKTIT_SELECTOR: Selector = Selector::parse(".monktit").unwrap();
    static ref EPISODE_LINK_SELECTOR: Selector = Selector::parse("ul li a").unwrap();
    static ref DATE_SPAN_SELECTOR: Selector = Selector::parse(".zeebr").unwrap();
    static ref RECOMMEND_SELECTOR: Selector = Selector::parse("#recommend-anime-series .isi-konten").unwrap();
    static ref RECOMMEND_A_SELECTOR: Selector = Selector::parse(".isi-anime a").unwrap();
    static ref RECOMMEND_IMG_SELECTOR: Selector = Selector::parse(".isi-anime img").unwrap();
    static ref RECOMMEND_TITLE_SELECTOR: Selector = Selector::parse(".judul-anime").unwrap();
}

pub fn parse_anime_detail(html: &str) -> AnimeDetail {
    let root = Html::parse_document(html);

    // Title
    let title = find_text(&root, &TITLE_SELECTOR)
        .unwrap_or_default()
        .replace("Subtitle Indonesia", "")
        .trim()
        .to_string();

    // Thumbnail
    let thumbnail = find_attr(&root, &THUMB_SELECTOR, "src");

    // Parse infozingle
    let mut info = std::collections::HashMap::new();
    let mut genres = Vec::new();

    for p in root.select(&INFOZINGLE_SELECTOR) {
        let b = p.select(&Selector::parse("b").unwrap()).next();
        if b.is_none() {
            continue;
        }
        let label = clean_text(Some(&b.unwrap().text().collect::<String>())).to_lowercase();

        if label.contains("genre") {
            for a in p.select(&Selector::parse("a").unwrap()) {
                genres.push(GenreLink {
                    name: clean_text(Some(&a.text().collect::<String>())),
                    slug: slug_from_url(a.value().attr("href")),
                });
            }
            continue;
        }

        let text = clean_text(Some(&p.text().collect::<String>()));
        let value = if text.contains(':') {
            text.split(':').skip(1).collect::<Vec<_>>().join(":").trim().to_string()
        } else {
            String::new()
        };
        info.insert(label, value);
    }

    // Synopsis
    let synopsis = find_text(&root, &SINOPSIS_SELECTOR);

    // Episode list and batch
    let mut episode_list = Vec::new();
    let mut batch = None;

    for block in root.select(&EPISODELIST_SELECTOR) {
        let header_text = block
            .select(&MONKTIT_SELECTOR)
            .next()
            .map(|el| clean_text(Some(&el.text().collect::<String>())))
            .unwrap_or_default();

        let links: Vec<_> = block.select(&EPISODE_LINK_SELECTOR).collect();

        if header_text.contains("Episode List") {
            for a in links {
                let li = a
                    .parent()
                    .and_then(|p| p.parent())
                    .and_then(ElementRef::wrap);
                let date = li
                    .and_then(|li| li.select(&DATE_SPAN_SELECTOR).next())
                    .map(|el| clean_text(Some(&el.text().collect::<String>())));

                episode_list.push(EpisodeLink {
                    title: clean_text(Some(&a.text().collect::<String>())),
                    slug: slug_from_url(a.value().attr("href")),
                    date,
                });
            }
        } else if header_text.contains("Batch") && !links.is_empty() {
            let a = &links[0];
            batch = Some(EpisodeLink {
                title: clean_text(Some(&a.text().collect::<String>())),
                slug: slug_from_url(a.value().attr("href")),
                date: None,
            });
        }
    }

    // Recommendations
    let mut recommendations = Vec::new();
    for item in root.select(&RECOMMEND_SELECTOR) {
        let a = item.select(&RECOMMEND_A_SELECTOR).next();
        let img = item.select(&RECOMMEND_IMG_SELECTOR).next();
        let title_el = item.select(&RECOMMEND_TITLE_SELECTOR).next();

        if let Some(a) = a {
            recommendations.push(RecommendationItem {
                title: title_el
                    .map(|el| clean_text(Some(&el.text().collect::<String>())))
                    .unwrap_or_default(),
                slug: slug_from_url(a.value().attr("href")),
                thumbnail: img.and_then(|el| el.value().attr("src").map(|s| s.to_string())),
            });
        }
    }

    AnimeDetail {
        title,
        japanese_title: info.get("japanese").cloned(),
        thumbnail,
        score: info.get("skor").cloned(),
        producer: info.get("produser").cloned(),
        r#type: info.get("tipe").cloned(),
        status: info.get("status").cloned(),
        total_episode: info.get("total episode").cloned(),
        duration: info.get("durasi").cloned(),
        release_date: info.get("tanggal rilis").cloned(),
        studio: info.get("studio").cloned(),
        genres,
        synopsis,
        episode_list,
        batch,
        recommendations,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use insta::assert_yaml_snapshot;

    #[test]
    fn test_parse_anime_detail() {
        let html = include_str!("../../tests/fixtures/anime-detail.html");
        let result = parse_anime_detail(html);
        assert_yaml_snapshot!("anime_detail", result);
    }
}