import { fetchHtml } from "@/lib/http";
import { parseGenreList } from "@/lib/scrapers/genres";
import { withUpstreamErrorHandling } from "@/lib/apiResponse";

export const dynamic = "force-dynamic";

export async function GET() {
  return withUpstreamErrorHandling(async () => {
    const html = await fetchHtml("/genre-list/");
    return parseGenreList(html);
  });
}
