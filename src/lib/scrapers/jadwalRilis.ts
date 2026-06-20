import * as cheerio from "cheerio";
import { cleanText, slugFromUrl } from "../parsing";
import { JadwalRilisResponse } from "../types";

export function parseJadwalRilis(html: string): JadwalRilisResponse {
  const $ = cheerio.load(html);
  const schedule: JadwalRilisResponse = {};

  $(".kglist321").each((_, block) => {
    const $block = $(block);
    const dayH2 = $block.find("h2").first();
    const day = dayH2.length ? cleanText(dayH2.text()) : "Unknown";

    const items = $block
      .find("ul li a")
      .toArray()
      .map((a) => {
        const $a = $(a);
        return { title: cleanText($a.text()), slug: slugFromUrl($a.attr("href")) };
      });

    schedule[day] = items;
  });

  return schedule;
}
