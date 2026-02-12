# feed-aggregator-worker

Cloudflare Worker that aggregates multiple RSS/Atom feeds (from a public Gist TOML) and emits a single Atom feed with CORS enabled.

## Config

- `GIST_URL`: Public Gist raw URL that contains `[feeds]` config.
- `FEED_TITLE`: Atom feed title (default: `Umbreon Aggregated Feed`).
- `FEED_SUBTITLE`: Atom feed subtitle.
- `FEED_LINK`: Feed link (used for `<id>` if set).
- `MAX_ITEMS`: Maximum entries in output (default: 200).
- `DAYS`: Only include entries within the last N days (default: 60).

## Request overrides

You can override via query params:

```
GET /?gist=<url>&title=...&subtitle=...&link=...&limit=200&days=60
```

## Example gist format

```toml
[feeds]
[feeds.sspai]
name = "少数派"
url = "https://rss.datuan.dev/sspai/index"

tags = ["StackLang:rust", "#rss"]
```

## Development

```
cd workers/feed-aggregator-worker
npm install
npm run dev
```

## Deploy

```
npm run deploy
```
