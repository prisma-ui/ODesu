import { NextResponse } from "next/server";

export const dynamic = "force-dynamic";

import { spec } from "@/lib/openapi";

export async function GET() {
  return NextResponse.json(spec);
}
