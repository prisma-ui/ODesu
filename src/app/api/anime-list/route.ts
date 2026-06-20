import { fetchHtml } from "@/lib/http";
import { parseAnimeList } from "@/lib/scrapers/animeList";
import { withUpstreamErrorHandling } from "@/lib/apiResponse";

export const dynamic = "force-dynamic";

export async function GET() {
  return withUpstreamErrorHandling(async () => {
    const html = await fetchHtml("/anime-list/");
    return parseAnimeList(html);
  });
}
