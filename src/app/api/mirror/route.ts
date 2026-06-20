import { NextRequest, NextResponse } from "next/server";
import { resolveMirrorEmbed, UpstreamFetchError } from "@/lib/http";

export const dynamic = "force-dynamic";

/**
 * Resolves an episode mirror's base64 data-content (obtained from /api/episode)
 * into an embeddable iframe HTML, replicating the site's own two-step AJAX flow:
 *   1. POST action=aa1208... (no params) -> returns a nonce
 *   2. POST decoded {id,i,q} + nonce + action=2a3505... -> returns base64 HTML (iframe)
 *
 * Accepts data_content either as a query param (?data_content=) or in a JSON body.
 */
export async function POST(req: NextRequest) {
  const fromQuery = req.nextUrl.searchParams.get("data_content");
  let dataContent = fromQuery;

  if (!dataContent) {
    try {
      const body = await req.json();
      dataContent = body?.data_content ?? null;
    } catch {
      // no JSON body provided
    }
  }

  if (!dataContent) {
    return NextResponse.json({ detail: "Missing 'data_content'" }, { status: 422 });
  }

  try {
    const decodedJson = Buffer.from(dataContent, "base64").toString("utf-8");
    const decoded = JSON.parse(decodedJson) as Record<string, string>;
    const embedHtml = await resolveMirrorEmbed(decoded);
    return NextResponse.json({ embed_html: embedHtml });
  } catch (e) {
    if (e instanceof UpstreamFetchError) {
      return NextResponse.json({ detail: `Upstream fetch failed: ${e.message}` }, { status: 502 });
    }
    return NextResponse.json({ detail: `Failed to resolve mirror: ${(e as Error).message}` }, { status: 502 });
  }
}
