//! Card-level types: [`Card`], [`CardId`], [`Grade`], and [`Review`].

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::fmt;

/// A stable, content-addressed identifier for a card.
///
/// IDs are derived from `sha256(deck_slug || "\n" || front)`. This means renaming
/// or moving a card *within* a deck is fine, but changing the front text creates
/// a new logical card. The collision domain is per-deck.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct CardId(String);

impl CardId {
    /// Create a `CardId` by hashing the given deck slug and card front.
    pub fn from_parts(deck_slug: &str, front: &str) -> Self {
        let mut hasher = Sha256::new();
        hasher.update(deck_slug.as_bytes());
        hasher.update(b"\n");
        hasher.update(front.as_bytes());
        let bytes = hasher.finalize();
        Self(format!("sha256:{:x}", bytes))
    }

    /// Construct from an explicit string. Use sparingly — prefer `from_parts`.
    pub fn from_string(s: impl Into<String>) -> Self {
        Self(s.into())
    }

    /// Returns the underlying string representation.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for CardId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

/// A flashcard, the atomic unit of learning content.
///
/// `front` and `back` are markdown strings. The renderer (TUI, web, plugin) is
/// responsible for layout — the engine treats them as opaque text.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Card {
    /// Stable ID (see [`CardId`]).
    pub id: CardId,
    /// Slug of the deck this card belongs to.
    pub deck_slug: String,
    /// Front of the card (the prompt).
    pub front: String,
    /// Back of the card (the answer). May be empty for cards where the front
    /// alone is enough to grade against.
    pub back: String,
    /// Optional reading (kana / pronunciation).
    pub reading: Option<String>,
    /// Free-form tags.
    pub tags: Vec<String>,
    /// Optional contextual metadata: where the user encountered the term.
    pub context: Option<String>,
    /// Scheduling state owned by the active [`crate::Scheduler`].
    pub state: CardState,
    /// When the card was created.
    pub created: DateTime<Utc>,
}

impl Card {
    /// Create a new card with default scheduler state and the current timestamp.
    pub fn new(deck_slug: impl Into<String>, front: impl Into<String>) -> Self {
        let deck_slug = deck_slug.into();
        let front = front.into();
        let id = CardId::from_parts(&deck_slug, &front);
        Self {
            id,
            deck_slug,
            front,
            back: String::new(),
            reading: None,
            tags: Vec::new(),
            context: None,
            state: CardState::default(),
            created: Utc::now(),
        }
    }

    /// Builder helper.
    pub fn with_reading(mut self, reading: impl Into<String>) -> Self {
        self.reading = Some(reading.into());
        self
    }

    /// Builder helper.
    pub fn with_back(mut self, back: impl Into<String>) -> Self {
        self.back = back.into();
        self
    }

    /// Builder helper.
    pub fn with_tags<I, S>(mut self, tags: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.tags = tags.into_iter().map(Into::into).collect();
        self
    }
}

/// Scheduler-owned state attached to every card.
///
/// The fields here describe the FSRS-style (difficulty, stability, last_review,
/// next_due) tuple. Schedulers that don't use FSRS may ignore the FSRS-specific
/// fields and rely on `next_due` alone.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CardState {
    /// FSRS difficulty (1–10). 5.0 is "average new card".
    pub difficulty: f64,
    /// FSRS stability in days.
    pub stability: f64,
    /// Total successful reviews. New cards = 0.
    pub reps: u32,
    /// Total lapses (graded `Again`).
    pub lapses: u32,
    /// Last time the card was reviewed (any grade), if ever.
    pub last_review: Option<DateTime<Utc>>,
    /// Next time the card is due to be shown.
    pub next_due: DateTime<Utc>,
}

impl Default for CardState {
    fn default() -> Self {
        Self {
            difficulty: 5.0,
            stability: 0.0,
            reps: 0,
            lapses: 0,
            last_review: None,
            next_due: Utc::now(),
        }
    }
}

/// User grade for a single review.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Grade {
    /// I forgot. Reset interval.
    Again,
    /// I remembered, but with effort.
    Hard,
    /// I remembered, normal effort.
    Good,
    /// I remembered instantly. Push the next interval out.
    Easy,
}

impl Grade {
    /// Numeric value used by FSRS-family schedulers.
    pub fn as_u8(self) -> u8 {
        match self {
            Grade::Again => 1,
            Grade::Hard => 2,
            Grade::Good => 3,
            Grade::Easy => 4,
        }
    }

    /// Inverse of [`Grade::as_u8`].
    pub fn from_u8(v: u8) -> Option<Self> {
        Some(match v {
            1 => Grade::Again,
            2 => Grade::Hard,
            3 => Grade::Good,
            4 => Grade::Easy,
            _ => return None,
        })
    }
}

/// One review event, append-only.
///
/// Reviews are written to `~/.kotoba/reviews/YYYY/MM.jsonl`. The card's
/// scheduling state is derivable from a card's full review history alone, which
/// makes sync conflict-free and enables verifiable progress portfolios.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Review {
    /// When the review happened (UTC).
    pub timestamp: DateTime<Utc>,
    /// Card that was reviewed.
    pub card_id: CardId,
    /// User grade.
    pub grade: Grade,
    /// Where the review came from: "cli", "vscode", "browser", "mcp", etc.
    pub context: String,
    /// Interval (in days) the scheduler picked as the next gap.
    pub interval_days: f64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn card_id_is_stable() {
        let a = CardId::from_parts("personal", "留学");
        let b = CardId::from_parts("personal", "留学");
        assert_eq!(a, b);
    }

    #[test]
    fn card_id_is_deck_scoped() {
        let a = CardId::from_parts("personal", "留学");
        let b = CardId::from_parts("jlpt-n3", "留学");
        assert_ne!(a, b);
    }

    #[test]
    fn grade_round_trips() {
        for g in [Grade::Again, Grade::Hard, Grade::Good, Grade::Easy] {
            assert_eq!(Grade::from_u8(g.as_u8()), Some(g));
        }
    }
}
