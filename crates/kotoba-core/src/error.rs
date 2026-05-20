//! Error types for `kotoba-core`.

use thiserror::Error;

/// Result alias used throughout `kotoba-core`.
pub type Result<T> = std::result::Result<T, Error>;

/// All errors that the Kotoba engine can produce.
///
/// Variants are intentionally specific so callers can match on them rather
/// than parsing strings.
#[derive(Debug, Error)]
pub enum Error {
    /// A card with the given ID was not found.
    #[error("card not found: {0}")]
    CardNotFound(String),

    /// A deck with the given slug was not found.
    #[error("deck not found: {0}")]
    DeckNotFound(String),

    /// The store returned an I/O error or a backend-specific failure.
    #[error("store error: {0}")]
    Store(String),

    /// Dictionary lookup failed for the given term.
    #[error("dictionary lookup failed for '{term}': {reason}")]
    Lookup {
        /// The term that was searched.
        term: String,
        /// Backend-specific reason.
        reason: String,
    },

    /// A deck file failed to parse.
    #[error("invalid deck file: {0}")]
    InvalidDeck(String),

    /// Configuration was missing or invalid.
    #[error("configuration error: {0}")]
    Config(String),

    /// Anything else wrapped from `anyhow` at the engine boundary.
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}
