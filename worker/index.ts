import { Hono } from "hono";
import { cors } from "hono/cors";
import { fetchHtml, resolveMirrorEmbed, setEnv, UpstreamFetchError } from "../src/lib/http";
import { spec } from "../src/lib/openapi";
import { parseHome } from "../src/lib/scrapers/home";
import { parseAnimeGrid } from "../src/lib/scrapers/animeGrid";
import { parseAnimeList } from "../src/lib/scrapers/animeList";
import { parseJadwalRilis } from "../src/lib/scrapers/jadwalRilis";
import { parseGenreList, parseGenreDetail } from "../src/lib/scrapers/genres";
import { parseSearch } from "../src/lib/scrapers/search";
import { parseAnimeDetail } from "../src/lib/scrapers/animeDetail";
import { parseEpisode } from "../src/lib/scrapers/episode";
import { parseBatch } from "../src/lib/scrapers/batch";

const app = new Hono();
app.use("/api/*", cors());

app.onError((e, c) =>
  e instanceof UpstreamFetchError
    ? c.json({ detail: `Upstream fetch failed: ${e.message}` }, 502)
    : c.json({ detail: `Internal error: ${e.message}` }, 500)
);

const pageOf = (v?: string) => Math.max(1, parseInt(v || "1", 10) || 1);
const paged = (base: string, page: number) => (page === 1 ? `${base}/` : `${base}/page/${page}/`);

app.get("/", (c) => c.redirect("/api/openapi.json"));
app.get("/api/ping", (c) => c.json({ status: "ok" }));
app.get("/api/openapi.json", (c) => c.json(spec));

app.get("/api/home", async (c) => c.json(parseHome(await fetchHtml("/"))));
app.get("/api/ongoing", async (c) =>
  c.json(parseAnimeGrid(await fetchHtml(paged("/ongoing-anime", pageOf(c.req.query("page"))))))
);
app.get("/api/complete", async (c) =>
  c.json(parseAnimeGrid(await fetchHtml(paged("/complete-anime", pageOf(c.req.query("page"))))))
);
app.get("/api/anime-list", async (c) => c.json(parseAnimeList(await fetchHtml("/anime-list/"))));
app.get("/api/jadwal-rilis", async (c) => c.json(parseJadwalRilis(await fetchHtml("/jadwal-rilis/"))));
app.get("/api/genres", async (c) => c.json(parseGenreList(await fetchHtml("/genre-list/"))));
app.get("/api/genres/:slug", async (c) =>
  c.json(
    parseGenreDetail(await fetchHtml(paged(`/genres/${c.req.param("slug")}`, pageOf(c.req.query("page")))))
  )
);
app.get("/api/search", async (c) => {
  const q = c.req.query("q");
  if (!q) return c.json({ detail: "Query parameter 'q' is required" }, 422);
  const html = await fetchHtml(`/?s=${encodeURIComponent(q)}&post_type=anime`);
  return c.json({ query: q, results: parseSearch(html) });
});
app.get("/api/anime/:slug", async (c) =>
  c.json(parseAnimeDetail(await fetchHtml(`/anime/${c.req.param("slug")}/`)))
);
app.get("/api/episode/:slug", async (c) =>
  c.json(parseEpisode(await fetchHtml(`/episode/${c.req.param("slug")}/`)))
);
app.get("/api/batch/:slug", async (c) =>
  c.json(parseBatch(await fetchHtml(`/batch/${c.req.param("slug")}/`)))
);

app.post("/api/mirror", async (c) => {
  let dataContent = c.req.query("data_content") ?? null;
  if (!dataContent) {
    try {
      dataContent = (await c.req.json())?.data_content ?? null;
    } catch {}
  }
  if (!dataContent) return c.json({ detail: "Missing 'data_content'" }, 422);
  try {
    const decoded = JSON.parse(atob(dataContent)) as Record<string, string>;
    return c.json({ embed_html: await resolveMirrorEmbed(decoded) });
  } catch (e) {
    return c.json({ detail: `Failed to resolve mirror: ${(e as Error).message}` }, 502);
  }
});


// Forward proxy bawaan: /proxy?url=<encoded URL>
const FORWARD = ["user-agent", "accept", "accept-language", "content-type", "cookie", "referer", "origin"];
app.all("/proxy", async (c) => {
  const env = c.env as Record<string, string | undefined>;
  if (env.PROXY_KEY && c.req.header("x-proxy-key") !== env.PROXY_KEY)
    return c.json({ detail: "unauthorized" }, 401);
  const target = c.req.query("url");
  if (!target) return c.json({ detail: "missing url" }, 400);
  let t: URL;
  try { t = new URL(target); } catch { return c.json({ detail: "invalid url" }, 400); }
  const allowed = (env.ALLOWED_HOSTS || new URL(env.TARGET_BASE_URL || "https://otakudesu.blog").hostname)
    .split(",").map((x) => x.trim().toLowerCase()).filter(Boolean);
  const host = t.hostname.toLowerCase();
  if (!/^https?:$/.test(t.protocol) || !allowed.some((a) => host === a || host.endsWith("." + a)))
    return c.json({ detail: "host not allowed" }, 403);

  const headers = new Headers();
  for (const h of FORWARD) { const v = c.req.header(h); if (v) headers.set(h, v); }
  const hasBody = !["GET", "HEAD"].includes(c.req.method);
  const r = await fetch(t, { method: c.req.method, headers, body: hasBody ? await c.req.arrayBuffer() : undefined });
  const out = new Headers();
  const ct = r.headers.get("content-type");
  if (ct) out.set("content-type", ct);
  for (const ck of r.headers.getSetCookie()) out.append("set-cookie", ck);
  return new Response(r.body, { status: r.status, headers: out });
});

export default {
  fetch(req: Request, env: Record<string, unknown>, ctx: unknown) {
    setEnv(env);
    return app.fetch(req, env, ctx as never);
  },
};
