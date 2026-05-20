//! `kotoba decks` — list known decks and their card counts.

use crate::store_fs;
use anyhow::Result;
use kotoba_core::store::Store;
use std::path::Path;

pub fn run(home: &Path) -> Result<()> {
    let store = store_fs::FsStore::new(home.to_path_buf());
    let slugs = store.list_decks()?;
    if slugs.is_empty() {
        println!("(no decks yet — run `kotoba init`)");
        return Ok(());
    }
    println!("decks in {}:", home.display());
    for slug in slugs {
        match store.load_deck(&slug) {
            Ok(deck) => {
                println!(
                    "  {:24} {:>4} cards   {}",
                    slug,
                    deck.cards.len(),
                    deck.meta.name
                );
            }
            Err(e) => {
                println!("  {:24}   (failed to load: {})", slug, e);
            }
        }
    }
    Ok(())
}
