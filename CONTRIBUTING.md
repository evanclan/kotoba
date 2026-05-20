# Contributing to Kotoba

Welcome! Whether you're here to fix a typo, curate a deck, build a VS Code plugin, or rewrite the SRS scheduler — thank you. Kotoba is built by and for its community.

This guide explains **how** to contribute. For **what** to contribute, browse:

- [Issues labeled `good first issue`](https://github.com/evanclan/kotoba/labels/good%20first%20issue)
- [Issues labeled `help wanted`](https://github.com/evanclan/kotoba/labels/help%20wanted)
- [Discussions](https://github.com/evanclan/kotoba/discussions) for ideas and questions
- The [Roadmap](docs/roadmap.md) for the bigger picture

---

## Table of Contents

1. [The 30-second contribution path](#the-30-second-contribution-path)
2. [Ways to contribute](#ways-to-contribute)
3. [Development setup](#development-setup)
4. [Code style](#code-style)
5. [Commit messages](#commit-messages)
6. [Pull request checklist](#pull-request-checklist)
7. [How decisions get made](#how-decisions-get-made)
8. [Recognition](#recognition)

---

## The 30-second contribution path

The fastest, lowest-stakes way to contribute:

1. Find a typo, an unclear sentence, or a missing example anywhere in this repo.
2. Click "edit" on GitHub. Fix it.
3. Open a PR with the title `docs: <what you changed>`.
4. We merge.

You are now a Kotoba contributor. Welcome.

---

## Ways to contribute

### 1. Curate a deck (no code required)

Decks live in [`decks/`](decks/) as markdown files. Anyone can:

- **Add cards** to an existing deck (PR a new entry to a `.md` file)
- **Create a new themed deck** (e.g. "Yotsuba&! volume 3 vocabulary", "Words for visiting a Japanese onsen", "JLPT N4 transitive/intransitive pairs")
- **Add example sentences** to existing cards
- **Improve translations**

The deck format is documented in [`docs/data-format.md`](docs/data-format.md). It's plain markdown — no special tools needed.

**Rules of thumb:**
- One deck per PR keeps reviews fast.
- Cite your source in the deck's frontmatter. (Example sentences from JMdict are pre-licensed; cite a textbook or your own writing otherwise.)
- Use `LICENSE: CC-BY-SA-4.0` in deck frontmatter unless you have a reason to use something else.

### 2. Translate the UI or docs

Kotoba's UI and docs are designed to be translatable. Strings live in [`crates/kotoba-cli/locales/`](crates/kotoba-cli/locales) and docs are translated as `README.<lang>.md`, `CONTRIBUTING.<lang>.md`, etc.

If your language isn't represented, we want you. Open an issue with the title `i18n: <language>` and we'll help you bootstrap.

### 3. Record audio (no code required)

Native-speaker audio for cards is one of the highest-impact, lowest-friction contributions. We accept short `.ogg` files for individual words and example sentences. Process and metadata standards live in [`docs/audio-contributions.md`](docs/audio-contributions.md) (TODO).

### 4. Build a shell or editor integration

[`shells/`](shells/) contains examples for zsh, fish, bash, nushell. We welcome:

- Starship modules
- Powerlevel10k segments
- tmux status-bar plugins
- VS Code, Neovim, Zed, Helix, Sublime, Emacs extensions
- Raycast / Alfred / Albert / wofi launchers

A first plugin can be as small as ~50 lines. See [`docs/plugin-api.md`](docs/plugin-api.md) for the local daemon's API surface.

### 5. Improve the core

The Rust core lives in [`crates/`](crates/). Core contributions include:

- Bug fixes
- Performance improvements
- Algorithm research (we use FSRS by default; experiments are welcome behind feature flags)
- New language pair support
- Dictionary backend implementations
- Sync backends (git, S3, WebDAV, iCloud, Dropbox)
- AI integrations (MCP server, Ollama / Claude / OpenAI plugins)

For non-trivial changes (>~100 lines or any change to a public API), please open an issue or [discussion](https://github.com/evanclan/kotoba/discussions) first.

### 6. Help with infra & maintenance

- Triage issues, label them, ask clarifying questions
- Review PRs (you don't need write access to leave a useful review)
- Improve CI workflows
- Write release notes
- Run a community deck-review session

---

## Development setup

### Prerequisites

- **Rust** 1.78 or newer (stable). Install via [rustup](https://rustup.rs).
- **Git** 2.30+
- (Optional) **Just** ([`cargo install just`](https://github.com/casey/just)) for convenient task running

### Clone and build

```bash
git clone https://github.com/evanclan/kotoba.git
cd kotoba
cargo build              # builds all crates in debug mode
cargo test               # runs the test suite
cargo run -p kotoba-cli -- --help
```

### Useful commands

```bash
cargo fmt --all                  # format
cargo clippy --all-targets       # lint
cargo test                       # test
cargo doc --no-deps --open       # browse generated docs

# end-to-end smoke test
cargo run -p kotoba-cli -- init
cargo run -p kotoba-cli -- lookup ありがとう
cargo run -p kotoba-cli -- add 留学
cargo run -p kotoba-cli -- review
```

### Project layout

```
kotoba/
├── crates/
│   ├── kotoba-core/     ← the engine (deck, SRS, dictionary, store)
│   ├── kotoba-cli/      ← the `kotoba` binary
│   ├── kotoba-daemon/   ← optional local HTTP/JSON-RPC API server
│   └── kotoba-mcp/      ← MCP server for AI agents
├── decks/               ← community-curated learning content
├── docs/                ← all design and architecture documentation
├── shells/              ← shell-prompt integrations
├── examples/            ← reference deck + plugin templates
└── scripts/             ← helper scripts (install, release)
```

See [`docs/architecture.md`](docs/architecture.md) for how the crates fit together.

---

## Code style

We use rustfmt and clippy in CI. Run them locally before pushing:

```bash
cargo fmt --all
cargo clippy --all-targets --all-features -- -D warnings
```

Beyond the formatters, a few principles:

- **Public APIs deserve doc comments.** Every public type/function in `kotoba-core` should have a `///` doc comment with at least one usage example.
- **Errors are typed, not stringly.** We use `thiserror` for library errors and `anyhow` only at the binary boundary.
- **No `unwrap()` in library code** unless an invariant guarantees safety, and that invariant is documented.
- **Keep dependencies lean.** Justify new dependencies in the PR description. We prefer `serde` + `serde_json` + small focused crates over kitchen-sink utilities.
- **Tests live next to the code** (`#[cfg(test)] mod tests`) for unit tests, and in `tests/` for integration tests.

For docs and markdown:

- Wrap at ~100 columns, but don't be religious about it.
- Use sentence-case headings (`## Quick start`, not `## Quick Start`).
- Prefer plain English over jargon. Translate jargon when it first appears.

---

## Commit messages

We follow [Conventional Commits](https://www.conventionalcommits.org):

```
<type>(<scope>): <subject>

[optional body]

[optional footer(s)]
```

Common types: `feat`, `fix`, `docs`, `refactor`, `perf`, `test`, `build`, `ci`, `chore`, `deck`.

Examples:

```
feat(cli): add `kotoba shell init zsh` subcommand
fix(srs): handle FSRS overdue cards when stability is 0
docs: clarify deck frontmatter required fields
deck(jlpt-n5): add 50 cards for body parts
```

This isn't enforced strictly, but consistent commits produce nicer changelogs.

---

## Pull request checklist

Before requesting a review:

- [ ] **Tests pass locally.** `cargo test` is green.
- [ ] **Code is formatted.** `cargo fmt --all`.
- [ ] **Clippy is clean.** `cargo clippy --all-targets`.
- [ ] **Public APIs are documented.** New `pub` items have doc comments.
- [ ] **Tests added** for behavioral changes (or a note in the PR explaining why not).
- [ ] **Docs updated** if behavior, flags, or formats change.
- [ ] **CHANGELOG.md** entry added under `## Unreleased` for user-facing changes.
- [ ] **PR description** explains *why* the change exists, not just *what* it does.

PRs that touch the SRS algorithm, the deck format, or any public API also need:

- [ ] An RFC discussion or issue with at least one maintainer +1 before implementation.
- [ ] Migration notes if the change is breaking.

---

## How decisions get made

- **Trivial changes** (typos, deck additions, small bug fixes): one maintainer review, merge.
- **Standard changes** (most features, refactors): two maintainer reviews, merge.
- **Significant changes** (new public APIs, algorithm changes, breaking changes): an RFC in [`docs/rfcs/`](docs/rfcs/) (TODO), at least 7 days of discussion, two maintainer +1s.

Maintainers are listed in [`.github/CODEOWNERS`](.github/CODEOWNERS) and are recognized in the [Governance doc](docs/governance.md).

If you've contributed substantively over time and want to become a maintainer, open a discussion. We grow the maintainer team intentionally rather than competitively.

---

## Recognition

- Every contributor's GitHub handle is automatically added to the [Contributors](https://github.com/evanclan/kotoba/graphs/contributors) page.
- Notable contributions are called out in release notes.
- Deck curators are credited inside the deck file's `authors:` field, forever.
- Translators are credited in the corresponding `README.<lang>.md` and locale files.

You don't have to be loud to be appreciated here. We see you.

---

## Questions?

- Open a [Discussion](https://github.com/evanclan/kotoba/discussions).
- File an issue with the `question` label.
- Ping a maintainer in a thread — we'd rather answer twice than not at all.

ありがとうございます、よろしくお願いします。
