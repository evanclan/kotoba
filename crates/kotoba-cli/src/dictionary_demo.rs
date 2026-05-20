//! Tiny built-in demo dictionary so v0.0.1 has something to look up out of the box.
//!
//! In v0.1 this is replaced by a JMdict-backed dictionary loaded from disk.

use kotoba_core::dictionary::{DictionaryEntry, DictionaryExample, InMemoryDictionary};

/// Build a small in-memory dictionary with a handful of entries.
pub fn demo_dictionary() -> InMemoryDictionary {
    InMemoryDictionary::with_entries([
        DictionaryEntry {
            term: "留学".to_string(),
            reading: Some("りゅうがく".to_string()),
            meanings: vec!["studying abroad".to_string()],
            pos: vec!["noun".to_string(), "suru-verb".to_string()],
            jlpt: Some("N3".to_string()),
            frequency_rank: Some(4212),
            examples: vec![
                DictionaryExample {
                    native: "妹は来年留学します。".to_string(),
                    gloss: "My younger sister will study abroad next year.".to_string(),
                },
            ],
        },
        DictionaryEntry {
            term: "食卓".to_string(),
            reading: Some("しょくたく".to_string()),
            meanings: vec!["dining table".to_string()],
            pos: vec!["noun".to_string()],
            jlpt: Some("N3".to_string()),
            frequency_rank: None,
            examples: vec![],
        },
        DictionaryEntry {
            term: "ありがとう".to_string(),
            reading: Some("ありがとう".to_string()),
            meanings: vec!["thank you".to_string()],
            pos: vec!["expression".to_string()],
            jlpt: Some("N5".to_string()),
            frequency_rank: Some(312),
            examples: vec![],
        },
        DictionaryEntry {
            term: "言葉".to_string(),
            reading: Some("ことば".to_string()),
            meanings: vec!["word".to_string(), "language".to_string()],
            pos: vec!["noun".to_string()],
            jlpt: Some("N4".to_string()),
            frequency_rank: Some(580),
            examples: vec![DictionaryExample {
                native: "言葉は橋です。".to_string(),
                gloss: "Words are bridges.".to_string(),
            }],
        },
        DictionaryEntry {
            term: "勉強".to_string(),
            reading: Some("べんきょう".to_string()),
            meanings: vec!["study".to_string()],
            pos: vec!["noun".to_string(), "suru-verb".to_string()],
            jlpt: Some("N5".to_string()),
            frequency_rank: Some(420),
            examples: vec![],
        },
    ])
}
