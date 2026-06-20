import * as cheerio from "cheerio";
import { cleanText, slugFromUrl } from "../parsing";
import { parsePagination } from "./animeGrid";
import { GenreDetailItem, GenreDetailResponse, GenreLink } from "../types";

export function parseGenreList(html: string): GenreLink[] {
  const $ = cheerio.load(html);
  return $("ul.genres li a")
    .toArray()
    .map((a) => {
      const $a = $(a);
      return { name: cleanText($a.text()), slug: slugFromUrl($a.attr("href")) };
    });
}

export function parseGenreDetail(html: string): GenreDetailResponse {
  const $ = cheerio.load(html);

  const items: GenreDetailItem[] = $(".col-anime-con .col-anime")
    .toArray()
    .map((card) => {
      const $card = $(card);
      const titleA = $card.find(".col-anime-title a").first();
      const studio = $card.find(".col-anime-studio").first();
      const eps = $card.find(".col-anime-eps").first();
      const rating = $card.find(".col-anime-rating").first();
      const cover = $card.find(".col-anime-cover img").first();
      const synopsisP = $card.find(".col-synopsis").first();
      const date = $card.find(".col-anime-date").first();
      const genres: GenreLink[] = $card
        .find(".col-anime-genre a")
        .toArray()
        .map((g) => {
          const $g = $(g);
          return { name: cleanText($g.text()), slug: slugFromUrl($g.attr("href")) };
        });

      return {
        title: titleA.length ? cleanText(titleA.text()) : "",
        slug: titleA.length ? slugFromUrl(titleA.attr("href")) : "",
        studio: studio.length ? cleanText(studio.text()) : null,
        episodes: eps.length ? cleanText(eps.text()) : null,
        rating: rating.length ? cleanText(rating.text()) : null,
        genres,
        thumbnail: cover.attr("src") ?? null,
        synopsis: synopsisP.length ? cleanText(synopsisP.text()) : null,
        season: date.length ? cleanText(date.text()) : null,
      };
    });

  const titleH1 = $(".rvad h1").first();

  return {
    genre_title: titleH1.length ? cleanText(titleH1.text()) : "",
    anime_list: items,
    pagination: parsePagination($),
  };
}
