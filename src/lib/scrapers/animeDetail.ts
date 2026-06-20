import { CheerioAPI } from "cheerio";
import * as cheerio from "cheerio";
import { cleanText, slugFromUrl } from "../parsing";
import { AnimeDetail, EpisodeLink, GenreLink } from "../types";

function parseInfozingle($: CheerioAPI): Record<string, string> {
  const info: Record<string, string> = {};
  $(".infozingle p").each((_, p) => {
    const $p = $(p);
    const b = $p.find("b").first();
    if (!b.length) return;
    const label = cleanText(b.text()).toLowerCase();
    if (label.includes("genre")) return; // genres handled separately (contains <a> tags)
    const text = cleanText($p.text());
    const value = text.includes(":") ? text.split(":").slice(1).join(":").trim() : "";
    info[label] = value;
  });
  return info;
}

export function parseAnimeDetail(html: string): AnimeDetail {
  const $ = cheerio.load(html);

  const h1 = $(".jdlrx h1").first();
  let title = h1.length ? cleanText(h1.text()) : "";
  title = title.replace("Subtitle Indonesia", "").trim();

  const thumb = $(".fotoanime img").first();
  const info = parseInfozingle($);

  const genres: GenreLink[] = [];
  $(".infozingle p").each((_, p) => {
    const $p = $(p);
    const b = $p.find("b").first();
    if (b.length && cleanText(b.text()).toLowerCase().includes("genre")) {
      $p.find("a").each((_, a) => {
        const $a = $(a);
        genres.push({ name: cleanText($a.text()), slug: slugFromUrl($a.attr("href")) });
      });
    }
  });

  const synopsisDiv = $(".sinopc").first();
  const synopsis = synopsisDiv.length ? cleanText(synopsisDiv.text()) : null;

  const episodeList: EpisodeLink[] = [];
  let batch: EpisodeLink | null = null;

  $(".episodelist").each((_, block) => {
    const $block = $(block);
    const header = $block.find(".monktit").first();
    const headerText = header.length ? cleanText(header.text()) : "";
    const links = $block.find("ul li a").toArray();

    if (headerText.includes("Episode List")) {
      for (const a of links) {
        const $a = $(a);
        const li = $a.closest("li");
        const dateSpan = li.find(".zeebr").first();
        episodeList.push({
          title: cleanText($a.text()),
          slug: slugFromUrl($a.attr("href")),
          date: dateSpan.length ? cleanText(dateSpan.text()) : null,
        });
      }
    } else if (headerText.includes("Batch") && links.length) {
      const $a = $(links[0]);
      batch = { title: cleanText($a.text()), slug: slugFromUrl($a.attr("href")) };
    }
  });

  const recommendations: AnimeDetail["recommendations"] = [];
  $("#recommend-anime-series .isi-konten").each((_, item) => {
    const $item = $(item);
    const a = $item.find(".isi-anime a").first();
    const img = $item.find(".isi-anime img").first();
    const titleSpan = $item.find(".judul-anime").first();
    if (a.length) {
      recommendations.push({
        title: titleSpan.length ? cleanText(titleSpan.text()) : "",
        slug: slugFromUrl(a.attr("href")),
        thumbnail: img.attr("src") ?? null,
      });
    }
  });

  return {
    title,
    japanese_title: info["japanese"] ?? null,
    thumbnail: thumb.attr("src") ?? null,
    score: info["skor"] ?? null,
    producer: info["produser"] ?? null,
    type: info["tipe"] ?? null,
    status: info["status"] ?? null,
    total_episode: info["total episode"] ?? null,
    duration: info["durasi"] ?? null,
    release_date: info["tanggal rilis"] ?? null,
    studio: info["studio"] ?? null,
    genres,
    synopsis,
    episode_list: episodeList,
    batch,
    recommendations,
  };
}
