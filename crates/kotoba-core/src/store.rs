//! Storage abstraction.
//!
//! `kotoba-core` is I/O-free; concrete persistence lives in implementations of
//! the [`Store`] trait. The CLI and daemon ship a filesystem-backed `Store`;
//! tests use [`MemoryStore`].

use crate::card::{Card, CardId, Review};
use crate::deck::Deck;
use crate::error::{Error, Result};
use std::collections::HashMap;
use std::sync::RwLock;

/// Pluggable persistence for decks and the review log.
///
/// Implementations must be safe to call from a single thread; the engine
/// serializes access. Distributed implementations may add their own locking.
pub trait Store: Send + Sync {
    /// List the slugs of all known decks.
    fn list_decks(&self) -> Result<Vec<String>>;

    /// Load a deck by slug. Returns [`Error::DeckNotFound`] if unknown.
    fn load_deck(&self, slug: &str) -> Result<Deck>;

    /// Persist a deck (creates or overwrites).
    fn save_deck(&self, deck: &Deck) -> Result<()>;

    /// Append a review to the log.
    fn append_review(&self, review: &Review) -> Result<()>;

    /// Find a card by ID across all decks. Returns [`Error::CardNotFound`] if absent.
    fn find_card(&self, id: &CardId) -> Result<Card>;
}

/// In-memory `Store` for tests and the v0.0.1 demo.
#[derive(Debug, Default)]
pub struct MemoryStore {
    decks: RwLock<HashMap<String, Deck>>,
    reviews: RwLock<Vec<Review>>,
}

impl MemoryStore {
    /// Empty store.
    pub fn new() -> Self {
        Self::default()
    }

    /// Snapshot of all reviews recorded so far. Useful in tests.
    pub fn reviews(&self) -> Vec<Review> {
        self.reviews.read().expect("poisoned lock").clone()
    }
}

impl Store for MemoryStore {
    fn list_decks(&self) -> Result<Vec<String>> {
        Ok(self
            .decks
            .read()
            .expect("poisoned lock")
            .keys()
            .cloned()
            .collect())
    }

    fn load_deck(&self, slug: &str) -> Result<Deck> {
        self.decks
            .read()
            .expect("poisoned lock")
            .get(slug)
            .cloned()
            .ok_or_else(|| Error::DeckNotFound(slug.to_string()))
    }

    fn save_deck(&self, deck: &Deck) -> Result<()> {
        self.decks
            .write()
            .expect("poisoned lock")
            .insert(deck.meta.slug.clone(), deck.clone());
        Ok(())
    }

    fn append_review(&self, review: &Review) -> Result<()> {
        self.reviews
            .write()
            .expect("poisoned lock")
            .push(review.clone());
        Ok(())
    }

    fn find_card(&self, id: &CardId) -> Result<Card> {
        let decks = self.decks.read().expect("poisoned lock");
        for deck in decks.values() {
            if let Some(c) = deck.cards.iter().find(|c| &c.id == id) {
                return Ok(c.clone());
            }
        }
        Err(Error::CardNotFound(id.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::deck::DeckMeta;

    #[test]
    fn save_and_load_round_trips() {
        let store = MemoryStore::new();
        let mut deck = Deck::new(DeckMeta::minimal("Test", "test"));
        deck.push(Card::new("test", "留学"));
        store.save_deck(&deck).unwrap();

        let loaded = store.load_deck("test").unwrap();
        assert_eq!(loaded.cards.len(), 1);
    }

    #[test]
    fn missing_deck_errors() {
        let store = MemoryStore::new();
        assert!(matches!(
            store.load_deck("nope"),
            Err(Error::DeckNotFound(_))
        ));
    }
}
