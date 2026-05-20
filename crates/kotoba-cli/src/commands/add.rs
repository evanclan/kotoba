//! `kotoba add <term>` — append a card to a deck.

use crate::{dictionary_demo, store_fs};
use anyhow::{Context, Result};
use clap::Parser;
use kotoba_core::card::Card;
use kotoba_core::dictionary::Dictionary;
use kotoba_core::engine::Engine;
use kotoba_core::scheduler::Fsrs;
use kotoba_core::store::Store;
use std::path::Path;
use std::sync::Arc;

#[derive(Debug, Parser)]
pub struct Args {
    /// The term (front of the card).
    pub term: String,

    /// Deck slug to add to. Defaults to "personal".
    #[arg(long, default_value = "personal")]
    pub deck: String,

    /// Override the back text. By default we look up the term in the dictionary.
    #[arg(long)]
    pub back: Option<String>,

    /// Override the reading.
    #[arg(long)]
    pub reading: Option<String>,

    /// Add tags (repeatable).
    #[arg(long, value_delimiter = ',')]
    pub tag: Vec<String>,
}

pub fn run(home: &Path, args: Args) -> Result<()> {
    store_fs::ensure_personal_deck(home).context("ensuring default deck exists")?;

    let store: Arc<dyn Store> = Arc::new(store_fs::FsStore::new(home.to_path_buf()));
    let dict = Arc::new(dictionary_demo::demo_dictionary());
    let scheduler = Arc::new(Fsrs::default());
    let engine = Engine::new(store, dict.clone(), scheduler);

    let dict_entries = dict.lookup(&args.term)?;
    let dict_first = dict_entries.first();

    let back = args.back.unwrap_or_else(|| {
        dict_first
            .map(|e| e.meanings.join("; "))
            .unwrap_or_default()
    });
    let reading = args.reading.or_else(|| dict_first.and_then(|e| e.reading.clone()));

    let mut card = Card::new(&args.deck, &args.term).with_back(back);
    if let Some(r) = reading {
        card = card.with_reading(r);
    }
    if !args.tag.is_empty() {
        card = card.with_tags(args.tag);
    }

    let saved = engine.add_card(&args.deck, card)?;
    println!("✓ added {} to deck '{}'", saved.front, saved.deck_slug);
    if !saved.back.is_empty() {
        println!("    {}", saved.back);
    }
    Ok(())
}
