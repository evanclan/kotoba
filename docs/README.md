# Kotoba documentation

Welcome to the Kotoba design and contributor docs.

## For users

- [Quick start](../README.md#-quick-start) — get running in five minutes
- [FAQ](faq.md) — short answers to common questions
- [Use cases](use-cases.md) — who Kotoba is for, today and in an AI-native future
- [Data format](data-format.md) — how decks and cards are stored

## For contributors

- [Architecture](architecture.md) — how the crates fit together
- [SRS algorithm](srs-algorithm.md) — what FSRS is and how we use it
- [Plugin API](plugin-api.md) — how external tools talk to the daemon
- [AI integration](ai-integration.md) — MCP, BYO LLM, agent-friendly design
- [Roadmap](roadmap.md) — milestones from v0.1 to v1.0
- [Governance](governance.md) — how decisions are made
- [i18n guide](i18n.md) — adding a new UI language
- [Licenses](licenses.md) — third-party data and code

## Conventions

- Docs are markdown, wrapped at ~100 columns.
- Code samples are tested where possible (run `cargo test --doc`).
- Japanese examples include reading (kana) and a literal English gloss.
- We prefer plain English over jargon. When jargon is unavoidable, define it on first use.
