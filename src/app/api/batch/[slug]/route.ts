import { fetchHtml } from "@/lib/http";
import { parseBatch } from "@/lib/scrapers/batch";
import { withUpstreamErrorHandling } from "@/lib/apiResponse";

export const dynamic = "force-dynamic";

export async function GET(_req: Request, { params }: { params: { slug: string } }) {
  return withUpstreamErrorHandling(async () => {
    const html = await fetchHtml(`/batch/${params.slug}/`);
    return parseBatch(html);
  });
}
