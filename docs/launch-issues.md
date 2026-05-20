# Launch-day issues

Copy each block as a separate GitHub issue. Suggested labels are listed; the body is ready to paste. Numbers are workflow order, not GitHub issue numbers (which will be assigned automatically).

The mix is intentional:
- **6 `good first issue`** — for newcomers
- **8 `help wanted`** — meaty work for engaged contributors
- **3 `needs-rfc` / `discussion`** — architecture decisions
- **3 meta / release** — process & community

---

## 1. docs: add a "your first 5 minutes" GIF to the README

**Labels:** `good first issue`, `docs`

**Why:** The README links to a quickstart but has no visual. A 12-second GIF (asciinema → `agg`) showing `init` → `lookup` → `add` → `today` would 3x the conversion rate of GitHub visitors to stars.

**Acceptance criteria:**
- A `.gif` (or `.svg` from [asciinema-svg](https://github.com/marionebl/svg-term-cli)) lives in `assets/quickstart.gif`
- README embeds it under the "What it looks like" section
- ≤ 1 MB filesize
- No personal information in the recording

---

## 2. docs: translate the README into Spanish

**Labels:** `good first issue`, `i18n`, `docs`

**Why:** Spanish is one of the largest under-served language-learner populations. A `README.es.md` opens the door for Spanish-speaking developers + Japanese learners worldwide.

**Acceptance criteria:**
- New file `README.es.md` mirrors the English README in scope
- Cultural adaptation is welcome; not required to be 1:1
- Add a `[Español]` link to the language-switcher row in `README.md` and `README.ja.md`

---

## 3. deck: JLPT N5 — body parts

**Labels:** `good first issue`, `deck`

**Why:** Body parts is the easiest, most concrete N5 set — a perfect first deck for any contributor who knows Japanese.

**Acceptance criteria:**
- New file `decks/jlpt-n5/body-parts.md`
- 30–50 cards
- Each card has `Reading:`, `Meaning:`, and at least one card per card includes an example sentence
- License: CC-BY-SA-4.0
- Cite the source in frontmatter (your own knowledge, Tatoeba, or a textbook with the appropriate license grant)

---

## 4. deck: business email vocabulary (EN → JA)

**Labels:** `good first issue`, `deck`

**Why:** Persona Hiroshi (Tokyo backend engineer in [`docs/use-cases.md`](use-cases.md)) needs this.

**Acceptance criteria:**
- New file `decks/business-en-ja/email.md`
- 30–80 cards covering opening/closing phrases, formal vs casual tone, and common business actions ("attach", "circle back", "for your reference", "loop in")
- At least 10 cards include an example sentence

---

## 5. docs: fix typos and improve clarity in `docs/architecture.md`

**Labels:** `good first issue`, `docs`

**Why:** This file gets read most by new contributors; small wins compound.

**Acceptance criteria:**
- Any typo, unclear paragraph, or missing example
- Keep the existing structure; rewrites beyond a few sentences should be discussed first

---

## 6. chore: add a `Justfile` for common dev tasks

**Labels:** `good first issue`, `build`

**Why:** Lower the friction for contributors who don't remember the cargo incantations.

**Acceptance criteria:**
- A `justfile` at repo root with at least: `check`, `test`, `fmt`, `clippy`, `run`, `demo`
- Document `cargo install just` in CONTRIBUTING.md
- CI is not modified

---

## 7. feat(cli): implement `kotoba review` TUI

**Labels:** `help wanted`, `enhancement`

**Why:** This is the daily-habit loop. v0.0.1 does everything *except* review. We need the TUI built on `ratatui`.

**Acceptance criteria:**
- New subcommand `kotoba review`
- Pulls due cards via `Engine::due_cards`
- For each card: show front, wait for `space` to reveal, then `1`/`2`/`3`/`4` to grade
- Calls `Engine::record_review` after each grade
- `q` quits gracefully
- Clean exit even on Ctrl-C
- An end-of-session summary (cards reviewed, time spent, retention)
- Tests at least the state machine (no need to mock the terminal)

This is the single most impactful PR for the v0.1 milestone. Expect ~400 lines of focused work.

---

## 8. feat(cli): implement `kotoba stats`

**Labels:** `help wanted`, `enhancement`

**Why:** Without stats, users have no feedback on their habit. With stats, they feel progress and stay.

**Acceptance criteria:**
- `kotoba stats` prints: cards added, reviewed, retention %, current streak, weakest 5 cards
- `--since YYYY-MM-DD` flag
- Use the append-only reviews log as source of truth
- A simple terminal sparkline for the retention curve (8 dots in `▁▂▃▄▅▆▇█`) is a stretch goal
- Output is also available as `--json`

---

## 9. feat(core): bundle JMdict and replace the demo dictionary

**Labels:** `help wanted`, `enhancement`

**Why:** The demo dictionary in `crates/kotoba-cli/src/dictionary_demo.rs` has 5 entries. JMdict has ~200,000.

**Acceptance criteria:**
- A new `Dictionary` impl that loads JMdict (XML or its EDICT2 form) into a compact in-memory index
- Memory budget: lookup ≤ 200 MB resident, startup ≤ 200 ms on a modern laptop
- Lazy-download to `~/.kotoba/data/jmdict.bin` on first use, prompt the user before downloading
- Honor the EDRDG license requirements (attribution + license file)
- Add unit tests on a small fixture subset of JMdict

This is a 1–2 week piece of work and unlocks the rest of v0.1.

---

## 10. feat(cli): `kotoba shell init <shell>` subcommand

**Labels:** `help wanted`, `enhancement`

**Why:** Replace the manual snippets in `shells/` with a generated `eval`-able output.

**Acceptance criteria:**
- `kotoba shell init zsh` prints the contents of `shells/zsh/kotoba.zsh`
- Same for `fish`, `bash`, `nu`
- The README shell-init section is updated to use the new command
- A snapshot test verifies the printed output

---

## 11. feat(integration): VS Code extension

**Labels:** `help wanted`, `integration`

**Why:** VS Code is the second-most-popular editor among Kotoba's target users. An extension that does `lookup` and `add` in a hover/command palette is a magnet for contributors.

**Acceptance criteria:**
- New folder `integrations/vscode/`
- TypeScript extension that talks to `kotoba-daemon` (block on issue #N — daemon impl)
- Commands: `Kotoba: Lookup`, `Kotoba: Add Selection to Deck`, `Kotoba: Today`
- Hover provider for kanji
- Published manifest only — actual marketplace publish gated on maintainer review

---

## 12. feat(integration): Starship module

**Labels:** `help wanted`, `integration`, `good first issue`

**Why:** [Starship](https://starship.rs) is the popular cross-shell prompt. A `kotoba` module is the most ambient possible practice surface.

**Acceptance criteria:**
- A `starship.toml` snippet documented in `shells/starship/README.md`
- An optional custom module via `[custom.kotoba]` that runs `kotoba today --json` and renders the term
- Cache for 6 hours to avoid prompt latency
- Doc PR to upstream Starship is a stretch goal

---

## 13. feat(core): swap simplified scheduler for a faithful FSRS port

**Labels:** `help wanted`, `enhancement`, `needs-rfc`

**Why:** The current `Fsrs` is a simplified placeholder. We need the real one for v0.1.

**Acceptance criteria:**
- RFC first: which reference implementation are we porting, what weights, what tests
- Algorithm matches [`open-spaced-repetition/fsrs-rs`](https://github.com/open-spaced-repetition/fsrs-rs) outputs on a fixed corpus
- Tests cover: initial review, lapses, post-lapse recovery, easy-streak interval growth
- Old `Fsrs` is renamed `FsrsSimple` and gated behind a feature flag for benchmarking

---

## 14. discussion: how should we sync decks across devices?

**Labels:** `discussion`, `needs-rfc`

**Why:** The roadmap promises sync. There are at least 4 reasonable approaches (git, S3, WebDAV, iCloud), and the answer changes our trust model.

**Asked of the community:**
- Which backends matter most to you?
- Should sync be conflict-free (CRDT on the reviews log) or last-write-wins on deck files?
- Should we ship an opinionated default or "BYO backend" only?

Outcome: an RFC in `docs/rfcs/0001-sync.md`.

---

## 15. discussion: MCP resource & tool design for `kotoba-mcp`

**Labels:** `discussion`, `needs-rfc`

**Why:** What the MCP server exposes shapes Kotoba's relationship with AI for years. Worth designing in public.

**Asked of the community:**
- Which resources / tools should be available read-only vs require explicit consent?
- How do we handle prompt-injection risks when an agent reads `learner-state`?
- What's the minimum useful set we can ship in v0.4?

Outcome: an RFC in `docs/rfcs/0002-mcp.md`.

---

## 16. discussion: deck registry — centralized index or fully decentralized?

**Labels:** `discussion`, `needs-rfc`

**Why:** `kotoba install <deck>` needs a source. We can run a curated registry (like crates.io) or be fully decentralized (point at any git repo). Each has trade-offs in trust, discoverability, and moderation.

**Asked of the community:**
- Would you contribute to a curated registry or feel locked in?
- Should there be a `Kotoba-Verified` deck label and what would qualify?
- How do we moderate without becoming a gatekeeper?

---

## 17. chore(ci): add MSRV check and `cargo audit`

**Labels:** `help wanted`, `ci`, `security`

**Why:** Lock the minimum supported Rust version and catch vulnerable dependencies before users see them.

**Acceptance criteria:**
- CI job runs `cargo audit` weekly via `schedule:` trigger
- CI job verifies build on the declared MSRV (`rust-toolchain.toml`)
- Renovate or Dependabot configured for cargo updates

---

## 18. meta: design the project logo

**Labels:** `help wanted`, `discussion`

**Why:** A simple, recognizable logo helps with stars, branding, and shareables. A stylized 言 character with a subtle "bridge" reference would be on-brand.

**Acceptance criteria:**
- SVG source in `assets/logo.svg`
- Light + dark variants
- 1024×1024 PNG export
- Favicon set (16, 32, 192, 512)
- Licensed CC-BY-SA-4.0 with the trademark clause from `docs/governance.md`

---

## 19. meta: open GitHub Discussions categories

**Labels:** `meta`

**Why:** Off-by-default. Need: Announcements, Q&A, Show-and-tell ("share your dotfiles + Kotoba prompt"), Ideas, RFCs.

**Acceptance criteria:**
- Categories created
- One welcome post pinned in Announcements
- One "introduce yourself" post pinned in Q&A

---

## 20. meta: prepare v0.1 release notes draft

**Labels:** `meta`, `release`

**Why:** Write the release notes *before* v0.1 ships, so we know we're building toward something concrete and quotable.

**Acceptance criteria:**
- `docs/releases/v0.1.md` drafts the user-facing changelog
- Includes a 2-line headline, an animated demo placeholder, the 3 things that landed, and known limitations
- Updated as work proceeds; finalized when we cut v0.1
