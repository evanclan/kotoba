//! Deck-level types and the markdown-frontmatter parser.

use crate::card::Card;
use crate::error::{Error, Result};
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

/// A deck: a named collection of cards plus metadata.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Deck {
    /// Metadata from the deck's YAML frontmatter.
    pub meta: DeckMeta,
    /// Cards in source order.
    pub cards: Vec<Card>,
}

impl Deck {
    /// Create a new empty deck with the given metadata.
    pub fn new(meta: DeckMeta) -> Self {
        Self {
            meta,
            cards: Vec::new(),
        }
    }

    /// Append a card to this deck.
    pub fn push(&mut self, card: Card) {
        self.cards.push(card);
    }

    /// Total number of cards.
    pub fn len(&self) -> usize {
        self.cards.len()
    }

    /// Whether the deck has zero cards.
    pub fn is_empty(&self) -> bool {
        self.cards.is_empty()
    }
}

/// Metadata serialized as YAML frontmatter at the top of a deck file.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeckMeta {
    /// Human-readable name.
    pub name: String,
    /// URL/ID-safe slug. Derived from the filename if absent in the source.
    pub slug: String,
    /// One-line description.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Language being learned (BCP-47).
    pub language: String,
    /// Reference language (BCP-47).
    pub target_language: String,
    /// SPDX license identifier.
    #[serde(default = "default_license")]
    pub license: String,
    /// Contributors.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub authors: Vec<String>,
    /// Creation date.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created: Option<NaiveDate>,
    /// Last update.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub updated: Option<NaiveDate>,
    /// Source attribution.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    /// Audience description, e.g. "JLPT N5".
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub audience: Option<String>,
    /// Optional semver version of the deck.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    /// Top-level tags applied to all cards.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tags: Vec<String>,
}

fn default_license() -> String {
    "CC-BY-SA-4.0".to_string()
}

impl DeckMeta {
    /// Construct a minimal metadata block for in-memory decks.
    pub fn minimal(name: impl Into<String>, slug: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            slug: slug.into(),
            description: None,
            language: "ja".to_string(),
            target_language: "en".to_string(),
            license: default_license(),
            authors: Vec::new(),
            created: None,
            updated: None,
            source: None,
            audience: None,
            version: None,
            tags: Vec::new(),
        }
    }
}

/// Parse a deck from the contents of a markdown file.
///
/// The file must start with a `---`-delimited YAML frontmatter block, followed
/// by zero or more cards. Each card is introduced by a `# Heading` and may
/// contain labeled fields (`**Reading:** ...`, `**Tags:** ...`) and free-form
/// markdown body.
///
/// This is a deliberately small parser intended for v0.0.1. A robust parser
/// using `pulldown-cmark` is planned for v0.1.
pub fn parse_markdown(contents: &str, fallback_slug: &str) -> Result<Deck> {
    let (frontmatter, body) = split_frontmatter(contents)?;
    let mut meta: DeckMeta = serde_yaml::from_str(frontmatter)
        .map_err(|e| Error::InvalidDeck(format!("frontmatter: {e}")))?;
    if meta.slug.is_empty() {
        meta.slug = fallback_slug.to_string();
    }
    let cards = parse_cards(body, &meta.slug);
    Ok(Deck { meta, cards })
}

fn split_frontmatter(contents: &str) -> Result<(&str, &str)> {
    let trimmed = contents.trim_start();
    let rest = trimmed.strip_prefix("---").ok_or_else(|| {
        Error::InvalidDeck("missing YAML frontmatter (deck must start with `---`)".to_string())
    })?;
    let rest = rest.trim_start_matches('\n');
    let end = rest
        .find("\n---")
        .ok_or_else(|| Error::InvalidDeck("missing closing `---` for frontmatter".to_string()))?;
    let frontmatter = &rest[..end];
    let after = &rest[end + 4..]; // skip "\n---"
    let body = after.trim_start_matches('\n');
    Ok((frontmatter, body))
}

fn parse_cards(body: &str, deck_slug: &str) -> Vec<Card> {
    let mut cards = Vec::new();
    let mut current: Option<Card> = None;
    let mut buffered_back: Vec<String> = Vec::new();

    let flush = |cards: &mut Vec<Card>, current: &mut Option<Card>, back: &mut Vec<String>| {
        if let Some(mut card) = current.take() {
            if !back.is_empty() {
                let joined = back.join("\n").trim().to_string();
                if !card.back.is_empty() {
                    card.back.push_str("\n\n");
                }
                card.back.push_str(&joined);
            }
            cards.push(card);
            back.clear();
        }
    };

    for line in body.lines() {
        let trimmed = line.trim();
        if let Some(rest) = trimmed.strip_prefix("# ") {
            flush(&mut cards, &mut current, &mut buffered_back);
            current = Some(Card::new(deck_slug, rest.trim()));
            continue;
        }

        let Some(card) = current.as_mut() else {
            continue;
        };

        if let Some(rest) = trimmed.strip_prefix("**Reading:**") {
            card.reading = Some(rest.trim().to_string());
        } else if let Some(rest) = trimmed.strip_prefix("**Meaning:**") {
            if !card.back.is_empty() {
                card.back.push('\n');
            }
            card.back.push_str(rest.trim());
        } else if let Some(rest) = trimmed.strip_prefix("**Back:**") {
            if !card.back.is_empty() {
                card.back.push('\n');
            }
            card.back.push_str(rest.trim());
        } else if let Some(rest) = trimmed.strip_prefix("**Tags:**") {
            card.tags = parse_tag_list(rest.trim());
        } else if let Some(rest) = trimmed.strip_prefix("**Context:**") {
            card.context = Some(rest.trim().to_string());
        } else if !trimmed.is_empty() {
            buffered_back.push(line.to_string());
        }
    }

    flush(&mut cards, &mut current, &mut buffered_back);
    cards
}

fn parse_tag_list(s: &str) -> Vec<String> {
    let s = s.trim().trim_start_matches('[').trim_end_matches(']');
    s.split(',')
        .map(|t| t.trim().trim_matches('"').to_string())
        .filter(|t| !t.is_empty())
        .collect()
}

/// Render a deck back to markdown. Round-trips the data we currently model.
pub fn to_markdown(deck: &Deck) -> Result<String> {
    let frontmatter = serde_yaml::to_string(&deck.meta)
        .map_err(|e| Error::InvalidDeck(format!("yaml serialize: {e}")))?;
    let mut out = String::new();
    out.push_str("---\n");
    out.push_str(&frontmatter);
    out.push_str("---\n\n");

    for card in &deck.cards {
        out.push_str("# ");
        out.push_str(&card.front);
        out.push('\n');
        if let Some(reading) = &card.reading {
            out.push_str("\n**Reading:** ");
            out.push_str(reading);
            out.push('\n');
        }
        if !card.back.is_empty() {
            out.push_str("\n**Meaning:** ");
            out.push_str(&card.back);
            out.push('\n');
        }
        if !card.tags.is_empty() {
            out.push_str("\n**Tags:** [");
            out.push_str(&card.tags.join(", "));
            out.push_str("]\n");
        }
        if let Some(ctx) = &card.context {
            out.push_str("\n**Context:** ");
            out.push_str(ctx);
            out.push('\n');
        }
        out.push('\n');
    }

    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "---\n\
        name: Test deck\n\
        slug: test\n\
        language: ja\n\
        target_language: en\n\
        license: CC-BY-SA-4.0\n\
        ---\n\
        \n\
        # 留学\n\
        \n\
        **Reading:** りゅうがく\n\
        **Meaning:** studying abroad\n\
        **Tags:** [education, n3]\n\
        \n\
        # 食卓\n\
        \n\
        **Reading:** しょくたく\n\
        **Meaning:** dining table\n";

    #[test]
    fn parse_minimal_deck() {
        let deck = parse_markdown(SAMPLE, "test").unwrap();
        assert_eq!(deck.meta.name, "Test deck");
        assert_eq!(deck.cards.len(), 2);
        assert_eq!(deck.cards[0].front, "留学");
        assert_eq!(deck.cards[0].reading.as_deref(), Some("りゅうがく"));
        assert_eq!(deck.cards[0].back, "studying abroad");
        assert_eq!(deck.cards[0].tags, vec!["education", "n3"]);
        assert_eq!(deck.cards[1].front, "食卓");
    }

    #[test]
    fn round_trips_through_markdown() {
        let deck = parse_markdown(SAMPLE, "test").unwrap();
        let rendered = to_markdown(&deck).unwrap();
        let reparsed = parse_markdown(&rendered, "test").unwrap();
        assert_eq!(deck.cards.len(), reparsed.cards.len());
        assert_eq!(deck.cards[0].front, reparsed.cards[0].front);
    }

    #[test]
    fn missing_frontmatter_errors() {
        let err = parse_markdown("# 留学\n", "test").unwrap_err();
        let msg = err.to_string();
        assert!(msg.contains("frontmatter"), "got: {msg}");
    }
}
