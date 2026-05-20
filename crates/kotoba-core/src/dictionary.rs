//! Dictionary lookup abstractions.
//!
//! The default Kotoba binary will ship with a JMdict-backed dictionary. This
//! crate intentionally only defines the trait — the concrete implementation
//! lives in `kotoba-cli` (and the daemon) so that `kotoba-core` stays I/O-free
//! and easy to test.

use crate::error::Result;
use serde::{Deserialize, Serialize};

/// One entry returned by a dictionary lookup.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DictionaryEntry {
    /// The headword (the form usually written, e.g. "留学").
    pub term: String,
    /// Phonetic reading (e.g. "りゅうがく"), if applicable.
    pub reading: Option<String>,
    /// One or more meanings, in order of frequency.
    pub meanings: Vec<String>,
    /// Parts of speech, e.g. ["noun", "suru-verb"].
    pub pos: Vec<String>,
    /// JLPT level if known ("N5".."N1").
    pub jlpt: Option<String>,
    /// Optional frequency rank (1 = most common).
    pub frequency_rank: Option<u32>,
    /// Example sentences, native + gloss pairs.
    pub examples: Vec<DictionaryExample>,
}

/// A single example sentence.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DictionaryExample {
    /// Native-language sentence.
    pub native: String,
    /// Translation / gloss in the user's reference language.
    pub gloss: String,
}

/// A pluggable dictionary backend.
pub trait Dictionary: Send + Sync {
    /// Look up a term and return any matching entries. May return an empty
    /// vector if nothing matched. Errors are reserved for backend failures
    /// (network, disk, parse), not for "not found".
    fn lookup(&self, term: &str) -> Result<Vec<DictionaryEntry>>;
}

/// A trivial in-memory dictionary, useful for tests and the v0.0.1 demo.
///
/// Real builds use a JMdict-backed implementation in `kotoba-cli`.
#[derive(Debug, Default, Clone)]
pub struct InMemoryDictionary {
    entries: Vec<DictionaryEntry>,
}

impl InMemoryDictionary {
    /// Construct an empty dictionary.
    pub fn new() -> Self {
        Self::default()
    }

    /// Insert one entry.
    pub fn insert(&mut self, entry: DictionaryEntry) {
        self.entries.push(entry);
    }

    /// Build from a static list of entries.
    pub fn with_entries<I: IntoIterator<Item = DictionaryEntry>>(entries: I) -> Self {
        Self {
            entries: entries.into_iter().collect(),
        }
    }
}

impl Dictionary for InMemoryDictionary {
    fn lookup(&self, term: &str) -> Result<Vec<DictionaryEntry>> {
        let term_lower = term.to_lowercase();
        Ok(self
            .entries
            .iter()
            .filter(|e| {
                e.term == term
                    || e.reading.as_deref() == Some(term)
                    || e.term.to_lowercase() == term_lower
                    || e.meanings.iter().any(|m| m.to_lowercase() == term_lower)
            })
            .cloned()
            .collect())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fixture() -> InMemoryDictionary {
        InMemoryDictionary::with_entries([DictionaryEntry {
            term: "留学".to_string(),
            reading: Some("りゅうがく".to_string()),
            meanings: vec!["studying abroad".to_string()],
            pos: vec!["noun".to_string(), "suru-verb".to_string()],
            jlpt: Some("N3".to_string()),
            frequency_rank: Some(4212),
            examples: vec![],
        }])
    }

    #[test]
    fn lookup_by_kanji() {
        let dict = fixture();
        let res = dict.lookup("留学").unwrap();
        assert_eq!(res.len(), 1);
    }

    #[test]
    fn lookup_by_kana() {
        let dict = fixture();
        let res = dict.lookup("りゅうがく").unwrap();
        assert_eq!(res.len(), 1);
    }

    #[test]
    fn lookup_by_meaning() {
        let dict = fixture();
        let res = dict.lookup("studying abroad").unwrap();
        assert_eq!(res.len(), 1);
    }

    #[test]
    fn lookup_miss_returns_empty() {
        let dict = fixture();
        let res = dict.lookup("nonsense-term").unwrap();
        assert!(res.is_empty());
    }
}
