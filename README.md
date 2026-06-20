# Otakudesu Scraper API (Next.js)

Next.js/TypeScript port of the original FastAPI + Playwright scraper for otakudesu.blog. Ported from Python to a Vercel-friendly stack: `fetch` with browser-like headers instead of a headless browser, and `cheerio` instead of BeautifulSoup/lxml.

## Endpoints
- `GET /api/home` — ongoing + complete preview
- `GET /api/ongoing?page=1`
- `GET /api/complete?page=1`
- `GET /api/anime-list` — alphabetical index
- `GET /api/jadwal-rilis`
- `GET /api/genres`
- `GET /api/genres/{slug}?page=1`
- `GET /api/search?q=`
- `GET /api/anime/{slug}` — detail + episode list + batch link
- `GET /api/episode/{slug}` — player iframe, mirrors, download links
- `GET /api/batch/{slug}` — batch download links
- `POST /api/mirror?data_content={base64}` (or JSON body `{ "data_content": "..." }`) — resolves a mirror's `data-content` (from `/api/episode`) into an embeddable iframe HTML
- `GET /api/ping` — health check
- `GET /docs` — interactive Swagger UI
- `GET /api/openapi.json` — raw OpenAPI 3.0 spec

## Env vars
Copy `.env.example` to `.env.local`:
- `TARGET_BASE_URL` — base URL of the target site, e.g. `https://otakudesu.blog`
- `CF_PROXY_URL` (optional) — Cloudflare Worker forward-proxy prefix, e.g. `https://your-worker.workers.dev/?url=`. If unset, requests go directly to `TARGET_BASE_URL` with browser-like headers.
- `REQUEST_TIMEOUT_MS` (optional) — default `20000`.

## Local run
```bash
npm install
npm run dev
```

## Deploy
Deploy to Vercel as a standard Next.js app — no Docker, no headless browser, no Playwright binary needed. Set `TARGET_BASE_URL` (and `CF_PROXY_URL` if needed) in the project's environment variables.

```bash
vercel
```

## Architecture notes vs. the original Python version
- **No headless browser.** The original used Playwright (`page.goto` + `page.content()`) primarily to send a realistic browser fingerprint/headers, not to execute JS — `wait_until="domcontentloaded"` confirms the target HTML doesn't require client-side rendering. This port uses `fetch` with the same `User-Agent`/`Accept-Language` headers, which is far cheaper to run on serverless (Vercel) and removes the Docker/Playwright dependency entirely.
- **`/api/mirror`** replicates the site's own two-step WordPress `admin-ajax.php` flow (fetch nonce, then fetch base64 embed HTML), using a minimal in-memory cookie jar to keep both POSTs in the same session, since the nonce is session-bound.
- **`cheerio`** replaces BeautifulSoup + lxml for HTML parsing; the sibling-node walk in the batch-page parser (`<b>Label</b>: value<br/>` format) is preserved using cheerio's underlying `domhandler` node references.
- If any endpoint returns empty/stripped data, the target server is likely serving a bot-detected variant — set `CF_PROXY_URL` to route requests through a Cloudflare Worker, same as the original.
- Download links (`link.desustream.com/?id=...`) are a redirect layer — the final host URL (Mega/GoFile/etc.) is resolved by following the redirect, not present directly in scraped HTML.
