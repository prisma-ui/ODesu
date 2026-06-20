const TARGET_BASE_URL = (process.env.TARGET_BASE_URL || "https://otakudesu.blog").replace(/\/+$/, "");
const CF_PROXY_URL = process.env.CF_PROXY_URL || "";
const REQUEST_TIMEOUT_MS = parseInt(process.env.REQUEST_TIMEOUT_MS || "20000", 10);

const BROWSER_HEADERS: Record<string, string> = {
  "User-Agent":
    "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/126.0.0.0 Safari/537.36",
  "Accept":
    "text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,*/*;q=0.8",
  "Accept-Language": "id-ID,id;q=0.9,en-US;q=0.8,en;q=0.7",
};

export class UpstreamFetchError extends Error {}

function targetUrl(path: string): string {
  const full = path.startsWith("http") ? path : `${TARGET_BASE_URL}${path}`;
  return CF_PROXY_URL ? `${CF_PROXY_URL}${encodeURIComponent(full)}` : full;
}

async function withTimeout<T>(promise: Promise<T>, ms: number): Promise<T> {
  const controller = new AbortController();
  const timer = setTimeout(() => controller.abort(), ms);
  try {
    return await promise;
  } finally {
    clearTimeout(timer);
  }
}

/** Fetches raw HTML for a given path (relative to TARGET_BASE_URL) or absolute URL. */
export async function fetchHtml(path: string): Promise<string> {
  const url = targetUrl(path);
  let res: Response;
  try {
    const controller = new AbortController();
    const timer = setTimeout(() => controller.abort(), REQUEST_TIMEOUT_MS);
    res = await fetch(url, { headers: BROWSER_HEADERS, signal: controller.signal, cache: "no-store" }).finally(() =>
      clearTimeout(timer)
    );
  } catch (e) {
    throw new UpstreamFetchError(`Failed to fetch ${url}: ${(e as Error).message}`);
  }
  if (!res.ok) {
    throw new UpstreamFetchError(`Upstream returned ${res.status} for ${url}`);
  }
  return await res.text();
}

/** Minimal cookie jar for the two-step WordPress admin-ajax nonce flow. */
class CookieJar {
  private jar = new Map<string, string>();

  absorb(res: Response) {
    // Node fetch (undici) exposes multiple Set-Cookie headers via getSetCookie()
    const anyHeaders = res.headers as unknown as { getSetCookie?: () => string[] };
    const setCookies: string[] = anyHeaders.getSetCookie ? anyHeaders.getSetCookie() : [];
    for (const cookie of setCookies) {
      const [pair] = cookie.split(";");
      const eq = pair.indexOf("=");
      if (eq === -1) continue;
      const name = pair.slice(0, eq).trim();
      const value = pair.slice(eq + 1).trim();
      if (name) this.jar.set(name, value);
    }
  }

  header(): string {
    return Array.from(this.jar.entries())
      .map(([k, v]) => `${k}=${v}`)
      .join("; ");
  }
}

async function postAdminAjax(
  ajaxUrl: string,
  jar: CookieJar,
  formData: Record<string, string>
): Promise<Record<string, unknown>> {
  const body = new URLSearchParams(formData).toString();
  const res = await withTimeout(
    fetch(ajaxUrl, {
      method: "POST",
      headers: {
        ...BROWSER_HEADERS,
        "Content-Type": "application/x-www-form-urlencoded",
        Cookie: jar.header(),
      },
      body,
      cache: "no-store",
    }),
    REQUEST_TIMEOUT_MS
  );
  jar.absorb(res);
  if (!res.ok) {
    throw new UpstreamFetchError(`admin-ajax.php returned ${res.status}`);
  }
  return (await res.json()) as Record<string, unknown>;
}

/**
 * Replicates the site's WordPress admin-ajax two-step flow used to resolve a
 * mirror's data-content into an embeddable iframe HTML:
 *   1. GET the homepage to obtain session cookies.
 *   2. POST action=aa1208... (no params) -> returns a nonce.
 *   3. POST decoded {id,i,q} + nonce + action=2a3505... -> returns base64 HTML.
 *
 * Both ajax calls share the same cookie jar since the nonce is session-bound.
 */
export async function resolveMirrorEmbed(decoded: Record<string, string>): Promise<string> {
  const jar = new CookieJar();
  const homeUrl = targetUrl("/");
  const ajaxUrl = targetUrl("/wp-admin/admin-ajax.php");

  const homeRes = await withTimeout(
    fetch(homeUrl, { headers: BROWSER_HEADERS, cache: "no-store" }),
    REQUEST_TIMEOUT_MS
  );
  jar.absorb(homeRes);
  await homeRes.text();

  const nonceResp = await postAdminAjax(ajaxUrl, jar, { action: "aa1208d27f29ca340c92c66d1926f13f" });
  const nonce = nonceResp.data;
  if (!nonce || typeof nonce !== "string") {
    throw new UpstreamFetchError("Failed to obtain nonce");
  }

  const result = await postAdminAjax(ajaxUrl, jar, {
    ...decoded,
    nonce,
    action: "2a3505c93b0035d3f455df82bf976b84",
  });

  const htmlB64 = typeof result.data === "string" ? result.data : "";
  return Buffer.from(htmlB64, "base64").toString("utf-8");
}
