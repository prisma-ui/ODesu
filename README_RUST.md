# Otakudesu API (Rust / axum)

Rust port of the [Next.js/TypeScript Otakudesu scraper](https://github.com/your-repo/otakudesu-api-nextjs) using **axum**, **reqwest**, **scraper**, and **utoipa**.

## Features

- 🚀 **Fast & Lightweight**: Native Rust binary, no Node.js runtime
- 📚 **Full API Parity**: All 15+ endpoints from the original
- 🔐 **Mirror Resolution**: Replicates WordPress admin-ajax two-step flow with cookie jar
- 📖 **OpenAPI/Swagger**: Auto-generated docs at `/docs`
- 🐳 **Docker Ready**: Multi-stage build, ~20MB final image
- ☁️ **Deploy Anywhere**: Fly.io, Railway, Render, Hugging Face Spaces, any VPS

## Endpoints

| Method | Path | Description |
|--------|------|-------------|
| `GET` | `/api/ping` | Health check |
| `GET` | `/api/home` | Ongoing + Complete preview |
| `GET` | `/api/ongoing?page=1` | Ongoing anime list |
| `GET` | `/api/complete?page=1` | Complete anime list |
| `GET` | `/api/anime-list` | Alphabetical index |
| `GET` | `/api/jadwal-rilis` | Release schedule |
| `GET` | `/api/genres` | Genre list |
| `GET` | `/api/genres/{slug}?page=1` | Genre detail |
| `GET` | `/api/search?q=` | Search anime |
| `GET` | `/api/anime/{slug}` | Anime detail + episodes + batch |
| `GET` | `/api/episode/{slug}` | Episode streams, mirrors, downloads |
| `GET` | `/api/batch/{slug}` | Batch download links |
| `POST` | `/api/mirror` | Resolve mirror `data_content` to embed HTML |
| `GET` | `/docs` | Swagger UI |
| `GET` | `/api/openapi.json` | Raw OpenAPI 3.0 spec |

## Quick Start

### Prerequisites

- Rust 1.75+ (via [rustup](https://rustup.rs/))
- Docker (for containerized deploy)

### Local Development

```bash
# Clone and enter
cd otakudesu-api

# Copy config template
cp Config.toml.example Config.toml

# Edit config if needed (optional - env vars take precedence)
# nano Config.toml

# Run
cargo run --release

# Server starts at http://localhost:7860
# Swagger UI at http://localhost:7860/docs
```

### Environment Variables

| Variable | Default | Description |
|----------|---------|-------------|
| `OTAKUDESU_TARGET_BASE_URL` | `https://otakudesu.blog` | Target site base URL |
| `OTAKUDESU_CF_PROXY_URL` | `` | Cloudflare Worker proxy prefix |
| `OTAKUDESU_REQUEST_TIMEOUT_MS` | `20000` | Request timeout (ms) |
| `OTAKUDESU_BIND_ADDR` | `0.0.0.0` | Server bind address |
| `OTAKUDESU_PORT` | `7860` | Server port |
| `OTAKUDESU_LOG_LEVEL` | `info` | Log level (trace/debug/info/warn/error) |
| `RUST_LOG` | - | Override log level (tracing format) |

### Using Cloudflare Proxy (Recommended)

The target site may serve bot-detected pages. Route requests through a Cloudflare Worker:

```bash
# 1. Create a Worker at dash.cloudflare.com
# 2. Add this code:
export default {
  async fetch(request, env, ctx) {
    const url = new URL(request.url);
    const target = url.searchParams.get('url');
    if (!target) return new Response('Missing url param', { status: 400 });

    const response = await fetch(target, {
      headers: {
        'User-Agent': 'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36',
        'Accept': 'text/html,application/xhtml+xml,application/xml;q=0.9,*/*;q=0.8',
        'Accept-Language': 'id-ID,id;q=0.9,en-US;q=0.8,en;q=0.7',
      },
    });
    return new Response(response.body, response);
  }
};

# 3. Set env var:
OTAKUDESU_CF_PROXY_URL=https://your-worker.workers.dev/?url=
```

## Deployment

### Fly.io (Recommended)

```bash
# Install flyctl
curl -L https://fly.io/install.sh | sh

# Launch (creates fly.toml, picks region)
fly launch --copy-config --no-deploy

# Set secrets
fly secrets set OTAKUDESU_TARGET_BASE_URL=https://otakudesu.blog
fly secrets set OTAKUDESU_CF_PROXY_URL=https://your-worker.workers.dev/?url=

# Deploy
fly deploy
```

**Free tier**: 3 shared-cpu VMs, 160GB/mo outbound, static IPv4/v6, no sleep.

### Railway

```bash
# Install railway CLI
npm i -g @railway/cli

# Login & init
railway login
railway init

# Set variables in dashboard or CLI
railway variables set OTAKUDESU_TARGET_BASE_URL=https://otakudesu.blog
railway variables set OTAKUDESU_CF_PROXY_URL=https://your-worker.workers.dev/?url=

# Deploy
railway up
```

### Render

1. Connect GitHub repo
2. Create **Web Service**
3. Build: `docker build -t otakudesu-api .`
4. Start: `otakudesu-api`
5. Add environment variables

### Hugging Face Spaces

```bash
# Create Space (select Docker SDK)
hf space create otakudesu-api --sdk docker

# Push
git remote add space https://huggingface.co/spaces/your-username/otakudesu-api
git push space main
```

⚠️ **Note**: HF Spaces sleeps after 48h inactivity (cold start ~30-60s). For production, use Fly.io/Railway.

### Docker (Any VPS)

```bash
# Build
docker build -t otakudesu-api .

# Run
docker run -d \
  -p 7860:7860 \
  -e OTAKUDESU_TARGET_BASE_URL=https://otakudesu.blog \
  -e OTAKUDESU_CF_PROXY_URL=https://your-worker.workers.dev/?url= \
  --name otakudesu-api \
  otakudesu-api
```

## Project Structure

```
src/
├── main.rs              # Entry point, router setup
├── lib.rs               # Module exports
├── config.rs            # Configuration (figment + env)
├── error.rs             # Error types & handling
├── http_client.rs       # reqwest client + cookie jar + mirror resolution
├── parsing.rs           # Shared parsing utilities
├── types.rs             # Serde models for all responses
├── handlers.rs          # All API handlers + OpenAPI doc
└── scrapers/
    ├── mod.rs           # Re-exports
    ├── anime_grid.rs    # Ongoing/Complete grid parsing
    ├── home.rs          # Home page parsing
    ├── anime_detail.rs  # Anime detail page
    ├── episode.rs       # Episode page (streams, mirrors, downloads)
    ├── batch.rs         # Batch download page
    ├── search.rs        # Search results
    ├── genres.rs        # Genre list & detail
    ├── jadwal_rilis.rs  # Release schedule
    └── anime_list.rs    # Alphabetical index
```

## Testing

```bash
# Run all tests
cargo test

# Run with output
cargo test -- --nocapture

# Update snapshots (insta)
cargo install cargo-insta
cargo insta test --accept
```

## Differences from TypeScript Version

| Aspect | TypeScript (Next.js) | Rust (axum) |
|--------|---------------------|-------------|
| Runtime | Node.js (Vercel) | Native binary |
| HTML Parser | cheerio | scraper (CSS selectors) |
| HTTP Client | fetch + tough-cookie | reqwest + cookie-store |
| OpenAPI | Manual (swagger-ui-react) | Auto (utoipa) |
| Deploy Target | Vercel | Fly.io, Railway, any Docker host |
| Cold Start | ~100ms (edge) | ~50ms (native) |
| Concurrency | Limited by Vercel | Unlimited (tokio) |

## License

MIT