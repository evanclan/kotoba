# Deck data format

Kotoba decks are **plain markdown files** with YAML frontmatter. This document is the canonical specification.

## Why markdown?

- Anyone can read it.
- Every editor can edit it.
- Git can diff it.
- Any AI model can understand it.
- It outlives every binary format we could invent.

## Minimum viable deck

```markdown
---
name: My personal vocabulary
language: ja
target_language: en
license: CC-BY-SA-4.0
authors:
  - your-handle
created: 2026-05-20
---

# 留学

**Reading:** りゅうがく
**Meaning:** studying abroad
**Tags:** [education, n3]

> 妹は来年アメリカに留学します。
> My younger sister will study abroad in America next year.

---

# 食卓

**Reading:** しょくたく
**Meaning:** dining table
**Tags:** [home, n3]
```

That's it. A v0.1 deck.

## Frontmatter schema

```yaml
name: string                  # Required. Human-readable deck name.
slug: string                  # Optional. URL/ID-safe slug. Derived from filename if absent.
description: string           # Optional. One-line description.
language: BCP-47              # Required. Language being learned (e.g. "ja", "en").
target_language: BCP-47       # Required. User's reference language (e.g. "en", "ja", "es").
license: SPDX                 # Required. e.g. "CC-BY-SA-4.0", "CC0-1.0", "MIT".
authors:                      # Optional. List of contributors.
  - handle
created: YYYY-MM-DD           # Required. Creation date (UTC).
updated: YYYY-MM-DD           # Optional. Last update.
source: string                # Optional. Where the content came from.
audience: string              # Optional. e.g. "JLPT N5", "business", "kids".
version: semver               # Optional. Deck version, e.g. "1.0.0".
tags:                         # Optional. Top-level tags applied to all cards.
  - tag
```

Unknown fields are ignored, but reserved for future use. Don't invent your own — propose them.

## Card grammar

Each `# Heading` starts a new card. The heading text is the **front** of the card by default.

The body is parsed for these labeled fields:

| Field          | Markdown form                       | Notes                                              |
|----------------|--------------------------------------|----------------------------------------------------|
| Reading        | `**Reading:** りゅうがく`            | The kana / pronunciation.                         |
| Meaning        | `**Meaning:** studying abroad`       | The translation / definition.                     |
| Tags           | `**Tags:** [education, n3]`          | Comma- or YAML-list-formatted.                    |
| Notes          | `**Notes:** ...`                     | Free-form notes shown after revealing the card.  |
| Examples       | Block quotes (`> ...`)               | Pairs of two consecutive lines: native, gloss.   |
| Audio          | `**Audio:** ./audio/abc.ogg`         | Relative or absolute path. Optional.              |
| Image          | `![alt](./images/abc.jpg)`           | Standard markdown image; rendered when supported.|
| Context        | `**Context:** url-or-path`           | Where the user first encountered this word.       |

Anything else in the card body is treated as additional notes and shown after the answer is revealed.

### Multiple front/back forms

For more controlled card layouts, use explicit `Front:` / `Back:` blocks:

```markdown
# 食卓

**Front:** 食卓
**Back:**
しょくたく — dining table

The kanji 食 means "eat" and 卓 means "table".
```

### Cloze cards

Use `{{c1::text}}` to mark deletions, Anki-style:

```markdown
# 留学する用法

**Front:** 来年、私は{{c1::アメリカ}}に{{c2::留学}}します。
**Meaning:** Next year, I will study abroad in America.
```

Each `c<n>` becomes one card.

## Card identifiers

A card's stable ID is the SHA-256 of its front text + deck slug. This means renaming or moving cards within a file is fine; changing the front text creates a new card. The ID is used in the review log to track progress across renames and across machines.

You can override the ID with `**Id:** ...` if you must.

## Reviews log

Reviews don't live in the deck file. They live in `~/.kotoba/reviews/YYYY/MM.jsonl`:

```
{"ts":"2026-05-20T08:32:11Z","card":"sha256:...","grade":3,"interval_d":4.2,"context":"cli"}
```

Append-only. Conflict-free. Sync-friendly.

## Deck linting

A `kotoba deck lint <path>` command (planned for v0.2) validates:

- Required frontmatter fields present
- Each card has at least one of (Reading, Meaning, Front, Back)
- License is a recognized SPDX identifier
- Audio / image paths exist
- No duplicate IDs

CI runs this on every PR that touches `decks/`.

## Importing

Migration tools planned for v0.3:

- `kotoba import anki <file.apkg>`
- `kotoba import csv <file.csv> --delimiter , --front 0 --back 1`
- `kotoba import wanikani <api-key>` (read-only)

Output is always plain markdown — never a proprietary binary.

## Best practices

- **Keep decks under ~1,000 cards.** Split by topic. Easier to review, fork, and license.
- **Name cards with the target-language form.** `# 留学`, not `# studying abroad`.
- **Cite your sources.** It builds trust, satisfies licensing, and helps reviewers.
- **One concept per card.** Cards with three meanings smushed together don't review well.
- **Add example sentences.** Cards without context are harder to remember.
- **License generously.** CC-BY-SA-4.0 is the default for a reason: it lets the community remix while keeping derivatives open.
