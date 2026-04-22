//! Prompt templates for Phase-3 card generation.
//!
//! Shape: `(Card, Tier) -> Result<Prompt, Unsupported>`. The *system*
//! prompt is constant — it teaches the engine's code conventions.
//! The *user* prompt swaps few-shot examples and instructions by
//! (tier × card shape): creature vs instant/sorcery, trigger vs
//! keyword-only.
//!
//! Few-shot examples are pulled from the actual seed-card source
//! files via [`include_str!`], so prompts cannot drift from the seed
//! corpus — if the card API changes, every regenerated prompt
//! reflects the change the moment the build passes.
//!
//! # Agent-mode vs API-mode
//!
//! This module is *API-mode only*: single-shot prompts, no tool
//! access, no environment — meant for bulk completion endpoints
//! (Anthropic Messages, OpenAI chat, Ollama chat, etc.). The tier-5
//! manual-triage path (Claude Code agent sessions that read engine
//! source and iterate with compile feedback) has a materially
//! different contract and, when it lands, will live in a separate
//! module. Don't retrofit it into this one.
//!
//! # v1 scope
//!
//! In scope (returns `Ok`):
//!   * T1 — vanilla creatures.
//!   * T2 — french-vanilla creatures (keyword-only rules text),
//!     single-effect instants / sorceries.
//!   * T3 — creatures with a triggered ability (ETB, "whenever you
//!     cast", etc.).
//!
//! Out of scope (returns `Err(Unsupported)` — see variants for
//! bucketing):
//!   * Basic lands — hand-written helpers, not LLM generation.
//!   * Activated-only abilities on non-creature permanents — no
//!     clean seed example yet; deferred until we add one.
//!   * T4 / T5 — structural complexity (planeswalkers, X costs,
//!     modal, multi-line, unsupported layout) needs manual routing.

use crate::classifier::Tier;
use crate::scryfall::Card;

// =============================================================================
// public API
// =============================================================================

/// A rendered prompt. `system` and `user` are provider-channel
/// bodies — Anthropic / OpenAI / Ollama-chat all take them as
/// separate fields. Providers that want a single concatenated
/// string must apply their own chat template (ChatML / Llama3 /
/// Alpaca / etc.); naive `system + "\n\n" + user` is wrong for
/// most of them, so we don't offer it here.
#[derive(Debug, Clone)]
pub struct Prompt {
    pub system: String,
    pub user: String,
    /// The sub-route chosen for this (card, tier). Surfaced so the
    /// bake-off driver can log / bucket failures by shape.
    pub shape: PromptShape,
}

/// The sub-route within a tier. Drives which few-shot pack and
/// which user-prompt body are used.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PromptShape {
    VanillaCreature,
    FrenchVanillaCreature,
    SingleEffectSpell,
    TriggeredAbilityCreature,
}

/// Why a (card, tier) combination is not currently renderable. The
/// pipeline driver buckets rejections by variant so coverage loss
/// is structured data rather than a silent `None`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Unsupported {
    /// T1 card that isn't a vanilla creature — i.e., a basic land.
    /// These are hand-written helpers, not LLM generation targets.
    BasicLand,
    /// (tier, card shape) has no few-shot pack in v1 scope — e.g.,
    /// T2 artifact/enchantment, T3 non-creature permanent. `detail`
    /// is a short, stable discriminator suitable for histogramming.
    NoFewShotForShape {
        tier: Tier,
        detail: &'static str,
    },
    /// Tier is structurally out of the automated pipeline. T4 needs
    /// per-card decomposition; T5 needs human routing.
    TierOutOfScope(Tier),
}

impl std::fmt::Display for Unsupported {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Unsupported::BasicLand => {
                write!(f, "basic land (hand-written helper, not LLM-generated)")
            }
            Unsupported::NoFewShotForShape { tier, detail } => write!(
                f,
                "T{}: no few-shot pack for shape '{detail}'",
                tier.as_number()
            ),
            Unsupported::TierOutOfScope(t) => write!(
                f,
                "T{} is out of automated scope (manual routing required)",
                t.as_number()
            ),
        }
    }
}

impl std::error::Error for Unsupported {}

/// Render a prompt for `card` at `tier`. Returns [`Unsupported`]
/// when the (tier, card) combination is outside v1 scope — callers
/// should bucket / histogram the error variant to track coverage
/// loss.
pub fn render_prompt(card: &Card, tier: Tier) -> Result<Prompt, Unsupported> {
    let shape = select_shape(card, tier)?;
    let user = match shape {
        PromptShape::VanillaCreature => user_vanilla_creature(card),
        PromptShape::FrenchVanillaCreature => user_french_vanilla_creature(card),
        PromptShape::SingleEffectSpell => user_single_effect_spell(card),
        PromptShape::TriggeredAbilityCreature => user_triggered_ability_creature(card),
    };
    Ok(Prompt { system: SYSTEM_PROMPT.to_string(), user, shape })
}

fn select_shape(card: &Card, tier: Tier) -> Result<PromptShape, Unsupported> {
    match tier {
        Tier::One => {
            // Basic lands route through a hand-written helper — not
            // an LLM target. Everything else at T1 is a true vanilla
            // creature (the classifier only sends these two shapes
            // to T1).
            if card.is_vanilla_creature() {
                Ok(PromptShape::VanillaCreature)
            } else {
                Err(Unsupported::BasicLand)
            }
        }
        Tier::Two => {
            if card.is_creature() {
                Ok(PromptShape::FrenchVanillaCreature)
            } else if card.is_instant() || card.is_sorcery() {
                Ok(PromptShape::SingleEffectSpell)
            } else {
                Err(Unsupported::NoFewShotForShape {
                    tier,
                    detail: "non-creature, non-instant/sorcery",
                })
            }
        }
        Tier::Three => {
            // v1: triggered-ability creatures only. Activated-only
            // permanents need a different few-shot pack; deferred.
            if card.is_creature() {
                Ok(PromptShape::TriggeredAbilityCreature)
            } else {
                Err(Unsupported::NoFewShotForShape {
                    tier,
                    detail: "non-creature permanent (activated/triggered)",
                })
            }
        }
        Tier::Four | Tier::Five => Err(Unsupported::TierOutOfScope(tier)),
    }
}

// =============================================================================
// system prompt
// =============================================================================

const SYSTEM_PROMPT: &str = r#"You generate Rust source files for the Arcana MTG engine's card catalog (crate `arcana-cards`). Each card is a single file: a doc comment, imports, a `pub fn register(reg: &mut CardRegistry) -> CardId`, and any supporting free functions referenced as `fn` pointers from the registry.

CRITICAL — API DISCIPLINE
Use only types, constructors, enum variants, and trait methods that appear in the reference examples attached below. Do not introduce helper traits, new types, speculative variants (e.g. a `KeywordAbility::Flash` that isn't shown), or imports not used in the references. If a piece of rules text cannot be expressed with the demonstrated API, still produce a best-effort file — the verify pipeline will flag the gap and a human will route it. An unambitious file that compiles is better than a feature-rich file that invents APIs.

OUTPUT FORMAT
- Emit exactly one Rust source file. No markdown fences. No prose before or after. No trailing explanation. Just the `.rs` contents.
- Start with a `//!` doc comment naming the card and summarising its rules text.
- Follow with `use` lines, then the `register` fn, then any resolver / trigger free functions.

ENGINE CONVENTIONS (match the reference examples exactly)
- Names, subtypes, and any other string identifiers are interned first: `let name = reg.interner_mut().intern("Card Name");`. Always intern before use.
- Mana costs: `ManaCost::parse("{1}{R}").expect("valid cost")`. Wrap in `Some(...)` when placed in `Characteristics.mana_cost`.
- Colors: `ColorSet::white()`, `blue()`, `black()`, `red()`, `green()` for monocolored. Colorless or multi-color shapes follow the examples.
- Types: `TypeLine::CREATURE`, `INSTANT`, `SORCERY`, `ARTIFACT`, `ENCHANTMENT`, `LAND`. Place in `Characteristics.types` via `.into()`.
- Power / toughness: `PtValue::Fixed(n)` wrapped in `Some(...)`. Omit on non-creatures (leave as default via `..Default::default()`).
- Keywords: `KeywordAbility::Flying`, `Vigilance`, `Reach`, `Trample`, `Haste`, `Lifelink`, `Deathtouch`, `FirstStrike`, `DoubleStrike`, `Menace`, `Defender`, `Hexproof`, `Shroud`, `Indestructible`. Place in `Characteristics.keywords` as a `Vec<KeywordAbility>`.
- Spell abilities: `.with_spell_ability(SpellAbilityDef { text, target_requirements, modal: None, effect: resolve })` where `resolve` is a free fn `fn(_: &GameState, entry: &StackEntry, _: &CardRegistry) -> Vec<Effect>`.
- Triggered abilities: `.with_triggered_ability(TriggeredAbilityDef { id, trigger_condition, intervening_if, effect, trigger_zones, frequency, target_requirements })`. `id` is a per-card `u32` starting at 1. `effect` is a free fn `fn(_: &GameState, trig: &PendingTrigger, _: &CardRegistry) -> Vec<Effect>`.
- Characteristics: build via struct literal with `..Default::default()` at the end. Do not omit `..Default::default()`.
- CardDefinition chaining ends with `reg.register(CardDefinition::new(name, chars).with_...(...))`."#;

// =============================================================================
// few-shot seed sources (kept in sync with arcana-cards via include_str!)
// =============================================================================

const FS_GRIZZLY_BEARS: &str =
    include_str!("../../arcana-cards/src/lea/grizzly_bears.rs");
const FS_SERRA_ANGEL: &str =
    include_str!("../../arcana-cards/src/lea/serra_angel.rs");
const FS_GIANT_SPIDER: &str =
    include_str!("../../arcana-cards/src/lea/giant_spider.rs");
const FS_LIGHTNING_BOLT: &str =
    include_str!("../../arcana-cards/src/lea/lightning_bolt.rs");
const FS_MURDER: &str =
    include_str!("../../arcana-cards/src/isd/murder.rs");
const FS_COUNTERSPELL: &str =
    include_str!("../../arcana-cards/src/lea/counterspell.rs");
const FS_ELVISH_VISIONARY: &str =
    include_str!("../../arcana-cards/src/lrw/elvish_visionary.rs");
const FS_YOUNG_PYROMANCER: &str =
    include_str!("../../arcana-cards/src/m14/young_pyromancer.rs");

// =============================================================================
// shared target-card spec block
// =============================================================================

/// Render the target card's identifying fields as a compact,
/// labelled block for insertion into user prompts. Keeps every
/// per-shape template consistent on what's given to the model.
fn card_spec(card: &Card) -> String {
    let mut lines: Vec<String> = Vec::new();
    lines.push(format!("Name: {}", card.name));
    if let Some(cost) = &card.mana_cost {
        lines.push(format!("Mana cost: {cost}"));
    }
    lines.push(format!("Type line: {}", card.type_line));
    if let (Some(p), Some(t)) = (&card.power, &card.toughness) {
        lines.push(format!("Power/Toughness: {p}/{t}"));
    }
    if !card.colors.is_empty() {
        lines.push(format!("Colors: {}", card.colors.join(", ")));
    }
    if !card.keywords.is_empty() {
        lines.push(format!(
            "Keywords (Scryfall-parsed): {}",
            card.keywords.join(", ")
        ));
    }
    let oracle = card.effective_oracle_text();
    let oracle_display = if oracle.trim().is_empty() {
        "(empty — vanilla)".to_string()
    } else {
        oracle
    };
    lines.push(format!("Oracle text:\n{oracle_display}"));
    lines.join("\n")
}

// =============================================================================
// per-shape user prompts
// =============================================================================

fn user_vanilla_creature(card: &Card) -> String {
    format!(
        "Generate a VANILLA CREATURE (no rules text — just stats, cost, types).

REFERENCE — Grizzly Bears ({{1}}{{G}} 2/2 Bear, vanilla):
```rust
{FS_GRIZZLY_BEARS}
```

=== TARGET CARD ===
{spec}

Generate the Rust source. Output only the file contents.",
        spec = card_spec(card),
    )
}

fn user_french_vanilla_creature(card: &Card) -> String {
    format!(
        "Generate a FRENCH-VANILLA CREATURE — a creature whose only rules text is keyword abilities (possibly with Scryfall reminder text in parens).

REFERENCE — Serra Angel ({{3}}{{W}}{{W}} 4/4 Angel with Flying + Vigilance):
```rust
{FS_SERRA_ANGEL}
```

REFERENCE — Giant Spider ({{3}}{{G}} 2/4 Spider with Reach):
```rust
{FS_GIANT_SPIDER}
```

=== TARGET CARD ===
{spec}

Use only `KeywordAbility` variants whose names correspond to the keywords Scryfall parsed out (shown in the spec above). Generate the Rust source. Output only the file contents.",
        spec = card_spec(card),
    )
}

fn user_single_effect_spell(card: &Card) -> String {
    format!(
        "Generate a SINGLE-EFFECT INSTANT OR SORCERY — one `SpellAbilityDef` plus a `resolve` fn that returns a `Vec<Effect>`.

REFERENCE — Lightning Bolt ({{R}} instant, 'deals 3 damage to any target'):
```rust
{FS_LIGHTNING_BOLT}
```

REFERENCE — Murder ({{1}}{{B}}{{B}} instant, 'destroy target creature'):
```rust
{FS_MURDER}
```

REFERENCE — Counterspell ({{U}}{{U}} instant, 'counter target spell'):
```rust
{FS_COUNTERSPELL}
```

=== TARGET CARD ===
{spec}

Generate the Rust source. Output only the file contents.",
        spec = card_spec(card),
    )
}

fn user_triggered_ability_creature(card: &Card) -> String {
    format!(
        "Generate a CREATURE WITH A TRIGGERED ABILITY — one `TriggeredAbilityDef` plus an `effect` fn that returns a `Vec<Effect>`.

REFERENCE — Elvish Visionary ({{1}}{{G}} 1/1 Elf Shaman, 'When ~ enters the battlefield, draw a card'):
```rust
{FS_ELVISH_VISIONARY}
```

REFERENCE — Young Pyromancer ({{1}}{{R}} 2/1 Human Shaman, 'Whenever you cast an instant or sorcery spell, create a 1/1 red Elemental creature token'):
```rust
{FS_YOUNG_PYROMANCER}
```

=== TARGET CARD ===
{spec}

Use `TriggerFrequency::EachTime` unless the oracle text says 'only the first time' or similar. Set `trigger_zones` to `vec![Zone::Battlefield]` for on-battlefield triggers. Generate the Rust source. Output only the file contents.",
        spec = card_spec(card),
    )
}

// =============================================================================
// tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    fn mk_card(configure: impl FnOnce(&mut Card)) -> Card {
        let mut c = Card {
            id: "test-id".into(),
            oracle_id: "test-oracle".into(),
            name: "Test Card".into(),
            mana_cost: Some("{1}".into()),
            cmc: 1.0,
            type_line: "Instant".into(),
            oracle_text: Some(String::new()),
            power: None,
            toughness: None,
            loyalty: None,
            defense: None,
            colors: vec![],
            color_identity: vec![],
            keywords: vec![],
            produced_mana: None,
            legalities: {
                let mut m = HashMap::new();
                m.insert("standard".into(), "legal".into());
                m
            },
            rarity: "common".into(),
            set: "tst".into(),
            layout: "normal".into(),
            card_faces: None,
        };
        configure(&mut c);
        c
    }

    // --- shape selection ------------------------------------------------

    #[test]
    fn t1_vanilla_creature_routes_to_vanilla_shape() {
        let c = mk_card(|c| {
            c.name = "Nyxborn Brute".into();
            c.type_line = "Enchantment Creature — Minotaur".into();
            c.oracle_text = Some(String::new());
            c.power = Some("5".into());
            c.toughness = Some("4".into());
        });
        let p = render_prompt(&c, Tier::One).expect("T1 vanilla yields a prompt");
        assert_eq!(p.shape, PromptShape::VanillaCreature);
    }

    #[test]
    fn t1_basic_land_returns_unsupported_basic_land() {
        // Basic lands hit T1 via is_basic_land, but aren't bake-off
        // targets. `is_vanilla_creature` is false, so select_shape
        // returns the BasicLand variant for pipeline bucketing.
        let c = mk_card(|c| {
            c.name = "Mountain".into();
            c.type_line = "Basic Land — Mountain".into();
            c.oracle_text = Some("({T}: Add {R}.)".into());
            c.mana_cost = None;
        });
        assert_eq!(
            render_prompt(&c, Tier::One).unwrap_err(),
            Unsupported::BasicLand
        );
    }

    #[test]
    fn t2_french_vanilla_creature_routes_to_french_vanilla() {
        let c = mk_card(|c| {
            c.name = "Wind Drake".into();
            c.type_line = "Creature — Drake".into();
            c.oracle_text = Some("Flying".into());
            c.keywords = vec!["Flying".into()];
            c.power = Some("2".into());
            c.toughness = Some("2".into());
        });
        let p = render_prompt(&c, Tier::Two).expect("T2 creature yields a prompt");
        assert_eq!(p.shape, PromptShape::FrenchVanillaCreature);
    }

    #[test]
    fn t2_instant_routes_to_single_effect_spell() {
        let c = mk_card(|c| {
            c.name = "Shock".into();
            c.type_line = "Instant".into();
            c.oracle_text = Some("Shock deals 2 damage to any target.".into());
        });
        let p = render_prompt(&c, Tier::Two).expect("T2 spell yields a prompt");
        assert_eq!(p.shape, PromptShape::SingleEffectSpell);
    }

    #[test]
    fn t2_artifact_returns_no_few_shot_for_shape() {
        let c = mk_card(|c| {
            c.name = "Mystery Artifact".into();
            c.type_line = "Artifact".into();
            c.oracle_text = Some("Some static ability.".into());
        });
        match render_prompt(&c, Tier::Two).unwrap_err() {
            Unsupported::NoFewShotForShape { tier, .. } => {
                assert_eq!(tier, Tier::Two);
            }
            other => panic!("expected NoFewShotForShape, got {other:?}"),
        }
    }

    #[test]
    fn t3_creature_routes_to_triggered() {
        let c = mk_card(|c| {
            c.name = "Phantom Helper".into();
            c.type_line = "Creature — Spirit".into();
            c.oracle_text = Some("When Phantom Helper enters, draw a card.".into());
            c.power = Some("2".into());
            c.toughness = Some("2".into());
        });
        let p = render_prompt(&c, Tier::Three).expect("T3 creature yields a prompt");
        assert_eq!(p.shape, PromptShape::TriggeredAbilityCreature);
    }

    #[test]
    fn t3_non_creature_returns_no_few_shot_for_shape() {
        let c = mk_card(|c| {
            c.name = "Icy Manipulator".into();
            c.type_line = "Artifact".into();
            c.oracle_text = Some("{1}, {T}: Tap target permanent.".into());
        });
        match render_prompt(&c, Tier::Three).unwrap_err() {
            Unsupported::NoFewShotForShape { tier, .. } => {
                assert_eq!(tier, Tier::Three);
            }
            other => panic!("expected NoFewShotForShape, got {other:?}"),
        }
    }

    #[test]
    fn t4_and_t5_return_tier_out_of_scope() {
        let c = mk_card(|c| {
            c.oracle_text = Some("Anything".into());
        });
        assert_eq!(
            render_prompt(&c, Tier::Four).unwrap_err(),
            Unsupported::TierOutOfScope(Tier::Four)
        );
        assert_eq!(
            render_prompt(&c, Tier::Five).unwrap_err(),
            Unsupported::TierOutOfScope(Tier::Five)
        );
    }

    #[test]
    fn unsupported_display_is_human_readable() {
        assert!(Unsupported::BasicLand.to_string().contains("basic land"));
        assert!(Unsupported::TierOutOfScope(Tier::Four)
            .to_string()
            .contains("T4"));
        let s = Unsupported::NoFewShotForShape {
            tier: Tier::Three,
            detail: "non-creature permanent",
        }
        .to_string();
        assert!(s.contains("T3"));
        assert!(s.contains("non-creature permanent"));
    }

    // --- rendered prompt content ----------------------------------------

    #[test]
    fn rendered_prompt_has_system_and_user_bodies() {
        let c = mk_card(|c| {
            c.name = "Shock".into();
            c.type_line = "Instant".into();
            c.oracle_text = Some("Shock deals 2 damage to any target.".into());
        });
        let p = render_prompt(&c, Tier::Two).expect("ok");
        assert!(!p.system.is_empty(), "system prompt must not be empty");
        assert!(!p.user.is_empty(), "user prompt must not be empty");
        // System prompt must name the output shape so the model
        // understands what file to produce.
        assert!(p.system.contains("fn register"), "system must mention register fn");
        assert!(p.system.contains("CardRegistry"), "system must mention CardRegistry");
    }

    #[test]
    fn user_prompt_includes_target_card_fields() {
        let c = mk_card(|c| {
            c.name = "Shock".into();
            c.mana_cost = Some("{R}".into());
            c.type_line = "Instant".into();
            c.oracle_text = Some("Shock deals 2 damage to any target.".into());
            c.colors = vec!["R".into()];
        });
        let p = render_prompt(&c, Tier::Two).expect("ok");
        assert!(p.user.contains("Shock"), "target name must appear in user prompt");
        assert!(p.user.contains("{R}"), "target cost must appear in user prompt");
        assert!(p.user.contains("Instant"), "type line must appear");
        assert!(p.user.contains("deals 2 damage"), "oracle text must appear");
    }

    #[test]
    fn vanilla_prompt_embeds_grizzly_bears_source() {
        let c = mk_card(|c| {
            c.name = "Some Vanilla Bear".into();
            c.type_line = "Creature — Bear".into();
            c.oracle_text = Some(String::new());
            c.power = Some("3".into());
            c.toughness = Some("3".into());
        });
        let p = render_prompt(&c, Tier::One).expect("ok");
        assert!(p.user.contains("Grizzly Bears"), "few-shot must include Grizzly Bears");
        assert!(
            p.user.contains("fn register(reg: &mut CardRegistry)"),
            "few-shot must include the register signature"
        );
    }

    #[test]
    fn french_vanilla_prompt_embeds_both_references() {
        let c = mk_card(|c| {
            c.name = "Hardy Drake".into();
            c.type_line = "Creature — Drake".into();
            c.oracle_text = Some("Flying, vigilance".into());
            c.keywords = vec!["Flying".into(), "Vigilance".into()];
            c.power = Some("3".into());
            c.toughness = Some("3".into());
        });
        let p = render_prompt(&c, Tier::Two).expect("ok");
        assert!(p.user.contains("Serra Angel"), "Serra Angel few-shot");
        assert!(p.user.contains("Giant Spider"), "Giant Spider few-shot");
        assert!(
            p.user.contains("Keywords (Scryfall-parsed): Flying, Vigilance"),
            "keyword list must appear in spec"
        );
    }

    #[test]
    fn spell_prompt_embeds_all_three_references() {
        let c = mk_card(|c| {
            c.name = "Disenchant".into();
            c.type_line = "Instant".into();
            c.oracle_text = Some("Destroy target artifact or enchantment.".into());
        });
        let p = render_prompt(&c, Tier::Two).expect("ok");
        assert!(p.user.contains("Lightning Bolt"));
        assert!(p.user.contains("Murder"));
        assert!(p.user.contains("Counterspell"));
    }

    #[test]
    fn triggered_prompt_embeds_both_references() {
        let c = mk_card(|c| {
            c.name = "Phantom Helper".into();
            c.type_line = "Creature — Spirit".into();
            c.oracle_text = Some("When Phantom Helper enters, draw a card.".into());
            c.power = Some("2".into());
            c.toughness = Some("2".into());
        });
        let p = render_prompt(&c, Tier::Three).expect("ok");
        assert!(p.user.contains("Elvish Visionary"));
        assert!(p.user.contains("Young Pyromancer"));
    }

    #[test]
    fn prompt_respects_multi_face_oracle_text() {
        // Adventure / split cards have per-face oracle text. The
        // prompt's spec block should surface `effective_oracle_text`,
        // not the frequently-None top-level field.
        use crate::scryfall::CardFace;
        let c = mk_card(|c| {
            c.name = "Cleaver Titan // Smash".into();
            c.type_line = "Creature — Giant".into();
            c.oracle_text = None;
            c.layout = "adventure".into();
            c.power = Some("4".into());
            c.toughness = Some("3".into());
            c.card_faces = Some(vec![
                CardFace {
                    name: "Cleaver Titan".into(),
                    mana_cost: Some("{2}{R}".into()),
                    type_line: Some("Creature — Giant".into()),
                    oracle_text: Some("When Cleaver Titan enters, draw a card.".into()),
                    power: Some("4".into()),
                    toughness: Some("3".into()),
                    loyalty: None,
                    colors: Some(vec!["R".into()]),
                },
                CardFace {
                    name: "Smash".into(),
                    mana_cost: Some("{1}{R}".into()),
                    type_line: Some("Instant — Adventure".into()),
                    oracle_text: Some("Smash deals 2 damage to any target.".into()),
                    power: None,
                    toughness: None,
                    loyalty: None,
                    colors: Some(vec!["R".into()]),
                },
            ]);
        });
        // Manually force shape to triggered — this test is about
        // oracle-text surfacing, not classifier routing.
        let p = render_prompt(&c, Tier::Three).expect("ok");
        assert!(
            p.user.contains("When Cleaver Titan enters"),
            "front-face text must surface in the prompt"
        );
    }

}
