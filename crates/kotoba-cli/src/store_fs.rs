//! Filesystem-backed implementation of [`kotoba_core::Store`].
//!
//! Decks live as markdown files in `<home>/decks/<slug>.md`. Reviews are
//! appended to `<home>/reviews/YYYY/MM.jsonl`.

use anyhow::Context;
use kotoba_core::card::{Card, CardId, Review};
use kotoba_core::deck::{self, Deck};
use kotoba_core::error::{Error, Result};
use kotoba_core::store::Store;
use std::fs;
use std::io::Write;
use std::path::PathBuf;

/// Filesystem store rooted at a Kotoba data directory.
pub struct FsStore {
    home: PathBuf,
}

impl FsStore {
    /// Construct a store rooted at `home`.
    pub fn new(home: PathBuf) -> Self {
        Self { home }
    }

    fn decks_dir(&self) -> PathBuf {
        self.home.join("decks")
    }

    fn deck_path(&self, slug: &str) -> PathBuf {
        self.decks_dir().join(format!("{slug}.md"))
    }

    fn reviews_path(&self, ts: chrono::DateTime<chrono::Utc>) -> PathBuf {
        let year = ts.format("%Y").to_string();
        let month = ts.format("%m").to_string();
        self.home.join("reviews").join(year).join(format!("{month}.jsonl"))
    }
}

impl Store for FsStore {
    fn list_decks(&self) -> Result<Vec<String>> {
        let dir = self.decks_dir();
        if !dir.exists() {
            return Ok(Vec::new());
        }
        let mut slugs = Vec::new();
        let entries = fs::read_dir(&dir)
            .map_err(|e| Error::Store(format!("read_dir {}: {e}", dir.display())))?;
        for entry in entries {
            let entry = entry.map_err(|e| Error::Store(format!("dir entry: {e}")))?;
            let path = entry.path();
            if path.extension().and_then(|s| s.to_str()) == Some("md") {
                if let Some(stem) = path.file_stem().and_then(|s| s.to_str()) {
                    slugs.push(stem.to_string());
                }
            }
        }
        slugs.sort();
        Ok(slugs)
    }

    fn load_deck(&self, slug: &str) -> Result<Deck> {
        let path = self.deck_path(slug);
        let contents = fs::read_to_string(&path).map_err(|e| {
            if e.kind() == std::io::ErrorKind::NotFound {
                Error::DeckNotFound(slug.to_string())
            } else {
                Error::Store(format!("reading {}: {e}", path.display()))
            }
        })?;
        deck::parse_markdown(&contents, slug)
    }

    fn save_deck(&self, deck: &Deck) -> Result<()> {
        let dir = self.decks_dir();
        fs::create_dir_all(&dir)
            .map_err(|e| Error::Store(format!("mkdir {}: {e}", dir.display())))?;
        let rendered = deck::to_markdown(deck)?;
        let path = self.deck_path(&deck.meta.slug);
        fs::write(&path, rendered)
            .map_err(|e| Error::Store(format!("writing {}: {e}", path.display())))?;
        Ok(())
    }

    fn append_review(&self, review: &Review) -> Result<()> {
        let path = self.reviews_path(review.timestamp);
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)
                .map_err(|e| Error::Store(format!("mkdir {}: {e}", parent.display())))?;
        }
        let line = serde_json::to_string(review)
            .map_err(|e| Error::Store(format!("serializing review: {e}")))?;
        let mut f = fs::OpenOptions::new()
            .append(true)
            .create(true)
            .open(&path)
            .map_err(|e| Error::Store(format!("opening {}: {e}", path.display())))?;
        writeln!(f, "{line}")
            .map_err(|e| Error::Store(format!("writing review: {e}")))?;
        Ok(())
    }

    fn find_card(&self, id: &CardId) -> Result<Card> {
        for slug in self.list_decks()? {
            let deck = self.load_deck(&slug)?;
            if let Some(c) = deck.cards.into_iter().find(|c| &c.id == id) {
                return Ok(c);
            }
        }
        Err(Error::CardNotFound(id.to_string()))
    }
}

/// Helper: ensure a default `personal.md` deck exists.
pub fn ensure_personal_deck(home: &std::path::Path) -> anyhow::Result<()> {
    let store = FsStore::new(home.to_path_buf());
    let slug = "personal";
    if store.load_deck(slug).is_ok() {
        return Ok(());
    }
    let meta = kotoba_core::deck::DeckMeta::minimal("Personal vocabulary", slug);
    let deck = Deck::new(meta);
    store.save_deck(&deck).context("creating starter deck")?;
    Ok(())
}
