const endpoints = [
  ["GET", "/api/home", "ongoing + complete preview"],
  ["GET", "/api/ongoing?page=1", ""],
  ["GET", "/api/complete?page=1", ""],
  ["GET", "/api/anime-list", "alphabetical index"],
  ["GET", "/api/jadwal-rilis", ""],
  ["GET", "/api/genres", ""],
  ["GET", "/api/genres/{slug}?page=1", ""],
  ["GET", "/api/search?q=", ""],
  ["GET", "/api/anime/{slug}", "detail + episode list + batch link"],
  ["GET", "/api/episode/{slug}", "player iframe, mirrors, download links"],
  ["GET", "/api/batch/{slug}", "batch download links"],
  ["POST", "/api/mirror?data_content={base64}", "resolves a mirror's data-content into an embeddable iframe HTML"],
  ["GET", "/api/ping", "health check"],
];

export default function Home() {
  return (
    <main style={{ maxWidth: 760, margin: "0 auto", padding: "48px 24px", lineHeight: 1.6 }}>
      <h1 style={{ marginBottom: 4 }}>🎬 Otakudesu Scraper API</h1>
      <p style={{ color: "#555" }}>
        Next.js/TypeScript scraper for otakudesu.blog. See{" "}
        <a href="/docs">/docs</a> for interactive Swagger UI.
      </p>
      <table style={{ width: "100%", borderCollapse: "collapse", marginTop: 24 }}>
        <tbody>
          {endpoints.map(([method, path, note]) => (
            <tr key={path} style={{ borderBottom: "1px solid #eee" }}>
              <td style={{ padding: "8px 12px 8px 0", fontWeight: 600, color: method === "POST" ? "#b8860b" : "#0066cc" }}>
                {method}
              </td>
              <td style={{ padding: "8px 12px", fontFamily: "monospace" }}>{path}</td>
              <td style={{ padding: "8px 0", color: "#777" }}>{note}</td>
            </tr>
          ))}
        </tbody>
      </table>
    </main>
  );
}
