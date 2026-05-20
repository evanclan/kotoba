//! The `kotoba` binary.
//!
//! This is the v0.0.1 reference implementation. See `docs/roadmap.md` for what
//! lands when. Today's commands are the smallest set that lets a user form a
//! daily habit:
//!
//! - `kotoba init`     — create `~/.kotoba/` and a starter deck.
//! - `kotoba lookup`   — dictionary lookup.
//! - `kotoba add`      — append a card to a deck.
//! - `kotoba today`    — word of the day for shell prompts.
//! - `kotoba decks`    — list known decks.

use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use std::process::ExitCode;

mod commands;
mod config;
mod dictionary_demo;
mod store_fs;

#[derive(Debug, Parser)]
#[command(
    name = "kotoba",
    version,
    about = "Local-first, terminal-native, scriptable language learning.",
    long_about = "Kotoba — your personal language-learning context layer.\n\
                  Decks live in plain markdown you own. AI is optional, BYO model.\n\
                  Docs: https://github.com/your-org/kotoba"
)]
struct Cli {
    /// Override the data directory (default: ~/.kotoba).
    #[arg(long, env = "KOTOBA_HOME", global = true)]
    home: Option<std::path::PathBuf>,

    /// Set log level (trace, debug, info, warn, error).
    #[arg(long, env = "KOTOBA_LOG", global = true, default_value = "warn")]
    log: String,

    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    /// Initialize the data directory with a starter deck.
    Init(commands::init::Args),
    /// Look up a term in the dictionary.
    Lookup(commands::lookup::Args),
    /// Add a card to a deck.
    Add(commands::add::Args),
    /// Print the word of the day for use in a shell prompt.
    Today(commands::today::Args),
    /// List known decks.
    Decks,
}

fn main() -> ExitCode {
    let cli = Cli::parse();

    if let Err(e) = init_logging(&cli.log) {
        eprintln!("kotoba: failed to initialize logging: {e}");
        return ExitCode::from(2);
    }

    match run(cli) {
        Ok(()) => ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("kotoba: error: {err:#}");
            ExitCode::FAILURE
        }
    }
}

fn run(cli: Cli) -> Result<()> {
    let home = config::resolve_home(cli.home.as_deref())
        .context("resolving Kotoba data directory")?;

    match cli.command {
        Command::Init(args) => commands::init::run(&home, args),
        Command::Lookup(args) => commands::lookup::run(&home, args),
        Command::Add(args) => commands::add::run(&home, args),
        Command::Today(args) => commands::today::run(&home, args),
        Command::Decks => commands::decks::run(&home),
    }
}

fn init_logging(level: &str) -> Result<()> {
    use tracing_subscriber::EnvFilter;
    let filter = EnvFilter::try_new(format!("kotoba={level}"))
        .context("invalid log filter")?;
    tracing_subscriber::fmt()
        .with_env_filter(filter)
        .with_target(false)
        .without_time()
        .with_writer(std::io::stderr)
        .try_init()
        .map_err(|e| anyhow::anyhow!("logging init: {e}"))?;
    Ok(())
}
