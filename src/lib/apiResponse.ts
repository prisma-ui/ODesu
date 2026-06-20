import { NextResponse } from "next/server";
import { UpstreamFetchError } from "./http";

export async function withUpstreamErrorHandling<T>(fn: () => Promise<T>): Promise<NextResponse> {
  try {
    const data = await fn();
    return NextResponse.json(data);
  } catch (e) {
    if (e instanceof UpstreamFetchError) {
      return NextResponse.json({ detail: `Upstream fetch failed: ${e.message}` }, { status: 502 });
    }
    return NextResponse.json({ detail: `Internal error: ${(e as Error).message}` }, { status: 500 });
  }
}
