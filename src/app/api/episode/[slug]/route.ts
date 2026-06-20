import { fetchHtml } from "@/lib/http";
import { parseEpisode } from "@/lib/scrapers/episode";
import { withUpstreamErrorHandling } from "@/lib/apiResponse";

export const dynamic = "force-dynamic";

export async function GET(_req: Request, { params }: { params: { slug: string } }) {
  return withUpstreamErrorHandling(async () => {
    const html = await fetchHtml(`/episode/${params.slug}/`);
    return parseEpisode(html);
  });
}
