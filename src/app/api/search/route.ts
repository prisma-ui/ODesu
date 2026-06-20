import { NextRequest, NextResponse } from "next/server";
import { fetchHtml } from "@/lib/http";
import { parseSearch } from "@/lib/scrapers/search";
import { UpstreamFetchError } from "@/lib/http";

export const dynamic = "force-dynamic";

export async function GET(req: NextRequest) {
  const q = req.nextUrl.searchParams.get("q");
  if (!q || q.length < 1) {
    return NextResponse.json({ detail: "Query parameter 'q' is required" }, { status: 422 });
  }

  try {
    const html = await fetchHtml(`/?s=${encodeURIComponent(q)}&post_type=anime`);
    return NextResponse.json({ query: q, results: parseSearch(html) });
  } catch (e) {
    if (e instanceof UpstreamFetchError) {
      return NextResponse.json({ detail: `Upstream fetch failed: ${e.message}` }, { status: 502 });
    }
    return NextResponse.json({ detail: `Internal error: ${(e as Error).message}` }, { status: 500 });
  }
}
