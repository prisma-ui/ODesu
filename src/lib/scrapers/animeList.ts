import * as cheerio from "cheerio";
import { cleanText, slugFromUrl } from "../parsing";
import { AnimeListGroups } from "../types";

export function parseAnimeList(html: string): AnimeListGroups {
  const $ = cheerio.load(html);
  const groups: AnimeListGroups = {};

  $(".bariskelom").each((_, row) => {
    const $row = $(row);
    const letterA = $row.find(".barispenz a").first();
    const letter = letterA.length ? cleanText(letterA.text()) : "#";

    const items = $row
      .find(".jdlbar a.hodebgst")
      .toArray()
      .map((a) => {
        const $a = $(a);
        return { title: cleanText($a.text()), slug: slugFromUrl($a.attr("href")) };
      });

    if (items.length) groups[letter] = items;
  });

  return groups;
}
