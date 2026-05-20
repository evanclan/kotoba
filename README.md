<h1 align="center">Kotoba (言葉)</h1>

<p align="center">
  <em>A local-first, terminal-native, scriptable language-learning toolkit.</em>
</p>

<p align="center">
  <em>ChatGPT teaches you anything once.<br/>
  Kotoba remembers what you've learned, schedules what you'll forget, and lives in every tool you already use.</em>
</p>

<p align="center">
  <a href="#-quick-start"><strong>Quick Start</strong></a> ·
  <a href="docs/architecture.md"><strong>Architecture</strong></a> ·
  <a href="docs/roadmap.md"><strong>Roadmap</strong></a> ·
  <a href="CONTRIBUTING.md"><strong>Contribute</strong></a> ·
  <a href="README.ja.md"><strong>日本語</strong></a>
</p>

<p align="center">
  <a href="https://github.com/evanclan/kotoba/actions"><img alt="CI" src="https://img.shields.io/badge/CI-pending-lightgrey"></a>
  <a href="LICENSE"><img alt="License" src="https://img.shields.io/badge/license-MIT-blue.svg"></a>
  <a href="docs/roadmap.md"><img alt="Status" src="https://img.shields.io/badge/status-alpha-orange"></a>
  <a href="CODE_OF_CONDUCT.md"><img alt="Contributor Covenant" src="https://img.shields.io/badge/Contributor%20Covenant-2.1-purple.svg"></a>
</p>

---

## Why Kotoba?

Most language-learning tools want to own you — your data, your schedule, your subscription. Kotoba goes the other way:

- **Your data is plain text in a folder you own.** Markdown decks, version-controlled in git. Diff your vocabulary growth across years. Fork your friend's deck. Send a pull request to a teacher's curriculum.
- **It lives where you live.** A CLI for your terminal, a TUI for focused review, a daemon for editor and browser plugins, an MCP server for AI agents. No Electron, no login, no telemetry.
- **It treats AI as a peer, not a master.** Bring your own LLM (Ollama, Claude, OpenAI, OpenRouter). Kotoba is the substrate; AI is one tool that plugs into it.
- **It works offline, on a plane, behind a firewall, on a Pi, forever.** A single binary, plain-text data, open dictionary sources.

Kotoba is built primarily for the **Japanese ↔ English** learning audience, with a generic architecture so it can serve any language pair.

---

## Who is this for?

| If you are... | Kotoba gives you... |
|---|---|
| A developer learning Japanese (or any language) | Word-of-the-day in your shell prompt, lookups in your editor, decks in your dotfiles repo |
| A serious self-learner past Duolingo | A modern, FSRS-based SRS where you actually own your decks |
| A teacher | Curriculum-as-code: publishable, forkable, versionable lesson decks |
| A study-abroad student | A vocabulary toolkit you take with you, online or offline |
| A polyglot | One engine, multiple language pairs, scriptable across them |
| A contributor | A clean Rust core, plain-text data, and a roadmap full of "good first issue" labels |

See [docs/use-cases.md](docs/use-cases.md) for detailed personas, today and in an AI-native future.

---

## What it looks like

```
$ kotoba today
[木曜日] 食卓 (しょくたく) — dining table   3 reviews due

$ kotoba lookup 留学
留学 (りゅうがく)
  noun, suru-verb
  studying abroad
  JLPT N3 · frequency rank 4,212
  examples:
    アメリカに留学する。 — to study abroad in America.
    妹は来年留学します。 — my younger sister will study abroad next year.

$ kotoba add 留学
✓ added to ~/.kotoba/decks/personal.md

$ kotoba review
┌─ Kotoba — 8 cards due ───────────────────────────────────┐
│                                                          │
│                       留学                                │
│                                                          │
│   [space] reveal answer    [q] quit    [s] suspend       │
└──────────────────────────────────────────────────────────┘
```

A 90-second daily review. That's the loop.

---

## 🚀 Quick Start

> **Status:** Kotoba is in early alpha. The instructions below describe the v0.0.1 reference implementation. The roadmap and architecture in this repo describe where it's going.

### Install (from source, alpha)

```bash
git clone https://github.com/evanclan/kotoba.git
cd kotoba
cargo build --release
./target/release/kotoba init
```

Once Kotoba ships its first stable release, install will be:

```bash
# macOS / Linux (Homebrew)
brew install kotoba

# any platform with Rust
cargo install kotoba

# Linux (one-liner)
curl -fsSL https://kotoba.dev/install.sh | sh
```

### First five minutes

```bash
kotoba init                    # creates ~/.kotoba with starter deck
kotoba lookup ありがとう         # try the dictionary
kotoba add 食卓                 # add a card to your personal deck
kotoba review                  # 90-second TUI review session
kotoba today                   # word of the day for your shell prompt
kotoba stats                   # your retention curve
```

### Add Kotoba to your shell prompt

```bash
# zsh — append to ~/.zshrc
eval "$(kotoba shell init zsh)"

# fish
kotoba shell init fish | source

# bash — append to ~/.bashrc
eval "$(kotoba shell init bash)"

# nushell — append to config.nu
kotoba shell init nu | save -a $nu.config-path
```

See [`shells/`](shells/) for full integration guides including Starship, tmux, and Powerlevel10k.

---

## How it works (10,000-foot view)

```
┌──────────────────────── Surfaces ────────────────────────┐
│   CLI    TUI    Editor plugins    Browser ext    Mobile  │
└──────────────────────────────────────────────────────────┘
                          │
                ┌─────────┴─────────┐
                │   kotoba daemon   │  ← optional local HTTP/JSON-RPC API
                │   kotoba-mcp      │  ← MCP server for AI agents
                └─────────┬─────────┘
                          │
                ┌─────────┴─────────┐
                │   kotoba-core     │  ← Rust library: SRS, deck, dictionary
                └─────────┬─────────┘
                          │
        ┌─────────────────┼─────────────────┐
        │                 │                 │
   ~/.kotoba/         JMdict +          FSRS scheduler
   decks (markdown)   KANJIDIC2         (open algorithm)
```

Plain-text decks at the bottom. A single Rust core in the middle. Many surfaces on top. Read the full design in [`docs/architecture.md`](docs/architecture.md).

---

## What makes Kotoba different

| | Anki | Duolingo | WaniKani / SaaS | **Kotoba** |
|---|---|---|---|---|
| Open source | ✅ | ❌ | ❌ | ✅ |
| Data ownership | Binary blob | None | None | **Plain text in your folder** |
| Modern algorithm (FSRS) | Via addon | ❌ | ❌ | **Default** |
| Terminal / dev integration | None | None | None | **First-class** |
| Scriptable / pipeable | ❌ | ❌ | ❌ | **Yes** |
| Works offline | ✅ | Limited | ❌ | **Yes** |
| BYO AI model | ❌ | ❌ | ❌ | **Yes** |
| MCP / agent-ready | ❌ | ❌ | ❌ | **Yes** |
| Cost | Free | Freemium | $9–15/mo | **Free, forever** |

Kotoba is not trying to be a better Duolingo or a better WaniKani. It's the **substrate** — the place where your learning data lives, that every other tool (today and tomorrow) plugs into.

---

## 🤝 How to contribute

**You don't have to be a Rust engineer.** Kotoba is designed so that everyone can contribute something meaningful.

### If you don't write code

- 🗂️ **Curate a deck.** Edit a markdown file. Send a PR. See [`decks/`](decks/).
- 🌏 **Translate the UI.** Help users in your language. See [`docs/i18n.md`](docs/i18n.md).
- 🎙️ **Record audio.** Native-speaker pronunciations are gold.
- ✏️ **Improve docs.** Find a typo, fix it. Find an unclear paragraph, rewrite it.
- 🐛 **Report bugs.** Use the issue templates in [`.github/ISSUE_TEMPLATE`](.github/ISSUE_TEMPLATE).
- 💡 **Suggest a feature.** Open a discussion, not just an issue.

### If you write code

- ⚙️ **Add a shell integration.** Bash, zsh, fish, nushell, Starship, tmux, Powerlevel10k.
- 🪟 **Build an editor plugin.** VS Code, Neovim, Zed, Helix, Sublime, Emacs.
- 🌐 **Build a browser extension.** Capture words while reading the web.
- 📱 **Help with the mobile companion.** iOS / Android / web.
- 🧪 **Improve the SRS algorithm.** FSRS is research-active; we welcome experiments.
- 🔌 **Add a new language pair.** Korean, Mandarin, Spanish, Vietnamese — anything.
- 🤖 **Wire up AI integrations.** MCP server, plugin templates for Ollama / Claude / OpenAI.

Read [CONTRIBUTING.md](CONTRIBUTING.md) for the full guide. Look for issues labeled [`good first issue`](https://github.com/evanclan/kotoba/labels/good%20first%20issue).

---

## Roadmap

A condensed view. Full details in [`docs/roadmap.md`](docs/roadmap.md).

| Milestone | Focus |
|---|---|
| **v0.1** | CLI core: lookup, add, review, today, stats. Markdown deck format. JMdict integration. FSRS algorithm. |
| **v0.2** | Shell integrations (zsh/fish/bash/nu). Furigana. Kanji breakdown. JLPT level metadata. |
| **v0.3** | Local daemon + JSON API. VS Code extension. Browser extension. |
| **v0.4** | MCP server. Ollama / Claude / OpenAI plugin examples. AI-generated example sentences. |
| **v0.5** | Web dashboard (read-only stats). Sync via git, S3, WebDAV, iCloud Drive. |
| **v0.6** | Second language pair (Korean ↔ Japanese or Spanish ↔ English — community vote). |
| **v1.0** | Mobile companion. Stable plugin API. First curriculum partnerships. |

We will not chase mobile-first or paid features. Forever.

---

## Governance & values

Kotoba is governed by a small core-maintainer team and a contributor-driven roadmap.

- **Local-first, plain-text, open formats — forever.** Your data outlives any company.
- **Privacy by default.** No telemetry. No accounts. Sync is opt-in and bring-your-own-backend.
- **Open algorithms, open data.** JMdict, KANJIDIC2, FSRS. We document and explain everything.
- **AI as a peer.** Never required. Always optional. Always swappable.
- **Globally inclusive.** UI, docs, and decks must support non-English first-languages from day one.
- **No dark patterns. Ever.**

See [`docs/governance.md`](docs/governance.md) and our [Code of Conduct](CODE_OF_CONDUCT.md).

---

## Sponsors & support

Kotoba will always be free and open source. If your organization benefits from it (a university, a language school, an enterprise team), consider sponsoring development via [GitHub Sponsors](.github/FUNDING.yml). Every sponsorship is acknowledged in releases.

---

## License

[MIT](LICENSE) — do whatever you want, just keep the notice.

Dictionary data: [JMdict](https://www.edrdg.org/jmdict/edict_doc.html) and [KANJIDIC2](https://www.edrdg.org/kanjidic/kanjd2index.html) are licensed under the [EDRDG license](https://www.edrdg.org/edrdg/licence.html). We comply with their attribution requirements; see [`docs/licenses.md`](docs/licenses.md).

---

<p align="center">
  Built with care in Tokyo. ⛩️<br/>
  <em>「言葉は橋。」 — words are bridges.</em>
</p>
