import * as cheerio from "cheerio";
import { parseDetpost } from "./animeGrid";
import { HomeResponse } from "../types";

export function parseHome(html: string): HomeResponse {
  const $ = cheerio.load(html);
  const sections = $(".venutama .rseries");

  let ongoing: HomeResponse["ongoing"] = [];
  let complete: HomeResponse["complete"] = [];

  sections.each((_, section) => {
    const $section = $(section);
    const header = $section.find(".rvad h1").first();
    const headerText = header.length ? header.text() : "";
    const items = $section
      .find(".venz ul li")
      .toArray()
      .map((li) => parseDetpost($, li));

    if (headerText.includes("On-going")) {
      ongoing = items;
    } else if (headerText.includes("Complete")) {
      complete = items;
    }
  });

  return { ongoing, complete };
}
