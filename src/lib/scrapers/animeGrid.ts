import { CheerioAPI } from "cheerio";
import * as cheerio from "cheerio";
import type { Element } from "domhandler";
import { cleanOrNull, cleanText, slugFromUrl } from "../parsing";
import { AnimeCard, AnimeGridResponse, Pagination } from "../types";

export function parseDetpost($: CheerioAPI, li: Element): AnimeCard {
  const $li = $(li);
  const a = $li.find(".thumb a").first();
  const img = $li.find(".thumb img").first();
  const titleEl = $li.find(".jdlflm").first();
  const epz = $li.find(".epz").first();
  const epztipe = $li.find(".epztipe").first();
  const newnime = $li.find(".newnime").first();

  return {
    title: cleanText(titleEl.text()),
    slug: a.length ? slugFromUrl(a.attr("href")) : "",
    thumbnail: img.attr("src") ?? null,
    episode: epz.length ? cleanText(epz.text()) : null,
    day: epztipe.length ? cleanText(epztipe.text()) : null,
    rating: null,
    date: newnime.length ? cleanText(newnime.text()) : null,
  };
}

/** Variant used by /ongoing-anime and /complete-anime where .epztipe holds a rating, not a day. */
export function parseDetpostGrid($: CheerioAPI, li: Element): AnimeCard {
  const card = parseDetpost($, li);
  return { ...card, rating: card.day, day: null };
}

export function parsePagination($: CheerioAPI): Pagination {
  const nav = $(".pagenavix").first();
  if (!nav.length) {
    return { current_page: 1, last_page: 1, has_next: false };
  }
  const current = nav.find(".page-numbers.current").first();
  const currentPage = current.length ? parseInt(cleanText(current.text()), 10) || 1 : 1;

  const pageNumbers: number[] = [];
  nav.find("a.page-numbers").each((_, a) => {
    const text = cleanText($(a).text());
    if (/^\d+$/.test(text)) pageNumbers.push(parseInt(text, 10));
  });
  const lastPage = Math.max(currentPage, ...pageNumbers, 1);
  const hasNext = nav.find("a.next.page-numbers").length > 0;

  return { current_page: currentPage, last_page: lastPage, has_next: hasNext };
}

/** Used for /ongoing-anime/ and /complete-anime/ (same .detpost structure). */
export function parseAnimeGrid(html: string): AnimeGridResponse {
  const $ = cheerio.load(html);
  const items: AnimeCard[] = $(".venz ul li")
    .toArray()
    .map((li) => parseDetpostGrid($, li));
  return { anime_list: items, pagination: parsePagination($) };
}

export { cleanOrNull };
