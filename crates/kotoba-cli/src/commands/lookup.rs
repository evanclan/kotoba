//! `kotoba lookup <term>` — dictionary lookup.

use crate::dictionary_demo;
use anyhow::Result;
use clap::Parser;
use kotoba_core::dictionary::Dictionary;
use std::path::Path;

#[derive(Debug, Parser)]
pub struct Args {
    /// The term to look up. Accepts kanji, kana, or English.
    pub term: String,

    /// Output as JSON instead of formatted text.
    #[arg(long)]
    pub json: bool,
}

pub fn run(_home: &Path, args: Args) -> Result<()> {
    let dict = dictionary_demo::demo_dictionary();
    let entries = dict.lookup(&args.term)?;

    if args.json {
        println!("{}", serde_json::to_string_pretty(&entries)?);
        return Ok(());
    }

    if entries.is_empty() {
        println!("no entries found for '{}'.", args.term);
        println!();
        println!("(v0.0.1 ships with a tiny demo dictionary. Full JMdict lands in v0.1.)");
        return Ok(());
    }

    for entry in &entries {
        println!("{}", entry.term);
        if let Some(reading) = &entry.reading {
            println!("  ({reading})");
        }
        if !entry.pos.is_empty() {
            println!("  {}", entry.pos.join(", "));
        }
        for meaning in &entry.meanings {
            println!("    {meaning}");
        }
        if let Some(level) = &entry.jlpt {
            println!("  JLPT {level}");
        }
        if let Some(rank) = entry.frequency_rank {
            println!("  frequency rank: {rank}");
        }
        if !entry.examples.is_empty() {
            println!("  examples:");
            for ex in &entry.examples {
                println!("    {}", ex.native);
                println!("      {}", ex.gloss);
            }
        }
        println!();
    }
    Ok(())
}
