import * as cheerio from "cheerio";
import { cleanText, slugFromUrl } from "../parsing";
import { GenreLink, SearchResult } from "../types";

export function parseSearch(html: string): SearchResult[] {
  const $ = cheerio.load(html);

  return $("ul.chivsrc > li")
    .toArray()
    .map((li) => {
      const $li = $(li);
      const img = $li.find("img").first();
      const titleA = $li.find("h2 a").first();

      const genres: GenreLink[] = $li
        .find(".set a[rel='tag']")
        .toArray()
        .map((g) => {
          const $g = $(g);
          return { name: cleanText($g.text()), slug: slugFromUrl($g.attr("href")) };
        });

      let status: string | null = null;
      let rating: string | null = null;

      $li.find(".set").each((_, setDiv) => {
        const $setDiv = $(setDiv);
        const b = $setDiv.find("b").first();
        if (!b.length) return;
        const label = cleanText(b.text()).toLowerCase();
        let value = cleanText($setDiv.text().replace(b.text(), ""));
        value = value.replace(/^[:\s]+/, "").trim();
        if (label.includes("status")) status = value;
        else if (label.includes("rating")) rating = value;
      });

      return {
        title: titleA.length ? cleanText(titleA.text()) : "",
        slug: titleA.length ? slugFromUrl(titleA.attr("href")) : "",
        thumbnail: img.attr("src") ?? null,
        genres,
        status,
        rating,
      };
    });
}
