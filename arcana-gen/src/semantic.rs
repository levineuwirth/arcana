//! Layer-3 semantic gate: stub detection.
//!
//! Layers 1 (compiles) and 2 (structural fingerprint matches
//! Scryfall) both pass a card whose `resolve`/`effect` body is a
//! stub — `return Vec::new()` with the rules text unimplemented.
//! The pilot showed this is the *dominant* silent failure for T2:
//! the model correctly obeys "don't invent APIs", so when the
//! few-shot `Effect` surface doesn't cover a card it emits a
//! well-formed shell that does nothing. Layers 1/2 cannot catch
//! that — by design, they check bones, not behaviour.
//!
//! This layer is a cheap, high-signal source heuristic, not a
//! semantic proof: a card whose *shape implies it must do
//! something* (an instant/sorcery, or a creature with a triggered
//! ability) must construct at least one `Effect::` value. If the
//! source never constructs one, the rules text is not implemented
//! and the card must not count as passed or be landed.
//!
//! Honest limits: it does NOT verify the *right* effects, only that
//! *some* effect is built. A card that builds the wrong `Effect`
//! still passes layer 3 (human review remains the backstop for
//! that). Vanilla / french-vanilla shapes are exempt — they
//! legitimately construct no effects.

/// Prompt shapes whose card, by construction, has rules text that
/// must compile to at least one `Effect`. Mirrors
/// [`crate::prompt::PromptShape`]'s `Display`/bake-off names.
fn shape_requires_effect(shape: Option<&str>) -> bool {
    matches!(
        shape,
        Some("SingleEffectSpell") | Some("TriggeredAbilityCreature")
    )
}

/// Pre-wired (Pass 2) `KeywordAbility` variants that compile and
/// certify structurally but whose rules behavior is NOT implemented.
/// A card carrying one is honest catalog *data* but a non-functional
/// *card*, so it must be quarantined as an L3 stub (never landed)
/// until a future pass implements real semantics. Names match the
/// enum variant idents exactly as they appear in source. The truly
/// static evasion keywords made real this pass (Fear, Intimidate,
/// Shadow, Horsemanship, Skulk) are deliberately absent — they pass
/// honestly.
const DEFERRED_KEYWORDS: &[&str] = &[
    "Banding", "Rampage", "Bushido", "Exalted", "Soulshift", "Unleash",
    "Bloodthirst", "Modular", "Flanking", "BattleCry", "Undying",
    "Persist", "Afterlife", "Mentor", "Riot", "Devour", "Sunburst",
    "Dethrone", "Scavenge", "Fading", "Vanishing", "Renown", "Evolve",
    "Graft", "Provoke", "Amplify", "Enlist", "Changeling", "Infect",
    "Wither", "Toxic",
];

/// Does `src` reference `KeywordAbility::<variant>` (the next char
/// after the ident is not an identifier char, so `Band` can't match
/// `Banding`)?
fn mentions_variant(src: &str, variant: &str) -> bool {
    let needle = format!("KeywordAbility::{variant}");
    let bytes = src.as_bytes();
    let mut from = 0;
    while let Some(rel) = src[from..].find(&needle) {
        let end = from + rel + needle.len();
        let boundary = bytes
            .get(end)
            .is_none_or(|&c| !(c as char).is_alphanumeric() && c != b'_');
        if boundary {
            return true;
        }
        from = end;
    }
    false
}

/// `Some(reason)` if `source` is a stub; `None` if the card is fine
/// or exempt.
pub fn stub_reason(shape: Option<&str>, source: &str) -> Option<String> {
    let clean = strip_comments(source);

    // Pass 2 honesty guard: a pre-wired deferred-keyword marker means
    // the card's rules are unimplemented regardless of shape. Without
    // this, an inert marker on a vanilla/french-vanilla card would
    // silently certify as `passed` and be landed as if functional.
    if let Some(kw) = DEFERRED_KEYWORDS.iter().find(|kw| mentions_variant(&clean, kw)) {
        return Some(format!(
            "layer-3 stub: KeywordAbility::{kw} is a pre-wired marker \
             — the keyword's rules are not implemented yet (Pass 2 \
             deferral); card data is correct but the card does not \
             function"
        ));
    }

    if !shape_requires_effect(shape) {
        return None;
    }
    if clean.contains("Effect::") {
        return None;
    }
    Some(format!(
        "layer-3 stub: shape {} requires rules text but the source \
         constructs no `Effect::` — resolver is a stub (rules not \
         implemented)",
        shape.unwrap_or("?"),
    ))
}

/// Strip `//`/`///`/`//!` line comments and `/* … */` block
/// comments so a `// GAP: needs Effect::Foo` note (which the spell
/// prompt explicitly asks for) doesn't read as a real effect.
/// String/char literals are not stripped: card files don't put
/// `Effect::` inside text strings, and keeping the scanner trivial
/// is worth more than that theoretical edge.
fn strip_comments(s: &str) -> String {
    let b = s.as_bytes();
    let mut out = String::with_capacity(s.len());
    let mut i = 0;
    while i < b.len() {
        if b[i] == b'/' && i + 1 < b.len() && b[i + 1] == b'/' {
            while i < b.len() && b[i] != b'\n' {
                i += 1;
            }
        } else if b[i] == b'/' && i + 1 < b.len() && b[i + 1] == b'*' {
            i += 2;
            while i + 1 < b.len() && !(b[i] == b'*' && b[i + 1] == b'/') {
                i += 1;
            }
            i += 2;
        } else {
            out.push(b[i] as char);
            i += 1;
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    const STUB_SPELL: &str = r#"
        //! Duress — {B} sorcery.
        fn resolve(_: &GameState, _: &StackEntry, _: &CardRegistry) -> Vec<Effect> {
            // GAP: needs Effect::DiscardChosen — hand manipulation not in API
            Vec::new()
        }
    "#;

    const REAL_SPELL: &str = r#"
        fn resolve(_: &GameState, e: &StackEntry, _: &CardRegistry) -> Vec<Effect> {
            vec![Effect::DrawCards { player: e.controller, count: 1 }]
        }
    "#;

    #[test]
    fn stub_spell_is_flagged() {
        assert!(stub_reason(Some("SingleEffectSpell"), STUB_SPELL).is_some());
    }

    #[test]
    fn real_spell_passes() {
        assert!(stub_reason(Some("SingleEffectSpell"), REAL_SPELL).is_none());
    }

    #[test]
    fn comment_mentioning_effect_does_not_rescue_a_stub() {
        let src = "fn resolve() -> Vec<Effect> { /* Effect::Foo */ Vec::new() }";
        assert!(stub_reason(Some("TriggeredAbilityCreature"), src).is_some());
    }

    #[test]
    fn vanilla_and_french_vanilla_are_exempt() {
        assert!(stub_reason(Some("VanillaCreature"), "no effects here").is_none());
        assert!(
            stub_reason(Some("FrenchVanillaCreature"), "keywords only").is_none()
        );
        assert!(stub_reason(None, "").is_none());
    }

    #[test]
    fn deferred_keyword_marker_is_quarantined_on_french_vanilla() {
        let src = "keywords: vec![KeywordAbility::Undying],";
        let r = stub_reason(Some("FrenchVanillaCreature"), src);
        assert!(r.is_some());
        assert!(r.unwrap().contains("Undying"));
    }

    #[test]
    fn real_evasion_keyword_is_not_quarantined() {
        // Fear/Intimidate/Shadow/Horsemanship/Skulk are implemented —
        // a french-vanilla card carrying only those passes the gate.
        let src = "keywords: vec![KeywordAbility::Fear, KeywordAbility::Shadow],";
        assert!(stub_reason(Some("FrenchVanillaCreature"), src).is_none());
    }

    #[test]
    fn variant_boundary_no_false_prefix_match() {
        // A hypothetical longer ident must not trip a shorter deferred
        // name (e.g. `Riot` vs `Riotous`).
        let src = "KeywordAbility::Riotous";
        assert!(!mentions_variant(src, "Riot"));
        assert!(mentions_variant("KeywordAbility::Riot,", "Riot"));
    }
}
