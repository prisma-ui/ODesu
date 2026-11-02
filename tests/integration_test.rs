//! Integration tests for the Otakudesu API

use otakudesu_api::scrapers::*;
use otakudesu_api::types::*;

#[test]
fn test_parse_home() {
    let html = include_str!("../tests/fixtures/home.html");
    let result = parse_home(html);

    assert_eq!(result.ongoing.len(), 2);
    assert_eq!(result.complete.len(), 1);

    // Check first ongoing anime
    let one_piece = &result.ongoing[0];
    assert_eq!(one_piece.title, "One Piece");
    assert_eq!(one_piece.slug, "one-piece");
    assert_eq!(one_piece.episode, Some("Episode 1120".to_string()));
    assert_eq!(one_piece.day, Some("Sunday".to_string()));
    assert!(one_piece.thumbnail.is_some());

    // Check complete anime
    let death_note = &result.complete[0];
    assert_eq!(death_note.title, "Death Note");
    assert_eq!(death_note.slug, "death-note");
    assert_eq!(death_note.day, Some("Completed".to_string()));
}

#[test]
fn test_slug_from_url() {
    assert_eq!(otakudesu_api::parsing::slug_from_url(Some("https://otakudesu.blog/anime/one-piece/")), "one-piece");
    assert_eq!(otakudesu_api::parsing::slug_from_url(Some("/anime/naruto/")), "naruto");
    assert_eq!(otakudesu_api::parsing::slug_from_url(Some("bleach")), "bleach");
    assert_eq!(otakudesu_api::parsing::slug_from_url(None), "");
    assert_eq!(otakudesu_api::parsing::slug_from_url(Some("")), "");
}

#[test]
fn test_clean_text() {
    assert_eq!(otakudesu_api::parsing::clean_text(Some("  hello   world  ")), "hello world");
    assert_eq!(otakudesu_api::parsing::clean_text(Some("")), "");
    assert_eq!(otakudesu_api::parsing::clean_text(None), "");
}

#[test]
fn test_anime_card_serialization() {
    let card = AnimeCard {
        title: "Test Anime".to_string(),
        slug: "test-anime".to_string(),
        thumbnail: Some("https://example.com/img.jpg".to_string()),
        episode: Some("Episode 1".to_string()),
        day: Some("Monday".to_string()),
        rating: Some("8.5".to_string()),
        date: Some("2024-01-15".to_string()),
    };

    let json = serde_json::to_string(&card).unwrap();
    assert!(json.contains("test-anime"));
    assert!(json.contains("Test Anime"));

    let deserialized: AnimeCard = serde_json::from_str(&json).unwrap();
    assert_eq!(deserialized, card);
}

#[test]
fn test_pagination_default() {
    let pagination = Pagination::default();
    assert_eq!(pagination.current_page, 1);
    assert_eq!(pagination.last_page, 1);
    assert!(!pagination.has_next);
}