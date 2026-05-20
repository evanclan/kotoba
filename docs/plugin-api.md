# Plugin API

The Kotoba daemon exposes a local HTTP / JSON-RPC API that editor plugins, browser extensions, the web dashboard, and the MCP server all use. This document is the contract.

## Status

- Status: **draft, planning for v0.3**
- Stability: subject to change until v1.0; major versions are pinned in the URL
- Default base URL: `http://127.0.0.1:6060/v1`
- Auth: `Authorization: Bearer <token>` where the token is in `~/.config/kotoba/daemon.token`

## Conventions

- All times are RFC 3339 UTC.
- All IDs are stable strings (see [data-format.md](data-format.md)).
- Errors are JSON: `{"error": {"code": "...", "message": "...", "details": {...}}}`.
- Requests and responses are JSON; `Content-Type: application/json`.

## Endpoints (REST surface)

### Lookups

```
GET  /v1/lookup?q=留学&lang=ja
→ {
    "term": "留学",
    "reading": "りゅうがく",
    "meanings": ["studying abroad"],
    "pos": ["noun", "suru-verb"],
    "jlpt": "N3",
    "frequency_rank": 4212,
    "examples": [
      {"native": "妹は来年留学します。", "gloss": "..."}
    ]
  }
```

### Decks

```
GET  /v1/decks
GET  /v1/decks/:slug
POST /v1/decks                  # create
GET  /v1/decks/:slug/cards
POST /v1/decks/:slug/cards      # add a card
```

### Cards

```
GET  /v1/cards/:id
GET  /v1/cards/due?limit=20
POST /v1/cards/:id/review       # body: {"grade": 1..4, "context": "vscode"}
```

### Today / stats

```
GET  /v1/today                  # word-of-the-day, due count, streak
GET  /v1/stats?since=2026-01-01
```

### Server info

```
GET  /v1/health                 # 200 OK + version
GET  /v1/version
```

## JSON-RPC surface

The same endpoints are also reachable via JSON-RPC 2.0 at `POST /v1/rpc`:

```json
{ "jsonrpc": "2.0", "id": 1, "method": "cards.due", "params": {"limit": 20} }
```

Method names:
- `lookup`
- `decks.list`, `decks.get`, `decks.create`
- `cards.list`, `cards.due`, `cards.get`, `cards.add`, `cards.review`
- `today`, `stats`, `health`, `version`

## Streaming

For long-running operations (review sessions, AI-generated content), the daemon supports **Server-Sent Events** at `/v1/stream/<endpoint>`. Example:

```
GET /v1/stream/review

event: card
data: {"id": "...", "front": "...", ...}

event: graded
data: {"id": "...", "next_due": "...", "interval_days": ...}
```

## Authentication

- The daemon writes a 256-bit token to `~/.config/kotoba/daemon.token` on first start (mode `0600`).
- All requests must include `Authorization: Bearer <token>`.
- When bound to non-loopback (e.g. for a classroom server), the daemon refuses to start without TLS configured.
- Plugins should read the token from the well-known path; if not present, prompt the user to start the daemon.

## CORS

CORS is **disabled by default**. Browser extensions must use the file-system-served token and proxy through native messaging — never embed the token in client-side JavaScript on a remote page.

## Versioning

- The URL contains the major version (`/v1/...`).
- Breaking changes get a new major (`/v2/...`).
- Inside a major, additions are non-breaking. Removals are forbidden.
- Plugins **must** include `Accept: application/vnd.kotoba.v1+json` to lock to a major version even if the URL shape changes.

## Reference implementations

- `kotoba-mcp` — MCP server for AI agents
- `examples/plugin-template/` — minimal Node and Rust client examples (planned)
- VS Code extension (planned for v0.3)
