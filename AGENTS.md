# AGENTS.md — guidance for AI coding assistants

This file is for AI assistants (Claude, Cursor, Copilot, etc.) working in this repository. Human contributors should read [CONTRIBUTING.md](CONTRIBUTING.md) instead.

## Mission of this project

Kotoba is a **local-first, terminal-native, scriptable language-learning toolkit**. We deliberately avoid AI dependence in the core, even though we welcome AI-driven surfaces on top. Every change should preserve:

1. **Plain-text deck format** in `decks/`
2. **Trait-based extensibility** in `kotoba-core` (Store, Dictionary, Scheduler)
3. **Zero default telemetry**
4. **Offline-first behavior**

## House rules for AI-assisted edits

- **Don't add hidden network calls.** Anything that reaches over the network must be opt-in, behind a config flag, and documented.
- **Don't add binary blobs.** Generated dictionary indices belong in `cache/`, not in the repo.
- **Match the existing trait shapes.** New backends are new `impl Trait`, not new traits, unless an RFC is opened.
- **Tests next to the code.** Every new public function gets at least one test.
- **Doc comments on public items.** `kotoba-core` is `#![deny(missing_docs)]`.
- **No `unwrap()` in library crates.** Use `?` or document the invariant.

## Where to add things

| Change                           | Location                                   |
|----------------------------------|--------------------------------------------|
| New CLI subcommand               | `crates/kotoba-cli/src/commands/`          |
| New domain type                  | `crates/kotoba-core/src/`                  |
| New deck or deck content         | `decks/`                                   |
| New shell integration            | `shells/<shell>/`                          |
| New design doc                   | `docs/`                                    |
| New CI step                      | `.github/workflows/ci.yml`                 |

## Things to avoid

- Re-implementing dictionary parsing inside `kotoba-cli`. It belongs behind the `Dictionary` trait in `kotoba-core` (or a new sibling crate).
- Adding cargo dependencies without justification in the PR description.
- Removing existing tests to "make CI green."
- Touching `docs/governance.md` or `docs/roadmap.md` without an RFC discussion.

## When in doubt

Open a [Discussion](https://github.com/evanclan/kotoba/discussions) describing the change you'd like to make and why. AI assistants should treat unanswered ambiguity as a signal to stop and ask.
