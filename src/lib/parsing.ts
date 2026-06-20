/**
 * Extract the last non-empty path segment from a URL.
 * Mirrors Python's slug_from_url().
 */
export function slugFromUrl(url: string | undefined | null): string {
  if (!url) return "";
  try {
    const parsed = new URL(url);
    const path = parsed.pathname.replace(/^\/+|\/+$/g, "");
    if (!path) return "";
    const parts = path.split("/");
    return parts[parts.length - 1] ?? "";
  } catch {
    // Not an absolute URL — treat as a raw path
    const path = url.replace(/^\/+|\/+$/g, "");
    if (!path) return "";
    const parts = path.split("/");
    return parts[parts.length - 1] ?? "";
  }
}

/** Collapse whitespace and trim. Mirrors Python's clean_text(). */
export function cleanText(text: string | undefined | null): string {
  if (!text) return "";
  return text.replace(/\s+/g, " ").trim();
}

/** Extract the first integer found in a string, or null. */
export function extractEpisodeCount(text: string | undefined | null): number | null {
  if (!text) return null;
  const m = text.match(/(\d+)/);
  return m ? parseInt(m[1], 10) : null;
}

/** Returns null if a string is empty after cleaning, otherwise the cleaned string. */
export function cleanOrNull(text: string | undefined | null): string | null {
  const cleaned = cleanText(text);
  return cleaned.length > 0 ? cleaned : null;
}
