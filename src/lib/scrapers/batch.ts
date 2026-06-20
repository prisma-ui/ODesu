import { CheerioAPI } from "cheerio";
import * as cheerio from "cheerio";
import type { AnyNode, Element } from "domhandler";
import { cleanText, slugFromUrl } from "../parsing";
import { BatchDetail, DownloadQuality, GenreLink } from "../types";

/**
 * Batch page uses `<b>Label</b>: value<br/>` format inside .infos, not <p> tags.
 * Walks sibling nodes after each <b> until the next <br>, mirroring the
 * Python implementation's next_sibling loop.
 */
function parseInfosBr($: CheerioAPI): { info: Record<string, string>; genres: GenreLink[] } {
  const info: Record<string, string> = {};
  const genres: GenreLink[] = [];
  const container = $(".infos").first();
  if (!container.length) return { info, genres };

  const isBr = (node: AnyNode | null | undefined) =>
    !!node && node.type === "tag" && (node as Element).name === "br";
  const isTag = (node: AnyNode | null | undefined, name: string) =>
    !!node && node.type === "tag" && (node as Element).name === name;

  let genreB: AnyNode | null = null;

  container
    .find("b")
    .toArray()
    .forEach((bEl) => {
      const label = cleanText($(bEl).text()).toLowerCase();
      if (label.includes("genre")) {
        genreB = bEl;
        return;
      }
      const valueParts: string[] = [];
      let node: AnyNode | null = (bEl as Element).nextSibling;
      while (node && !isBr(node)) {
        if (node.type === "text") {
          valueParts.push((node as unknown as { data: string }).data);
        } else if (node.type === "tag") {
          valueParts.push($(node).text());
        }
        node = node.nextSibling;
      }
      const value = cleanText(valueParts.join("")).replace(/^[:\s]+/, "").trim();
      info[label] = value;
    });

  if (genreB) {
    let node: AnyNode | null = (genreB as Element).nextSibling;
    while (node && !isBr(node)) {
      if (isTag(node, "a")) {
        const $a = $(node as Element);
        genres.push({ name: cleanText($a.text()), slug: slugFromUrl($a.attr("href")) });
      }
      node = node.nextSibling;
    }
  }

  return { info, genres };
}

export function parseBatch(html: string): BatchDetail {
  const $ = cheerio.load(html);

  const h1 = $(".jdlrx h1").first();
  let title = h1.length ? cleanText(h1.text()) : "";
  title = title.replace(/\[BATCH\]\s*Subtitle Indonesia/i, "").trim();

  const thumb = $(".imganime img").first();
  const { info, genres } = parseInfosBr($);
  const infoWithGenres: Record<string, unknown> = { ...info, genres };

  const downloadLinkA = $(".totalepisode h3 a").first();

  const downloads: DownloadQuality[] = [];
  $(".batchlink ul").each((_, ul) => {
    $(ul)
      .find("li")
      .each((_, li) => {
        const $li = $(li);
        const strong = $li.find("strong").first();
        if (!strong.length) return;
        const label = cleanText(strong.text());
        const parts = label.split(/\s+/).filter(Boolean);
        const fmt = parts[0] || "";
        const quality = parts[1] || "";
        const sizeTag = $li.find("i").first();
        const size = sizeTag.length ? cleanText(sizeTag.text()) : null;
        const links = $li
          .find("a[href]")
          .toArray()
          .map((a) => {
            const $a = $(a);
            return { host: cleanText($a.text()), url: $a.attr("href") || "" };
          });
        downloads.push({ format: fmt, quality, size, links });
      });
  });

  return {
    title,
    thumbnail: thumb.attr("src") ?? null,
    anime_slug: downloadLinkA.length ? slugFromUrl(downloadLinkA.attr("href")) : null,
    info: infoWithGenres,
    downloads,
  };
}
