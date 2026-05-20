# Architecture

This document describes how Kotoba's pieces fit together. It's the first thing a new contributor should read.

## Goals

1. **Local-first.** A user with no network can do everything important.
2. **Plain-text data.** No proprietary formats. Markdown for content, TOML/YAML for config.
3. **Composable.** Every layer has a clean public API; surfaces and integrations build on top of those APIs, not on internals.
4. **Substrate, not silo.** Kotoba is a layer that AI agents and other tools build *on*, never a closed app trying to absorb the world.
5. **Polyglot from day one.** Japanese ↔ English is the flagship pair, but no architectural decision should hardcode it.

## High-level diagram

```
┌──────────────────── User-facing surfaces ────────────────────┐
│  CLI    TUI    Editor plugins    Browser ext    Mobile       │
│   │      │            │              │             │         │
└───┼──────┼────────────┼──────────────┼─────────────┼─────────┘
    │      │            │              │             │
    │      │     ┌──────┴──────────────┴───┐    ┌────┴────┐
    │      │     │     kotoba-daemon       │    │ Native  │
    │      │     │  (HTTP/JSON-RPC, local) │    │  embed  │
    │      │     └──────────┬──────────────┘    └────┬────┘
    │      │                │                        │
    │      │           ┌────┴────┐                   │
    │      │           │  MCP    │  ← AI agents      │
    │      │           │ server  │                   │
    │      │           └────┬────┘                   │
    │      │                │                        │
    └──────┴────────────────┴────────────────────────┘
                            │
                  ┌─────────┴─────────┐
                  │   kotoba-core     │
                  │   (Rust library)  │
                  └─────────┬─────────┘
                            │
          ┌─────────────────┼─────────────────┐
          │                 │                 │
   Deck store        Dictionary store     Scheduler
   ~/.kotoba/decks   JMdict + KANJIDIC2   FSRS
   (markdown)        (open data)
```

## Crates

### `kotoba-core`

The engine. **Has no I/O of its own** beyond a pluggable `Store` trait. This makes it embeddable and testable. Public surface:

- `Card`, `Deck`, `Note`, `Review` — domain types
- `Store` trait — read/write decks
- `Dictionary` trait — lookup adapters (JMdict by default)
- `Scheduler` trait — SRS algorithms (FSRS by default)
- `Engine` — orchestrator that ties everything together

`kotoba-core` is the only crate that other crates depend on directly. If it changes, everything moves.

### `kotoba-cli`

The `kotoba` binary. Owns:

- Argument parsing (`clap`)
- Subcommands (`init`, `lookup`, `add`, `review`, `today`, `stats`, `shell`, `daemon`, `deck`)
- TUI for review (`ratatui`)
- Concrete `Store` implementations (filesystem-backed)
- Configuration loading (`~/.config/kotoba/config.toml`)

The CLI is the canonical reference UX. Anything we build in another surface should be expressible as a sequence of CLI commands first.

### `kotoba-daemon`

An optional, locally-bound HTTP / JSON-RPC server.

- Default bind: `127.0.0.1:6060`
- Authentication: per-install token in `~/.config/kotoba/daemon.token`
- Endpoints: `/cards/due`, `/cards`, `/cards/:id/review`, `/decks`, `/lookup`, `/stats`, `/today`
- Speaks JSON-RPC 2.0 *or* a small REST surface — both routed to the same handlers

This is the integration backbone. Editor plugins, browser extensions, and the future web dashboard talk to the daemon.

### `kotoba-mcp`

An [MCP](https://modelcontextprotocol.io) server that exposes Kotoba's capabilities to AI agents (Claude Desktop, Cursor, etc.).

- Resources: `learner-state`, `due-cards`, `recent-mistakes`, `decks/*`
- Tools: `lookup`, `add_card`, `record_review`, `generate_example_sentence` (delegates to a configured LLM, BYO model)
- Authentication & consent: handled by the user's MCP client

The MCP server is how Kotoba stays useful as AI agents become the default interface.

## Data flow: a typical review

```
1. User runs `kotoba review`
2. kotoba-cli loads config → opens FilesystemStore
3. Engine asks Scheduler for due cards
4. CLI launches TUI; for each card:
   a. Display front
   b. Wait for grade input (1–4)
   c. Engine.record_review(card_id, grade, now)
   d. Scheduler updates next_due, interval, stability
   e. Store.write_card(updated_card)
5. CLI prints session summary; exits cleanly
```

The same flow happens via the daemon for editor plugins — the only change is HTTP at step (a)/(b).

## Storage layout

The default user data directory is `~/.kotoba/` (overridable). Layout:

```
~/.kotoba/
├── config.toml             # user-level config (overrides ~/.config/kotoba/config.toml)
├── decks/
│   ├── personal.md         # the default "things I added" deck
│   ├── jlpt-n5.md          # imported community decks
│   └── work-tech-en.md
├── reviews/
│   └── 2026/
│       └── 05.jsonl        # append-only review log, one JSON object per review
├── audio/                  # cached audio (not source of truth)
└── cache/                  # parsed dictionary indices, never source of truth
```

**Plain text. Diffable. Mergeable. Forever.**

The `decks/` folder *is* the source of truth. Reviews append to a JSONL log so progress is reconstructable from history alone — useful for sync, undo, and verifiable language portfolios (see [use-cases.md](use-cases.md)).

## Cross-cutting decisions

### Concurrency

`kotoba-core` is single-threaded by design. The CLI and daemon may use Tokio for I/O concurrency, but never call into core from multiple threads simultaneously. Locks live at the `Store` level via cooperative file locking (`flock`/Windows equivalent).

### Errors

- Library code: `thiserror` enums. Every public function's error type is enumerable.
- Binary code: `anyhow` at the boundary, with rich context.
- TUI: panics are converted to user-readable messages; `panic = "abort"` in release builds.

### Logging & telemetry

- We use `tracing` for structured logs.
- Default log level: `warn`.
- **No telemetry of any kind ships by default.** Any future opt-in metrics will be documented, source-visible, and toggled off in the default config.

### Configuration

- System-wide: `/etc/kotoba/config.toml` (rare, mostly for kiosks/classrooms)
- User: `~/.config/kotoba/config.toml`
- Workspace: `~/.kotoba/config.toml` (overrides the above)
- Environment: `KOTOBA_*` env vars override individual keys
- Flags: CLI flags override env vars

This precedence chain is implemented once in `kotoba-core::config` and reused everywhere.

### I18n

- UI strings live in fluent `.ftl` files at `crates/kotoba-cli/locales/<lang>/`.
- The CLI selects locale from `LC_ALL` / `LANG`, with `KOTOBA_LANG` overriding.
- Adding a new locale is a docs-only change followed by a translation PR.

## Forward-compatible decisions we're making now

These choices are deliberate so that future capabilities (AI, MCP, sync, mobile) don't require rewriting the core:

1. **Reviews are append-only.** Enables conflict-free sync, time-travel debugging, and verifiable portfolios.
2. **Cards have a `context` field** capturing where the user encountered the word (URL, file path, sentence). Useful for AI tutoring later.
3. **Card front/back are markdown**, not strings. Enables furigana, images, audio refs, and inline kanji breakdowns without schema migration.
4. **Dictionary is a trait**, not a hardcoded JMdict. We can plug in domain dictionaries (medical, legal, technical) and per-language adapters.
5. **Scheduler is a trait.** FSRS today; SuperMemo, Half-Life Regression, custom research algorithms tomorrow — all behind feature flags.
6. **The daemon's API is versioned** (`/v1/...`). Plugins target a major version and we promise compatibility within it.

## What this architecture is *not*

- **Not a microservice mesh.** Everything ships as a single binary by default; the daemon is opt-in.
- **Not a database.** Plain text is the database. We index in memory and cache aggressively, but the source of truth is files you can read with `cat`.
- **Not "just a wrapper around an LLM."** AI is one optional surface. The product works fully offline with no model.
- **Not a curriculum.** We don't ship a syllabus. Decks are community-driven.

## Reading order for a new contributor

1. This file.
2. [`docs/data-format.md`](data-format.md) — the deck format, with examples.
3. [`docs/srs-algorithm.md`](srs-algorithm.md) — what FSRS is and why we chose it.
4. [`docs/plugin-api.md`](plugin-api.md) — the daemon's public surface.
5. [`crates/kotoba-core/src/lib.rs`](../crates/kotoba-core/src/lib.rs) — the actual code.

If you can read those and the public API still feels confusing, **that's a docs bug** — please file it.
