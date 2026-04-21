//! [`FormatConfig`] — per-game configuration: starting life, hand
//! sizes, deck constraints, mulligan rule.
//!
//! Spec §34 / Phase 1 Task #22 (v0.2 tail-end addition).
//!
//! # Phase 1 scope
//!
//! The format is consulted at game-start (starting life, opening
//! hand size) and during the cleanup step (max hand size). Deck
//! validation (min/max size, max copies per card) is enforced by
//! [`validate_deck`]; it is a soft check callers can skip in tests
//! that want illegal decks.
//!
//! Ban lists, restricted lists, commander rules, and set-legality
//! live in an extended format type that ships in Phase 2
//! (`format_ext.rs` in `arcana-session`, spec §39). Everything here
//! is what the core engine needs in isolation.

use serde::{Deserialize, Serialize};

// =============================================================================
// FormatConfig
// =============================================================================

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct FormatConfig {
    pub name: String,
    pub starting_life: i32,
    pub starting_hand_size: u32,
    pub max_hand_size: u32,
    pub min_deck_size: u32,
    /// `None` means no upper bound (Standard/Modern/Legacy don't
    /// cap deck size; Commander caps at 100).
    pub max_deck_size: Option<u32>,
    pub max_copies_per_card: u32,
    pub mulligan_rule: MulliganRule,
    /// True for formats that use the command zone (Commander,
    /// Brawl). Phase 1 does not implement command-zone mechanics —
    /// this flag is carried for Phase 2 to read.
    pub use_command_zone: bool,
}

impl FormatConfig {
    /// The default format for Phase 1: Standard 2026. Matches every
    /// hard-coded constant Phase 1 used before this type existed
    /// (20 life, 7 cards, 60-card decks, 4-of limit, London
    /// mulligan, no command zone).
    pub fn standard_2026() -> Self {
        Self {
            name: "Standard 2026".into(),
            starting_life: 20,
            starting_hand_size: 7,
            max_hand_size: 7,
            min_deck_size: 60,
            max_deck_size: None,
            max_copies_per_card: 4,
            mulligan_rule: MulliganRule::London,
            use_command_zone: false,
        }
    }

    /// Commander format preset. Wires the basics Phase 2 will
    /// extend with `CommanderRules` — color identity, 21-damage
    /// lethal, partner/companion.
    pub fn commander() -> Self {
        Self {
            name: "Commander".into(),
            starting_life: 40,
            starting_hand_size: 7,
            max_hand_size: 7,
            min_deck_size: 100,
            max_deck_size: Some(100),
            max_copies_per_card: 1,
            mulligan_rule: MulliganRule::London,
            use_command_zone: true,
        }
    }
}

impl Default for FormatConfig {
    fn default() -> Self { Self::standard_2026() }
}

// =============================================================================
// MulliganRule
// =============================================================================

/// Which historical mulligan procedure the game uses (CR 103.4).
///
/// - `London` — current default. Shuffle hand back, redraw 7, owe
///   one card to the bottom per mulligan taken.
/// - `Paris` — legacy. Shuffle hand back, redraw one card fewer
///   than the previous hand size; no bottoming.
/// - `Vancouver` — legacy. Same redraw as Paris; after keeping, if
///   any mulligans were taken, scry 1. The scry step is pending
///   separate wiring (it needs a scry-outside-stack dispatch path);
///   until then Vancouver's redraw path matches Paris exactly.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MulliganRule {
    London,
    Paris,
    Vancouver,
}

// =============================================================================
// Deck validation
// =============================================================================

/// Reasons a deck might fail [`FormatConfig::validate_deck`].
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DeckValidationError {
    TooFewCards { minimum: u32, actual: u32 },
    TooManyCards { maximum: u32, actual: u32 },
    TooManyCopies {
        card: crate::types::CardId,
        maximum: u32,
        actual: u32,
    },
}

impl FormatConfig {
    /// Check that `deck` satisfies this format's quantitative
    /// constraints. Returns every violation (not just the first)
    /// so UI can surface all problems at once.
    ///
    /// Phase 1 checks deck size and copy limits. Ban lists,
    /// restricted lists, set legality, and commander color
    /// identity live in `arcana-session::format_ext` (Phase 2).
    pub fn validate_deck(
        &self,
        deck: &[crate::types::CardId],
    ) -> Result<(), Vec<DeckValidationError>> {
        let mut errors = Vec::new();
        let size = deck.len() as u32;

        if size < self.min_deck_size {
            errors.push(DeckValidationError::TooFewCards {
                minimum: self.min_deck_size,
                actual: size,
            });
        }
        if let Some(cap) = self.max_deck_size {
            if size > cap {
                errors.push(DeckValidationError::TooManyCards {
                    maximum: cap,
                    actual: size,
                });
            }
        }

        // Copy counting — single pass, stable order via sort.
        let mut counts = crate::collections::HashMap::<crate::types::CardId, u32>::default();
        for id in deck {
            *counts.entry(*id).or_insert(0) += 1;
        }
        let mut offenders: Vec<(crate::types::CardId, u32)> = counts.into_iter()
            .filter(|(_, n)| *n > self.max_copies_per_card)
            .collect();
        offenders.sort_by_key(|(id, _)| *id);
        for (card, actual) in offenders {
            errors.push(DeckValidationError::TooManyCopies {
                card,
                maximum: self.max_copies_per_card,
                actual,
            });
        }

        if errors.is_empty() { Ok(()) } else { Err(errors) }
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn standard_2026_matches_phase1_constants() {
        let f = FormatConfig::standard_2026();
        assert_eq!(f.starting_life, 20);
        assert_eq!(f.starting_hand_size, 7);
        assert_eq!(f.max_hand_size, 7);
        assert_eq!(f.min_deck_size, 60);
        assert_eq!(f.max_copies_per_card, 4);
        assert_eq!(f.mulligan_rule, MulliganRule::London);
        assert!(!f.use_command_zone);
    }

    #[test]
    fn default_is_standard_2026() {
        assert_eq!(FormatConfig::default(), FormatConfig::standard_2026());
    }

    #[test]
    fn commander_preset_has_40_life_and_100_card_cap() {
        let f = FormatConfig::commander();
        assert_eq!(f.starting_life, 40);
        assert_eq!(f.min_deck_size, 100);
        assert_eq!(f.max_deck_size, Some(100));
        assert_eq!(f.max_copies_per_card, 1);
        assert!(f.use_command_zone);
    }

    #[test]
    fn validate_deck_ok_for_60_card_four_of_deck() {
        let f = FormatConfig::standard_2026();
        // 15 different cards × 4 copies = 60.
        let deck: Vec<_> = (1..=15u32).flat_map(|c| std::iter::repeat(c).take(4)).collect();
        assert!(f.validate_deck(&deck).is_ok());
    }

    #[test]
    fn validate_deck_reports_too_few_cards() {
        let f = FormatConfig::standard_2026();
        let deck = vec![1u32; 30];
        let errs = f.validate_deck(&deck).unwrap_err();
        assert!(errs.iter().any(|e| matches!(e,
            DeckValidationError::TooFewCards { minimum: 60, actual: 30 })));
    }

    #[test]
    fn validate_deck_reports_too_many_copies() {
        let f = FormatConfig::standard_2026();
        // 60 copies of card 1 — way over the 4-of limit.
        let deck = vec![1u32; 60];
        let errs = f.validate_deck(&deck).unwrap_err();
        assert!(errs.iter().any(|e| matches!(e,
            DeckValidationError::TooManyCopies { card: 1, maximum: 4, actual: 60 })));
    }

    #[test]
    fn validate_deck_reports_too_many_cards() {
        let f = FormatConfig::commander(); // caps at 100
        let deck: Vec<_> = (1..=101u32).collect();
        let errs = f.validate_deck(&deck).unwrap_err();
        assert!(errs.iter().any(|e| matches!(e,
            DeckValidationError::TooManyCards { maximum: 100, actual: 101 })));
    }

    #[test]
    fn validate_deck_collects_multiple_violations() {
        // 30 cards (too few) with the same id repeated (too many).
        let f = FormatConfig::standard_2026();
        let deck = vec![7u32; 30];
        let errs = f.validate_deck(&deck).unwrap_err();
        assert!(errs.len() >= 2);
    }

    #[test]
    fn format_config_serde_roundtrip() {
        let f = FormatConfig::commander();
        let json = serde_json::to_string(&f).unwrap();
        let back: FormatConfig = serde_json::from_str(&json).unwrap();
        assert_eq!(f, back);
    }
}
