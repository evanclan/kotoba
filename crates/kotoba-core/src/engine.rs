//! High-level orchestration on top of [`Store`], [`Dictionary`], and [`Scheduler`].

use crate::card::{Card, CardId, Grade, Review};
use crate::deck::Deck;
use crate::dictionary::{Dictionary, DictionaryEntry};
use crate::error::Result;
use crate::scheduler::Scheduler;
use crate::store::Store;
use chrono::{DateTime, Utc};
use std::sync::Arc;

/// The top-level Kotoba engine. Built once at startup and shared across surfaces.
pub struct Engine {
    store: Arc<dyn Store>,
    dictionary: Arc<dyn Dictionary>,
    scheduler: Arc<dyn Scheduler>,
}

impl Engine {
    /// Construct a new engine wiring the given backends.
    pub fn new(
        store: Arc<dyn Store>,
        dictionary: Arc<dyn Dictionary>,
        scheduler: Arc<dyn Scheduler>,
    ) -> Self {
        Self { store, dictionary, scheduler }
    }

    /// Look up a term in the configured dictionary.
    pub fn lookup(&self, term: &str) -> Result<Vec<DictionaryEntry>> {
        self.dictionary.lookup(term)
    }

    /// List all decks.
    pub fn decks(&self) -> Result<Vec<String>> {
        self.store.list_decks()
    }

    /// Load a deck.
    pub fn deck(&self, slug: &str) -> Result<Deck> {
        self.store.load_deck(slug)
    }

    /// Add a card to a deck and persist the deck. Returns the new card.
    pub fn add_card(&self, deck_slug: &str, mut card: Card) -> Result<Card> {
        let mut deck = self.store.load_deck(deck_slug)?;
        card.deck_slug = deck.meta.slug.clone();
        card.id = CardId::from_parts(&deck.meta.slug, &card.front);
        deck.push(card.clone());
        self.store.save_deck(&deck)?;
        Ok(card)
    }

    /// Cards across all decks that are due at or before `now`.
    pub fn due_cards(&self, now: DateTime<Utc>, limit: usize) -> Result<Vec<Card>> {
        let mut due = Vec::new();
        for slug in self.store.list_decks()? {
            let deck = self.store.load_deck(&slug)?;
            for card in deck.cards.into_iter().filter(|c| c.state.next_due <= now) {
                due.push(card);
                if due.len() >= limit {
                    break;
                }
            }
            if due.len() >= limit {
                break;
            }
        }
        due.sort_by(|a, b| a.state.next_due.cmp(&b.state.next_due));
        Ok(due)
    }

    /// Record a review for `card_id`. Updates the card's scheduling state in
    /// its deck and appends a [`Review`] to the log.
    pub fn record_review(
        &self,
        card_id: &CardId,
        grade: Grade,
        context: impl Into<String>,
    ) -> Result<Review> {
        let now = Utc::now();
        let card = self.store.find_card(card_id)?;
        let result = self.scheduler.schedule(&card, grade, now);

        let mut deck = self.store.load_deck(&card.deck_slug)?;
        if let Some(c) = deck.cards.iter_mut().find(|c| c.id == card.id) {
            c.state = result.state.clone();
        }
        self.store.save_deck(&deck)?;

        let review = Review {
            timestamp: now,
            card_id: card.id.clone(),
            grade,
            context: context.into(),
            interval_days: result.interval_days,
        };
        self.store.append_review(&review)?;
        Ok(review)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::deck::{Deck, DeckMeta};
    use crate::dictionary::InMemoryDictionary;
    use crate::scheduler::Fsrs;
    use crate::store::MemoryStore;

    fn make_engine() -> Engine {
        let store = Arc::new(MemoryStore::new());
        let mut deck = Deck::new(DeckMeta::minimal("Personal", "personal"));
        deck.push(Card::new("personal", "留学"));
        store.save_deck(&deck).unwrap();
        Engine::new(store, Arc::new(InMemoryDictionary::new()), Arc::new(Fsrs::default()))
    }

    #[test]
    fn add_card_persists_and_returns_id() {
        let engine = make_engine();
        let card = Card::new("personal", "食卓").with_reading("しょくたく");
        let saved = engine.add_card("personal", card).unwrap();
        assert_eq!(saved.deck_slug, "personal");
        let deck = engine.deck("personal").unwrap();
        assert_eq!(deck.cards.len(), 2);
    }

    #[test]
    fn record_review_updates_state_and_logs() {
        let engine = make_engine();
        let due = engine.due_cards(Utc::now(), 10).unwrap();
        let id = due[0].id.clone();
        let review = engine.record_review(&id, Grade::Good, "test").unwrap();
        assert!(review.interval_days > 0.0);
    }
}
