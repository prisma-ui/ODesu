import { NextRequest } from "next/server";
import { fetchHtml } from "@/lib/http";
import { parseGenreDetail } from "@/lib/scrapers/genres";
import { withUpstreamErrorHandling } from "@/lib/apiResponse";

export const dynamic = "force-dynamic";

export async function GET(req: NextRequest, { params }: { params: { slug: string } }) {
  const pageParam = req.nextUrl.searchParams.get("page");
  const page = Math.max(1, parseInt(pageParam || "1", 10) || 1);
  const { slug } = params;

  return withUpstreamErrorHandling(async () => {
    const path = page === 1 ? `/genres/${slug}/` : `/genres/${slug}/page/${page}/`;
    const html = await fetchHtml(path);
    return parseGenreDetail(html);
  });
}
