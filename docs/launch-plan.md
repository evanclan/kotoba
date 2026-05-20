# Launch plan

How we go from "the repo exists" to "the right 1,000 people know about it and 100 are using it."

The goal of launch week is **not stars**. It is:

1. Find the first 10 contributors (deck curators, plugin authors, doc translators).
2. Surface the design questions we need to answer before v0.1.
3. Establish the repo's tone — careful, generous, ambitious — in public.

Stars are a side-effect.

---

## Pre-launch checklist

Do all of these before posting anywhere.

### Code & repo

- [ ] `cargo test` green on macOS and Linux locally
- [ ] `cargo clippy --all-targets -- -D warnings` clean
- [ ] CI passing on the default branch in GitHub Actions
- [ ] `cargo run -p kotoba-cli -- init && lookup ありがとう && add 留学 && today` works on a fresh machine
- [ ] Replace `evanclan/kotoba` everywhere — `grep -r 'evanclan' .`
- [ ] Replace placeholder emails (`conduct@`, `security@`, `legal@`) with real ones or your handle
- [ ] Tag the first release `v0.0.1` once happy with the demo

### GitHub setup

- [ ] Repository description set ("Local-first, terminal-native, scriptable language-learning toolkit.")
- [ ] Topics added: `rust`, `cli`, `language-learning`, `japanese`, `spaced-repetition`, `srs`, `flashcards`, `local-first`, `mcp`
- [ ] Pin 5 issues from `docs/launch-issues.md`: #1 (README GIF), #3 (first deck), #7 (review TUI), #11 (VS Code), #14 (sync RFC)
- [ ] Discussions enabled with categories: Announcements, Q&A, Show & Tell, Ideas, RFCs
- [ ] CODEOWNERS rewritten to use real handles (or removed until teams exist)
- [ ] License & community files surfaced (GitHub auto-detects from root)
- [ ] Add a project README banner image (`assets/banner.png` — 1280×640)
- [ ] Apply the label set in `.github/labels.yml`

### Content prepared

- [ ] 12-second demo GIF in `assets/quickstart.gif`
- [ ] Launch announcement blog post drafted (see template below)
- [ ] Show HN title and copy drafted (see template below)
- [ ] r/LearnJapanese post drafted
- [ ] X / Mastodon thread drafted
- [ ] At least 2 community decks merged so the repo isn't empty

### Personal readiness

- [ ] You have a 4-hour window the launch day to respond to comments
- [ ] You have notifications on for GitHub mentions and issue creations
- [ ] You're well-rested. Bad launches happen when the maintainer is tired and snippy

---

## Timing

The aim is to hit the audiences when they're online.

| Audience | Best window |
|---|---|
| Hacker News | **Sunday 18:00–21:00 UTC** (US morning, post-coffee). Avoid weekday mornings — too much competition. |
| r/LearnJapanese | Wednesday or Thursday, 12:00–15:00 UTC (US lunch, JP early evening) |
| r/rust | Tuesday or Wednesday, 14:00 UTC |
| Japanese tech Twitter | JP weekday morning, 22:00–00:00 UTC (07:00–09:00 JST) |
| Global dev Twitter | US morning, 14:00–16:00 UTC |

Hit Hacker News *first*. Everything else can follow on subsequent days.

---

## Launch-day timeline (Hacker News Sunday)

Times are illustrative; adapt to your time zone.

**T-2 days (Friday):**
- Final tests on a fresh VM (Linode/Hetzner, macOS in CI)
- Send the draft Show HN title to 1–2 friends for honest reactions
- Schedule the announcement blog post if you have one

**T-1 day (Saturday):**
- Last walkthrough of README, README.ja, CONTRIBUTING. Read aloud — typos hide better visually.
- Confirm CI is green
- Verify `gh repo view` shows the right description, topics, and pinned issues
- Mentally rehearse: what will I say when the first hostile comment lands?

**T-0 (Sunday):**

| Time (UTC) | Action |
|---|---|
| 17:00 | Final cargo test on your laptop |
| 17:30 | Post to **Hacker News** with the Show HN title |
| 17:35 | Post the first comment yourself (project context, what feedback you want — see template) |
| 17:40 | Pin the HN URL to your personal bio if you use Twitter/Mastodon |
| 18:00–22:00 | **Sit at the computer.** Reply to *every* comment within 30 minutes. Be brief, warm, and curious. Avoid defensive replies. |
| 22:00 | First exhale. Star count is now whatever it is. |

**T+1 day (Monday):**
- Post a thread on dev Twitter / Mastodon referencing the HN discussion
- Submit to https://lobste.rs (one good story per week max)
- Submit to https://this-week-in-rust.org (issues#) as a project intro
- Reply to overnight comments

**T+2 day (Tuesday):**
- Post to **r/rust** with a slightly different angle ("a small Rust workspace I built — terminal-native language learning")
- Reach out individually to 5 Japanese-learning Discord moderators with a friendly note

**T+3 day (Wednesday):**
- Post to **r/LearnJapanese** — with their rules in mind (no spam, contribution focus)
- Post to r/japaneselearning
- Send a friendly note to a few existing tools (Yomitan maintainers, jpdb.io community) introducing yourself and proposing interop

**T+7 day:**
- Write a "what happened in launch week" thread or blog post. Cite numbers, quote contributors, name names. This is the post that converts star-givers into committers.

---

## Templates

### Show HN title

> **Show HN: Kotoba – terminal-native language learning, where your decks are markdown in git**

Avoid:
- "AI-powered" (over-saturated)
- "Anki killer" (combative)
- Vague claims ("the best…")

Variants to A/B in your head:
- "Show HN: A local-first, scriptable alternative to Anki, written in Rust"
- "Show HN: Kotoba — practice Japanese from your shell"

### Show HN body (paste as the second comment under your own post)

> Hi HN — I built Kotoba over the past few weeks because I wanted Anki's substrate, the Unix philosophy's manners, and ChatGPT's *peer*, not its master.
>
> The core idea: your learning data should be plain text in a folder you own. Reviews are appended to a JSONL log. The CLI lives in your terminal, the daemon serves editor plugins and an MCP server for AI agents, and a future browser extension captures words from any webpage.
>
> Today it's v0.0.1 — `init`, `lookup`, `add`, `today`, `decks` work; the full FSRS scheduler, JMdict integration, and review TUI are tracked as the next milestones in [docs/roadmap.md]().
>
> I'm a non-Rust IT manager in Japan working in primary education and language learning — so this scratches a specific itch, but the architecture is language-agnostic.
>
> What I'd love feedback on:
> 1. The deck format ([docs/data-format.md]())
> 2. The MCP / AI-integration design ([docs/ai-integration.md]())
> 3. Whether the niche (developer-first language tool) feels real to you
>
> All contributions welcome — decks, docs, plugins, code. There's a contributor-friendly roadmap and 20 pre-triaged issues to start with.

### r/LearnJapanese post

> **I built an open-source, terminal-based vocabulary tool — Anki's data model meets the Unix philosophy. Looking for early users and deck curators.**
>
> Hi r/LearnJapanese — long-time lurker, occasional contributor.
>
> Kotoba ([link]) is a CLI tool for Japanese vocabulary that stores everything as plain markdown in a folder you own. You can `kotoba lookup 留学` for an instant dictionary, `kotoba add` to drop a word into your personal deck, and the FSRS-style scheduler decides when to show it again.
>
> It's not a replacement for WaniKani's curriculum or BunPro's grammar. It's the *data substrate* — a place to capture words you encounter (in NHK Easy, in a manga, in a textbook) that lives in your dotfiles forever.
>
> Free, open source (MIT), works offline, no account. Looking for:
> - **Curators** to build the first community decks (JLPT N5–N1, themed, anime, manga, business)
> - **Plugin authors** — VS Code, Neovim, browser extensions
> - **Honest feedback** on whether this would fit into your study

Avoid the marketing voice; this audience can smell it from a mile away.

### Twitter / X thread

1. *Hook:* I built [Kotoba]() — a Japanese language-learning tool that lives in your terminal and stores your decks as markdown in git.
2. *Why:* I wanted my study data to be diffable, forkable, and outlive any company.
3. *Demo GIF*
4. *What's different:* Local-first. Plain text. BYO AI model. Free forever.
5. *Roadmap:* v0.0.1 ships today. v0.1 = full FSRS + JMdict + review TUI. v0.4 = MCP server for AI agents.
6. *Call to action:* 20 starter issues, 4 starter decks waiting for owners. Link to repo.

---

## Channels & where to post

### Tier 1 (high reach, do once and well)

- **Hacker News** — Show HN
- **Lobsters** — `tag:practices` and `tag:rust`
- **r/rust** — flair "project"
- **r/LearnJapanese** — read sidebar first
- **r/commandline** — niche but engaged

### Tier 2 (sustained, do over a month)

- **r/japaneselearning** (less strict than r/LearnJapanese)
- **r/learnprogramming** (if angle is "study while coding")
- **r/dotfiles** — share your config with `kotoba_today_prompt`
- **r/neovim**, **r/vscode** (after editor plugins land)
- **this-week-in-rust** (issue submission)
- **Japanese tech Twitter** — tag in JP, link the JP README
- **Mastodon `fediverse.org` / `mastodon.social`** — `#rust` `#japanese` `#learning`
- **bsky.app** — devs are migrating
- **Daily.dev** — submit the launch post
- **Indie Hackers** — "I built X" thread (if/when revenue model exists; not yet)

### Tier 3 (specific communities; reach out personally)

- **Yomitan / jpdb.io Discords** — interop proposals
- **fsrs-rs maintainers** — say hi, share that you're using their work
- **Japanese-language teacher associations** — for educator personas
- **University CS departments with Japanese minors** — your study-abroad audience

### Don't bother

- Product Hunt (terminal tools rarely perform well)
- LinkedIn (wrong demographic)
- Cold DMs to influencers

---

## Metrics to track (and what they actually mean)

Keep a simple spreadsheet for week 1.

| Metric | "Good for week 1" | What it actually means |
|---|---|---|
| GitHub stars | 200–800 | Reach. Vanity, but useful early signal. |
| Forks | 20+ | Mild positive intent. |
| Issues opened | 20+ | People care enough to ask. |
| PRs opened | 3+ | Actual investment. **This is the metric that matters.** |
| Discord/Discussions sign-ups | 30+ | Community starting to form. |
| Repo `view` traffic | 1k+ unique | If high but PRs are 0, message is wrong. |
| Twitter follows | 50+ | Reach into the dev community. |
| Mentioned by another tool | any | Outsized signal; reach out and thank them. |

Anti-metric: HN score. It feels important on the day and matters very little after.

---

## When things go wrong

A few honest scenarios to mentally prepare for.

### "This already exists" (most common)

There are a lot of language tools. Acknowledge them. Be specific about what's different (local-first, plain text, terminal-native, BYO AI). Don't defend; differentiate.

### Someone finds a bug in the demo

Fix it within the day. Public, calm. "Thank you, fixed in [commit]." This is the *best* possible PR you could get on launch day.

### The HN post flops (≤ 50 points)

Normal. The post is one of many that day. Iterate on the angle, post to r/rust on Tuesday, write a longer-form blog post in week 2. Most great projects had a launch nobody noticed.

### Someone is unkind in a comment

Reply once, kindly, factually. Then disengage. Future you will thank present you.

### A contributor proposes something you don't want to build

"That's interesting — would you be willing to write an RFC in `docs/rfcs/`? I'm not sure I want it in core, but it might be a great plugin." Channel energy into the model the project supports rather than rejecting outright.

---

## Post-launch routine (after week 1)

The make-or-break is what happens in **weeks 2–8**, not launch day. A consistent rhythm:

### Daily (10 min)

- Skim notifications
- Reply to any new issue/PR with at least an acknowledgement, even if you can't deep-engage

### Weekly (2 hours, e.g. Sunday morning)

- Triage all open issues; label, link to related, ask clarifying questions
- Review pending PRs
- One small PR of your own (a deck, a doc, a fix) — to model contribution and stay in the codebase
- Post a "weekly progress" update in Discussions

### Monthly

- Cut a release, even a small one
- Write a "what shipped this month" post
- Identify the 3 most active contributors and thank them publicly
- Re-read `docs/roadmap.md`; update if reality has shifted

### Quarterly

- Roadmap review (RFC-driven changes if needed)
- Consider nominating a co-maintainer if anyone has earned it
- A small retrospective with the maintainer team

---

## The thing that actually matters

Reply within 24 hours to every issue and PR for the first 90 days. That's it. That's the moat.

Communities collapse when maintainers go silent. They flourish when contributors feel seen. You are the maintainer. Be present.
