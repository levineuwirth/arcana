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
        Some("SingleEffectSpell")
            | Some("TriggeredAbilityCreature")
            // Wave-1 shapes: each has rules text by construction.
            // A mana ability counts — `Effect::AddMana` matches the
            // `Effect::` scan, so a pure mana rock / mana land
            // passes honestly. Statics pass via
            // `Effect::InstallContinuousEffect`; Equipment has a
            // dedicated `with_equip` carve-out in [`stub_reason`].
            | Some("TriggeredEnchantment")
            | Some("ActivatedArtifact")
            | Some("UtilityLand")
            | Some("Equipment")
            | Some("StaticEnchantment")
            // Auras install an `attached_*` ContinuousEffect via
            // `Effect::InstallContinuousEffect`; a GAP'd Aura (control
            // change, host-trigger) constructs no `Effect::` and is
            // correctly caught as a stub.
            | Some("Aura")
            // Planeswalkers: each loyalty ability's effect fn builds an
            // `Effect::`. A PW whose every ability is GAP'd (emblem-only,
            // etc.) constructs none → correctly caught as a stub.
            | Some("Planeswalker")
            // Multi-ability creatures: each triggered/activated effect fn
            // builds an `Effect::`. A creature whose every ability is
            // GAP'd (or that's purely keywords, which would have routed
            // french-vanilla) constructs none → caught as a stub.
            | Some("MultiAbilityCreature")
    )
}

/// Pre-wired (Pass 2) `KeywordAbility` variants that compile and
/// certify structurally but whose rules behavior is NOT implemented.
/// A card carrying one is honest catalog *data* but a non-functional
/// *card*, so it must be quarantined as an L3 stub (never landed)
/// until a future pass implements real semantics. Names match the
/// enum variant idents exactly as they appear in source. Keywords
/// made real in earlier passes are deliberately absent — they pass
/// honestly: the Pass-2 evasion set (Fear, Intimidate, Shadow,
/// Horsemanship, Skulk), the Pass-3.1 damage-as-counters set
/// (Wither, Infect, Toxic), the Pass-3.2 death-triggered set
/// (Undying, Persist, Afterlife) and the Pass-3.3 attack /
/// combat-damage set (Exalted, BattleCry, Mentor, Dethrone, Renown,
/// Enlist) and the Pass-3.4 block-time combat statics (Flanking,
/// Rampage, Bushido, Provoke), the Pass-3.5 ETB-scaling set and the
/// Pass-3.6 long tail (Evolve/Fading/Vanishing real; Soulshift/
/// Scavenge/Changeling recognized with a documented Phase-1 policy).
/// The lone remaining deferral is Warp (a cast-time
/// alternative-cost mechanic — a recognized no-op would
/// misrepresent the card); its cards stay honestly quarantined
/// until a dedicated pass.
const DEFERRED_KEYWORDS: &[&str] = &[
    "Warp",
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
    // Equipment whose only printed text is the equip line (or whose
    // static is an honest GAP) is still a functional card through
    // `with_equip` alone — the builder synthesizes the whole CR 702.6
    // Equip activation, so no `Effect::` construction is required.
    if shape == Some("Equipment") && clean.contains(".with_equip(") {
        return None;
    }
    Some(format!(
        "layer-3 stub: shape {} requires rules text but the source \
         constructs no `Effect::` — resolver is a stub (rules not \
         implemented)",
        shape.unwrap_or("?"),
    ))
}

/// Oracle phrases that denote a *resolution-time computed magnitude*
/// (an amount/count that scales with game state), as opposed to a
/// filter or a fixed number. Deliberately tight to keep false
/// positives low — bare "equal to" / bare "X" are excluded because
/// they also appear in target filters ("creature with power equal
/// to …") and reminder text.
const DYNAMIC_CUES: &[&str] = &[
    "for each",
    "for every",
    "equal to the number of",
    "equal to its power",
    "equal to its toughness",
    "equal to that creature's power",
    "equal to that creature's toughness",
    "equal to the number",
    "x is the number of",
    "where x is",
    "converge",       // # of colors of mana spent
    "domain",         // # of basic land types you control
    "number of cards in",
];

/// `Some(reason)` if the card's oracle text calls for a
/// resolution-time computed amount but the resolver builds only
/// literal amounts — i.e. it hardcoded a placeholder (`amount: 0`,
/// `count: 1`) or `// GAP`-commented the scaling while still emitting
/// a fixed-size effect. The C2 audit found this is the dominant
/// *materially-wrong* (not merely partial) failure: the card looks
/// functional and passes the stub gate, but does the wrong thing.
///
/// Heuristic, like [`stub_reason`]: a resolver that legitimately
/// computes the amount uses the `script::` prelude or the cast's
/// `x_value`; presence of either clears the card. The check only
/// runs for effect-requiring shapes and assumes [`stub_reason`]
/// already passed (so the source does build an `Effect`).
pub fn dynamic_literal_reason(
    shape: Option<&str>,
    oracle: &str,
    source: &str,
) -> Option<String> {
    if !shape_requires_effect(shape) {
        return None;
    }
    let ol = oracle.to_lowercase();
    let cue = DYNAMIC_CUES.iter().find(|c| ol.contains(**c))?;
    let clean = strip_comments(source);
    // The resolver computed the magnitude the sanctioned way.
    if clean.contains("script::") || clean.contains("x_value") {
        return None;
    }
    // Builders whose "for each" scaling is INTRINSIC and engine-side, so a
    // literal amount is the SANCTIONED representation (not a hardcoded
    // placeholder), exactly like `script::`/`x_value`:
    //  - attack_tax(src, N, dur): the engine charges {N} once PER attacking
    //    creature (GameState::attack_tax_total → "Paid per attacking creature";
    //    combat.rs charges `tax` in the per-declaration loop) — Ghostly Prison.
    //  - *_per_match(src, .., count_filter, per_p, per_t, dur): the engine
    //    multiplies the literal per-match P/T by count_matching(count_filter) at
    //    apply time (attached_pt_per_match / filtered_pump_per_match) — the
    //    "for each [count_filter]" lives in the builder, not a script amount
    //    (Blanchwood Armor, Hold the Gates).
    if clean.contains("attack_tax(")
        || clean.contains("_per_match(")
    {
        return None;
    }
    Some(format!(
        "layer-3 dynamic-literal: oracle says \"{cue}\" (a computed \
         amount) but the resolver uses no `script::` / `x_value` — it \
         hardcoded a literal or GAP'd the scaling, which materially \
         misrepresents the card (C2-audit WRONG class)"
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

    const DYN_LITERAL: &str = r#"
        fn resolve(state: &GameState, e: &StackEntry, _: &CardRegistry) -> Vec<Effect> {
            // GAP: number-of-Gates scaling
            vec![Effect::DealDamage { source: e.source, target: dt, amount: 1 }]
        }
    "#;
    const DYN_SCRIPTED: &str = r#"
        fn resolve(state: &GameState, e: &StackEntry, reg: &CardRegistry) -> Vec<Effect> {
            let n = script::count_matching(state, &ObjectFilter::creature(), e.controller);
            vec![Effect::DrawCards { player: e.controller, count: n }]
        }
    "#;

    #[test]
    fn dynamic_oracle_with_literal_resolver_is_flagged() {
        // "deal X damage to each creature, where X is the number of Gates"
        let r = dynamic_literal_reason(
            Some("SingleEffectSpell"),
            "deals X damage to each creature, where X is the number of Gates you control",
            DYN_LITERAL);
        assert!(r.is_some(), "literal-for-dynamic must be flagged");
    }

    #[test]
    fn dynamic_oracle_with_script_resolver_passes() {
        assert!(dynamic_literal_reason(
            Some("SingleEffectSpell"),
            "Draw a card for each creature you control.",
            DYN_SCRIPTED).is_none());
    }

    #[test]
    fn nondynamic_oracle_never_flagged() {
        // "equal to" here is a filter, not a computed amount.
        assert!(dynamic_literal_reason(
            Some("SingleEffectSpell"),
            "Destroy target creature with power equal to 2.",
            DYN_LITERAL).is_none());
        // Non-effect shapes are exempt.
        assert!(dynamic_literal_reason(
            Some("FrenchVanillaCreature"),
            "Draw a card for each Forest you control.",
            DYN_LITERAL).is_none());
    }

    #[test]
    fn attack_tax_clears_dynamic_for_each_attacker() {
        // Ghostly Prison / Propaganda: "pays {2} for each creature that's
        // attacking you" — the "for each" is the attacker count, charged
        // engine-side per attacker, so a literal generic is correct.
        let src = r#"fn etb_install(_s:&GameState,trig:&PendingTrigger,_r:&CardRegistry)->Vec<Effect>{
            vec![Effect::InstallContinuousEffect{ effect:
                ContinuousEffect::attack_tax(trig.source, 2, Duration::WhileSourceOnBattlefield) }]}"#;
        assert!(dynamic_literal_reason(
            Some("StaticEnchantment"),
            "Creatures can't attack you unless their controller pays {2} for each creature they control that's attacking you.",
            src).is_none());
    }

    #[test]
    fn per_match_builder_clears_dynamic_for_each() {
        // Hold the Gates: "+0/+1 for each Gate you control" — the "for each"
        // is the count_filter, multiplied engine-side, so a literal per-match
        // P/T is correct (no script:: amount needed).
        let src = r#"fn etb(_s:&GameState,trig:&PendingTrigger,_r:&CardRegistry)->Vec<Effect>{
            vec![Effect::InstallContinuousEffect{ effect:
                ContinuousEffect::filtered_pump_per_match(trig.source, you, gates, 0, 1,
                    Duration::WhileSourceOnBattlefield) }]}"#;
        assert!(dynamic_literal_reason(
            Some("StaticEnchantment"),
            "Creatures you control get +0/+1 for each Gate you control.",
            src).is_none());
    }

    #[test]
    fn x_value_resolver_clears_dynamic() {
        let src = r#"fn resolve(_:&GameState,e:&StackEntry,_:&CardRegistry)->Vec<Effect>{
            vec![Effect::DealDamage{source:e.source,target:dt,amount:e.x_value.unwrap_or(0)}]}"#;
        assert!(dynamic_literal_reason(
            Some("SingleEffectSpell"),
            "X is the number of cards in your hand.", src).is_none());
    }

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
    fn wave1_shapes_require_effects() {
        // A stub resolver on any Wave-1 shape is flagged …
        for shape in ["TriggeredEnchantment", "ActivatedArtifact",
                      "UtilityLand", "StaticEnchantment", "Equipment"] {
            assert!(
                stub_reason(Some(shape), STUB_SPELL).is_some(),
                "{shape} stub must be flagged"
            );
        }
        // … and a mana ability / continuous-effect install passes.
        let mana_rock = r#"fn add_mana(_: &GameState, ctx: &ActivationContext, _: &CardRegistry) -> Vec<Effect> {
            vec![Effect::AddMana { player: ctx.controller, mana: vec![] }]
        }"#;
        assert!(stub_reason(Some("ActivatedArtifact"), mana_rock).is_none());
        assert!(stub_reason(Some("UtilityLand"), mana_rock).is_none());
        let anthem = r#"fn etb(_: &GameState, trig: &PendingTrigger, _: &CardRegistry) -> Vec<Effect> {
            vec![Effect::InstallContinuousEffect { effect: ContinuousEffect::anthem(trig.source, trig.controller, 1, 1, Duration::WhileSourceOnBattlefield) }]
        }"#;
        assert!(stub_reason(Some("StaticEnchantment"), anthem).is_none());
    }

    #[test]
    fn equipment_with_equip_alone_passes() {
        // Pure "Equip {N}" Equipment (or an honest keyword-GAP) is
        // functional via the with_equip builder — no Effect:: needed.
        let src = r#"
            // GAP: 'equipped creature has flying' — attached_pt covers P/T only
            reg.register(CardDefinition::new(name, chars)
                .with_equip(ManaCost::parse("{2}").expect("valid cost")))
        "#;
        assert!(stub_reason(Some("Equipment"), src).is_none());
        // …but an Equipment source with NEITHER an Effect nor
        // with_equip is a stub.
        assert!(stub_reason(Some("Equipment"), "fn register() {}").is_some());
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
        // Warp is the lone remaining deferral.
        let src = "keywords: vec![KeywordAbility::Warp],";
        let r = stub_reason(Some("FrenchVanillaCreature"), src);
        assert!(r.is_some());
        assert!(r.unwrap().contains("Warp"));
    }

    #[test]
    fn real_keyword_is_not_quarantined() {
        // Implemented keywords pass the gate on a french-vanilla card:
        // Pass-2 evasion, Pass-3.1 damage-as-counters, Pass-3.2 death.
        let src = "keywords: vec![KeywordAbility::Fear, KeywordAbility::Shadow],";
        assert!(stub_reason(Some("FrenchVanillaCreature"), src).is_none());
        let src = "keywords: vec![KeywordAbility::Wither, KeywordAbility::Infect],";
        assert!(stub_reason(Some("FrenchVanillaCreature"), src).is_none());
        let src = "keywords: vec![KeywordAbility::Undying, KeywordAbility::Persist],";
        assert!(stub_reason(Some("FrenchVanillaCreature"), src).is_none());
        let src = "keywords: vec![KeywordAbility::Afterlife(2)],";
        assert!(stub_reason(Some("FrenchVanillaCreature"), src).is_none());
        // Pass 3.6/3.7: long tail + Ward/Cycling are not quarantined.
        let src = "keywords: vec![KeywordAbility::Evolve, KeywordAbility::Changeling],";
        assert!(stub_reason(Some("FrenchVanillaCreature"), src).is_none());
        let src = "keywords: vec![KeywordAbility::Ward(c), KeywordAbility::Cycling(c)],";
        assert!(stub_reason(Some("FrenchVanillaCreature"), src).is_none());
        // Pass 3.8: Banding recognized (choice-control, Phase-1 inert).
        let src = "keywords: vec![KeywordAbility::Banding],";
        assert!(stub_reason(Some("FrenchVanillaCreature"), src).is_none());
    }

    #[test]
    fn warp_is_the_lone_remaining_quarantine() {
        let src = "keywords: vec![KeywordAbility::Warp],";
        let r = stub_reason(Some("FrenchVanillaCreature"), src);
        assert!(r.is_some());
        assert!(r.unwrap().contains("Warp"));
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
