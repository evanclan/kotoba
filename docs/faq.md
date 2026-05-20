# FAQ

Short answers. Where you want depth, follow the link.

### Is Kotoba an Anki replacement?

For some people, yes. We don't import the kitchen sink, but for terminal-friendly users with simple decks, you may not miss Anki. See [the comparison table](../README.md#what-makes-kotoba-different).

### Can I import my Anki decks?

Planned for v0.3. Output will be plain markdown — never a binary. See [roadmap](roadmap.md).

### Why not build it in TypeScript / Python?

We chose Rust for three reasons: (1) one binary, no runtime, runs anywhere; (2) the TUI ecosystem (Ratatui) is excellent; (3) Rust attracts engineers who care about correctness. We'll likely ship a TypeScript SDK for plugin authors.

### Why JMdict and KANJIDIC2?

They're the gold standard open Japanese dictionaries. Curated by the EDRDG community for decades. License obligations are explicit and we comply ([licenses.md](licenses.md)).

### Will Kotoba ever require a cloud account?

No. Sync will be opt-in and bring-your-own-backend (git, S3, WebDAV, iCloud). See [governance.md](governance.md).

### Will it always be free?

Yes. The open core will always be free under MIT. We may eventually offer paid hosted sync for schools/enterprises to fund maintenance, but that will never disable any feature for individual users.

### Can I use it for languages other than Japanese?

Yes, by design. Japanese ↔ English is the flagship pair because that's the founding context, but the architecture is language-agnostic. See [architecture.md](architecture.md).

### Does Kotoba work offline?

Yes, fully. AI features fail closed when no model is configured.

### What about telemetry?

There is none, by default. Any future opt-in metrics will be granular, source-visible, and toggled off in the default config.

### How do I help if I can't code?

Easiest contributions: curate a deck, translate the UI, record audio, fix a typo. See [CONTRIBUTING.md](../CONTRIBUTING.md).

### Will there be a mobile app?

Yes, planned for v1.0. Initially review-only, syncing from the local data file.

### Why is the project called "Kotoba"?

言葉 (kotoba) means "word" or "language" in Japanese. It's the smallest unit of what we help you learn — and a nice short name that's easy to type.
