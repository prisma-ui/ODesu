import { fetchHtml } from "@/lib/http";
import { parseHome } from "@/lib/scrapers/home";
import { withUpstreamErrorHandling } from "@/lib/apiResponse";

export const dynamic = "force-dynamic";

export async function GET() {
  return withUpstreamErrorHandling(async () => {
    const html = await fetchHtml("/");
    return parseHome(html);
  });
}
