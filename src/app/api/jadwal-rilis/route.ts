import { fetchHtml } from "@/lib/http";
import { parseJadwalRilis } from "@/lib/scrapers/jadwalRilis";
import { withUpstreamErrorHandling } from "@/lib/apiResponse";

export const dynamic = "force-dynamic";

export async function GET() {
  return withUpstreamErrorHandling(async () => {
    const html = await fetchHtml("/jadwal-rilis/");
    return parseJadwalRilis(html);
  });
}
