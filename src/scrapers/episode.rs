use crate::parsing::*;
use crate::types::{DownloadQuality, DownloadLink, EpisodeDetail, EpisodeLink, GenreLink, MirrorHost};
use scraper::{Html, Selector};
use std::collections::HashMap;

lazy_static::lazy_static! {
    static ref TITLE_SELECTOR: Selector = Selector::parse("h1.posttl").unwrap();
    static ref STREAM_IFRAME_SELECTOR: Selector = Selector::parse(".responsive-embed-stream iframe").unwrap();
    static ref MIRROR_UL_SELECTOR: Selector = Selector::parse(".mirrorstream ul").unwrap();
    static ref MIRROR_LINK_SELECTOR: Selector = Selector::parse("li a[data-content]").unwrap();
    static ref DOWNLOAD_UL_SELECTOR: Selector = Selector::parse(".download ul").unwrap();
    static ref DOWNLOAD_LI_SELECTOR: Selector = Selector::parse("li").unwrap();
    static ref STRONG_SELECTOR: Selector = Selector::parse("strong").unwrap();
    static ref SIZE_TAG_SELECTOR: Selector = Selector::parse("i").unwrap();
    static ref DOWNLOAD_LINK_SELECTOR: Selector = Selector::parse("a[href]").unwrap();
    static ref SELECT_COG_SELECTOR: Selector = Selector::parse("select#selectcog").unwrap();
    static ref OPTION_SELECTOR: Selector = Selector::parse("option").unwrap();
    static ref FLIR_SELECTOR: Selector = Selector::parse(".flir").unwrap();
    static ref CUKDER_SELECTOR: Selector = Selector::parse(".cukder").unwrap();
    static ref CUKDER_INFO_SELECTOR: Selector = Selector::parse(".infozingle p").unwrap();
    static ref PREV_EPISODE_SELECTOR: Selector = Selector::parse("a[title='Episode Sebelumnya']").unwrap();
}

pub fn parse_episode(html: &str) -> EpisodeDetail {
    let root = Html::parse_document(html);

    // Title
    let title = find_text(&root, &TITLE_SELECTOR).unwrap_or_default();

    // Stream URL
    let stream_url = find_attr(&root, &STREAM_IFRAME_SELECTOR, "src");

    // Mirrors - grouped by quality (m360p, m480p, m720p, etc.)
    let mut mirrors: HashMap<String, Vec<MirrorHost>> = HashMap::new();
    for ul in root.select(&MIRROR_UL_SELECTOR) {
        let classes = ul.value().attr("class").unwrap_or("").split_whitespace().collect::<Vec<_>>();
        let mut quality: Option<String> = None;

        for cls in classes {
            if !cls.starts_with('m') {
                continue;
            }
            let rest = &cls[1..]; // e.g. "360p"
            let stripped = rest.trim_end_matches('p');
            if !stripped.is_empty() && stripped.chars().all(|c| c.is_ascii_digit()) {
                quality = Some(rest.to_string());
                break;
            }
        }

        let Some(quality) = quality else {
            continue;
        };

        let mut hosts = Vec::new();
        for a in ul.select(&MIRROR_LINK_SELECTOR) {
            hosts.push(MirrorHost {
                host: clean_text(Some(&a.text().collect::<String>())),
                data_content: a.value().attr("data-content").unwrap_or("").to_string(),
            });
        }

        if !hosts.is_empty() {
            mirrors.insert(quality, hosts);
        }
    }

    // Download links
    let mut downloads = Vec::new();
    for ul in root.select(&DOWNLOAD_UL_SELECTOR) {
        for li in ul.select(&DOWNLOAD_LI_SELECTOR) {
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
                .select(&DOWNLOAD_LINK_SELECTOR)
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

    // Episode navigation (select#selectcog)
    let mut episode_list = Vec::new();
    if let Some(select) = root.select(&SELECT_COG_SELECTOR).next() {
        for option in select.select(&OPTION_SELECTOR) {
            let value = option.value().attr("value").unwrap_or("");
            if value.is_empty() || value == "0" {
                continue;
            }
            episode_list.push(EpisodeLink {
                title: clean_text(Some(&option.text().collect::<String>())),
                slug: slug_from_url(Some(value)),
                date: None,
            });
        }
    }

    // Previous episode and anime slug from .flir
    let mut prev_episode = None;
    let mut anime_slug = None;

    if let Some(flir) = root.select(&FLIR_SELECTOR).next() {
        // Previous episode
        if let Some(prev_a) = flir.select(&PREV_EPISODE_SELECTOR).next() {
            prev_episode = Some(slug_from_url(prev_a.value().attr("href")));
        }

        // Anime slug (first link that's not "Episode Sebelumnya")
        for a in flir.select(&Selector::parse("a").unwrap()) {
            if anime_slug.is_some() {
                break;
            }
            let title_attr = a.value().attr("title").unwrap_or("");
            if title_attr != "Episode Sebelumnya" {
                anime_slug = Some(slug_from_url(a.value().attr("href")));
            }
        }
    }

    // Info block (.cukder .infozingle)
    let mut info = HashMap::new();
    let mut genres = Vec::new();

    if let Some(cukder) = root.select(&CUKDER_SELECTOR).next() {
        for p in cukder.select(&CUKDER_INFO_SELECTOR) {
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
            info.insert(label, serde_json::Value::String(value));
        }
    }
    info.insert("genres".to_string(), serde_json::to_value(genres).unwrap());

    EpisodeDetail {
        title,
        stream_url,
        mirrors,
        downloads,
        episode_list,
        prev_episode,
        anime_slug,
        info,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use insta::assert_yaml_snapshot;

    #[test]
    fn test_parse_episode() {
        let html = include_str!("../../tests/fixtures/episode.html");
        let result = parse_episode(html);
        assert_yaml_snapshot!("episode", result);
    }
}