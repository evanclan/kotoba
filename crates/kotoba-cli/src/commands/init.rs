//! `kotoba init` — create the data directory and seed a starter deck.

use crate::store_fs;
use anyhow::Result;
use clap::Parser;
use std::path::Path;

#[derive(Debug, Parser)]
pub struct Args {}

pub fn run(home: &Path, _args: Args) -> Result<()> {
    store_fs::ensure_personal_deck(home)?;
    println!("✓ initialized Kotoba at {}", home.display());
    println!();
    println!("Next steps:");
    println!("  kotoba lookup ありがとう");
    println!("  kotoba add 留学");
    println!("  kotoba today");
    Ok(())
}
