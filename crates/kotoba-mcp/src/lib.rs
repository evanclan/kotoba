//! # kotoba-mcp
//!
//! Model Context Protocol server for the Kotoba engine. Exposes learner state
//! and tools to AI agents (Claude Desktop, Cursor, etc.).
//!
//! Status: **planned for v0.4.** This crate is scaffolding — see
//! `docs/ai-integration.md` for the design and
//! `docs/roadmap.md#v04--make-it-smart-without-making-it-dependent` for the
//! milestone.
//!
//! Planned resources:
//! - `kotoba://learner-state`
//! - `kotoba://due-cards`
//! - `kotoba://recent-mistakes`
//! - `kotoba://decks`
//! - `kotoba://decks/<slug>/cards`
//!
//! Planned tools:
//! - `lookup`
//! - `add_card`
//! - `record_review`
//! - `generate_example_sentence` (BYO LLM)
//! - `suggest_next_card`

#![warn(missing_docs)]

/// The semver version of `kotoba-mcp`.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Stub entry point that returns an explanatory message until the server is implemented.
pub fn placeholder_message() -> &'static str {
    "kotoba-mcp is scaffolded but not yet implemented. \
     See docs/ai-integration.md and docs/roadmap.md."
}
