import * as cheerio from "cheerio";
import { cleanText, slugFromUrl } from "../parsing";
import { DownloadQuality, EpisodeDetail, EpisodeLink, GenreLink, MirrorHost } from "../types";

export function parseEpisode(html: string): EpisodeDetail {
  const $ = cheerio.load(html);

  const h1 = $("h1.posttl").first();
  const title = h1.length ? cleanText(h1.text()) : "";

  const iframe = $(".responsive-embed-stream iframe").first();
  const streamUrl = iframe.attr("src") ?? null;

  // mirror quality groups: ul.m360p / ul.m480p / ul.m720p
  const mirrors: Record<string, MirrorHost[]> = {};
  $(".mirrorstream ul").each((_, ul) => {
    const $ul = $(ul);
    const classes = ($ul.attr("class") || "").split(/\s+/).filter(Boolean);
    let quality: string | null = null;
    for (const cls of classes) {
      if (!cls.startsWith("m")) continue;
      const rest = cls.slice(1); // e.g. "360p"
      const stripped = rest.replace(/p$/, ""); // e.g. "360"
      if (stripped.length > 0 && /^\d+$/.test(stripped)) {
        quality = rest; // keep as-is, e.g. "360p"
        break;
      }
    }
    if (!quality) return;
    const hosts: MirrorHost[] = [];
    $ul.find("li a[data-content]").each((_, a) => {
      const $a = $(a);
      hosts.push({
        host: cleanText($a.text()),
        data_content: $a.attr("data-content") || "",
      });
    });
    mirrors[quality] = hosts;
  });

  // download links: .download ul li -> <strong>FORMAT QUALITY</strong> + <a> hosts + <i>size</i>
  const downloads: DownloadQuality[] = [];
  $(".download ul").each((_, ul) => {
    $(ul)
      .find("li")
      .each((_, li) => {
        const $li = $(li);
        const strong = $li.find("strong").first();
        if (!strong.length) return;
        const label = cleanText(strong.text()); // e.g. "Mp4 360p" or "MKV 1080p"
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

  // episode navigation
  const select = $("select#selectcog").first();
  const episodeList: EpisodeLink[] = [];
  if (select.length) {
    select.find("option").each((_, option) => {
      const $option = $(option);
      const value = $option.attr("value") || "";
      if (!value || value === "0") return;
      episodeList.push({ title: cleanText($option.text()), slug: slugFromUrl(value) });
    });
  }

  const flir = $(".flir").first();
  let prevSlug: string | null = null;
  let animeSlug: string | null = null;
  if (flir.length) {
    const prevA = flir.find("a[title='Episode Sebelumnya']").first();
    prevSlug = prevA.length ? slugFromUrl(prevA.attr("href")) : null;

    flir.find("a").each((_, a) => {
      const $a = $(a);
      if (animeSlug) return;
      if ($a.attr("title") !== "Episode Sebelumnya") {
        animeSlug = slugFromUrl($a.attr("href"));
      }
    });
  }

  // info block (.cukder .infozingle) - Credit/Encoder/Genres/Duration/Tipe
  const info: Record<string, unknown> = {};
  const genres: GenreLink[] = [];
  const cukder = $(".cukder").first();
  if (cukder.length) {
    cukder.find(".infozingle p").each((_, p) => {
      const $p = $(p);
      const b = $p.find("b").first();
      if (!b.length) return;
      const label = cleanText(b.text()).toLowerCase();
      if (label.includes("genre")) {
        $p.find("a").each((_, a) => {
          const $a = $(a);
          genres.push({ name: cleanText($a.text()), slug: slugFromUrl($a.attr("href")) });
        });
        return;
      }
      const text = cleanText($p.text());
      const value = text.includes(":") ? text.split(":").slice(1).join(":").trim() : "";
      info[label] = value;
    });
  }
  info["genres"] = genres;

  return {
    title,
    stream_url: streamUrl,
    mirrors,
    downloads,
    episode_list: episodeList,
    prev_episode: prevSlug,
    anime_slug: animeSlug,
    info,
  };
}
