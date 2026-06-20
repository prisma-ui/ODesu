import { fetchHtml } from "@/lib/http";
import { parseAnimeDetail } from "@/lib/scrapers/animeDetail";
import { withUpstreamErrorHandling } from "@/lib/apiResponse";

export const dynamic = "force-dynamic";

export async function GET(_req: Request, { params }: { params: { slug: string } }) {
  return withUpstreamErrorHandling(async () => {
    const html = await fetchHtml(`/anime/${params.slug}/`);
    return parseAnimeDetail(html);
  });
}
