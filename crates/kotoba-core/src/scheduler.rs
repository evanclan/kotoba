//! Spaced-repetition scheduler trait and a small FSRS-derived default.
//!
//! The default scheduler in this v0.0.1 reference implementation is a
//! **simplified FSRS-derived stability/difficulty update**. It is *not* the
//! full reference FSRS implementation; the real one will land in v0.1 with
//! optimized weights and parameter tuning. The shape of the math is correct
//! and the trait is stable — implementations behind feature flags can replace
//! the default freely.
//!
//! See [`docs/srs-algorithm.md`](../../docs/srs-algorithm.md) for background.

use crate::card::{Card, CardState, Grade};
use chrono::{DateTime, Duration, Utc};

/// A pluggable scheduler. Implementors must be deterministic given the inputs.
pub trait Scheduler: Send + Sync {
    /// Compute the new [`CardState`] for `card` given a `grade` recorded `now`.
    /// Returns the new state and the chosen interval in days.
    fn schedule(&self, card: &Card, grade: Grade, now: DateTime<Utc>) -> ScheduleResult;
}

/// Output of a single scheduling decision.
#[derive(Debug, Clone)]
pub struct ScheduleResult {
    /// The updated card state to persist.
    pub state: CardState,
    /// The interval (in days, fractional) before the next review.
    pub interval_days: f64,
}

/// Simplified FSRS-derived scheduler. Default for v0.0.1.
///
/// **Not** the full FSRS reference implementation. Good enough to ship and
/// iterate against; will be replaced by the full FSRS port in v0.1.
#[derive(Debug, Clone, Copy)]
pub struct Fsrs {
    /// Target retrievability. Default 0.9 = 90% recall probability.
    pub retention: f64,
}

impl Default for Fsrs {
    fn default() -> Self {
        Self { retention: 0.9 }
    }
}

impl Fsrs {
    /// New scheduler with a custom target retention.
    pub fn with_retention(retention: f64) -> Self {
        Self { retention }
    }

    fn update_difficulty(d: f64, grade: Grade) -> f64 {
        let delta = match grade {
            Grade::Again => 1.0,
            Grade::Hard => 0.5,
            Grade::Good => 0.0,
            Grade::Easy => -0.3,
        };
        (d + delta).clamp(1.0, 10.0)
    }

    fn update_stability(s: f64, d: f64, grade: Grade) -> f64 {
        let factor = match grade {
            Grade::Again => 0.0,
            Grade::Hard => 1.2,
            Grade::Good => 2.5,
            Grade::Easy => 4.0,
        };
        let difficulty_penalty = 1.0 - (d - 5.0) * 0.05;
        if s == 0.0 {
            // First review: stability seeded by grade.
            match grade {
                Grade::Again => 0.5,
                Grade::Hard => 1.0,
                Grade::Good => 3.0,
                Grade::Easy => 7.0,
            }
        } else {
            (s * factor * difficulty_penalty).max(0.5)
        }
    }
}

impl Scheduler for Fsrs {
    fn schedule(&self, card: &Card, grade: Grade, now: DateTime<Utc>) -> ScheduleResult {
        let mut state = card.state.clone();

        let new_difficulty = Self::update_difficulty(state.difficulty, grade);
        let new_stability = Self::update_stability(state.stability, new_difficulty, grade);

        // FSRS convention: at the canonical 0.9 retention target, interval = stability.
        // For other targets, scale by ln(target) / ln(0.9). Stricter retention
        // shrinks the interval; more relaxed retention grows it.
        let canonical_ln = 0.9_f64.ln();
        let interval_days = (new_stability * (self.retention.ln() / canonical_ln)).max(0.5);

        state.difficulty = new_difficulty;
        state.stability = new_stability;
        state.last_review = Some(now);
        state.next_due = now + Duration::milliseconds((interval_days * 86_400_000.0) as i64);

        if matches!(grade, Grade::Again) {
            state.lapses += 1;
        } else {
            state.reps += 1;
        }

        ScheduleResult {
            state,
            interval_days,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::card::Card;

    fn fresh_card() -> Card {
        Card::new("test", "留学")
    }

    #[test]
    fn first_good_review_pushes_due_into_future() {
        let card = fresh_card();
        let now = Utc::now();
        let result = Fsrs::default().schedule(&card, Grade::Good, now);
        assert!(result.interval_days > 0.5);
        assert!(result.state.next_due > now);
    }

    #[test]
    fn again_increments_lapses() {
        let card = fresh_card();
        let now = Utc::now();
        let result = Fsrs::default().schedule(&card, Grade::Again, now);
        assert_eq!(result.state.lapses, 1);
        assert_eq!(result.state.reps, 0);
    }

    #[test]
    fn easy_pushes_further_than_good() {
        let card = fresh_card();
        let now = Utc::now();
        let easy = Fsrs::default()
            .schedule(&card, Grade::Easy, now)
            .interval_days;
        let good = Fsrs::default()
            .schedule(&card, Grade::Good, now)
            .interval_days;
        assert!(easy > good, "easy ({easy}) should exceed good ({good})");
    }

    #[test]
    fn difficulty_stays_in_range() {
        let mut card = fresh_card();
        card.state.difficulty = 5.0;
        let result = Fsrs::default().schedule(&card, Grade::Again, Utc::now());
        assert!(result.state.difficulty >= 1.0 && result.state.difficulty <= 10.0);
    }
}
