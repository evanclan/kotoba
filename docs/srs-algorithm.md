# Spaced repetition: how Kotoba schedules reviews

## TL;DR

Kotoba schedules reviews using **FSRS** (Free Spaced Repetition Scheduler) — the same modern algorithm Anki adopted in 2023, an open replacement for the SM-2 family. We treat the algorithm as a **trait** so it can be swapped, upgraded, or experimented with — all behind feature flags, all backed by tests.

## What is spaced repetition?

The simple idea: review information just before you forget it. Each successful recall stretches the gap further. Each lapse shortens it. Done daily, this turns 5 minutes of effort into long-term retention.

## What is FSRS?

FSRS models each card as a 3-tuple **(difficulty, stability, retrievability)**:

- **Difficulty (D)** — how intrinsically hard this card is for *you*. Updated on every review.
- **Stability (S)** — how many days the card stays at >90% recall probability. Grows with successful reviews, shrinks with lapses.
- **Retrievability (R)** — the probability you'll recall the card *now*, given S and time-since-last-review.

Reviews are scheduled to hit a target retrievability (default 0.9 = 90%). The math is well-documented; see the [FSRS papers](https://github.com/open-spaced-repetition/fsrs4anki/wiki).

## Why FSRS, not SM-2?

| | SM-2 (Anki classic) | FSRS |
|---|---|---|
| Era | 1980s | 2022+, actively researched |
| Personalization | Fixed parameters | Per-deck or per-user weight optimization |
| Lapse handling | Crude | Models stability decay properly |
| Empirical performance | Worse | ~30% fewer reviews for same retention in published benchmarks |
| License | N/A (just an idea) | MIT-licensed reference implementations |

## How Kotoba uses it

```rust
// Pseudo-code; real types in kotoba-core::scheduler.
trait Scheduler {
    fn next_review(&self, card: &Card, grade: Grade, now: DateTime<Utc>) -> ReviewResult;
}

struct ReviewResult {
    next_due: DateTime<Utc>,
    new_state: CardState,        // (D, S) snapshot
    interval_days: f64,
}
```

`kotoba-core` ships with `FsrsScheduler` as the default. A `Sm2Scheduler` exists for benchmarking and for users importing decks where SM-2 history must be preserved.

Future schedulers (research, personalized, multi-objective) will live behind feature flags:

```toml
[dependencies]
kotoba-core = { version = "0.1", features = ["scheduler-experimental"] }
```

## Grade scale

Kotoba uses a **4-button grade scale**:

| Key | Grade   | Meaning                                                        |
|-----|---------|----------------------------------------------------------------|
| `1` | Again   | I forgot.                                                      |
| `2` | Hard    | I remembered, but it took effort or I almost missed it.        |
| `3` | Good    | I remembered, normal effort.                                   |
| `4` | Easy    | I remembered instantly. Push the next interval out.            |

This matches FSRS reference implementations. Some users prefer a 2-button scale (Anki's "again / good"); a config flag will support that in v0.2.

## Parameter tuning

Out of the box, FSRS uses default weights tuned on a large public corpus. Once a user has accumulated enough reviews (~1,000), `kotoba scheduler tune` will optimize personal weights against their own review log. This is opt-in and runs locally — no data leaves the machine.

## "Why am I seeing this card again?"

Every card carries an audit trail. `kotoba why <card-id>` (planned for v0.2) prints:

```
Card: 留学
Last reviewed: 2026-05-15 (5 days ago)
Last grade: Good
Stability: 4.2 days
Difficulty: 5.6 / 10
Predicted retrievability today: 0.31
Target retrievability: 0.9
Decision: due, scheduling now.
```

This transparency is non-negotiable. Users — and especially educators — should be able to inspect the algorithm.

## Caveats

- FSRS is a model, not magic. It assumes you grade honestly.
- FSRS does not understand semantic similarity between cards. "留学" and "海外留学" are scheduled independently. We may add similarity-aware scheduling later as a research feature.
- FSRS optimizes for retention. It does not directly optimize for *speed of acquisition* (how fast you learn new material). A separate "learning rate" subsystem may be added in v0.4+.

## References

- FSRS papers and reference implementation: https://github.com/open-spaced-repetition
- Kotoba's implementation: [`crates/kotoba-core/src/srs.rs`](../crates/kotoba-core/src/srs.rs)
- Discussion: [GitHub Discussions tag `srs`](../../discussions)
