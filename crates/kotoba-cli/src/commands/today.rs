//! `kotoba today` — print the word of the day for shell prompts and status bars.

use crate::store_fs;
use anyhow::Result;
use chrono::{Datelike, Utc};
use clap::Parser;
use kotoba_core::store::Store;
use std::path::Path;

#[derive(Debug, Parser)]
pub struct Args {
    /// Output as JSON.
    #[arg(long)]
    pub json: bool,
}

pub fn run(home: &Path, args: Args) -> Result<()> {
    let store = store_fs::FsStore::new(home.to_path_buf());

    let mut all_cards = Vec::new();
    for slug in store.list_decks()? {
        let deck = store.load_deck(&slug)?;
        all_cards.extend(deck.cards);
    }

    if all_cards.is_empty() {
        if args.json {
            println!("{{\"empty\": true}}");
        } else {
            println!("(no cards yet — try `kotoba add <word>` to add one)");
        }
        return Ok(());
    }

    let today = Utc::now();
    let day_of_year = today.ordinal() as usize;
    let card = &all_cards[day_of_year % all_cards.len()];
    let due_count = all_cards
        .iter()
        .filter(|c| c.state.next_due <= today)
        .count();

    if args.json {
        let payload = serde_json::json!({
            "term": card.front,
            "reading": card.reading,
            "back": card.back,
            "deck": card.deck_slug,
            "due": due_count,
        });
        println!("{}", payload);
    } else {
        let weekday = match today.weekday().number_from_monday() {
            1 => "月曜日",
            2 => "火曜日",
            3 => "水曜日",
            4 => "木曜日",
            5 => "金曜日",
            6 => "土曜日",
            _ => "日曜日",
        };
        let reading = card.reading.as_deref().unwrap_or("");
        let gloss: &str = if card.back.is_empty() {
            ""
        } else {
            card.back.as_str()
        };
        let due_part = if due_count > 0 {
            format!("   {due_count} reviews due")
        } else {
            String::new()
        };
        println!("[{weekday}] {} ({reading}) — {gloss}{due_part}", card.front);
    }
    Ok(())
}
