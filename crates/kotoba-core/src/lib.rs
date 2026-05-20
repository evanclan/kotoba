//! # kotoba-core
//!
//! The Kotoba engine. This crate is **pure logic with no I/O of its own** — except
//! through the [`Store`] and [`Dictionary`] traits, which callers (typically
//! `kotoba-cli` or `kotoba-daemon`) implement against the filesystem, an HTTP
//! API, or a test harness.
//!
//! ## Quick tour
//!
//! - [`Card`] / [`Deck`] / [`Note`] / [`Review`] — the domain types.
//! - [`Store`] — the persistence abstraction (`fs`, in-memory, future: cloud).
//! - [`Dictionary`] — the lookup abstraction (JMdict, custom, AI-backed).
//! - [`Scheduler`] — spaced-repetition scheduler trait. The default
//!   implementation is FSRS-derived; see [`scheduler::Fsrs`].
//! - [`Engine`] — orchestrator that ties storage, dictionary, and scheduler together.
//!
//! ## Stability
//!
//! Until v1.0, the public API of `kotoba-core` is subject to change behind a
//! semver minor bump. We document breaking changes in the workspace
//! `CHANGELOG.md`.

#![deny(missing_docs)]
#![warn(rust_2018_idioms)]

pub mod card;
pub mod deck;
pub mod dictionary;
pub mod engine;
pub mod error;
pub mod scheduler;
pub mod store;

pub use card::{Card, CardId, Grade, Review};
pub use deck::{Deck, DeckMeta};
pub use dictionary::{Dictionary, DictionaryEntry};
pub use engine::Engine;
pub use error::{Error, Result};
pub use scheduler::Scheduler;
pub use store::Store;

/// The semver version of `kotoba-core`, populated from `Cargo.toml`.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
