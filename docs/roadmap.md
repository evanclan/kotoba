# Roadmap

This is a living document. Dates are intentions, not promises. Scope is more important than dates.

## Principles

- **Ship the smallest useful thing first.** A working `kotoba lookup` beats a beautiful unreleased plan.
- **Stable formats > stable APIs > stable internals.** We commit hardest at the file-format layer.
- **No feature without docs.** No docs without examples.
- **Local-first, then optional sync, then optional AI.** In that order. Always.

## v0.1 — Core CLI (target: ~1 month after kickoff)

**Goal:** A single contributor can install Kotoba, run their first review, and tell a friend.

- [ ] `kotoba init` — creates `~/.kotoba/` with a starter deck and config
- [ ] `kotoba lookup <word>` — JMdict-backed dictionary lookup
- [ ] `kotoba add <word>` — append to default deck (and link the JMdict entry)
- [ ] `kotoba review` — basic Ratatui TUI, FSRS-scheduled reviews
- [ ] `kotoba today` — word of the day, due count
- [ ] `kotoba stats` — minimal retention curve in the terminal
- [ ] Deck markdown format frozen at v1.0
- [ ] FSRS reference scheduler implemented and tested
- [ ] Cross-platform release pipeline (macOS, Linux, Windows)
- [ ] Homebrew tap (`brew install kotoba`)
- [ ] `cargo install kotoba`

## v0.2 — Make it daily

**Goal:** Users come back the next day without being told.

- [ ] Shell prompt integrations: zsh, fish, bash, nushell
- [ ] Starship module
- [ ] tmux status-bar segment
- [ ] Furigana rendering and kanji breakdown in `lookup` and `review`
- [ ] JLPT level metadata from KANJIDIC2 + frequency lists
- [ ] `kotoba scheduler tune` — local FSRS weight optimization
- [ ] `kotoba why <card>` — algorithm transparency
- [ ] `kotoba deck lint` and CI integration
- [ ] Two more language pairs scaffolded (community vote)

## v0.3 — Make it everywhere

**Goal:** Kotoba lives in the tools you already open.

- [ ] `kotoba-daemon` shipped, JSON-RPC + REST API at `/v1`
- [ ] VS Code extension (lookup, add, review)
- [ ] Neovim plugin
- [ ] Browser extension (Chromium + Firefox) with native messaging to the daemon
- [ ] Capture-from-clipboard hotkey
- [ ] Yomitan-compatible export

## v0.4 — Make it smart (without making it dependent)

**Goal:** AI integrations exist, but Kotoba still works fully without them.

- [ ] `kotoba-mcp` MCP server
- [ ] `--enrich` example sentence generation (Ollama / OpenAI / Anthropic / OpenRouter)
- [ ] `kotoba reading --topic` comprehensible-input generator
- [ ] `kotoba conversation` (alpha): chat practice with weak-grammar targeting
- [ ] Logging and audit for AI prompts
- [ ] BYO embedding model for similarity / dedupe (alpha)

## v0.5 — Make it shareable

**Goal:** Two people can practice from the same deck without sharing a database.

- [ ] Sync backends: git, S3, WebDAV, iCloud Drive, Dropbox
- [ ] Deck registry: a curated list of community decks
- [ ] `kotoba install <deck>` — fetch a community deck
- [ ] Web dashboard (read-only stats, self-hosted)
- [ ] Audio crowdsourcing pipeline (record → review → publish)

## v0.6 — A second flagship language pair

**Goal:** Prove the architecture is truly polyglot.

- [ ] Community vote on the next pair (likely Korean ↔ Japanese, Spanish ↔ English, or Vietnamese ↔ English)
- [ ] Tokenizer + dictionary adapter for the chosen pair
- [ ] Deck migration guide for cross-language learners

## v1.0 — Stable

**Goal:** A version we promise compatibility for.

- [ ] Plugin API frozen (semantic versioning kicks in)
- [ ] Mobile companion (iOS/Android, MVP — review-only)
- [ ] Curriculum templates for educators
- [ ] First school / language-program partnership pilot
- [ ] Funded maintenance plan

## Beyond v1.0 (roughly, in priority order)

- Native mobile apps with full feature parity
- AR/glasses companion (capture words from the environment)
- Federated FSRS weight tuning
- Verifiable language portfolio (signed receipts)
- Classroom mode (teacher dashboard, anonymized aggregates)
- Cross-language transfer learning
- Embedded learning agents (smart speakers, watches)
- A small commercial offering for schools/enterprises (cloud sync hosting, premium support) — proceeds fund the open core

## What we will *not* do

- Build a paid SaaS that holds users' data hostage
- Add telemetry without explicit, granular consent
- Lock features behind a subscription
- Adopt a CLA — we use the [Developer Certificate of Origin](https://developercertificate.org/) instead

## How priorities are decided

1. Maintainer team picks the milestone-spanning themes.
2. Within a milestone, issues are prioritized by:
   - Number of users blocked
   - Strategic alignment with the [architecture goals](architecture.md#goals)
   - Available contributor capacity
3. RFC discussions in [`docs/rfcs/`](rfcs/) (once that folder exists) gate any breaking change.
4. The roadmap is reviewed quarterly; this file is updated transparently.

If you want to influence the roadmap: open a Discussion and back it with a use case. We weigh use cases more than upvotes.
