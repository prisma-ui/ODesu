use scraper::{ElementRef, Html, Node, Selector};
use std::borrow::Cow;

/// Extract the last non-empty path segment from a URL.
/// Mirrors Python's slug_from_url().
pub fn slug_from_url(url: Option<&str>) -> String {
    let url = match url {
        Some(u) if !u.is_empty() => u,
        _ => return String::new(),
    };

    // Try to parse as absolute URL
    if let Ok(parsed) = url::Url::parse(url) {
        let path = parsed.path().trim_matches('/');
        if path.is_empty() {
            return String::new();
        }
        return path.split('/').last().unwrap_or("").to_string();
    }

    // Treat as raw path
    let path = url.trim_matches('/');
    if path.is_empty() {
        return String::new();
    }
    path.split('/').last().unwrap_or("").to_string()
}

/// Collapse whitespace and trim. Mirrors Python's clean_text().
pub fn clean_text(text: Option<&str>) -> String {
    match text {
        Some(t) => t.split_whitespace().collect::<Vec<_>>().join(" "),
        None => String::new(),
    }
}

/// Returns None if a string is empty after cleaning, otherwise the cleaned string.
pub fn clean_or_null(text: Option<&str>) -> Option<String> {
    let cleaned = clean_text(text);
    if cleaned.is_empty() {
        None
    } else {
        Some(cleaned)
    }
}

/// Extract the first integer found in a string, or None.
pub fn extract_episode_count(text: Option<&str>) -> Option<u32> {
    text.and_then(|t| {
        regex::Regex::new(r"(\d+)")
            .ok()?
            .captures(t)
            .and_then(|cap| cap.get(1))
            .and_then(|m| m.as_str().parse().ok())
    })
}

/// Get text content from an ElementRef, cleaned
pub fn element_text(el: ElementRef<'_>) -> String {
    clean_text(Some(el.text().collect::<String>().as_str()))
}

/// Get attribute value from an ElementRef, cleaned
pub fn element_attr(el: ElementRef<'_>, attr: &str) -> Option<String> {
    el.value().attr(attr).map(|s| s.to_string())
}

/// Find first matching element and return its text
pub fn find_text(root: &Html, selector: &Selector) -> Option<String> {
    root.select(selector).next().map(|el| element_text(el))
}

/// Find first matching element and return its attribute
pub fn find_attr(root: &Html, selector: &Selector, attr: &str) -> Option<String> {
    root.select(selector).next().and_then(|el| element_attr(el, attr))
}

/// Select multiple elements and map to a vector
pub fn select_map<T>(root: &Html, selector: &Selector, f: impl Fn(ElementRef<'_>) -> T) -> Vec<T> {
    root.select(selector).map(f).collect()
}

/// Parse sibling nodes after a <b> tag until <br/> - used for batch page parsing
/// This mirrors the Python implementation's next_sibling loop
pub fn parse_infos_br(
    root: &Html,
    container_selector: &Selector,
) -> (std::collections::HashMap<String, String>, Vec<GenreLink>) {
    use crate::types::GenreLink;

    let mut info = std::collections::HashMap::new();
    let mut genres = Vec::new();

    let container = match root.select(container_selector).next() {
        Some(c) => c,
        None => return (info, genres),
    };

    let is_br = |node: &Node| matches!(node.value(), Node::Element(el) if el.name() == "br");
    let is_tag = |node: &Node, name: &str| {
        matches!(node.value(), Node::Element(el) if el.name() == name)
    };

    let mut genre_b_node: Option<Node> = None;

    for b_el in container.select(&Selector::parse("b").unwrap()) {
        let label = clean_text(Some(&b_el.text().collect::<String>())).to_lowercase();

        if label.contains("genre") {
            genre_b_node = Some(b_el.value().clone());
            continue;
        }

        let mut value_parts = Vec::new();
        let mut node = b_el.next_sibling();

        while let Some(n) = node {
            if is_br(&n) {
                break;
            }
            match n.value() {
                Node::Text(text) => value_parts.push(text.to_string()),
                Node::Element(el) => value_parts.push(el.text().collect::<String>()),
                _ => {}
            }
            node = n.next_sibling();
        }

        let value = clean_text(Some(&value_parts.join("")))
            .trim_start_matches(':')
            .trim()
            .to_string();
        info.insert(label, value);
    }

    // Parse genres after the genre <b> tag
    if let Some(genre_b) = genre_b_node {
        let mut node = Node::next_sibling(&genre_b);
        while let Some(n) = node {
            if is_br(&n) {
                break;
            }
            if is_tag(&n, "a") {
                if let Node::Element(el) = n.value() {
                    let name = clean_text(Some(&el.text().collect::<String>()));
                    let slug = slug_from_url(el.attr("href"));
                    if !name.is_empty() {
                        genres.push(GenreLink { name, slug });
                    }
                }
            }
            node = n.next_sibling();
        }
    }

    (info, genres)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_slug_from_url() {
        assert_eq!(slug_from_url(Some("https://otakudesu.blog/anime/one-piece/")), "one-piece");
        assert_eq!(slug_from_url(Some("/anime/naruto/")), "naruto");
        assert_eq!(slug_from_url(Some("bleach")), "bleach");
        assert_eq!(slug_from_url(None), "");
        assert_eq!(slug_from_url(Some("")), "");
    }

    #[test]
    fn test_clean_text() {
        assert_eq!(clean_text(Some("  hello   world  ")), "hello world");
        assert_eq!(clean_text(Some("")), "");
        assert_eq!(clean_text(None), "");
    }

    #[test]
    fn test_clean_or_null() {
        assert_eq!(clean_or_null(Some("  hello  ")), Some("hello".to_string()));
        assert_eq!(clean_or_null(Some("   ")), None);
        assert_eq!(clean_or_null(None), None);
    }

    #[test]
    fn test_extract_episode_count() {
        assert_eq!(extract_episode_count(Some("Episode 24")), Some(24));
        assert_eq!(extract_episode_count(Some("Total 12 episodes")), Some(12));
        assert_eq!(extract_episode_count(Some("No numbers")), None);
        assert_eq!(extract_episode_count(None), None);
    }
}