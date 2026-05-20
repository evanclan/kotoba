# Community decks

This folder contains community-curated decks. Anyone can contribute by editing a markdown file.

## Folder layout

```
decks/
├── starter/         ← what new users see right after `kotoba init`
├── jlpt-n5/         ← JLPT N5 vocabulary
├── jlpt-n4/         ← JLPT N4 vocabulary
├── business-en-ja/  ← business vocabulary (English ↔ Japanese)
└── README.md        ← this file
```

Each subfolder contains one or more `.md` files. Each `.md` is one deck.

## Adding a deck

1. Read [`docs/data-format.md`](../docs/data-format.md) — it's short.
2. Pick a folder (or create a new one for a new language pair / theme).
3. Copy [`examples/deck.md`](../examples/deck.md) as a starting point.
4. Open a PR.

Pull-request CI will run `kotoba deck lint <your-file>` (planned for v0.2) to validate frontmatter and basic structure.

## Licensing

Default: **CC-BY-SA-4.0**. Override in your deck's frontmatter only if you have a reason. Always cite your source.

## Conventions

- Filename: lowercase, hyphenated, descriptive (`jlpt-n5-body-parts.md`, not `Body Parts.md`).
- Card order: by frequency or pedagogical order, not alphabetical.
- ~50–500 cards per deck. Larger sets should be split.
- Cite your source in `source:` frontmatter.

## What we will not accept

- Decks scraped from copyrighted, non-redistributable sources (textbooks, paid SaaS) without a clear license grant.
- Decks of profanity, slurs, or harassing content.
- AI-generated decks without human review of every card. (Hybrid AI-drafted, human-edited is fine — disclose in the source field.)
