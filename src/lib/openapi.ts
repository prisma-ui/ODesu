export const spec = {
  openapi: "3.0.3",
  info: {
    title: "Otakudesu Scraper API",
    version: "1.0.0",
    description: "Next.js/TypeScript scraper API for otakudesu.blog.",
  },
  servers: [{ url: "/" }],
  paths: {
    "/api/ping": {
      get: {
        summary: "Health check",
        tags: ["Health"],
        responses: { "200": { description: "OK" } },
      },
    },
    "/api/home": {
      get: {
        summary: "Homepage ongoing + complete preview",
        tags: ["Listing"],
        responses: { "200": { description: "OK" } },
      },
    },
    "/api/ongoing": {
      get: {
        summary: "Ongoing anime list",
        tags: ["Listing"],
        parameters: [
          { name: "page", in: "query", schema: { type: "integer", default: 1 } },
        ],
        responses: { "200": { description: "OK" } },
      },
    },
    "/api/complete": {
      get: {
        summary: "Completed anime list",
        tags: ["Listing"],
        parameters: [
          { name: "page", in: "query", schema: { type: "integer", default: 1 } },
        ],
        responses: { "200": { description: "OK" } },
      },
    },
    "/api/anime-list": {
      get: {
        summary: "Alphabetical anime index",
        tags: ["Listing"],
        responses: { "200": { description: "OK" } },
      },
    },
    "/api/jadwal-rilis": {
      get: {
        summary: "Weekly release schedule",
        tags: ["Listing"],
        responses: { "200": { description: "OK" } },
      },
    },
    "/api/genres": {
      get: {
        summary: "List of all genres",
        tags: ["Genres"],
        responses: { "200": { description: "OK" } },
      },
    },
    "/api/genres/{slug}": {
      get: {
        summary: "Anime list filtered by genre",
        tags: ["Genres"],
        parameters: [
          { name: "slug", in: "path", required: true, schema: { type: "string" } },
          { name: "page", in: "query", schema: { type: "integer", default: 1 } },
        ],
        responses: { "200": { description: "OK" } },
      },
    },
    "/api/search": {
      get: {
        summary: "Search anime by title",
        tags: ["Search"],
        parameters: [
          { name: "q", in: "query", required: true, schema: { type: "string" } },
        ],
        responses: { "200": { description: "OK" } },
      },
    },
    "/api/anime/{slug}": {
      get: {
        summary: "Anime detail (info, episode list, batch link, recommendations)",
        tags: ["Detail"],
        parameters: [
          { name: "slug", in: "path", required: true, schema: { type: "string" } },
        ],
        responses: { "200": { description: "OK" } },
      },
    },
    "/api/episode/{slug}": {
      get: {
        summary: "Episode detail (player iframe, mirrors, download links)",
        tags: ["Detail"],
        parameters: [
          { name: "slug", in: "path", required: true, schema: { type: "string" } },
        ],
        responses: { "200": { description: "OK" } },
      },
    },
    "/api/batch/{slug}": {
      get: {
        summary: "Batch download detail",
        tags: ["Detail"],
        parameters: [
          { name: "slug", in: "path", required: true, schema: { type: "string" } },
        ],
        responses: { "200": { description: "OK" } },
      },
    },
    "/api/mirror": {
      post: {
        summary: "Resolve a mirror's data-content into an embeddable iframe HTML",
        tags: ["Detail"],
        parameters: [
          { name: "data_content", in: "query", required: false, schema: { type: "string" } },
        ],
        requestBody: {
          required: false,
          content: {
            "application/json": {
              schema: {
                type: "object",
                properties: { data_content: { type: "string" } },
              },
            },
          },
        },
        responses: { "200": { description: "OK" } },
      },
    },
  },
};
