export const metadata = {
  title: "Otakudesu Scraper API",
  description: "Next.js/TypeScript scraper API for otakudesu.blog",
};

export default function RootLayout({ children }: { children: React.ReactNode }) {
  return (
    <html lang="en">
      <body style={{ margin: 0, fontFamily: "system-ui, sans-serif" }}>{children}</body>
    </html>
  );
}
