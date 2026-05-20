# Use cases

Why Kotoba exists, told through the people who use it. We use this document when we make product decisions: every feature should make at least one of these stories true.

## Personas (today)

### A. Hiroshi — The Japanese developer learning English

Hiroshi works on a Tokyo backend team. He wants to contribute to international OSS but his technical English vocabulary is shallow. His terminal is open all day.

- He runs `kotoba add` whenever he sees a new term in a PR.
- A 5-minute review fits between `cargo build`s.
- His decks live in his dotfiles repo. They sync between his work and home machines via git.
- A word-of-the-day in his prompt makes practice ambient.

**What we ship for Hiroshi:** the CLI, FSRS, shell integrations, JMdict + EDICT.

### B. Maya — The Berlin engineer learning Japanese

Mirror of Hiroshi. She's planning to move to Tokyo. She also wants:

- Furigana on every kanji until she's solid (a config flag)
- A browser extension that lets her capture words from NHK Easy News (v0.3)

### C. Carlos — The serious self-learner past Duolingo

N3-level Japanese in Madrid. He owns nothing in WaniKani's database. He wants:

- A unified system for vocab, kanji, and grammar
- Custom decks based on the manga he reads
- A way to fork his friend's deck and merge updates

**What we ship for Carlos:** plain-text decks in git, deck lint, deck registry.

### D. Tanaka-sensei — The primary-school teacher

Builds vocabulary decks tied to each unit of her textbook. Wants to:

- Publish them as open curriculum that other teachers fork
- See aggregate weak points across her class without seeing individual students' raw data

**What we ship for Tanaka-sensei:** classroom mode (planned), deck registry, anonymized aggregates.

### E. Kenta — The study-abroad student

Leaving Osaka for a year at the University of Toronto. Needs:

- Academic English vocabulary (lectures, essays)
- Practical English (dorms, banking, healthcare)
- Capture words he hears in lectures in real-time (mobile, microphone)

**What we ship for Kenta:** mobile companion, capture-from-context, pre-departure deck templates.

### F. Sarah — The expat in Japan

Three years in Tokyo, plateaued at conversational. Wants to break through to professional fluency. Reads NHK in browser; captures words on the fly.

### G. Reza — The polyglot

Learning Japanese and Korean simultaneously. Wants a single tool that handles both, and the ability to script across them — find Sino-Korean ↔ Sino-Japanese cognates, share grammar tags, etc.

### H. You, in five years — The parent

Teaching your child Japanese (or English). A `kotoba kids` mode with picture cards and audio, built on the same engine.

---

## Future personas (AI-native era)

### I. Yui — Asks "what should I review for tomorrow's interview?"

Her AI agent reads `kotoba://recent-mistakes` and `kotoba://learner-state` via MCP. It generates a 15-minute focused study session targeting business Japanese N2 weak points specifically.

### J. Marco — Reading a Japanese novel on his Kindle

A future companion app captures words he highlights, sends them to Kotoba, generates a comprehension passage at his level using only known vocabulary + 5 new words from the chapter ahead. Tomorrow he'll see the words again in review and again in tomorrow's reading.

### K. Hana — Voice conversation with an agent

Her tutor-agent reads her weak grammar (`～ておく`, conditional `たら`), then orchestrates an unscripted scenario (ordering takeout) that forces those grammar points. Mistakes flow back into her Kotoba deck.

### L. Wendy — In Tokyo wearing AR glasses

Looking at a sign. Her glasses overlay reading + meaning *only* on words she doesn't know. Capture is one tap. Card is added to Kotoba with the photo as the memory anchor. Reviewed on her commute home.

### M. Carlos's daughter — Has a verifiable language portfolio

Five years later, Carlos's daughter applies to a Japanese university. Her Kotoba portfolio: 24,318 cards reviewed, 91% retention, 2,400 hours of practice — cryptographically signed. The university accepts it alongside JLPT.

### N. Tanaka-sensei in 2030 — Generates personalized homework

For each student, an agent reads their Kotoba state and produces a tailored 25-minute homework PDF that targets their three weakest words this week. Tanaka approves with one click.

---

## What this means for design decisions

The user we are *not* building for: the casual learner who wants gamified streaks and push notifications. Duolingo serves them well. Kotoba is for people serious enough to type a command, edit a markdown file, or want their data to live with them for a decade. From this:

- **CLI is the canonical UX.** Everything else is a surface on top of it.
- **Plain text wins.** Even at small ergonomic cost.
- **AI is opt-in, BYO model.** No required account, no required cloud.
- **Schedulers, dictionaries, and storage are traits.** The product becomes whatever shape the future demands.

If a feature doesn't help one of the personas above (today's or tomorrow's), it probably doesn't belong.
