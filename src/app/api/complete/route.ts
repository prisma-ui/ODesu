import { NextRequest } from "next/server";
import { fetchHtml } from "@/lib/http";
import { parseAnimeGrid } from "@/lib/scrapers/animeGrid";
import { withUpstreamErrorHandling } from "@/lib/apiResponse";

export const dynamic = "force-dynamic";

export async function GET(req: NextRequest) {
  const pageParam = req.nextUrl.searchParams.get("page");
  const page = Math.max(1, parseInt(pageParam || "1", 10) || 1);

  return withUpstreamErrorHandling(async () => {
    const path = page === 1 ? "/complete-anime/" : `/complete-anime/page/${page}/`;
    const html = await fetchHtml(path);
    return parseAnimeGrid(html);
  });
}
