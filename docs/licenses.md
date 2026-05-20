# Licenses & attributions

This document tracks all third-party data, code, and assets used by Kotoba and the licenses under which we use them.

## Project license

- **Source code:** [MIT](../LICENSE) (Copyright Kotoba contributors)
- **Documentation:** the same MIT license, unless an individual file says otherwise
- **Default deck content:** [CC-BY-SA-4.0](https://creativecommons.org/licenses/by-sa/4.0/) unless a specific deck states otherwise

## Dictionary data

### JMdict

- Source: <https://www.edrdg.org/jmdict/edict_doc.html>
- Maintainer: The Electronic Dictionary Research and Development Group (EDRDG)
- License: [EDRDG license](https://www.edrdg.org/edrdg/licence.html)
- Use: bundled (or downloaded on first run) as Kotoba's default Japanese ↔ English dictionary
- Attribution requirement: any product using JMdict must credit the EDRDG and link to the license. We do this in:
  - `kotoba --about`
  - The TUI's help screen
  - This file

### KANJIDIC2

- Source: <https://www.edrdg.org/kanjidic/kanjd2index.html>
- Same maintainer, same license, same attribution requirements as JMdict.
- Use: kanji metadata (stroke count, JLPT level, radicals, readings).

## Algorithms

### FSRS (Free Spaced Repetition Scheduler)

- Reference implementation: <https://github.com/open-spaced-repetition>
- License: MIT
- Our implementation lives in [`crates/kotoba-core/src/srs.rs`](../crates/kotoba-core/src/srs.rs) and is independently written; we credit the FSRS authors and link to the original work.

## Code dependencies

Run `cargo about generate about.hbs > about.html` (planned) for a full SBOM. Major dependencies are:

| Crate           | License         | Purpose                              |
|-----------------|------------------|--------------------------------------|
| `clap`          | MIT / Apache-2.0 | CLI argument parsing                 |
| `ratatui`       | MIT             | TUI rendering                        |
| `tokio`         | MIT             | Async runtime (daemon)               |
| `axum`          | MIT             | HTTP server (daemon)                 |
| `serde`         | MIT / Apache-2.0 | Serialization                        |
| `tracing`       | MIT             | Structured logging                   |
| `chrono`        | MIT / Apache-2.0 | Date/time                            |
| `pulldown-cmark`| MIT             | Markdown parsing                     |
| `directories`   | MIT / Apache-2.0 | Cross-platform user-dir resolution   |

## Audio contributions

User-contributed audio is licensed CC-BY-SA-4.0 with the contributor named in the deck. Contributors retain copyright; the license grant is irrevocable for the version contributed.

## Logos & visual assets

Project logo TBD. When created:

- Logo files in `assets/`
- License: CC-BY-SA-4.0 with the trademark restriction in [governance.md](governance.md#trademark).

## Reporting a license issue

If you believe Kotoba is using something incorrectly, please email **legal@kotoba.dev** (placeholder) or open a private security advisory. We take this seriously.
