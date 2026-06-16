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
//!   * Activated-only abilities (mana dorks, `{T}:` creatures, and
//!     non-creature permanents) — the triggered few-shot pack has no
//!     trigger to anchor on; deferred to a later pass.
//!   * T4 / T5 — structural complexity (planeswalkers, X costs,
//!     modal, multi-line, unsupported layout) needs manual routing.

use crate::classifier::Tier;
use crate::scryfall::Card;
use crate::verify::CompileError;

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
    /// Creature whose only printed text is one or more activated
    /// abilities (mana dorks, pingers, sac-for-value, equipment-style
    /// granters). Excludes creatures with triggered abilities (those
    /// route to [`Self::TriggeredAbilityCreature`]).
    ActivatedAbilityCreature,
    /// CR 715 — Adventurer. Creature card with an Adventure (instant
    /// or sorcery) face. Layout "adventure" in Scryfall.
    AdventureCreature,
    /// CR 712.4 — Modal double-faced card (MDFC). Both faces are
    /// first-class; either can be cast/played. Layout "modal_dfc".
    ModalDfcCreature,
    /// CR 712 — transforming double-faced card. Creature front cast
    /// normally; a transform ability flips it to the back face (a
    /// permanent — creature, land, or planeswalker). Layout
    /// "transform".
    TransformCreature,
    /// CR 716 — Saga enchantment. Adds a lore counter on enter + on
    /// post-draw, with chapter abilities triggered by counter
    /// placements. Layout "saga".
    Saga,
    /// CR 717 — Class enchantment. Levels up via activated abilities;
    /// each level grants new abilities. Typeline includes "Class".
    ClassEnchantment,
    /// MOM Battles. Battle subtype permanent that enters with defense
    /// counters; attackers attack the battle's controller. Layout
    /// "battle".
    Battle,
    /// Wave-1: non-Aura, non-Saga/Class enchantment whose printed
    /// text is triggered abilities ("Whenever a player casts a
    /// spell, …", "At the beginning of your upkeep, …"). Wired
    /// exactly like a triggered creature minus the P/T.
    TriggeredEnchantment,
    /// Wave-1: non-creature, non-Equipment artifact whose text is
    /// activated abilities — mana rocks ("{T}: Add {C}"), utility
    /// activations ("{2}, {T}: …") — optionally plus a triggered
    /// ability.
    ActivatedArtifact,
    /// Wave-1: nonbasic land — mana abilities (with color choices as
    /// one mana ability per color), enters-tapped, and at most one
    /// extra simple activated ability. `mana_cost: None` by
    /// construction.
    UtilityLand,
    /// Wave-1: artifact — Equipment. `with_equip(cost)` synthesizes
    /// the CR 702.6 Equip activation; the "equipped creature gets
    /// +P/+T" static installs via an ETB-triggered
    /// `ContinuousEffect::attached_pt` (Bonesplitter pattern).
    Equipment,
    /// Wave-1: enchantment whose text is a single static continuous
    /// effect (anthems, "creatures you control have flying") —
    /// the Glorious Anthem ETB-install pattern.
    StaticEnchantment,
    /// Wave-2: Aura enchantment ("Enchant creature. Enchanted creature
    /// …"). `with_enchant(filter)` carries the enchant target to the
    /// cast path; the engine attaches the Aura on resolution (CR
    /// 303.4f). The grant is an ETB-installed `attached_*`
    /// `ContinuousEffect` that follows `source.attached_to` — the
    /// Equipment idiom (Holy Strength / Pacifism patterns).
    Aura,
    /// Wave-2: single-faced planeswalker. `loyalty: Some(N)` printed
    /// starting loyalty + one `ActivatedAbilityDef` per loyalty ability
    /// (`add_self_counter`/`remove_self_counter` for +N/-N, empty for 0;
    /// `is_loyalty_ability: true`). CR 606 — the Chandra, Pyromaster
    /// idiom; the engine enforces sorcery-speed / once-per-turn / 0-SBA.
    Planeswalker,
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

/// The previous (failed) attempt for a retry prompt. Carries the
/// code the model emitted plus the compile errors `verify::check`
/// extracted from it, so the next prompt can surface exactly what
/// went wrong.
#[derive(Debug, Clone)]
pub struct PreviousAttempt<'a> {
    pub code: &'a str,
    pub errors: &'a [CompileError],
}

/// Render a prompt for `card` at `tier`. Returns [`Unsupported`]
/// when the (tier, card) combination is outside v1 scope — callers
/// should bucket / histogram the error variant to track coverage
/// loss.
pub fn render_prompt(card: &Card, tier: Tier) -> Result<Prompt, Unsupported> {
    let shape = select_shape(card, tier)?;
    let user = user_for_shape(card, shape);
    Ok(Prompt { system: SYSTEM_PROMPT.to_string(), user, shape })
}

/// Render a retry prompt. Same (card, tier) routing as the one-shot
/// path — same system prompt, same few-shots, same target spec —
/// with an appended block carrying the previous attempt's source
/// and the compile errors `verify::check` extracted from it.
///
/// Design: keeping the full original prompt context (not just "here
/// was your code, here were the errors") means the model retains
/// the API-discipline framing and the shape-specific reference
/// cards at the moment it needs them most. The retry block is
/// additive, not substitutive.
pub fn render_retry_prompt(
    card: &Card,
    tier: Tier,
    previous: &PreviousAttempt,
) -> Result<Prompt, Unsupported> {
    let shape = select_shape(card, tier)?;
    let mut user = user_for_shape(card, shape);
    user.push_str("\n\n=== PREVIOUS ATTEMPT ===\n```rust\n");
    user.push_str(previous.code);
    if !previous.code.ends_with('\n') {
        user.push('\n');
    }
    user.push_str("```\n\n=== COMPILE ERRORS ===\n");
    if previous.errors.is_empty() {
        user.push_str("(no structured errors captured — the previous attempt failed but produced no parseable diagnostics; common for syntax errors. Look for unbalanced braces, missing semicolons, or stray tokens.)\n");
    } else {
        for err in previous.errors {
            user.push_str(&format_compile_error(err));
            user.push('\n');
        }
    }
    user.push_str(
        "\nProduce a corrected version of the file. Address the specific errors above. \
        Output only the Rust source — no markdown fences, no prose, no explanation.",
    );
    Ok(Prompt { system: SYSTEM_PROMPT.to_string(), user, shape })
}

fn user_for_shape(card: &Card, shape: PromptShape) -> String {
    match shape {
        PromptShape::VanillaCreature => user_vanilla_creature(card),
        PromptShape::FrenchVanillaCreature => user_french_vanilla_creature(card),
        PromptShape::SingleEffectSpell => user_single_effect_spell(card),
        PromptShape::TriggeredAbilityCreature => user_triggered_ability_creature(card),
        PromptShape::ActivatedAbilityCreature => user_activated_ability_creature(card),
        PromptShape::AdventureCreature => user_adventure_creature(card),
        PromptShape::ModalDfcCreature => user_mdfc_creature(card),
        PromptShape::TransformCreature => user_transform_creature(card),
        PromptShape::Saga => user_saga(card),
        PromptShape::ClassEnchantment => user_class_enchantment(card),
        PromptShape::Battle => user_battle(card),
        PromptShape::TriggeredEnchantment => user_triggered_enchantment(card),
        PromptShape::ActivatedArtifact => user_activated_artifact(card),
        PromptShape::UtilityLand => user_utility_land(card),
        PromptShape::Equipment => user_equipment(card),
        PromptShape::StaticEnchantment => user_static_enchantment(card),
        PromptShape::Aura => user_aura(card),
        PromptShape::Planeswalker => user_planeswalker(card),
    }
}

/// One-line textual rendering of a compile error for inclusion in a
/// retry prompt. Shape: `<file>:<line>:<col> [<code>] <level>: <message>`.
/// Missing error codes render as `-` (common for syntax errors).
fn format_compile_error(err: &CompileError) -> String {
    format!(
        "{}:{}:{} [{}] {}: {}",
        err.file,
        err.line,
        err.column,
        err.code.as_deref().unwrap_or("-"),
        err.level,
        err.message,
    )
}

fn select_shape(card: &Card, tier: Tier) -> Result<PromptShape, Unsupported> {
    // Typed-card layouts take precedence over tier-based routing —
    // these cards need their own prompt regardless of how the
    // classifier scored their oracle text (they often score as
    // Tier::Four "multiple ability lines" because every chapter /
    // level / level-up clause counts as an ability line).
    if card.is_adventure_layout() && card.is_creature() {
        return Ok(PromptShape::AdventureCreature);
    }
    if card.is_mdfc_layout() && card.is_creature() {
        return Ok(PromptShape::ModalDfcCreature);
    }
    if card.is_transform_layout() && card.is_creature() {
        return Ok(PromptShape::TransformCreature);
    }
    if card.is_saga() {
        return Ok(PromptShape::Saga);
    }
    if card.is_class() {
        return Ok(PromptShape::ClassEnchantment);
    }
    if card.is_battle() {
        return Ok(PromptShape::Battle);
    }
    // Wave-2: single-faced planeswalker (loyalty abilities). Transform/
    // MDFC PW faces have a non-normal layout and stay refused.
    if card.is_planeswalker() && card.layout == "normal" {
        return Ok(PromptShape::Planeswalker);
    }
    // Tier 4/5 are explicitly out-of-scope for the classifier; refuse
    // AFTER the typed-card layout dispatch so Saga/Class/Battle
    // (which would otherwise score as Tier::Four on multi-line text)
    // still route to their proper prompts.
    if matches!(tier, Tier::Four | Tier::Five) {
        return Err(Unsupported::TierOutOfScope(tier));
    }
    // ---- Wave-1 permanent-type routing (catalog breadth plan §2) ----
    // Card-TYPE dispatch comes before the per-tier creature/spell
    // routing below, so enchantments, non-creature artifacts, and
    // nonbasic lands reach their own few-shot packs instead of the
    // legacy "non-creature" refusals. Runs only for T1–T3 cards —
    // T4/T5 were refused just above (multi-line cards stay out of
    // Wave 1, except the two-line land / mana-rock / Equipment
    // carve-out the classifier now routes to T3).
    let text = card.effective_oracle_text();
    if card.is_enchantment() && !card.is_creature() && !card.is_land() {
        // Auras (Wave-2): the engine attaches the Aura to its enchant
        // target on resolution (CR 303.4f via `with_enchant` +
        // `finalize_resolved_spell`), and the `attached_*` continuous
        // effects carry the grant. Route buff / keyword / restriction
        // Auras to their own pack.
        if card.type_line.contains("Aura") {
            return Ok(PromptShape::Aura);
        }
        if crate::classifier::has_triggered_ability(&text) {
            return Ok(PromptShape::TriggeredEnchantment);
        }
        if crate::classifier::has_activated_ability(&text) {
            // Activated-ability enchantments (e.g. shrines with
            // "{cost}: …") have no Wave-1 pack — neither the
            // triggered nor the static pack would anchor them.
            return Err(Unsupported::NoFewShotForShape {
                tier,
                detail: "enchantment with activated ability (no pack yet)",
            });
        }
        return Ok(PromptShape::StaticEnchantment);
    }
    if card.is_artifact() && !card.is_creature() && !card.is_land() {
        if crate::classifier::is_equipment(card) {
            return Ok(PromptShape::Equipment);
        }
        return Ok(PromptShape::ActivatedArtifact);
    }
    // Nonbasic lands only — basic lands classify T1 and keep the
    // hand-written-helper refusal in the Tier::One arm below.
    if card.is_land()
        && !crate::scryfall::type_part(&card.type_line).contains("Basic")
    {
        return Ok(PromptShape::UtilityLand);
    }
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
            // v1: triggered-ability creatures only. The triggered
            // few-shot pack (Elvish Visionary, Young Pyromancer) has
            // nothing to anchor on for a creature whose only ability
            // is activated/static — a mana dork, a "{T}: …" creature.
            // The classifier routes those to T3 via `has_activated_
            // ability`; they need their own pack (a later pass), so
            // defer them rather than emit a mismatched prompt.
            // Non-creature permanents likewise.
            if !card.is_creature() {
                Err(Unsupported::NoFewShotForShape {
                    tier,
                    detail: "non-creature permanent (activated/triggered)",
                })
            } else if crate::classifier::has_triggered_ability(
                &card.effective_oracle_text(),
            ) {
                Ok(PromptShape::TriggeredAbilityCreature)
            } else {
                // Tier::Three with no triggered ability — must have at
                // least one activated ability (otherwise the
                // classifier would have routed elsewhere). Route to
                // the activated-permanent prompt.
                Ok(PromptShape::ActivatedAbilityCreature)
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
Use only types, constructors, enum variants, and trait methods that appear in the reference examples attached below or are explicitly listed under ENGINE CONVENTIONS. Do not introduce helper traits, new types, speculative variants (e.g. a `KeywordAbility::Banding` that is neither shown nor listed), or imports not used in the references. If a piece of rules text cannot be expressed with the demonstrated API, still produce a best-effort file — the verify pipeline will flag the gap and a human will route it. An unambitious file that compiles is better than a feature-rich file that invents APIs.

OUTPUT FORMAT
- Emit exactly one Rust source file. No markdown fences. No prose before or after. No trailing explanation. Just the `.rs` contents.
- Start with a `//!` doc comment naming the card and summarising its rules text.
- Follow with `use` lines, then the `register` fn, then any resolver / trigger free functions.

CANONICAL IMPORT PRELUDE — use ONLY paths from this list. Most cards need a subset; do NOT improvise other paths. (Wrong paths are by far the #1 layer-1 failure class. ObjectFilter lives in `targets`, NOT `objects`. OptionalPaymentKind lives in `actions`, NOT `effects`. KeywordAbility lives in `effects`, the `keywords::KeywordAbility` re-export also works.)
```rust
use arcana_core::actions::OptionalPaymentKind;          // OptionalPayment cost shape
use arcana_core::effects::{Effect, KeywordAbility};     // Effect variants + KeywordAbility enum
use arcana_core::events::{DamageTarget, GameEvent};     // DamageTarget for DealDamage; rarely GameEvent
use arcana_core::layers::Duration;                      // Duration::EndOfTurn / WhileSourceOnBattlefield
use arcana_core::mana::{ManaCost, ManaUnit};
use arcana_core::objects::{Characteristics, ObjectId};   // ObjectId for intervening_if fn signatures
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::script;                                 // script::count_matching etc. (resolution-time amounts)
use arcana_core::conditions;                             // conditions::you_control_at_least etc. (intervening-if predicates)
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetChoice, TargetCount,
    TargetFilter, TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::{Phase, Step, DayNight};         // DayNight for Effect::SetDayNight (CR 726)
use arcana_core::types::{
    CardId, ColorSet, CounterKind, ManaColor, PlayerId, PtValue, SubtypeSet,
    SupertypeSet, TypeLine,
};
use arcana_core::zones::Zone;
```
Trim unused `use`s from your final file. Do not invent paths like `arcana_core::types::CounterType` / `arcana_core::counters::*` / `arcana_core::keywords::KeywordAbility` (those work via re-export but the canonical path above is preferred). Do NOT write `arcana_core::objects::ObjectFilter` / `arcana_core::effects::OptionalPaymentKind` — those used to fail; they now work via re-export, but `targets::` / `actions::` is canonical.

COMMON L1 ERRORS — pre-emission self-check (these caused the prior 10% L1 fail rate; if you see yourself about to write any of these, STOP and write the right form):
- `TargetChoice` has FOUR variants: `Object(ObjectId)`, `Player(PlayerId)`, `ObjectOrPlayer(...)`, `ChosenColor(_)`. When you read a target via `match` you MUST handle `ObjectOrPlayer` for any-target effects. For single-`creature` triggers it's fine to `let Some(TargetChoice::Object(id)) = trig.targets.targets.first() else { return Vec::new(); };` and skip the rest.
- `trig.targets` is a `TargetSelection` whose `.targets` field is the `Vec<TargetChoice>`. So it's `trig.targets.targets.first()`, NOT `trig.targets.first()`.
- `Effect::Pump { target, power, toughness, duration: Duration::EndOfTurn, keywords: vec![] }` — the duration field is `duration`, NOT `until_end_of_turn`.
- `Effect::GrantKeyword { target, keyword, duration }` — same: field is `duration`.
- `Effect::CreateToken { controller, token }` has ONLY two fields. There is NO `count` / `owner` / `blueprint`. For 'create N tokens', repeat the `Effect::CreateToken` value N times in the vec.
- `Effect::AddCounters { target, kind, count }` — the counter-kind field is `kind`, NOT `counter`.
- `Effect::Sequence(Vec<Effect>)` is a tuple variant — write `Effect::Sequence(vec![Effect::A, Effect::B])`, NOT `Effect::Sequence { effects: ... }`.
- `Effect::Discard { player, count, choice: DiscardChoice::ControllerChooses }` — the `choice` field is REQUIRED; default is `DiscardChoice::ControllerChooses`. Import `DiscardChoice` from `arcana_core::effects`.
- `OptionalPaymentKind` variants in v1: ONLY `Mana(ManaCost)` and `Life(u32)`. There is NO `Sacrifice`/`Discard`/`ExileFromGraveyard` (those gates are GAP material).
- `ObjectFilter`'s controller method is `.controlled_by(ControllerConstraint::You)`, NOT `.with_controller(...)`.
- `TriggerCondition::ZoneChange { filter, from: Option<Zone>, to: Zone }` — fields are `filter`/`from`/`to`. There is NO `controller_constraint` — restrict via the `filter`'s `.controlled_by(...)`.
- `Zone` enum has NO `None`/`Any` variants. For "from any zone" pass `from: None` (the field is `Option<Zone>`).
- `reg.interner_mut().intern("…")` borrows `reg` mutably. Bind all subtype symbols in `register()` ONE AT A TIME with `let s = reg.interner_mut().intern("…");`. Don't try to chain multiple intern calls in the same expression — the second is a duplicate mutable borrow.
- `script::count_matching(state, &filter, you)` takes THREE arguments. `script::ids_matching(state, &filter, you)` is the same. Don't pass two or four.

ENGINE CONVENTIONS (match the reference examples exactly)
- Names, subtypes, and any other string identifiers are interned first: `let name = reg.interner_mut().intern("Card Name");`. Always intern before use.
- Mana costs: `ManaCost::parse("{1}{R}").expect("valid cost")`. Wrap in `Some(...)` when placed in `Characteristics.mana_cost`.
- Colors: `ColorSet::white()`, `blue()`, `black()`, `red()`, `green()` for monocolored. For multicolor, OR the constructors together: a red-green card is `ColorSet::red() | ColorSet::green()`; a W/U/B card is `ColorSet::white() | ColorSet::blue() | ColorSet::black()`. For colorless use `ColorSet::colorless()`. The color set must match the card's actual colors (the `Colors:` line in the spec) even when the mana cost uses hybrid symbols like `{R/G}`.
- Types: `TypeLine::CREATURE`, `INSTANT`, `SORCERY`, `ARTIFACT`, `ENCHANTMENT`, `LAND`. Place in `Characteristics.types` via `.into()`.
- Power / toughness: `PtValue::Fixed(n)` wrapped in `Some(...)`. Omit on non-creatures (leave as default via `..Default::default()`).
- Supertypes: `Characteristics.supertypes` is a `SupertypeSet`, a bitflag wrapper — NOT a set. For a Legendary card use `supertypes: SupertypeSet(SupertypeSet::LEGENDARY)`; for a Basic land `SupertypeSet(SupertypeSet::BASIC)`. Otherwise leave it to `..Default::default()`. Never call `.insert()` / `.0.insert()` on it (that idiom is for `SubtypeSet` only).
- Keywords (unit variants, fully implemented): `KeywordAbility::Flying`, `Vigilance`, `Reach`, `Trample`, `Haste`, `Lifelink`, `Deathtouch`, `FirstStrike`, `DoubleStrike`, `Menace`, `Defender`, `Hexproof`, `Shroud`, `Indestructible`, `Flash`, `Fear`, `Intimidate`, `Shadow`, `Horsemanship`, `Skulk`, `Wither`, `Infect`, `Undying`, `Persist`, `Exalted`, `BattleCry`, `Mentor`, `Dethrone`, `Enlist`, `Flanking`, `Provoke`, `Sunburst`, `Unleash`, `Riot`, `Evolve`, `Changeling`, `Banding`. Place in `Characteristics.keywords` as a `Vec<KeywordAbility>`. (Scryfall `Battle cry` → `KeywordAbility::BattleCry`.)
- Toxic (parametrized, fully implemented): for Scryfall keyword `Toxic N` (the N is in the card's rules text, e.g. "Toxic 2"), emit `KeywordAbility::Toxic(N)` where N is that integer as a `u8` — e.g. `KeywordAbility::Toxic(2)`. If no number is given treat it as `Toxic(1)`.
- Afterlife (parametrized, fully implemented): for Scryfall keyword `Afterlife N` (the N is in the card's rules text, e.g. "Afterlife 2"), emit `KeywordAbility::Afterlife(N)` where N is that integer as a `u32` — e.g. `KeywordAbility::Afterlife(2)`. If no number is given treat it as `Afterlife(1)`.
- Renown (parametrized, fully implemented): for Scryfall keyword `Renown N` (the N is in the card's rules text, e.g. "Renown 1"), emit `KeywordAbility::Renown(N)` where N is that integer as a `u32` — e.g. `KeywordAbility::Renown(1)`. If no number is given treat it as `Renown(1)`.
- Rampage / Bushido (parametrized, fully implemented): for Scryfall keyword `Rampage N` / `Bushido N` (the N is in the card's rules text, e.g. "Bushido 2"), emit `KeywordAbility::Rampage(N)` / `KeywordAbility::Bushido(N)` where N is that integer as a `u8` — e.g. `KeywordAbility::Bushido(2)`. If no number is given treat it as `(1)`.
- Modular / Graft / Bloodthirst / Amplify / Devour (parametrized, fully implemented): for Scryfall keyword `<Kw> N` (the N is in the card's rules text, e.g. "Modular 1", "Devour 2"), emit `KeywordAbility::Modular(N)` / `Graft(N)` / `Bloodthirst(N)` / `Amplify(N)` / `Devour(N)` where N is that integer as a `u8`. If no number is given treat it as `(1)`.
- Fading / Vanishing (parametrized, fully implemented): for Scryfall keyword `Fading N` / `Vanishing N` (the N is in the card's rules text, e.g. "Vanishing 3"), emit `KeywordAbility::Fading(N)` / `KeywordAbility::Vanishing(N)` where N is that integer as a `u8`. If no number is given treat it as `(1)`.
- Soulshift (parametrized, fully implemented): for Scryfall keyword `Soulshift N` (the N is in the card's rules text, e.g. "Soulshift 4"), emit `KeywordAbility::Soulshift(N)` where N is that integer as a `u8`. If no number is given treat it as `Soulshift(1)`.
- Landwalk: for a Scryfall keyword `Plainswalk`/`Islandwalk`/`Swampwalk`/`Mountainwalk`/`Forestwalk`, use `KeywordAbility::Landwalk(reg.interner_mut().intern("<Type>"))` where `<Type>` is the matching basic land subtype — `Forestwalk` → `KeywordAbility::Landwalk(reg.interner_mut().intern("Forest"))`. Intern the subtype the same way card subtypes are interned. Ignore the generic `Landwalk` umbrella keyword Scryfall also lists; only the specific `<type>walk` entry maps. (Generic/nonbasic landwalk is NOT supported — emit `keywords: vec![]` for those.)
- Ward (parametrized, fully implemented): for Scryfall keyword `Ward {cost}` with a MANA cost (e.g. "Ward {2}"), emit `KeywordAbility::Ward(ManaCost::parse("{2}").expect("valid cost"))`. If the ward cost is non-mana ("Ward—Pay 3 life", "Ward—Discard a card"), that form is NOT expressible — emit `keywords: vec![]` and note the gap.
- Cycling (parametrized, fully implemented): for Scryfall keyword `Cycling {cost}` (a mana cost in the rules text, e.g. "Cycling {2}"), emit `KeywordAbility::Cycling(ManaCost::parse("{2}").expect("valid cost"))`. The engine synthesizes the "[cost], discard this card: draw a card" activated ability from the keyword. (Typecycling/landcycling variants: just emit the generic `Cycling` with its printed cost; the type-search variant is not separately modeled.)
- Scavenge (parametrized, fully implemented): for Scryfall keyword `Scavenge {cost}` (a mana cost in the rules text, e.g. "Scavenge {1}{B/G}"), emit `KeywordAbility::Scavenge(ManaCost::parse("{1}{B/G}").expect("valid cost"))`. The engine synthesizes the graveyard activated ability ("[cost], exile this card from your graveyard: put +1/+1 counters equal to this card's power on target creature; sorcery speed").
- Pre-wired marker keyword (recognized but rules NOT yet implemented): `Warp`. If the card's Scryfall keyword is `Warp`, DO emit it as `KeywordAbility::Warp` so the catalog records it. It will be quarantined by the verify pipeline as "rules unimplemented" (the warp cast mechanic is deferred) — that is expected and correct; still emit the marker.
- The above is the COMPLETE usable keyword surface. Any other keyword (Prowess, Protection, Convoke, etc.) is NOT available for this card class: emit `keywords: vec![]` and note the gap in the doc comment.
- Spell abilities: `.with_spell_ability(SpellAbilityDef { text, target_requirements, modal: None, effect: resolve })` where `resolve` is a free fn `fn(_: &GameState, entry: &StackEntry, _: &CardRegistry) -> Vec<Effect>`.
- Modal spells (Charms, "Choose one — …"): use `modal: Some(ModalSpec { min_modes: 1, max_modes: 1, clauses: vec![ModeClause { text: "…".into(), target_requirements: vec![req_for_this_mode] }, …] })`. Each clause owns its own targets (the engine concatenates chosen clauses' targets in card order). Set `effect: arcana_core::registry::dispatch_modal_effect` (a STATIC function pointer — do not invent your own dispatcher). Then attach per-mode callbacks: `.with_mode_effects(vec![mode_0_resolve, mode_1_resolve, …])`. Each `mode_N_resolve` is a free fn with the same `(&GameState, &StackEntry, &CardRegistry) -> Vec<Effect>` signature; the dispatcher calls only the chosen ones. For "Choose one OR both" use `min_modes: 1, max_modes: 2`; for "Choose two" use `min_modes: 2, max_modes: 2`. Import `ModalSpec, ModeClause, dispatch_modal_effect` from `arcana_core::registry`.
- Triggered abilities: `.with_triggered_ability(TriggeredAbilityDef { id, trigger_condition, intervening_if, effect, trigger_zones, frequency, target_requirements })`. `id` is a per-card `u32` starting at 1. `effect` is a free fn `fn(_: &GameState, trig: &PendingTrigger, _: &CardRegistry) -> Vec<Effect>`.
- INTERVENING-IF (CR 603.4) — when a trigger has an "if" clause GATING whether it happens at all ("At the beginning of your upkeep, IF you control three or more artifacts, …"; "Whenever a creature dies, IF you have 13 or less life, …"; "At the beginning of your upkeep, IF ~ has no +1/+1 counters on it, …"), set `intervening_if: Some(my_condition)` — do NOT bake the check into the effect body, and do NOT GAP it / fire unconditionally. `my_condition` is a free fn `fn(_: &GameState, source: ObjectId, you: PlayerId, _reg: &CardRegistry) -> bool` (state, the ability's source object, its controller, the card registry — the registry param is REQUIRED even when unused; name it `_reg`) that returns true to allow the trigger. Implement it with the `arcana_core::conditions` predicates (`use arcana_core::conditions;`), or `script::` for amounts. Available `conditions::` helpers: `you_control_at_least(s, you, &filter, n)` · `you_control_at_most(..)` · `you_control_a(s, you, &filter)` · `life_at_least(s, you, n)` · `life_at_most(s, you, n)` · `hand_at_least(s, you, n)` · `hand_empty(s, you)` · `graveyard_at_least(s, you, n)` · `source_has_counter(s, source, kind)` · `source_counters_at_least(s, source, kind, n)`. The engine checks `intervening_if` both as the trigger goes on the stack and (per CR 603.4) when it resolves. Import `ObjectId` from `arcana_core::objects`. Example:
```rust
fn if_control_three_artifacts(
    s: &GameState, _src: ObjectId, you: PlayerId, _reg: &CardRegistry,
) -> bool {
    conditions::you_control_at_least(
        s, you,
        &ObjectFilter { types: Some(TypeLine::ARTIFACT.into()), ..Default::default() },
        3,
    )
}
// …then on the TriggeredAbilityDef: intervening_if: Some(if_control_three_artifacts),
```
(A trigger with no "if" clause keeps `intervening_if: None`. "if able" / "may" are NOT intervening-if — those are resolution-time choices, not a gate.)
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
const FS_WELDFAST_ENGINEER: &str =
    include_str!("../../arcana-cards/src/aer/weldfast_engineer.rs");
const FS_LLANOWAR_ELVES: &str =
    include_str!("../../arcana-cards/src/lea/llanowar_elves.rs");
const FS_PRODIGAL_SORCERER: &str =
    include_str!("../../arcana-cards/src/lea/prodigal_sorcerer.rs");
const FS_BONECRUSHER_GIANT: &str =
    include_str!("../../arcana-cards/src/eld/bonecrusher_giant.rs");
const FS_PREORDAIN: &str =
    include_str!("../../arcana-cards/src/m11/preordain.rs");
const FS_SERVO_EXHIBITION: &str =
    include_str!("../../arcana-cards/src/aer/servo_exhibition.rs");
// Wave-1 seed exemplars (catalog breadth plan §2).
const FS_ANGELIC_CHORUS: &str =
    include_str!("../../arcana-cards/src/bbd/angelic_chorus.rs");
const FS_PHYREXIAN_TYRANNY: &str =
    include_str!("../../arcana-cards/src/s2x2/phyrexian_tyranny.rs");
const FS_UNDERWORLD_DREAMS: &str =
    include_str!("../../arcana-cards/src/leg/underworld_dreams.rs");
const FS_MIND_STONE: &str =
    include_str!("../../arcana-cards/src/wth/mind_stone.rs");
const FS_ROGUES_PASSAGE: &str =
    include_str!("../../arcana-cards/src/rtr/rogue_s_passage.rs");
const FS_DIMIR_GUILDGATE: &str =
    include_str!("../../arcana-cards/src/rtr/dimir_guildgate.rs");
const FS_GLORIOUS_ANTHEM: &str =
    include_str!("../../arcana-cards/src/po2/glorious_anthem.rs");
const FS_BONESPLITTER: &str =
    include_str!("../../arcana-cards/src/mrd/bonesplitter.rs");
// Wave-2 Aura seed exemplars.
const FS_HOLY_STRENGTH: &str =
    include_str!("../../arcana-cards/src/lea/holy_strength.rs");
const FS_PACIFISM: &str =
    include_str!("../../arcana-cards/src/mir/pacifism.rs");
const FS_CHANDRA_PYROMASTER: &str =
    include_str!("../../arcana-cards/src/m15/chandra_pyromaster.rs");

// =============================================================================
// shared target-card spec block
// =============================================================================

/// Render the target card's identifying fields as a compact,
/// labelled block for insertion into user prompts. Keeps every
/// per-shape template consistent on what's given to the model.
fn card_spec(card: &Card) -> String {
    let mut lines: Vec<String> = Vec::new();
    lines.push(format!("Name: {}", card.name));
    // Front-face accessors: MDFC (and other faces-only layouts) leave
    // these empty at the top level — the printed cost / P/T / colors
    // live on `card_faces[0]`. Without the fallback the spec would
    // omit the mana cost, P/T, and colors entirely and the model
    // would have to guess them (the source of systematic MDFC
    // mis-transcription).
    if let Some(cost) = card.front_mana_cost() {
        lines.push(format!("Mana cost: {cost}"));
    }
    lines.push(format!("Type line: {}", card.type_line));
    if let (Some(p), Some(t)) = (card.front_power(), card.front_toughness()) {
        lines.push(format!("Power/Toughness: {p}/{t}"));
    }
    let colors = card.front_colors();
    if !colors.is_empty() {
        lines.push(format!("Colors: {}", colors.join(", ")));
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
// shared engine-effect catalog (binding-parameterised)
// =============================================================================

/// The `Effect`-construction + card-scripting reference. Resolver-bearing
/// shapes (single-effect spells, triggered abilities) build the same
/// `Vec<Effect>` from the same engine API — only the binding that carries
/// `.controller` / `.source` / `.targets` differs (`entry: &StackEntry`
/// for a spell, `trig: &PendingTrigger` for a triggered ability). The
/// `{BINDING}` marker is resolved by [`effect_catalog`].
///
/// NOTE: `user_single_effect_spell` still carries its own inline copy of
/// this material (it predates the extraction); migrating it onto this
/// const is a clean, behaviour-neutral follow-up.
const ENGINE_EFFECT_CATALOG: &str = r#"ENGINE EFFECT CATALOG — these `Effect` variants are part of the engine API. Construct each EXACTLY as written: use only the field names shown, never add a field (no `optional`, no `count` on `CreateToken`) and never rename one. For 'do this N times' / 'create N tokens', repeat the whole `Effect` value N times in the `vec!` — there is no count field. `p` means a `PlayerId` (use `{BINDING}.controller` for 'you'; for 'target player'/'target opponent' read it from the target — the `TargetChoice::Player` arm). `id` means an `ObjectId` read from `{BINDING}.targets.targets.first()` (single-target shape).

Imports (use these EXACT paths): `Effect`, `TokenDefinition`, `DiscardChoice`, `KeywordAbility`, `DelayedWhen`, `DelayedAction` from `arcana_core::effects`; `Duration` from `arcana_core::layers`; `CounterKind`, `ManaColor` from `arcana_core::types`; `Zone` from `arcana_core::zones`; `ObjectFilter`, `TargetRequirement`, `TargetFilter`, `TargetCount`, `TargetChoice`, `ControllerConstraint` from `arcana_core::targets`; `ManaUnit` from `arcana_core::mana`; `ReplacementDuration` from `arcana_core::replacement`; `script` from `arcana_core` (i.e. `use arcana_core::script;`). (`KeywordAbility` is in `arcana_core::effects`, NOT `arcana_core::types`. The registry module is `arcana_core::registry`, NOT `card_registry`.)

TYPE-LINE RULE: `TypeLine::CREATURE` / `LAND` / `ARTIFACT` / `INSTANT` / `SORCERY` etc. are bitflag CONSTS, not `TypeLine` values. Anywhere a `TypeLine` is needed (an `ObjectFilter`'s `with_types` / `with_types_any`, a `TokenDefinition.types`) write `TypeLine::LAND.into()` for one type, or `TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE)` to combine. Never pass a bare `TypeLine::LAND`, and NEVER a bare bitwise-or `TypeLine::INSTANT | TypeLine::SORCERY` (that's a `u16`) — wrap combined consts in `TypeLine(..)`.

TARGET SPEC — an ability that targets ("target creature", "deals damage to target player") declares `target_requirements: Vec<TargetRequirement>` on its def; a non-targeting ability uses `Vec::new()`. Helper constructors: `TargetRequirement::target_creature()`, `TargetRequirement::target_player()`, `TargetRequirement::any_target()`. For anything else use the struct literal with EXACTLY these THREE fields — `filter`, `count`, `controller` (omit none):
- `TargetRequirement { filter: TargetFilter::Permanent(ObjectFilter::new().with_types(TypeLine::ARTIFACT.into())), count: TargetCount::Exactly(1), controller: None }`
- `TargetFilter` — the COMPLETE set, never invent one: `TargetFilter::Creature`, `TargetFilter::Player`, `TargetFilter::AnyTarget`, `TargetFilter::Permanent(ObjectFilter)`, `TargetFilter::Spell(ObjectFilter)`, `TargetFilter::Card { zone: Zone::Graveyard(0), filter: ObjectFilter::creature() }` (a card in a graveyard).
- `TargetCount` — the COMPLETE set: `TargetCount::Exactly(1)`, `TargetCount::UpTo(n)`, `TargetCount::Any`. `controller` is `Option<ControllerConstraint>` — use `None` and constrain inside the `ObjectFilter`.
The handler READS a chosen target from `{BINDING}.targets` — match it, never copy it (`TargetChoice` is NOT `Copy`):
    let Some(target) = {BINDING}.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(id) = target else { return Vec::new(); };
    // `id` is `&ObjectId`; pass it to an Effect as `target: *id`.
Use the `TargetChoice::Player(p)` arm for a player target (`p` is `&PlayerId`).

Card flow (no target — player is `{BINDING}.controller` or a target player):
- `Effect::DrawCards { player: p, count: u32 }`
- `Effect::Discard { player: p, count: u32, choice: DiscardChoice::ControllerChooses }`  (or `::OpponentChooses` / `::Random`)
- `Effect::Mill { player: p, count: u32 }`  ·  `Effect::Surveil { player: p, count: u32 }`  ·  `Effect::Scry { player: p, count: u32 }`
- `Effect::DigTopN { player: p, count: u32, filter: Option<ObjectFilter>, rest: DigRest }`  — "Look at the top N cards of your library. You may put one [card matching filter] into your hand. Put the rest [per rest]." The canonical impulse / dig. The chosen card always goes to hand; `filter: None` means any card is takeable, else build an `ObjectFilter` (e.g. `Some(ObjectFilter { types: Some(TypeLine::CREATURE.into()), ..ObjectFilter::default() })` for "a creature card"). `rest` is `DigRest::BottomRandom` ("put the rest on the bottom in a random order" — the common tail) or `DigRest::Graveyard` ("the rest into your graveyard"). Import `DigRest` from `arcana_core::effects`, `ObjectFilter` from `arcana_core::targets`. Single-take only — "put TWO into your hand" is not expressible. Use this for "look at the top N, ... into your hand" instead of GAP-ing.
- `Effect::RevealUntil { player: p, filter: ObjectFilter, found_dest: RevealDest, rest: DigRest, max_reveal: Option<u32> }`  — "Reveal cards from the top of your library until you reveal a [filter] card. Put it [per found_dest], and the rest [per rest]." Deterministic: the FIRST matching card is taken (no choice). `found_dest` is `RevealDest::Hand` ("into your hand") or `RevealDest::Battlefield` ("onto the battlefield"). `rest` is `DigRest::BottomRandom` or `DigRest::Graveyard`. `max_reveal: None` digs the whole library; `Some(n)` caps the depth. Import `RevealDest` and `DigRest` from `arcana_core::effects`. Use this for "reveal until you find a ~" rather than GAP-ing. (Distinct from `DigTopN`, which looks at a fixed count with an optional pick.)

Life:
- `Effect::GainLife { player: p, amount: u32 }`  ·  `Effect::LoseLife { player: p, amount: u32 }`
- `Effect::GainEnergy { player: p, amount: u32 }`  — "you get N {E}" (energy counters). Use instead of GAP-ing energy gain. (Spending energy as a cost is not yet a cost field — GAP the spend side if a card pays {E}.)
- `Effect::SetLifeTotal { player: p, amount: u32 }`

Single permanent / card target (`id` from the first target):
- `Effect::DestroyPermanent { target: id }`  ·  `Effect::ExilePermanent { target: id }`
- `Effect::ReturnToHand { target: id }`  (bounce a permanent)
- `Effect::Tap { target: id }`  ·  `Effect::Untap { target: id }`
- `Effect::PutOnTopOfLibrary { target: id }`  ·  `Effect::PutOnBottomOfLibrary { target: id }`
- `Effect::ReturnFromGraveyardToHand { target: id }`  ·  `Effect::ReturnFromGraveyardToBattlefield { target: id }`
- `Effect::ReturnFromExileToBattlefield { target: id }`  (blink/flicker return)
- `Effect::ChangeControl { target: id, new_controller: {BINDING}.controller }`  (PERMANENT gain-control — Mind Control / Take Control class).
- `Effect::ChangeControlEot { target: id, new_controller: {BINDING}.controller }`  (Threaten / Act of Treason — gain control until end of turn; the engine schedules an automatic revert at the next end step). Pair with `Effect::Untap { target: id }` and `Effect::GrantKeyword { target: id, keyword: KeywordAbility::Haste, duration: Duration::EndOfTurn }` for the full Threaten suite.
- `Effect::ExileFromGraveyard { target: id }`
- `Effect::AddCounters { target: id, kind: CounterKind::PlusOnePlusOne, count: u32 }`  ·  `Effect::RemoveCounters { .. }`. Valid `CounterKind` variants: `PlusOnePlusOne`, `MinusOneMinusOne`, `Loyalty`, `Charge`, `Time`, `Fade`, `Quest`, `Study`, `Poison`, `Energy`, `Shield`, `Stun`, `Lore`, `Defense`, `Level`, and `Named(SmallString)` for any other named counter (intern the name). Use the matching variant — e.g. "stun counter" → `CounterKind::Stun`, "charge counter" → `CounterKind::Charge` — and only fall back to `Named` for a counter with no dedicated variant.
- `Effect::Pump { target: id, power: i32, toughness: i32, duration: Duration::EndOfTurn, keywords: vec![] }`  ('+X/+X until end of turn'; granted evergreen `KeywordAbility` values go in `keywords`)
- `Effect::SetBasePT { target: id, power: i32, toughness: i32, duration: Duration::EndOfTurn }`
- `Effect::GrantKeyword { target: id, keyword: KeywordAbility::Trample, duration: Duration::EndOfTurn }`
- `Effect::CantBeBlocked { target: id, duration: Duration::EndOfTurn }`  — "target creature can't be blocked this turn". For a creature's own static "~ can't be blocked", target its own id with `Duration::WhileSourceOnBattlefield`. Use this instead of GAP-ing "can't be blocked".
- `Effect::ForbidBlocking { target: id, duration: Duration::EndOfTurn }`  — "target creature can't block this turn".
- `Effect::LoseAllAbilities { target: id, duration: Duration::EndOfTurn }`  — "target creature loses all abilities" (strips keyword abilities; pair with `Effect::SetPt` for "becomes a 1/1 with no abilities"). Use instead of GAP-ing "loses all abilities".
- `Effect::AddType { target: id, types: TypeLine::ARTIFACT.into(), duration: Duration::EndOfTurn }`  — "target becomes an artifact / is also a creature [in addition to its other types]". Additive (keeps existing types). For permanent animation of a noncreature, combine with `Effect::SetPt`. Use instead of GAP-ing type changes.
- `Effect::SetColor { target: id, colors: ColorSet::black(), duration: Duration::EndOfTurn }`  — "target becomes black" (replaces the color set; for two colors use `ColorSet::black() | ColorSet::green()`). Import `ColorSet` from `arcana_core::types`.
- `Effect::Regenerate { target: id }`  ·  `Effect::Transform { target: id }`

Two-object / combat:
- `Effect::Fight { a: id1, b: id2 }`

Tokens:
- `Effect::CreateToken { controller: p, token: TokenDefinition { name, colors, types, subtypes, power, toughness, keywords: vec![], abilities: vec![] } }`  (repeat the `Effect::CreateToken` for 'create N'). `TokenDefinition` has NO `Default` — write ALL EIGHT fields every time: `name` is an interned `SmallString` (intern the token name in `register` and pass it via a module-level helper or rebuild via `reg.interner()` lookup in the resolver), `types` is a `TypeLine` value (`TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE)`), `keywords`/`abilities` are usually `vec![]`.
- `Effect::CreateTokenSacEot { controller: p, token: TokenDefinition { /* same eight fields */ .. } }`  — `CreateToken` plus a one-shot delayed destroy at the next end step (token-faithful 'create, then sacrifice at end of turn').
- `Effect::CreateCommodityToken { controller: p, kind: CommodityToken::Treasure, count: N }`  — mints N artifact tokens of a canonical commodity (Treasure / Clue / Food / Powerstone / Incubator). The engine wires their printed activated ability automatically: Clue and Food resolve faithfully; Treasure and Powerstone work with a FIDELITY GAP (current placeholder adds colorless mana — the color choice and Powerstone's "can't be spent on nonartifact spells" rider are future work); Incubator mints the bare token only (its {{2}}: Transform is still engine debt). Blood (`{{1}}, {{T}}, Sacrifice, Discard a card: Draw a card`) and Map (`{{1}}, {{T}}, Sacrifice: target creature you control explores`, sorcery-speed) are also wired. PREFER this over hand-rolling a `TokenDefinition` for these commodities — that's how the activations get wired. Import `CommodityToken` from `arcana_core::effects`. Variants: `Treasure`, `Clue`, `Food`, `Powerstone`, `Incubator`, `Blood`, `Map`.

Common token recipes — build via plain `Effect::CreateToken`. Pre-intern the subtype string at REGISTRATION time via `reg.interner_mut().intern("Treasure")`; at resolve time read it via `reg.interner().lookup("Treasure")`. The token's `name` field is its primary subtype's interner id.
- Treasure / Clue / Food / Powerstone / Incubator → use `Effect::CreateCommodityToken` (see above). Do NOT hand-roll TokenDefinitions for these; the bespoke variant is the only path that gets the activations wired.
- 1/1 creature tokens (Servo, Thopter, Soldier, Spirit, Saproling, Zombie, Cat, Bird, Snake, Squirrel, Goblin, Elemental, …): `colors` per oracle, `TypeLine::CREATURE.into()` for non-artifact, `TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE)` for Servo / Thopter / Construct artifact creatures, subtype string interned + inserted into a `SubtypeSet`, `power/toughness: Some(PtValue::Fixed(N))`. Include `keywords` only for keywords printed on the token (Flying for Thopter/Spirit/Drake/Bird, Deathtouch for Snake, Lifelink for some Vampire/Cat tokens — read the oracle, do not invent).

Mana:
- `Effect::AddMana { player: p, mana: vec![ManaUnit::plain(ManaColor::Red, {BINDING}.source); 3] }`  (one `ManaUnit::plain(color, source)` per pip; `source` is `{BINDING}.source`).

Damage / prevention:
- `Effect::DealDamage { target: DamageTarget::Object(id), amount: u32, source: {BINDING}.source }`  (`DamageTarget::Player(p)` for a player; import `DamageTarget` from `arcana_core::events`)
- `Effect::DealDamageDivided { source: {BINDING}.source, targets: vec![DamageTarget::Object(id1), DamageTarget::Object(id2)], total: 3 }`  — "deals N damage divided as you choose among …". Declare the matching `TargetRequirement` with `count: TargetCount::UpTo(k)` (or `TargetCount::Any` for "any number of targets"), then in the resolver map EVERY chosen target into the `targets` vec: `let targets = {BINDING}.targets.targets.iter().filter_map(|t| match t { TargetChoice::Object(id) => Some(DamageTarget::Object(*id)), TargetChoice::Player(p) => Some(DamageTarget::Player(*p)), _ => None }).collect();` then `vec![Effect::DealDamageDivided { source: {BINDING}.source, targets, total: N }]`. The engine spreads `total` across the chosen targets (even split, earliest absorb the remainder — the player's exact division is a documented fidelity gap). Use this instead of GAP-ing "divided damage" or emitting 1 damage to the first target.
- `Effect::PreventDamage { target: DamageTarget::Object(id), amount: Some(3), duration: ReplacementDuration::EndOfTurn }`  (`amount: None` prevents ALL damage to that one target, any source)
- `Effect::RedirectDamage { from: DamageTarget::Object(id), to: DamageTarget::Player({BINDING}.controller), duration: ReplacementDuration::EndOfTurn }`  — CR 614.9 "the next time a source would deal damage to `from`, it's dealt to `to` instead" (Palisade Giant / "redirect damage to you"). Use instead of GAP-ing damage redirection.
- `Effect::PreventDamageFrom { source_filter: ObjectFilter::creature(), target_filter: TargetFilter::Player, amount: None, duration: ReplacementDuration::EndOfTurn }`  — source/target-FILTERED prevention: "prevent all damage that would be dealt by [source_filter] to [target_filter]" (Fog Bank-style board-wide, "prevent all damage from flying creatures to you"). `amount: Some(n)` for "up to n". Filter both ends; for "prevent all damage to you (any source)" use a broad `source_filter` (`ObjectFilter::permanent()`) + `TargetFilter::Player`. Use instead of GAP-ing board-wide / source-filtered prevention.

Sacrifice:
- `Effect::Sacrifice { player: p, filter: ObjectFilter::creature(), count: u32 }`

Search the library (shuffle is automatic):
- `Effect::TutorToHand { player: p, filter: ObjectFilter::creature(), reveal: true }`
- `Effect::TutorToBattlefield { player: p, filter: ObjectFilter::creature(), tapped: false }`
- SEARCH BY EXACT CARD NAME ("search your library for a card named Llanowar Sentinel") IS expressible: `ObjectFilter` has a `name: Option<SmallString>` field, and the effect fn's `reg` gives a read-only handle lookup. In the effect fn write `let nm = reg.interner().lookup("Llanowar Sentinel");` then `Effect::TutorToHand { player: p, filter: ObjectFilter { name: nm, ..ObjectFilter::default() }, reveal: true }`. (`lookup` returns `Option<SmallString>`; an `Option` is exactly the field type, so pass it straight through — a name never interned this game yields `None` and matches nothing, which is correct: you found no such card.) Combine with a type filter if the text restricts it ("a creature card named ~"). Use this instead of GAP-ing "tutor by name not expressible".
- `Effect::Reanimate { player: p, filter: ObjectFilter::creature(), from_zone: Zone::Graveyard(p) }`  — non-targeted "return a creature from a graveyard"; for the targeted form prefer `ReturnFromGraveyardToBattlefield { target: id }` (above).
- `Effect::PutFromHandOntoBattlefield { player: p, filter: ObjectFilter::creature(), tapped: false }`  — "put a [filter] card from your hand onto the battlefield" (Elvish Piper / Quicksilver Amulet class). Posts a pick over the player's OWN hand; no-op if nothing matches. Use this instead of GAP-ing hand→battlefield puts.
- `Effect::PutFromHandOntoBattlefieldTappedAttacking { player: p, filter: ObjectFilter::creature() }`  — "… tapped and attacking" (Kaalia class).
- `Effect::ChoosePlayerThen { chooser: p, opponents_only: false, then: Box::new(<Effect>) }`  — "choose a player; that player [draws/loses life/discards/sacrifices…]": the engine posts the player pick and substitutes the chosen player into `then`'s player field (placeholder ignored).
- `Effect::ExileUntilSourceLeaves { source: trig.source, target: id }`  — O-ring templating: "exile target [permanent] until ~ leaves the battlefield" (Oblivion Ring, Banishing Light, Journey to Nowhere, Fiend Hunter). The engine links the exiled card to the jailer and returns it to the battlefield under its owner's control the moment the jailer leaves — do NOT hand-roll the return trigger, and never GAP "until leaves linkage".
- `Effect::ChooseAnyNumberFromZone { chooser: {BINDING}.controller, zone: Zone::Battlefield, filter: <ObjectFilter>, action: PickAction::ReturnToHand }`  — the PLAYER-CHOSEN VARIABLE-COUNT selection: "you may return/discard/sacrifice/exile ANY NUMBER of [filter]". The engine posts a min-0 / max-all pick over the matching cards, then applies `action`. Import `use arcana_core::effects::PickAction;`. `zone` ∈ `Zone::Battlefield` / `Zone::Hand({BINDING}.controller)` / `Zone::Graveyard({BINDING}.controller)`. `action` ∈ `PickAction::ReturnToHand` ("return any number of Swamps you control to your hand" — Sweep) / `PickAction::Discard` ("discard any number of cards") / `PickAction::Sacrifice` ("sacrifice any number of …") / `PickAction::Exile`. Build `filter` as usual (`ObjectFilter::creature().controlled_by(ControllerConstraint::You)`, `script::subtype_filter(reg, "Swamp")`, etc.). Use this instead of GAP-ing "any number of" / variable-count player choices.

Mechanics — named MTG primitives the engine implements. Use the named variant instead of trying to assemble the effect from primitives:
- `Effect::Proliferate`  — "Proliferate." Adds another counter of an already-present kind to every permanent / player that has at least one counter.
- `Effect::Manifest { player: p }`  — "Manifest the top card of your library." Face-down 2/2 creature, may be flipped face up by paying its mana cost.
- `Effect::Goad { target: id, goader: {BINDING}.controller, duration: Duration::EndOfTurn }`  — "Goad target creature." The goaded creature must attack each combat, can't attack `goader`.
- `Effect::ForbidAttacking { target: id, duration: Duration::EndOfTurn }`  — "Target creature can't attack" (counterpart to `Goad`).
- `Effect::Cascade { source: {BINDING}.source, controller: {BINDING}.controller }`  — "Cascade" body (exile-until-lower-cost-card, free-cast it).
- `Effect::CopySpell { target: id }`  — "Copy target spell" (`id` is the spell's stack-object id).
- `Effect::CopyPermanent { target: id }`  — "Create a token that's a copy of target permanent/creature." This MINTS A NEW TOKEN copy (it does not transform an existing object), so it IS the answer for any "create a token that's a copy of …" line — use it instead of GAP-ing "no token-copy effect". For "copy of target X", read `id` from `{BINDING}.targets.targets.first()` (the `TargetChoice::Object(id)` arm). For "create a token that's a copy of EACH creature you control" (or "of it"/"of this creature"), build one `CopyPermanent` per id and wrap them: `let ids = script::ids_matching(state, &ObjectFilter::creature().controlled_by(ControllerConstraint::You), {BINDING}.controller); Effect::Sequence(ids.into_iter().map(|id| Effect::CopyPermanent { target: id }).collect())` (note: `Effect::ForEach` is a COUNT primitive that reruns ONE fixed effect — it does NOT substitute per-id, so use `Sequence` over the ids for per-id copies).
- `Effect::Attach { equipment_or_aura: {BINDING}.source, target: id }`  — Equip / attach an Aura. For an ETB-Aura ("enchant target creature" — Auramancer's-class), pass `{BINDING}.source` as the Aura and `id` as the chosen target.
- `Effect::BecomeRenowned { target: id }`  — "Target permanent becomes renowned" (CR 702.111).
- `Effect::Explore {{ player: {BINDING}.controller, target: id }}`  — "[creature] explores" (CR 701.40). Reveals top of `player`'s library; lands go to hand, nonlands put a +1/+1 counter on `target` and prompt the controller for the may-mill. For "this creature explores", pass `target: {BINDING}.source` (entering creature) or `trig.entering_object().unwrap_or(trig.source)`. For "target creature explores", read `target` from `{BINDING}.targets.targets.first()` via the `TargetChoice::Object(id)` match.
- `Effect::Discover {{ player: {BINDING}.controller, mana_value: N }}`  — "Discover N" (CR 701.49). Exile cards off the top of `player`'s library until a nonland card with mv ≤ N is exiled; the controller is prompted to cast it for free or put it into hand; the rest go to the bottom in random order. Match the N in the rules text exactly ("Discover 3" → `mana_value: 3`).
- `Effect::Amass { controller: {BINDING}.controller, count: N, army_subtype, race_subtype }`  — "Amass N" / "Amass <Race> N" (CR 701.46). Grows an Army you control (or mints a 0/0 black Army token) by N +1/+1 counters. `army_subtype` / `race_subtype` are interned subtype handles — get them in the effect fn via the read-only lookup: `let army_subtype = reg.interner().lookup("Army").unwrap_or_default(); let race_subtype = reg.interner().lookup("Zombie").unwrap_or_default();` (plain "Amass N" uses race "Zombie"; "Amass Goblins N" uses "Goblin", "Amass Orcs N" → "Orc", etc. — intern the named race). Then `vec![Effect::Amass { controller: {BINDING}.controller, count: N, army_subtype, race_subtype }]`. The create-or-grow + counters happen atomically. Use this instead of GAP-ing Amass.
- `Effect::NameCardAndExile {{ chooser: {BINDING}.controller, target: <player> }}`  — "name a card, then [target] reveals their hand and you exile all cards with that name from their hand, graveyard, and library; they shuffle" (Lost Legacy, Necromentia, Memoricide, Cranial Extraction, Slaughter Games). `target` is whose deck is stripped — for "target opponent" read it from the `TargetChoice::Player` arm, or `script::opponents(state, {BINDING}.controller)[0]` for "target opponent" with no explicit target. The engine picks the named card deterministically (most-copied → highest mana value) — a documented fidelity gap. Use this for "name a card" deck-disruption instead of GAP-ing it.
- `Effect::ImpulseExile {{ player: {BINDING}.controller, count: N }}`  — IMPULSE DRAW: "exile the top N cards of your library. Until end of turn, you may play them." (Reckless Impulse, Wrenn's Resolve, Light Up the Stage.) The engine flags the exiled cards so the controller can cast them at normal cost until end of turn, then the permission lapses. Use this for "exile the top … you may play/cast" — do NOT use `DigTopN` (that goes to hand, not exile-with-play-permission). Match N exactly. (Playing exiled LANDS is a known partial — nonland spells are fully enabled.)
- `Effect::Venture {{ player: {BINDING}.controller }}`  — "Venture into the dungeon" (CR 309). Enters the dungeon's first room (or advances one room if already in a dungeon) and fires that room's effect; completing a dungeon clears the position. Use this for any "venture into the dungeon" payoff instead of GAP-ing it. (Phase-1: a single dungeon — Lost Mine of Phandelver — is modeled; dungeon choice and branch choice are deterministic.)
- `Effect::Incubate {{ controller: {BINDING}.controller, n: N }}`  — "Incubate N" (CR 701.51). Mints a colorless artifact Incubator token with N +1/+1 counters. The printed "{{2}}: Transform this token" activation is deferred engine work (same posture as Treasure / Clue tokens — emit the token cleanly, do not try to author the activated ability). Match N exactly ("Incubate 3" → `n: 3`).
- `Effect::Suspect {{ target: id }}`  — "Suspect target creature" (CR 702.176). Flips the creature's `suspected` flag — the engine then grants menace and disallows blocking while the flag is set. For "suspect this creature": `target: {BINDING}.source`. For "suspect target creature": read from `{BINDING}.targets.targets.first()` via `TargetChoice::Object(id)`.
- `Effect::GrantTriggeredAbility {{ target: id, ability: Box::new(<TriggeredAbilityDef>), duration: Duration::EndOfTurn }}`  — grant a creature an ARBITRARY triggered ability ("until end of turn, whenever ~ deals combat damage to a player, draw a card" — Warrior's Lesson). Build the `TriggeredAbilityDef` exactly like a printed one (a fresh free `fn` for its `effect`), BUT its `id` MUST be in the granted range: `id: arcana_core::triggers::GRANTED_TRIGGER_ID_BASE + 1` (so the engine routes it). Import `use arcana_core::triggers::{{TriggeredAbilityDef, TriggerCondition, TriggerFrequency, GRANTED_TRIGGER_ID_BASE}};`. `duration` ∈ `Duration::EndOfTurn` (most "until end of turn" grants — auto-removed at cleanup) / `Duration::WhileSourceOnBattlefield` (static "has '<triggered ability>'"). Other `Duration` values for ContinuousEffects: `Duration::Permanent` (no stated duration), `Duration::UntilYourNextTurn(p)` ('until your next turn'), `Duration::UntilNextUpkeepOf(p)` ('until your next upkeep' — Halfdane/Erhnam class). For "target creature gains …" read `id` from `{BINDING}.targets.targets.first()`; for "~ gains …" use `{BINDING}.source`. Use this instead of GAP-ing "grant a triggered ability".
- NOT IMPLEMENTED: `Conjure` (Alchemy / Arena-only — creating a card by name). There is NO `Effect::Conjure` variant. If the oracle says "conjure a card named X", emit `Vec::new()` with `// GAP: Conjure not modeled (Arena-only mechanic; would need registry-by-name lookup in Effect::execute)`. Do NOT invent the variant.

Composites (wrap the above):
- `Effect::ForEach { targets: vec![/* ObjectIds */], effect: Box::new(Effect::DestroyPermanent { target: arcana_core::objects::NULL_OBJECT_ID }) }`  — 'affect EACH/ALL matching': enumerate ids from `state` (see CARD SCRIPTING) and apply the inner effect once per id.
- `Effect::Conditional { condition, then: Box::new(..), otherwise: Some(Box::new(..)) }`  ·  `Effect::Sequence(vec![..])`
- `Effect::FlipCoin { player: p, win: Box::new(<effect>), lose: Some(Box::new(<effect>)) }`  — "Flip a coin. If you win the flip, [win]. If you lose, [lose]." Fair, deterministic, replayable. For "if you win the flip, X" with no downside use `lose: None`; for "flip a coin, if you LOSE [bad]" put the punishment in `lose` and pass `win: Box::new(Effect::Sequence(vec![]))`. Wrap multi-step branches in `Effect::Sequence`. Use this instead of GAP-ing coin flips.
- `Effect::DelayedAction { source: id, controller: {BINDING}.controller, when: DelayedWhen::NextEndStep, action: DelayedAction::Sacrifice }`  — schedule a one-shot on a KNOWN id; when ∈ `NextEndStep` / `NextUpkeep` (the next turn's upkeep) / `ThisDies`; action ∈ `Sacrifice` / `Exile` / `ReturnToHand` / `ReturnFromExileToBattlefield` / `ControllerDrawsCard` ('that player draws a card at the beginning of the next turn's upkeep' — schedule with `controller:` = that player).
- `Effect::OptionalPayment {{ chooser: {BINDING}.controller, cost: OptionalPaymentKind::Mana(ManaCost::parse(\"{{1}}\").expect(\"valid cost\")), then: Box::new(Effect::DrawCards {{ player: {BINDING}.controller, count: 1 }}), else_effect: None }}` — "You may pay X. If you do, Y. (Otherwise, Z.)" Use this for ANY "you may pay N" / "unless you pay N" / "unless they pay N life" gate. Import: `use arcana_core::actions::OptionalPaymentKind;`. `cost` ∈ `OptionalPaymentKind::Mana(ManaCost::parse(\"{{B}}\")…)` for mana payments, or `OptionalPaymentKind::Life(2)` for life payments. The "you may pay X. If you do, Y" shape uses `then: Box::new(<the Y effect>)`, `else_effect: None`. The "Z unless you pay X" shape (Sangrophage, Plague Fiend) inverts the polarity — the engine still prompts on `chooser`, but the punishment goes in `else_effect` and `then` is `Box::new(Effect::Sequence(vec![]))` (no-op on pay). `chooser` is whoever is paying — typically `{BINDING}.controller` ('you'), but for 'unless its controller pays' / 'unless that player pays' read it off the relevant target/event (e.g. `script::target_controller(state, id, {BINDING}.controller)`).

CARD SCRIPTING — when an amount or a board-wide set is computed at resolution ('equal to its power', 'for each creature you control', 'destroy all Goblins'), the handler's FIRST parameter is the live `&GameState` (name it `state`, not `_state`) and you may call ONLY these total, panic-free helpers from `arcana_core::script` (add `use arcana_core::script;`). Each returns a plain value — bind it to a `let`, then put it in an ordinary literal-amount `Effect`:
- `script::count_matching(state, &filter, {BINDING}.controller) -> u32`  — battlefield permanents matching an `ObjectFilter`.
- `script::ids_matching(state, &filter, {BINDING}.controller) -> Vec<ObjectId>`  — the matching ids; feed into `Effect::ForEach { targets: <this>, effect: Box::new(..) }`.
- `script::subtype_filter(reg, "Goblin")`  — an `ObjectFilter` for a named creature subtype (resolver's 3rd param — name it `reg`, not `_reg`).
- `ObjectFilter` refinements (chain onto `creature()` / `permanent()` / `subtype_filter(..)`): `.controlled_by(ControllerConstraint::You|Opponent)`, `.with_colors(ColorSet::black())`, `.without_colors(..)`, `.with_types_any(TypeLine(TypeLine::INSTANT | TypeLine::SORCERY))` (TYPE OR, for 'instant or sorcery' / 'creature or planeswalker'), `.without_types(TypeLine::ARTIFACT.into())`, `.with_max_cmc(n)` / `.with_min_cmc(n)`, `.with_min_power(n)` / `.with_max_power(n)` / `.with_max_toughness(n)`, `.tokens_only()` / `.nontoken()`, `.tapped_only()` / `.untapped_only()`, `.with_supertypes(SupertypeSet::new().with(SupertypeSet::LEGENDARY))` ('Legendary creature', 'Basic land', 'Snow permanent'), `.without_supertypes(SupertypeSet::new().with(SupertypeSet::LEGENDARY))` ('nonlegendary creature'), `.with_subtypes_any(vec![reg.interner_mut().intern("Human"), reg.interner_mut().intern("Warrior")])` (SUBTYPE OR — author this in `register` since it needs the interner; pass the resulting filter into the resolver via a closure). For a single subtype prefer `script::subtype_filter(reg, "Spirit")`.
- `script::power_of(state, id) -> i32` · `script::toughness_of(state, id) -> i32`.
- `script::devotion(state, {BINDING}.controller, colors) -> u32`  — CR 700.5 devotion: colored mana symbols of `colors` among the mana costs of permanents you control ("X is your devotion to green" / "...to white and black"). `colors` is a `ColorSet`: `ColorSet::green()` for one color, `ColorSet::white() | ColorSet::black()` for two (import `use arcana_core::types::ColorSet;`). Hybrid/Phyrexian pips count for each of their colors. Use for any "devotion to ~" amount instead of GAP-ing.
- `script::hand_size(state, p) -> u32` · `script::graveyard_size(state, p) -> u32` · `script::library_size(state, p) -> u32` · `script::life(state, p) -> i32`.
- `script::all_players(state) -> Vec<PlayerId>` · `script::opponents(state, {BINDING}.controller) -> Vec<PlayerId>`  — for 'each player' / 'each opponent': build one inner `Effect` per player, wrap in `Effect::Sequence`.
- Per-turn event counters (use for "[N] died this turn" / "spells cast this turn" / "drew this turn" / "discarded this turn" — the turn slice resets on each new turn):
  · `script::creatures_of_subtype_died_this_turn(state, reg, "Zubera") -> u32`  ('for each Zubera that died this turn' — Silent-Chant Zubera class)
  · `script::spells_cast_this_turn(state, &filter, {BINDING}.controller) -> u32`  (filtered count; e.g. instant-or-sorcery spells cast this turn → use `ObjectFilter::new().with_types_any(TypeLine(TypeLine::INSTANT | TypeLine::SORCERY))`)
  · `script::cards_drawn_this_turn(state, p) -> u32`  ·  `script::cards_discarded_this_turn(state, p) -> u32`
- `script::target_controller(state, id, {BINDING}.controller) -> PlayerId`  — for 'target's controller' / 'its owner'.
MANDATORY: if the oracle says 'for each', 'for every', 'equal to the number of', 'equal to its power/toughness', or 'X is the number of', the amount is DYNAMIC — compute it with a `script::*` helper (or `Effect::ForEach` over `script::ids_matching`). Do NOT hardcode a literal and do NOT `// GAP` the scaling while emitting a fixed-size effect: a literal where the text is dynamic is a materially WRONG card, auto-quarantined by verify. If you genuinely cannot compute it, GAP the WHOLE effect (`Vec::new()`).

No other `state` access is permitted (no field access, no other methods) — anything else is a GAP. Amount fields are `u32`; a possibly-negative `i32` (a power, a life total) becomes an amount via `.max(0) as u32`."#;

/// [`ENGINE_EFFECT_CATALOG`] with the `{BINDING}` marker resolved to the
/// shape's actual binding identifier (`entry` / `trig`).
fn effect_catalog(binding: &str) -> String {
    ENGINE_EFFECT_CATALOG.replace("{BINDING}", binding)
}

/// The `TriggerCondition` reference for the triggered-creature shape.
/// The two few-shots only show `SelfEntersBattlefield` and `SpellCast`;
/// this enumerates the rest so the model picks an existing variant
/// instead of inventing one (the dominant T3 layer-1 failure).
const TRIGGER_CONDITION_CATALOG: &str = r#"TRIGGER CONDITION CATALOG — `TriggeredAbilityDef.trigger_condition` is a `TriggerCondition`. Pick the ONE variant matching the oracle's trigger clause; these are the COMPLETE set. Import `TriggerCondition` (and `TriggerSelf` if used) from `arcana_core::triggers`, `Step` / `Phase` from `arcana_core::turn`, `ControllerConstraint` from `arcana_core::targets`. `ControllerConstraint` ∈ `You` / `Opponent` / `Any`.

Self — the creature itself:
- `TriggerCondition::SelfEntersBattlefield` (unit) — "When ~ enters [the battlefield]". (Elvish Visionary reference.)
- `TriggerCondition::SelfDies` (unit) — "When ~ dies".
- `TriggerCondition::SelfAttacks` (unit) — "Whenever ~ attacks".
- `TriggerCondition::SelfBecomesBlocked` (unit) — "When ~ becomes blocked".
- `TriggerCondition::SelfBlocks` (unit) — "Whenever ~ blocks" / "Whenever ~ blocks a creature".
- `TriggerCondition::SelfBlocksOrBecomesBlocked` (unit) — "Whenever ~ blocks or becomes blocked [by a creature]" (Aisling Leprechaun, Tangle Asp, Rock Basilisk, Sawtooth Ogre, Corrosive Ooze class). Picks BOTH sides of a block in a single trigger. Use `trig.other_combatant()` to read the OTHER creature ("that creature").
- `TriggerCondition::SelfBecomesTapped` (unit) — "Whenever ~ becomes tapped".
- `TriggerCondition::BecomesTapped { filter: ObjectFilter }` — "Whenever a [filter] becomes tapped" (any permanent, not just this one): "Whenever an Island becomes tapped" → `filter: script::subtype_filter(reg, "Island")`; "a creature an opponent controls" → `ObjectFilter::creature().controlled_by(ControllerConstraint::Opponent)`.
- `TriggerCondition::SelfSpecializes` (unit) — "When this creature specializes" (CR 711, Bloomburrow). Use this for the trigger; the specialize action itself (the colored-back transform) emits `Effect::Specialize {{ target }}` and is a documented partial (the event fires the trigger; the face swap is unmodeled).
- `TriggerCondition::SelfAttacksUnblocked` (unit) — "Whenever ~ attacks and isn't blocked".
- `TriggerCondition::SelfAttacksAlone` (unit) — "Whenever ~ attacks alone" (it was the only declared attacker).
- `TriggerCondition::AttacksAlone { filter: ObjectFilter }` — "Whenever a [filter] creature you control attacks alone" (the watcher isn't the attacker): `filter: ObjectFilter::creature().controlled_by(ControllerConstraint::You)` (+ subtype). Read the sole attacker via `trig.lone_attacker() -> Option<ObjectId>`.
- `TriggerCondition::SelfTransforms { to_face: Option<u8> }` — "When this creature transforms [into <back-face name>]": `to_face: Some(1)` = into the back face, `Some(0)` = back to front, `None` = either direction.
- `TriggerCondition::SelfIsDealtDamage { combat_only: bool }` — "Whenever ~ is dealt damage" → `combat_only: false`; "Whenever ~ is dealt combat damage" → `combat_only: true`.
- `TriggerCondition::SelfBecomesTarget { caster: ControllerConstraint }` — CR 702.21a "Whenever ~ becomes the target of a spell or ability [an opponent controls]". `caster` ∈ `You`/`Opponent`/`Any`. (Matches spells + activated abilities only; triggered-ability targets are chosen mid-resolution and don't emit the event.)

Spell cast:
- `TriggerCondition::SpellCast { filter: Option<ObjectFilter>, caster: ControllerConstraint }` — "Whenever you cast a spell" → `caster: ControllerConstraint::You, filter: None`. "an opponent casts" → `caster: ControllerConstraint::Opponent`. Filtered ("an instant or sorcery") → `filter: Some(ObjectFilter::new().with_types_any(TypeLine(TypeLine::INSTANT | TypeLine::SORCERY)))`. (Young Pyromancer reference.)

Turn structure:
- `TriggerCondition::StepBegins { step: Step, whose: ControllerConstraint }` — "At the beginning of your upkeep" → `step: Step::Upkeep, whose: ControllerConstraint::You`; "each upkeep" → `whose: Any`. `Step` ∈ `Untap` `Upkeep` `Draw` `Main` `BeginCombat` `DeclareAttackers` `DeclareBlockers` `EndCombat` `End` `Cleanup` (end step = `Step::End`).
- `TriggerCondition::PhaseBegins { phase: Phase, whose: ControllerConstraint }` — "At the beginning of combat on your turn" → `phase: Phase::Combat, whose: You`. `Phase` ∈ `Beginning` `PreCombatMain` `Combat` `PostCombatMain` `Ending`.

Other permanents / events:
- `TriggerCondition::ZoneChange { filter: ObjectFilter, from: Option<Zone>, to: Zone }` — "Whenever a creature enters under your control" → `filter: ObjectFilter::creature().controlled_by(ControllerConstraint::You), from: None, to: Zone::Battlefield`; "whenever a creature dies" → `from: Some(Zone::Battlefield), to: Zone::Graveyard(0)`.
- `TriggerCondition::CreatureAttacks { filter: ObjectFilter }` — "Whenever a creature you control attacks".
- `TriggerCondition::DamageDealt { source_filter: ObjectFilter, target_filter: TargetFilter, combat_only: bool }` — "Whenever ~ deals combat damage to a player" → `target_filter: TargetFilter::Player, combat_only: true`.
- `TriggerCondition::LifeGained { player: ControllerConstraint }` · `TriggerCondition::CardDrawn { player: ControllerConstraint }` · `TriggerCondition::CardDiscarded { player: ControllerConstraint }`.
- `TriggerCondition::Sacrificed { filter: ObjectFilter }` — "Whenever you sacrifice a permanent".
- `TriggerCondition::CounterAdded { on: TriggerSelf, kind: Option<CounterKind>, chapter: Option<u32> }` — "Whenever a +1/+1 counter is put on ~" → `on: TriggerSelf::Source, kind: Some(CounterKind::PlusOnePlusOne), chapter: None`. For Saga chapter dispatch ("II — do X") use `kind: Some(CounterKind::Lore), chapter: Some(2)` — the trigger fires only when the lore-counter-add event's `count` equals 2.

`TriggeredAbilityDef` always: `id` is a per-card `u32` from 1; `intervening_if: None` (unless the oracle has an "if" clause — then `// GAP:` it and use `None`); `trigger_zones: vec![Zone::Battlefield]`; `frequency: TriggerFrequency::EachTime` (or `OncePerTurn` for "once each turn"); `target_requirements: Vec::new()` unless the trigger targets. The effect fn does NOT need to inspect `trig.trigger_event` for the common cases — read `trig.controller` and `trig.source`. If the oracle's trigger truly matches no variant above, pick the closest, add `// GAP: trigger — <describe>`, and do NOT invent a variant or a `GameEvent` arm."#;

/// Worked triggered-ability effect fn — shows the `&PendingTrigger`
/// binding and the script-prelude pattern for a computed amount.
const WORKED_TRIGGER_FN: &str = r#"fn on_trigger(state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    // "draw a card for each creature you control"
    let n = script::count_matching(
        state,
        &ObjectFilter::creature().controlled_by(ControllerConstraint::You),
        trig.controller);
    vec![Effect::DrawCards { player: trig.controller, count: n }]
}"#;

/// Typed `PendingTrigger` accessors — what the trigger fn can pull
/// out of `trig` without pattern-matching `trig.trigger_event` (which
/// is a footgun: invented `GameEvent` variants are a recurring L1
/// fail). T3-only — spell resolvers receive a `&StackEntry` that
/// doesn't have these.
const TRIGGER_PENDING_ACCESSORS: &str = r#"PENDING-TRIGGER ACCESSORS — beyond `trig.controller` / `trig.source` / `trig.targets`, your effect fn can call typed accessors on `trig` that extract event-specific data without pattern-matching `trig.trigger_event` (DO NOT match on `GameEvent` directly — inventing variants is the #1 L1 fail). Each returns `Option<...>` keyed on the event the trigger fired on:
- `trig.dying_object() -> Option<ObjectId>` — pairs with `SelfDies` and graveyard-bound `ZoneChange`. For "deals damage equal to its power" / dies-rider effects: `let n = script::power_of(state, trig.dying_object().unwrap_or(trig.source)).max(0) as u32;`.
- `trig.damage_amount() -> Option<u32>` — pairs with `SelfIsDealtDamage` / `DamageDealt`. For "draw that many cards" / "deals X damage where X = damage taken": `let n = trig.damage_amount().unwrap_or(0);`.
- `trig.damaged_player() -> Option<PlayerId>` — pairs with `DamageDealt`. For "that player discards" / "deals damage to a player → that player loses life": `let Some(p) = trig.damaged_player() else { return Vec::new(); };`.
- `trig.defending_player() -> Option<PlayerId>` — pairs with `SelfAttacks` / `CreatureAttacks` / `SelfAttacksUnblocked`. For "the defending player loses 1 life": `let Some(p) = trig.defending_player() else { return Vec::new(); };`.
- `trig.triggering_caster() -> Option<PlayerId>` — pairs with `SpellCast`. For "that player draws a card" / "that player loses life".
- `trig.entering_object() -> Option<ObjectId>` — pairs with `SelfEntersBattlefield` and battlefield-bound `ZoneChange`. For "put X +1/+1 counters where X = power of the entering creature": `let id = trig.entering_object().unwrap_or(trig.source); let n = script::power_of(state, id).max(0) as u32;`.
- `trig.other_combatant() -> Option<ObjectId>` — pairs with `SelfBlocks` / `SelfBecomesBlocked` / `SelfBlocksOrBecomesBlocked`. The OTHER creature in this block — the attacker if we're blocking, the (first) blocker if we became blocked. For "that creature becomes green" / "destroy that creature": `let Some(id) = trig.other_combatant() else { return Vec::new(); };`.
- `trig.lone_attacker() -> Option<ObjectId>` — pairs with `AttacksAlone { filter }` / `SelfAttacksAlone`: THE sole declared attacker ("it gets +2/+0", "put a counter on it").
- `trig.attacking_creature() -> Option<ObjectId>` — pairs with `CreatureAttacks { filter }` / `SelfAttacks`. THE attacking creature ("whenever a creature attacks, [tap it / it gets +1/+0 / its controller sacrifices…]"): `let Some(id) = trig.attacking_creature() else { return Vec::new(); };`.
- "THAT PLAYER" = the entering permanent's CONTROLLER ("whenever a land enters the battlefield under an opponent's control, that player …"): combine `trig.entering_object()` with a state lookup — `let Some(them) = trig.entering_object().and_then(|id| state.objects.get(id)).map(|o| o.controller) else { return Vec::new(); };`.
- "THAT PLAYER" for step/phase triggers ("at the beginning of each player's upkeep, that player…"): NOT an accessor — steps always belong to the active player and an upkeep trigger resolves during that same upkeep, so read `let them = state.active_player();` in the effect fn. (For "each OPPONENT's upkeep" use `whose: ControllerConstraint::Opponent` on the condition; `state.active_player()` is still the upkeep owner.)
- COUNTER COUNTS ("for each charge counter on ~" / "remove a verse counter"): read `state.objects.get(id).map_or(0, |o| o.count_counters(CounterKind::Charge))`. Set-specific kinds not in the enum: intern the name ONCE in `register` (`let verse = reg.interner_mut().intern("verse");`) and use `CounterKind::Named(verse)` in declarative specs; inside an effect fn (only `&CardRegistry`) recover it with `reg.interner().lookup("verse").map(CounterKind::Named)`. Every named counter is supported — never GAP "X counter not in CounterKind".

DYNAMIC-X (up-to-that-many targets, where the max comes from the trigger event):
- For "return up to that many target permanents" / "exile up to N cards where N = X" / "deals X damage to any target, where X is …" — declare `count: TargetCount::X` on the relevant `TargetRequirement`, then attach a `dynamic_x` resolver to the CardDefinition at registration: `.with_trigger_dynamic_x(<TRIGGER_ID>, |trig: &PendingTrigger| -> u32 { trig.damage_amount().unwrap_or(0) })`. The engine evaluates the closure as the trigger fires and stamps the result into the stack entry's `x_value`; `TargetCount::X` then validates the player's chosen target count against that materialized N.
- Recipe for Cephalid Constable ("deals combat damage to a player → return up to that many target permanents that player controls"): `target_requirements: vec![TargetRequirement { filter: TargetFilter::Permanent(ObjectFilter::permanent()), count: TargetCount::X, controller: None }]` PLUS `.with_trigger_dynamic_x(1, |trig| trig.damage_amount().unwrap_or(0))`.
No imports beyond what's already in scope. These accessors are stable engine API — never pattern-match `trig.trigger_event` instead."#;

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

REFERENCE — Preordain ({{U}} sorcery, 'Scry 2, then draw a card' — shows a no-target resolver returning a multi-effect `vec![Effect::A, Effect::B]`):
```rust
{FS_PREORDAIN}
```

REFERENCE — Servo Exhibition ({{1}}{{W}} sorcery, 'Create two 1/1 colorless Servo artifact creature tokens' — shows `Effect::CreateToken` with a `TokenDefinition` built from an interned subtype name):
```rust
{FS_SERVO_EXHIBITION}
```

ENGINE EFFECT CATALOG — these `Effect` variants are part of the engine API and are ALL permitted in addition to the ones in the references above. Construct each EXACTLY as written: use only the field names shown, never add a field (no `optional`, no `count` on `CreateToken`, no `creature_a` on `Fight`) and never rename one. For 'do this N times' / 'create N tokens', repeat the whole `Effect` value N times in the `vec!` — there is no count field. `p` means a `PlayerId` (use `entry.controller` for 'you'; for 'target player'/'target opponent' read it from the target like Lightning Bolt's `TargetChoice::Player` arm). `id` means an `ObjectId` read from `entry.targets.targets.first()` (single-target shape — see Murder/Lightning Bolt).

Imports (use these EXACT paths): `Effect`, `TokenDefinition`, `DiscardChoice`, `KeywordAbility` from `arcana_core::effects`; `Duration` from `arcana_core::layers`; `CounterKind`, `ManaColor` from `arcana_core::types`; `Zone` from `arcana_core::zones`; `ObjectFilter`, `TargetRequirement`, `TargetFilter`, `TargetCount`, `ControllerConstraint` from `arcana_core::targets`; `ManaUnit` from `arcana_core::mana`; `ReplacementDuration` from `arcana_core::replacement`. (`KeywordAbility` is NOT in `arcana_core::types`; `ControllerConstraint` is NOT in `arcana_core::types`.)

TYPE-LINE RULE: `TypeLine::CREATURE` / `LAND` / `ARTIFACT` / `INSTANT` / `SORCERY` / `PLANESWALKER` etc. are bitflag CONSTS, not `TypeLine` values. Anywhere a `TypeLine` is needed (an `ObjectFilter`'s `with_types` / `with_types_any` / `without_types`, a `TokenDefinition.types`) write `TypeLine::LAND.into()` for one type, or `TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE)` to combine — exactly as the Servo Exhibition reference does. Never pass a bare `TypeLine::LAND` and NEVER a bare bitwise-or like `TypeLine::INSTANT | TypeLine::SORCERY` (that's a `u16`, not a `TypeLine` — the same trap applies to `with_types_any`: wrap the combined consts in `TypeLine(..)`).

Card flow (no target — player is `entry.controller` or a target player):
- `Effect::DrawCards {{ player: p, count: u32 }}`
- `Effect::Discard {{ player: p, count: u32, choice: DiscardChoice::ControllerChooses }}`  (or `::OpponentChooses` / `::Random` — pick per oracle text; 'that player discards' → the target player)
- `Effect::Mill {{ player: p, count: u32 }}`
- `Effect::Surveil {{ player: p, count: u32 }}`  ·  `Effect::Scry {{ player: p, count: u32 }}`
- `Effect::DigTopN {{ player: p, count: u32, filter: Option<ObjectFilter>, rest: DigRest }}`  — 'Look at the top N cards of your library. You may put one [matching filter] into your hand. Put the rest [per rest].' The chosen card always goes to hand; `filter: None` = any card, else an `ObjectFilter` (`Some(ObjectFilter {{ types: Some(TypeLine::CREATURE.into()), ..ObjectFilter::default() }})` for 'a creature card'). `rest` is `DigRest::BottomRandom` or `DigRest::Graveyard`. Import `DigRest` from `arcana_core::effects`. Single-take only. Use this for 'look at the top N ... into your hand' rather than GAP-ing.
- `Effect::RevealUntil {{ player: p, filter: ObjectFilter, found_dest: RevealDest, rest: DigRest, max_reveal: Option<u32> }}`  — 'Reveal from the top of your library until you reveal a [filter] card. Put it [per found_dest], the rest [per rest].' Deterministic — the FIRST match is taken (no choice). `found_dest` is `RevealDest::Hand` or `RevealDest::Battlefield`; `rest` is `DigRest::BottomRandom` or `DigRest::Graveyard`; `max_reveal: None` = whole library, `Some(n)` caps depth. Import `RevealDest`/`DigRest` from `arcana_core::effects`. Use for 'reveal until you find a ~' (distinct from `DigTopN`'s fixed-count optional pick).

Life:
- `Effect::GainLife {{ player: p, amount: u32 }}`  ·  `Effect::LoseLife {{ player: p, amount: u32 }}`
- `Effect::GainEnergy {{ player: p, amount: u32 }}`  — 'you get N energy ({{E}})'. (Spending energy as a cost is not yet expressible — GAP the spend side.)
- `Effect::SetLifeTotal {{ player: p, amount: u32 }}`

Single permanent / card target (`id` from the first target):
- `Effect::DestroyPermanent {{ target: id }}`  ·  `Effect::ExilePermanent {{ target: id }}`
- `Effect::ReturnToHand {{ target: id }}`  (bounce a permanent)
- `Effect::Tap {{ target: id }}`  ·  `Effect::Untap {{ target: id }}`
- `Effect::PutOnTopOfLibrary {{ target: id }}`  ·  `Effect::PutOnBottomOfLibrary {{ target: id }}`
- `Effect::ReturnFromGraveyardToHand {{ target: id }}`  (Raise Dead — target a creature card in a graveyard)
- `Effect::ReturnFromGraveyardToBattlefield {{ target: id }}`  (Reanimate — target a creature card in a graveyard)
- `Effect::ReturnFromExileToBattlefield {{ target: id }}`  (blink/flicker return — the matching primitive for cards exiled by an earlier `Effect::ExilePermanent`; no-op if the target isn't currently in exile)
- `Effect::ExileUntilSourceLeaves {{ source: trig.source, target: id }}`  (O-ring: 'exile target ... until ~ leaves the battlefield' — the engine returns it to its owner automatically when the jailer leaves; do NOT hand-roll the return)
- `Effect::PutFromHandOntoBattlefield {{ player: p, filter: ObjectFilter::creature(), tapped: false }}`  ('put a [filter] card from your hand onto the battlefield' — Elvish Piper class)
- `Effect::PutFromHandOntoBattlefieldTappedAttacking {{ player: p, filter: ObjectFilter::creature() }}`  ('… onto the battlefield tapped and attacking' — Kaalia class; outside combat it enters tapped, not attacking)
- `Effect::ChooseNFromZone {{ chooser: p, zone: Zone::Battlefield, filter: <ObjectFilter>, min: 1, max: 1, action: PickAction::Destroy }}`  ('destroy/sacrifice/exile a [filter] of THAT PLAYER's choice' — the chooser may be any player; actions: `Destroy` / `Sacrifice` / `Exile` / `Discard` / `ReturnToHand`. min is clamped to available candidates.)
- `Effect::ScheduleFloatingTrigger {{ source: id, controller: p, condition: <TriggerCondition>, effect: my_fn, until: FloatingUntil::YourNextTurn }}`  ('until end of turn / until your next turn, WHENEVER [condition], [effect]' — a REPEATING floating window; fires on every match until it lapses. `until` ∈ `FloatingUntil::EndOfTurn` / `FloatingUntil::YourNextTurn`. Import `FloatingUntil` from `arcana_core::effects`.)
- `Effect::ScheduleDelayedEffect {{ source: id, controller: p, when: DelayedWhen::NextUpkeep, effect: my_delayed_fn }}`  (schedule an ARBITRARY effect for the next end step / upkeep: `my_delayed_fn` is a module-level `fn(&GameState, &PendingTrigger, &CardRegistry) -> Vec<Effect>` — bake the card's amounts into it; it runs even if the source has left play.)
- `Effect::ChoosePlayerThen {{ chooser: entry.controller, opponents_only: false, then: Box::new(Effect::LoseLife {{ player: 0, amount: 2 }}) }}`  ('choose a player; that player …' — the engine substitutes the picked player into `then`'s player field; the placeholder value is ignored. `then` may be a `Sequence`.)
- `Effect::ChangeControl {{ target: id, new_controller: entry.controller }}`  (permanent gain-control — Mind Control / Take Control family; the controller change persists.)
- `Effect::ChangeControlEot {{ target: id, new_controller: entry.controller }}`  (Threaten / Act of Treason temporary control — reverts automatically at the next end step. Compose with `Untap` + `GrantKeyword Haste` for the full Threaten package.)
- `Effect::ExileFromGraveyard {{ target: id }}`
- `Effect::AddCounters {{ target: id, kind: CounterKind::PlusOnePlusOne, count: u32 }}`  ·  `Effect::RemoveCounters {{ .. }}`. Valid `CounterKind` variants: `PlusOnePlusOne`, `MinusOneMinusOne`, `Loyalty`, `Charge`, `Time`, `Fade`, `Quest`, `Study`, `Poison`, `Energy`, `Shield`, `Stun`, `Lore`, `Defense`, `Level`, and `Named(SmallString)` for any other named counter. Use the matching variant — 'stun counter' → `CounterKind::Stun`, 'charge counter' → `CounterKind::Charge` — and only fall back to `Named` for a counter with no dedicated variant.
- `Effect::Pump {{ target: id, power: i32, toughness: i32, duration: Duration::EndOfTurn, keywords: vec![] }}`  ('+X/+X until end of turn'; put granted evergreen `KeywordAbility` values in `keywords`)
- `Effect::SetBasePT {{ target: id, power: i32, toughness: i32, duration: Duration::EndOfTurn }}`  ('becomes a 1/1')
- `Effect::GrantKeyword {{ target: id, keyword: KeywordAbility::Trample, duration: Duration::EndOfTurn }}`
- `Effect::CantBeBlocked {{ target: id, duration: Duration::EndOfTurn }}`  — 'target creature can't be blocked this turn'. For a creature's own static 'this can't be blocked', target its own id with `Duration::WhileSourceOnBattlefield`. Use this instead of GAP-ing 'can't be blocked'.
- `Effect::ForbidBlocking {{ target: id, duration: Duration::EndOfTurn }}`  — 'target creature can't block this turn'.
- `Effect::LoseAllAbilities {{ target: id, duration: Duration::EndOfTurn }}`  — 'target loses all abilities' (strips keyword abilities; pair with SetPt for 'becomes a 1/1 with no abilities').
- `Effect::AddType {{ target: id, types: TypeLine::ARTIFACT.into(), duration: Duration::EndOfTurn }}`  — 'target becomes an artifact / is also a creature in addition'. Additive. Combine with `Effect::SetPt` to animate a noncreature.
- `Effect::SetColor {{ target: id, colors: ColorSet::black(), duration: Duration::EndOfTurn }}`  — 'target becomes black' (replaces color set; two colors via `ColorSet::black() | ColorSet::green()`). Import `ColorSet` from `arcana_core::types`.
- `Effect::Regenerate {{ target: id }}`  ·  `Effect::Transform {{ target: id }}`

Two-object / combat:
- `Effect::Fight {{ a: id1, b: id2 }}`  (a fights b — for 'target creature fights another target creature' read two targets)

Tokens:
- `Effect::CreateToken {{ controller: p, token: TokenDefinition {{ .. }} }}`  (see Servo Exhibition for the full `TokenDefinition` shape; repeat the `Effect::CreateToken` for 'create N')
- `Effect::CreateTokenSacEot {{ controller: p, token: TokenDefinition {{ .. }} }}`  — same as `CreateToken` PLUS schedules a one-shot delayed destroy at the beginning of the next end step (token-faithful sacrifice). Use this for 'create N tokens, sacrifice them at the beginning of the next end step' (Thatcher Revolt / Lithobraking / Goblin Sleigh-Ride class) — the resolver cannot reference the new token id so plain `CreateToken` + `DelayedAction` will NOT compose. Repeat for 'create N'.
- `Effect::CreateCommodityToken {{ controller: p, kind: CommodityToken::Treasure, count: N }}`  — mints N artifact tokens of a canonical commodity (Treasure / Clue / Food / Powerstone / Incubator). The engine wires the printed activated ability automatically: Clue and Food resolve faithfully; Treasure and Powerstone work with a FIDELITY GAP (current placeholder adds colorless mana); Incubator mints the bare token only ({{2}}: Transform still engine debt). Blood (rummage: discard a card, draw a card) and Map (target creature you control explores, sorcery-speed) are also wired. PREFER this over hand-rolling a `TokenDefinition`. Variants: `Treasure`/`Clue`/`Food`/`Powerstone`/`Incubator`/`Blood`/`Map`. Import `CommodityToken` from `arcana_core::effects`.

Mana (ritual class):
- `Effect::AddMana {{ player: p, mana: vec![ManaUnit::plain(ManaColor::Red, entry.source); 3] }}`  (Pyretic Ritual: 'Add {{R}}{{R}}{{R}}' — one `ManaUnit::plain(color, source)` per pip; for mixed colors build the `Vec<ManaUnit>` explicitly. `ManaColor::White|Blue|Black|Red|Green|Colorless`. The `source` is the spell's own object id — `entry.source`.)

Damage prevention (Healing Salve class):
- `Effect::PreventDamage {{ target: DamageTarget::Object(id), amount: Some(3), duration: ReplacementDuration::EndOfTurn }}`  ('Prevent the next 3 damage that would be dealt to any target this turn' — `amount: None` prevents ALL damage. `DamageTarget::Player(p)` for a player target. `ReplacementDuration` from `arcana_core::replacement`.)
- `Effect::RedirectDamage {{ from: DamageTarget::Object(id), to: DamageTarget::Player(entry.controller), duration: ReplacementDuration::EndOfTurn }}`  — CR 614.9 redirect damage from one target to another ('redirect damage to you'). Use instead of GAP-ing redirection.
- `Effect::PreventDamageFrom {{ source_filter: ObjectFilter::creature(), target_filter: TargetFilter::Player, amount: None, duration: ReplacementDuration::EndOfTurn }}`  — source/target-filtered prevention ('prevent all damage dealt by [source_filter] to [target_filter]', Fog Bank-style). `amount: Some(n)` for up-to-n. Broad `source_filter` (`ObjectFilter::permanent()`) + `TargetFilter::Player` = 'prevent all damage to you'. Use instead of GAP-ing board-wide/source-filtered prevention.

Sacrifice:
- `Effect::Sacrifice {{ player: p, filter: ObjectFilter::creature(), count: u32 }}`  ('that player sacrifices a creature' → player = the target player, filter selects what; chain the `ObjectFilter` refinements above for 'sacrifices an artifact', etc.)

Search the library (shuffle is automatic):
- `Effect::TutorToHand {{ player: p, filter: ObjectFilter::creature(), reveal: true }}`
- `Effect::TutorToBattlefield {{ player: p, filter: ObjectFilter::creature(), tapped: false }}`
- SEARCH BY EXACT CARD NAME is expressible: `ObjectFilter` has `name: Option<SmallString>`. In the effect fn, `let nm = reg.interner().lookup('Card Name');` (use real double-quotes in your code) then `Effect::TutorToHand {{ player: p, filter: ObjectFilter {{ name: nm, ..ObjectFilter::default() }}, reveal: true }}`. `lookup` returns `Option<SmallString>` — pass it straight into the field. Use this for 'search for a card named ~' instead of GAP-ing.
- `ObjectFilter` builders: `ObjectFilter::creature()`, `ObjectFilter::permanent()`, `ObjectFilter::new().with_types(TypeLine::LAND.into())` (chain `.with_colors(ColorSet::...)`, `.with_types_any(TypeLine::X.into())`, `.without_types(TypeLine::X.into())`).

TARGET SPEC — `target_requirements` on `SpellAbilityDef` (the references show `TargetRequirement::any_target()` and a creature target). Helper constructors: `TargetRequirement::target_creature()`, `TargetRequirement::target_player()`, `TargetRequirement::any_target()`. For anything else use the struct literal with EXACTLY these three fields:
- `TargetRequirement {{ filter: TargetFilter::Permanent(ObjectFilter::new().with_types(TypeLine::LAND.into())), count: TargetCount::Exactly(1), controller: None }}`  (target land / artifact / enchantment — set the type in the inner `ObjectFilter`)
- `TargetFilter` variants: `TargetFilter::Creature`, `TargetFilter::Player`, `TargetFilter::AnyTarget`, `TargetFilter::Permanent(ObjectFilter)`, `TargetFilter::Spell(ObjectFilter)` (counter-spells — filter the spell by type), `TargetFilter::AbilityOnStack {{ activated: true, triggered: false, source_filter: None }}` ('counter target activated ability' — abilities on the stack; `triggered: true` for 'or triggered'; `source_filter: Some(<ObjectFilter>)` for 'from a noncreature source'; resolve with `Effect::Counter`), `TargetFilter::Card {{ zone: Zone::Graveyard(0), filter: ObjectFilter::creature() }}` (target a card in a graveyard, e.g. Raise Dead/Reanimate).
- `TargetCount`: `TargetCount::Exactly(1)`, `TargetCount::UpTo(n)`, `TargetCount::Any`. `controller` is `Option<...>` — use `None` and constrain inside the `ObjectFilter` instead.

Stack:
- `Effect::Counter {{ target: id }}`  (counter target spell — `id` is the spell's stack-object id; see Counterspell)
- `Effect::CounterUnlessPays {{ target: id, cost: ManaCost::parse(\"{{2}}\").expect(\"valid cost\") }}`  (soft counter — 'counter target spell unless its controller pays {{N}}'; Spell Pierce / Mana Leak / Miscast. The engine finds the spell's controller and prompts them; you supply only the spell id and the tax cost.)

Composites (wrap the above):
- `Effect::ForEach {{ targets: vec![/* ObjectIds */], effect: Box::new(Effect::DestroyPermanent {{ target: arcana_core::objects::NULL_OBJECT_ID }}) }}`  — 'destroy/affect EACH/ALL matching': enumerate the ids from `state` and apply the inner effect once per id. Use this for board wipes and 'deals N damage to each creature'.
- `Effect::Conditional {{ condition, then: Box::new(..), otherwise: Some(Box::new(..)) }}`  ·  `Effect::Sequence(vec![..])`
- `Effect::FlipCoin {{ player: p, win: Box::new(<effect>), lose: Some(Box::new(<effect>)) }}`  — 'Flip a coin. If you win the flip, [win]. If you lose, [lose].' Deterministic/replayable. 'If you win, X' with no downside → `lose: None`; 'if you LOSE [bad]' → punishment in `lose`, `win: Box::new(Effect::Sequence(vec![]))`. Use instead of GAP-ing coin flips.
- `Effect::DelayedAction {{ source: id, controller: entry.controller, when: DelayedWhen::NextEndStep, action: DelayedAction::Sacrifice }}`  — schedule a one-shot on a KNOWN id (a target you already have): when ∈ `NextEndStep` / `NextUpkeep` / `ThisDies`; action ∈ `Sacrifice` / `Exile` / `ReturnToHand` / `ReturnFromExileToBattlefield` / `ControllerDrawsCard`. Use for 'exile target creature, return it to the battlefield/hand at the beginning of the next end step' (`ExilePermanent` now + `DelayedAction{{when:NextEndStep, action:ReturnFromExileToBattlefield}}` for Cloudshift/Ghostway-class blink; or `action:ReturnToHand` for 'to its owner\'s hand') and dies-on-target riders. NOT for 'create a token then sacrifice it' (the token id isn't available — GAP that). Import `DelayedWhen`, `DelayedAction` from `arcana_core::effects`.

CARD SCRIPTING — when an amount or a board-wide set is computed at resolution ('equal to its power', 'for each creature you control', 'destroy all Goblins'), the resolver's FIRST parameter is the live `&GameState` (name it `state`, not `_state`) and you may call ONLY these total, panic-free helpers from `arcana_core::script` (add `use arcana_core::script;`). Each returns a plain value — bind it to a `let`, then put it in an ordinary literal-amount `Effect`:
- `script::count_matching(state, &filter, entry.controller) -> u32`  — battlefield permanents matching an `ObjectFilter` ('number of creatures you control' = `ObjectFilter::creature().controlled_by(ControllerConstraint::You)`).
- `script::ids_matching(state, &filter, entry.controller) -> Vec<ObjectId>`  — the matching ids, in stable order; feed straight into `Effect::ForEach {{ targets: <this>, effect: Box::new(..) }}` for 'destroy/return/damage EACH/ALL <filter>' (the filter, not just `creature()`, selects the subset — this is how filtered board wipes work).
- `script::subtype_filter(reg, 'Goblin')`  — an `ObjectFilter` for creatures of a named subtype (resolved via the resolver's 3rd param — name it `reg`, not `_reg`); matches nothing if no such card exists. Use for tribal wipes: `script::ids_matching(state, &script::subtype_filter(reg, 'Zombie'), entry.controller)`.
- `ObjectFilter` refinements (chain onto `creature()` / `permanent()` / `subtype_filter(..)`): `.controlled_by(ControllerConstraint::You|Opponent)`, `.with_colors(ColorSet::black())`, `.without_colors(ColorSet::black())` (nonblack), `.without_types(TypeLine::ARTIFACT.into())`, `.with_max_cmc(n)` / `.with_min_cmc(n)` / `.with_exact_cmc(n)`, `.with_min_power(n)` / `.with_max_power(n)` / `.with_max_toughness(n)`, `.tokens_only()` / `.nontoken()`, `.tapped_only()` / `.untapped_only()`. SUBTYPE: `.with_subtypes_any(vec![reg.interner_mut().intern(\"Vampire\")])` (matches ANY listed subtype — author in `register` since it needs the interner, then close over the filter; for a single creature subtype prefer `script::subtype_filter(reg, \"Vampire\")`). SUPERTYPE: `.with_supertypes(SupertypeSet::new().with(SupertypeSet::LEGENDARY))` ('Legendary'/'Basic'/'Snow' — `SupertypeSet::SNOW` and `BASIC` exist), `.without_supertypes(SupertypeSet::new().with(SupertypeSet::LEGENDARY))` ('nonlegendary'). For 'each creature AND each planeswalker' (Star of Extinction-class wipes) use `ObjectFilter::permanent().with_types_any(TypeLine(TypeLine::CREATURE | TypeLine::PLANESWALKER))`. Covers 'each Goblin', 'all nonlegendary creatures', 'destroy target tapped Vampire', 'each Snow land', etc. These same builders compose inside `TargetFilter::Permanent(..)` — use them so 'target Vampire creature' / 'target nonlegendary creature' is the actual target requirement (build the subtyped filter in `register` and reference it), NOT a bare `TargetFilter::Creature`. Do NOT GAP subtype/supertype filters — they are fully supported.
- `script::power_of(state, id) -> i32` · `script::toughness_of(state, id) -> i32`  — a permanent's current P/T (0 if gone).
- `script::devotion(state, entry.controller, colors) -> u32`  — CR 700.5 devotion: colored pips of `colors` among the mana costs of permanents you control. `colors` is a `ColorSet` (`ColorSet::green()`, or `ColorSet::white() | ColorSet::black()` for two colors). Use for 'X is your devotion to ~'.
- `script::hand_size(state, p) -> u32` · `script::graveyard_size(state, p) -> u32` · `script::library_size(state, p) -> u32` · `script::life(state, p) -> i32`.
- `script::graveyard_matching(state, &filter, player, entry.controller) -> u32`.
- `script::all_players(state) -> Vec<PlayerId>` · `script::opponents(state, entry.controller) -> Vec<PlayerId>`  — for 'each player' / 'each opponent' effects: build one inner `Effect` per player and wrap them in `Effect::Sequence`. e.g. 'each player discards a card': `Effect::Sequence(script::all_players(state).into_iter().map(|p| Effect::Discard {{ player: p, count: 1, choice: DiscardChoice::ControllerChooses }}).collect())`. Same pattern for each-opponent loses-life / mills / draws.
- `script::target_controller(state, id, entry.controller) -> PlayerId`  — for 'target's controller' / 'its owner' / 'create a token under that player's control': use this as the `controller`/`player` field instead of `entry.controller`. Falls back to your controller if the target is gone. e.g. 'destroy target creature; its controller creates a 3/3 Beast token' → `vec![Effect::DestroyPermanent {{ target: id }}, Effect::CreateToken {{ controller: script::target_controller(state, id, entry.controller), token: TokenDefinition {{ .. }} }}]`.
MANDATORY: if the oracle text says 'for each', 'for every', 'equal to the number of', 'equal to its power/toughness', 'X is the number of', 'Converge', or 'Domain', the amount/count is DYNAMIC and you MUST compute it with the matching `script::*` helper (or `Effect::ForEach` over `script::ids_matching`). Do NOT hardcode a literal (`amount: 1`, `count: 0`) and do NOT `// GAP` the scaling while emitting a fixed-size effect — a literal where the text is dynamic is a materially WRONG card and is auto-quarantined by verify, strictly worse than an honest full GAP. If you genuinely cannot compute it from the helpers, GAP the WHOLE effect (`Vec::new()`), do not emit a fixed-size stand-in.

No other `state` access is permitted (no field access, no other methods) — if the needed quantity is not one of the above, treat it as a GAP. Amount fields are `u32`; a possibly-negative `i32` (a power, a life total) becomes an amount via `.max(0) as u32`. `i32` Pump fields take `script::power_of(..)` directly. Worked resolver:
```rust
fn resolve(state: &GameState, entry: &StackEntry, _reg: &CardRegistry) -> Vec<Effect> {{
    let n = script::count_matching(
        state,
        &ObjectFilter::creature().controlled_by(ControllerConstraint::You),
        entry.controller);
    vec![Effect::DrawCards {{ player: entry.controller, count: n }}]
}}
```

=== TARGET CARD ===
{spec}

BONES ARE AUTHORITATIVE AND COME ONLY FROM THE TARGET CARD SPEC ABOVE — NOT from the reference cards (those are for code structure only). Transcribe verbatim, do not infer or recall from the card's name:
- `mana_cost`: pass the spec's `Mana cost` string EXACTLY into `ManaCost::parse(\"…\")` (same pips, same generic number). If the spec has no `Mana cost` line, omit `mana_cost`.
- `colors`: exactly the colors of the mana cost's colored pips (W→white, U→blue, B→black, R→red, G→green; colorless/no pips → `ColorSet::new()`); combine with `|`. Never add a color the cost doesn't have.
- `types`: exactly the spec's `Type line` (Instant → `TypeLine::INSTANT.into()`, Sorcery → `TypeLine::SORCERY.into()`).
- power/toughness/supertypes: exactly as in the spec (instants/sorceries have none).

Then the resolver returns the `Effect`s implementing the rules text, using the references AND the ENGINE EFFECT CATALOG above. Only if the effect genuinely cannot be expressed with any catalog variant, return `Vec::new()` AND add a `// GAP: <what is missing>` comment naming the specific capability — never invent an `Effect` variant or a field not shown. Generate the Rust source. Output only the file contents.",
        spec = card_spec(card),
    )
}

fn user_triggered_ability_creature(card: &Card) -> String {
    format!(
        "Generate a CREATURE WITH A TRIGGERED ABILITY — a `CardDefinition` carrying one `TriggeredAbilityDef`, plus a free `effect` fn (referenced as a fn pointer) that returns `Vec<Effect>`.

REFERENCE — Elvish Visionary ({{1}}{{G}} 1/1 Elf Shaman, 'When ~ enters the battlefield, draw a card' — the `TriggeredAbilityDef` shape and a no-target effect fn):
```rust
{FS_ELVISH_VISIONARY}
```

REFERENCE — Young Pyromancer ({{1}}{{R}} 2/1 Human Shaman, 'Whenever you cast an instant or sorcery spell, create a 1/1 red Elemental creature token' — a filtered `SpellCast` condition and an effect fn that interns a token subtype via the `reg` param):
```rust
{FS_YOUNG_PYROMANCER}
```

REFERENCE — Weldfast Engineer ({{1}}{{B}}{{R}} 3/3 Human Artificer, 'At the beginning of combat on your turn, target artifact creature you control gets +2/+0 until end of turn' — a `PhaseBegins{{Combat, You}}` condition AND a targeted trigger: `target_requirements` declares the legal targets up front, and the effect fn reads the chosen object from `trig.targets.targets.first()` via a `TargetChoice::Object(id)` match. Adopt this exact target-read pattern for any 'target X' trigger):
```rust
{FS_WELDFAST_ENGINEER}
```

{trigcat}

{paccess}

BINDING — your effect fn's signature is `fn(_: &GameState, trig: &PendingTrigger, reg: &CardRegistry) -> Vec<Effect>` (see the references). The EFFECT CATALOG and CARD SCRIPTING sections below are shared with the spell generator; YOUR binding is `trig` — it carries `trig.controller` (the ability's controller — use for 'you'), `trig.source` (this creature's `ObjectId`), `trig.targets` (declared targets), and the typed accessors listed just above. The catalog text already says `trig.<field>` — use exactly that.

{cat}

WORKED EFFECT FN — the trigger effect fn combining the catalog and the script prelude:
```rust
{worked}
```

=== TARGET CARD ===
{spec}

BONES ARE AUTHORITATIVE AND COME ONLY FROM THE TARGET CARD SPEC ABOVE — not from the reference cards (those are for code structure only). Transcribe verbatim, never infer from the card's name:
- `mana_cost`: the spec's `Mana cost` string EXACTLY into `ManaCost::parse(\"…\")`.
- `colors`: exactly the colored pips of that cost (combine with `|`); never add a color the cost lacks.
- `types`: exactly the spec's `Type line` (Creature → `TypeLine::CREATURE.into()`; an Enchantment Creature → `TypeLine(TypeLine::ENCHANTMENT | TypeLine::CREATURE)`).
- power/toughness: exactly the spec's `Power/Toughness`, as `Some(PtValue::Fixed(n))`.

Then build the ONE `TriggeredAbilityDef` whose `trigger_condition` matches the oracle's trigger clause and whose `effect` fn returns the `Effect`s for what follows it. If the effect genuinely cannot be expressed with any catalog variant, the effect fn returns `Vec::new()` with a `// GAP: <what is missing>` comment — never invent an `Effect` / `TriggerCondition` / `GameEvent` variant or a field not shown. Generate the Rust source. Output only the file contents.",
        spec = card_spec(card),
        trigcat = TRIGGER_CONDITION_CATALOG,
        paccess = TRIGGER_PENDING_ACCESSORS,
        cat = effect_catalog("trig"),
        worked = WORKED_TRIGGER_FN,
        FS_ELVISH_VISIONARY = FS_ELVISH_VISIONARY,
        FS_YOUNG_PYROMANCER = FS_YOUNG_PYROMANCER,
        FS_WELDFAST_ENGINEER = FS_WELDFAST_ENGINEER,
    )
}

/// Per-card prompt block for ActivatedAbilityCreature — a creature
/// whose only printed text is one or more activated abilities. Common
/// archetypes: mana dorks (`{T}: Add {C}`), pingers (`{T}: deals 1
/// damage`), sac-for-value (`Sacrifice ~: do X`), pump activations
/// (`{2}{R}: ~ gets +X/+X until end of turn`), counter-tap-untap loops.
fn user_activated_ability_creature(card: &Card) -> String {
    format!(
        "Generate a CREATURE WITH ONE OR MORE ACTIVATED ABILITIES — a `CardDefinition` carrying one or more `ActivatedAbilityDef` values plus a free `effect` fn (referenced as a fn pointer) for each ability.

REFERENCE — Llanowar Elves ({{G}} 1/1 Elf Druid, '{{T}}: Add {{G}}' — the mana-dork archetype: `ActivationCost::tap_only()` + `is_mana_ability: true` + an `Effect::AddMana` resolver):
```rust
{FS_LLANOWAR_ELVES}
```

REFERENCE — Prodigal Sorcerer ({{2}}{{U}} 1/1 Human Wizard, '{{T}}: this creature deals 1 damage to any target' — the pinger archetype: tap-cost + `TargetRequirement::any_target()` + a `TargetChoice` match that handles all three variants `Object` / `Player` / `ObjectOrPlayer` and yields a `DamageTarget`):
```rust
{FS_PRODIGAL_SORCERER}
```

REFERENCE — Walking Ballista (the counter-removal-as-cost archetype with TWO activated abilities on one card: `{{4}}` + 'put a +1/+1 counter on it' AND 'remove a +1/+1 counter: deal 1 damage to any target'). The same source has multiple `.with_activated_ability(...)` chained on the `CardDefinition`. Read the canonical version at `arcana-cards/src/aer/walking_ballista.rs` if you need a non-tap cost + counter manipulation pattern.

{actcost}

ACTIVATED EFFECT FN BINDING — your effect fn's signature is `fn(_: &GameState, ctx: &ActivationContext, reg: &CardRegistry) -> Vec<Effect>`. `ctx` carries `ctx.controller` (the activator), `ctx.source` (this creature's `ObjectId`), `ctx.targets` (the declared targets, same `TargetSelection` shape as triggered abilities — read via `ctx.targets.targets.first()` + a `TargetChoice` match), `ctx.x_value: Option<u32>` (for X-cost activations), and `ctx.card_id` (the printed-form id). Targets are read EXACTLY the same way as the spell program.

{cat}

=== TARGET CARD ===
{spec}

BONES ARE AUTHORITATIVE AND COME ONLY FROM THE TARGET CARD SPEC ABOVE — not from the reference cards (those are for code structure only). Transcribe verbatim:
- `mana_cost`: exactly the spec's `Mana cost` string into `ManaCost::parse(\"…\")`.
- `colors`: exactly the colored pips of that cost; never add a color the cost lacks.
- `types`: exactly the spec's `Type line` (Creature → `TypeLine::CREATURE.into()`).
- power/toughness: exactly as in the spec.

Then build ONE `ActivatedAbilityDef` per activated clause in the oracle text. The ability's `cost` field is built from the oracle's cost (e.g. `{{T}}: …` → `ActivationCost::tap_only()`; `{{2}}{{R}}: …` → `ActivationCost {{ mana_cost: ManaCost::parse(\"{{2}}{{R}}\").unwrap(), ..ActivationCost::default() }}`; `{{1}}, {{T}}: …` → `ActivationCost {{ mana_cost: ManaCost::parse(\"{{1}}\").unwrap(), tap: true, ..ActivationCost::default() }}`; `Sacrifice ~: …` → `ActivationCost {{ sacrifice: true, ..ActivationCost::default() }}`; `Pay 2 life: …` → `ActivationCost {{ life: 2, ..ActivationCost::default() }}`). If the effect genuinely cannot be expressed with any catalog variant, the effect fn returns `Vec::new()` with a `// GAP: <what is missing>` comment — never invent an `Effect` / `ActivationCost` field. Generate the Rust source. Output only the file contents.",
        spec = card_spec(card),
        actcost = ACTIVATION_COST_CATALOG,
        cat = effect_catalog("ctx"),
        FS_LLANOWAR_ELVES = FS_LLANOWAR_ELVES,
        FS_PRODIGAL_SORCERER = FS_PRODIGAL_SORCERER,
    )
}

/// Canonical catalog of `ActivationCost` shapes for the
/// ActivatedAbilityCreature prompt. Mirrors the trigger / target
/// catalogs in style — each line is a copy-pasteable construction
/// keyed to a recognisable oracle phrasing.
const ACTIVATION_COST_CATALOG: &str = r#"ACTIVATION COST CATALOG — `ActivatedAbilityDef.cost` is a `struct ActivationCost { mana_cost, tap, sacrifice, life, remove_self_counter, add_self_counter, discard_self, exile_self, min_self_counters, sacrifice_other, discard_other, discard_other_count, discard_hand, tap_other, tap_other_count, activation_condition, once_per_turn }`. Build via struct literal with `..ActivationCost::default()` for unused fields. Map oracle costs to fields as follows:
- `{T}: …` (tap alone) → `ActivationCost::tap_only()`.
- `{N}: …` or `{R}: …` (mana only, no tap) → `ActivationCost { mana_cost: ManaCost::parse("{N}").unwrap(), ..ActivationCost::default() }`.
- `{N}, {T}: …` (mana + tap) → `ActivationCost { mana_cost: ManaCost::parse("{N}").unwrap(), tap: true, ..ActivationCost::default() }`.
- `Sacrifice ~: …` (sacrifice-self only) → `ActivationCost { sacrifice: true, ..ActivationCost::default() }`. The engine routes the sacrifice automatically as part of activation cost payment.
- `{N}, Sacrifice ~: …` → mana_cost + sacrifice: true.
- `Sacrifice another creature: …` / `Sacrifice an artifact: …` / `Sacrifice a [type]: …` (the sacrificed permanent is CHOSEN, NOT this card) → `ActivationCost { sacrifice_other: Some(ObjectFilter { types: Some(TypeLine::CREATURE.into()), ..ObjectFilter::default() }), ..ActivationCost::default() }`. Set the filter to what the cost requires (`TypeLine::ARTIFACT.into()` for "an artifact", a `subtypes` entry for "a Goblin"). The engine enumerates one activation per matching permanent you control and always excludes this card itself — so "Sacrifice another ~" and "Sacrifice a ~" both map here. Import `ObjectFilter` from `arcana_core::targets`. Do NOT use the bare `sacrifice: true` field for this — that one always sacrifices THIS card.
- `Tap an untapped [type] you control: …` (Springleaf Drum, "tap two untapped Wizards you control") → `ActivationCost { tap_other: Some(ObjectFilter { types: Some(TypeLine::CREATURE.into()), ..ObjectFilter::default() }), ..ActivationCost::default() }` (add `tap_other_count: 2` for "two"). The engine enumerates one activation per matching UNTAPPED permanent you control (source excluded) and taps it as the cost — do NOT model this as an effect or GAP it.
- `Discard a card: …` / `Discard a [type] card: …` (a chosen card from your hand, NOT this card) → `ActivationCost { discard_other: Some(ObjectFilter::default()), ..ActivationCost::default() }`. Use `ObjectFilter::default()` for "a card"; add a `types`/`subtypes` filter for "a creature card" etc. The engine enumerates one activation per matching hand card.
- `Discard two cards: …` / `Discard N cards: …` → same as above plus `discard_other_count: 2` (or N): `ActivationCost { discard_other: Some(ObjectFilter::default()), discard_other_count: 2, ..ActivationCost::default() }`. The engine enumerates one activation per N-card combination; the ability is unactivatable with fewer than N matching cards in hand.
- `Discard your hand: …` (discard ALL cards in hand) → `ActivationCost { discard_hand: true, ..ActivationCost::default() }`. Deterministic — every card is discarded (an empty hand still pays the cost).
- `Pay N life: …` → `ActivationCost { life: N, ..ActivationCost::default() }`.
- `Remove a +1/+1 counter from ~: …` (Walking-Ballista-style) → `ActivationCost { remove_self_counter: Some((CounterKind::PlusOnePlusOne, 1)), ..ActivationCost::default() }`. For other counter kinds use the matching `CounterKind` variant.
- `Put a +1/+1 counter on ~ : …` (rare; mainly planeswalker loyalty +N) → `ActivationCost { add_self_counter: Some((CounterKind::PlusOnePlusOne, 1)), ..ActivationCost::default() }`. For planeswalkers use `CounterKind::Loyalty`.
- `{N}, Discard ~: …` (cycling — but cycling cards aren't activated creatures, this is rare) → `ActivationCost { mana_cost: ..., discard_self: true, ..ActivationCost::default() }` PLUS `activation_zone: ActivationZone::Hand`.
- `min_self_counters: Option<(CounterKind, u32)>` — PURE PRECONDITION (not a cost — counters are not removed; only legality is gated). The source must currently have at least `count` counters of `kind`. Use this for CR 717.5b Class level-up gating: the Level-N activation sets `min_self_counters: Some((CounterKind::Level, N - 1))`. Do NOT use this for "remove N counters" oracle text (that is `remove_self_counter`, which both gates legality and consumes counters). Leave at `None` for activations with no counter precondition.

OTHER FIELDS on `ActivatedAbilityDef`:
- `text: String` — the oracle text of THIS ability only, e.g. `"{T}: Add {G}.".into()`.
- `target_requirements: Vec<TargetRequirement>` — empty for non-targeted; `vec![TargetRequirement::target_creature()]` for "target creature", `vec![TargetRequirement::any_target()]` for "any target" (creature OR player OR planeswalker — read via the THREE-arm match including the `TargetChoice::ObjectOrPlayer(ObjectOrPlayer::{Object,Player})` shape).
- `is_mana_ability: true` ONLY when the cost has no target AND the effect's ONLY result is `Effect::AddMana` (CR 605). Mana abilities skip the stack. For everything else (pingers, sac creatures, pump activations), `is_mana_ability: false`.
- `is_loyalty_ability: false` for creatures (loyalty is planeswalker-only).
- `activation_zone: ActivationZone::Battlefield` — the default for creature activated abilities. Use `ActivationZone::Hand` for cycling / channel / "you may activate from hand" specials. Use `ActivationZone::Graveyard` for graveyard-activated abilities ("{cost}: … Activate only from your graveyard." — unearth, embalm-style, or "{cost}, Exile ~ from your graveyard: …" patterns). All three variants exist; pick the zone the oracle says the ability is activated from.
- `is_instant_speed: false` for the default sorcery-speed activated abilities of permanents (but tap-for-mana mana abilities are implicitly instant-speed via `is_mana_ability`).
- `face_gate: None` — single-face cards. (`Some(n)` is for MDFC back-face abilities.)
- `effect: <your_resolver_fn>` — the `ActivatedEffectFn` you author below.

CHAINING multiple activated abilities — chain `.with_activated_ability(...)` per ability:
```
reg.register(
    CardDefinition::new(name, chars)
        .with_activated_ability(ActivatedAbilityDef { /* first ability */ })
        .with_activated_ability(ActivatedAbilityDef { /* second ability */ })
)
```

Import the activated-ability primitives from `arcana_core::registry`:
```rust
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
```"#;

/// Per-card prompt block for AdventureCreature (CR 715). The card has
/// a creature main face AND an Adventure (instant/sorcery) face with
/// its own name + mana cost + text. Engine wiring already complete via
/// `.with_adventure(CardFace)`.
fn user_adventure_creature(card: &Card) -> String {
    format!(
        "Generate an ADVENTURE CARD (CR 715 — Adventurer): a creature card with a printed Adventure face (instant or sorcery). Engine wires via `.with_adventure(CardFace {{ name, characteristics, spell_ability: Some(...) }})` on the CardDefinition; the cast pipeline routes Adventure casts to the face's spell ability and exiles the card on resolution (CR 715.4); the owner may later cast it as a creature from exile.

REFERENCE — Bonecrusher Giant // Stomp ({{1}}{{R}} Giant 4/3 creature face + 'Stomp' {{1}}{{R}} instant face dealing 2 damage to any target). Shows the full `with_adventure` wiring with separate name/chars/spell_ability for the Adventure face:
```rust
{FS_BONECRUSHER_GIANT}
```

{cat}

=== TARGET CARD ===
{spec}

BONES — both faces:
- Creature face: `mana_cost`/`colors`/`types`/subtypes/power/toughness from the main type-line entry (the creature half).
- Adventure face: separate `name` (Scryfall lists the face name e.g. 'Stomp'), separate `mana_cost`, `types` (Instant or Sorcery), and a `SpellAbilityDef` with the Adventure's effect.

Build via:
```rust
let main_chars = Characteristics {{ name, mana_cost: ..., colors: ..., types: TypeLine::CREATURE.into(), subtypes, power, toughness, ..Default::default() }};
let adv_chars = Characteristics {{ name: adv_name, mana_cost: ..., colors: ..., types: TypeLine::INSTANT.into() /* or SORCERY */, ..Default::default() }};
let adv_ability = SpellAbilityDef {{ text: \"…\".into(), target_requirements: vec![...], modal: None, effect: adv_resolve }};
let adventure = CardFace {{ name: adv_name, characteristics: adv_chars, spell_ability: Some(adv_ability) }};
reg.register(CardDefinition::new(name, main_chars).with_adventure(adventure))
```
If the Adventure's effect can't be expressed with any catalog variant, the spell-ability `effect` fn returns `Vec::new()` with `// GAP: <missing>` — never invent a variant. Generate the Rust source. Output only the file contents.",
        spec = card_spec(card),
        cat = effect_catalog("entry"),
        FS_BONECRUSHER_GIANT = FS_BONECRUSHER_GIANT,
    )
}

/// Per-card prompt block for ModalDfcCreature (CR 712.4). Two
/// first-class faces — either may be cast. Engine wiring exists via
/// `AlternateFace::Mdfc(CardFace)` and `with_mdfc_back`.
fn user_mdfc_creature(card: &Card) -> String {
    format!(
        "Generate a MODAL DOUBLE-FACED CARD (CR 712.4 / 717): two first-class faces, either castable for its own printed mana cost. Engine wires via `.with_mdfc_back(CardFace {{ name, characteristics, spell_ability }})` on CardDefinition.

For MVP: build the front (main) face fully — name, mana_cost, colors, types, subtypes, P/T, abilities — exactly as you'd build a normal creature/spell. Attach a back face via `with_mdfc_back`. If the back face is a land or a permanent type other than instant/sorcery, set `spell_ability: None`. If the back face is instant/sorcery, give it a SpellAbilityDef like a normal spell.

ENGINE STATUS — back-face resolution now works:
- A back face that is a PERMANENT (creature / land / planeswalker) resolves onto the battlefield with the BACK face's full characteristics (P/T, type line, colors, keywords; planeswalker backs enter with their printed `loyalty: Some(N)` as loyalty counters). Author the back `characteristics` completely. Land backs are played via the land drop (`PlayLand {{ mdfc_back: true }}`), non-land permanent backs are cast via `CastModifier::MdfcBack`.
- A back face that is an instant/sorcery resolves its `spell_ability` then goes to the graveyard, like a normal spell.
- The back face's own ACTIVATED abilities work via `face_gate: Some(1)` on each ActivatedAbilityDef (front-face activated abilities take `face_gate: Some(0)` or `None`).
- Back-face TRIGGERED abilities: put them on the one CardDefinition and gate each to the back face with `.with_trigger_face_gate(trigger_id, 1)` (an MDFC back resolves as a permanent showing face 1). Front-face triggers take `with_trigger_face_gate(id, 0)` or no gate if shared. Build both faces' static characteristics regardless.

{cat}

=== TARGET CARD ===
{spec}

Generate the Rust source. Output only the file contents.",
        spec = card_spec(card),
        cat = effect_catalog("entry"),
    )
}

/// Per-card prompt block for TransformCreature (CR 712). A
/// transforming DFC: cast the front face; a transform ability flips
/// it to the back face (a permanent). Engine wires the swap via
/// `with_transform_back(CardFace)` + `Effect::Transform`.
fn user_transform_creature(card: &Card) -> String {
    format!(
        "Generate a TRANSFORMING DOUBLE-FACED CARD (CR 712, layout \"transform\"). Unlike an MDFC you NEVER cast the back face — the card is cast as its FRONT face, and a transform ability later flips it to the BACK face (a permanent: usually a bigger creature, sometimes a planeswalker or land). Werewolves, Innistrad flip-walkers, etc.

ENGINE STATUS — transform primitives are in place:
- Build the FRONT face fully on the CardDefinition (name, mana_cost, colors, types, subtypes, P/T, abilities) exactly as a normal creature. The front-face name is the registered base name (Scryfall's combined \"Front // Back\" is split — register the FRONT name only).
- Declare the BACK face with `.with_transform_back(CardFace {{ name, characteristics, spell_ability: None }})`. The back `characteristics` is the full back-face sheet: name, type line (`TypeLine::CREATURE`/`PLANESWALKER`/etc.), colors, P/T (or, for a planeswalker back, `loyalty: Some(N)`), and any keywords. `spell_ability` is always `None` (the back is a permanent face, never cast).
- TRANSFORM the permanent with `Effect::Transform {{ target }}`. This swaps the live characteristics to the back face (and, for a planeswalker back, seeds starting loyalty automatically). Calling it again swaps back to the front — so a werewolf's two triggers (front->back, back->front) both use `Effect::Transform`.
- FACE-GATE the directional triggers/abilities: an ability that should only be active on a particular face takes `face_gate: Some(0)` (front) or `Some(1)` (back) on its ActivatedAbilityDef. Triggered abilities that only exist on one face: author them and note the face in a comment.

DEFERRED engine debt (document as GAPs where they apply):
- FACE-GATED TRIGGERS: a transforming DFC's triggered abilities live on the one CardDefinition (shared across faces). Put BOTH faces' triggers on the def, and gate each to the face it belongs to with `.with_trigger_face_gate(trigger_id, face)` (face 0 = front, 1 = back). A front-only trigger → `with_trigger_face_gate(id, 0)`; a back-only trigger → `with_trigger_face_gate(id, 1)`. The engine fires a gated trigger only while the object shows that face. (Shared/both-face triggers need no gate.) Activated abilities use the per-ability `face_gate: Some(0|1)` field the same way.
- WEREWOLF / day-night transform conditions are now expressible via `intervening_if` (see the intervening-if section): \"if no spells were cast last turn, transform\" → on the front→back transform trigger set `intervening_if: Some(if_no_spells_last_turn)` where `fn if_no_spells_last_turn(s,_,_,_) -> bool {{ conditions::no_spells_cast_last_turn(s) }}`; \"if a player cast two or more spells last turn\" → `conditions::a_player_cast_two_or_more_last_turn(s)`. For day/night cards, `conditions::it_is_day` / `it_is_night`; introduce day/night with `Effect::SetDayNight {{ value: DayNight::Day }}`. Meld remains unmodeled (GAP it).

BUILD PATTERN:
```rust
let chars = Characteristics {{
    name, // FRONT name only
    mana_cost: ..., colors: ..., types: TypeLine::CREATURE.into(),
    power: Some(PtValue::Fixed(..)), toughness: Some(PtValue::Fixed(..)),
    ..Default::default()
}};
let back_name = reg.interner_mut().intern(\"Back Face Name\");
let back = CardFace {{
    name: back_name,
    characteristics: Characteristics {{
        name: back_name,
        colors: ...,
        types: TypeLine::CREATURE.into(), // or PLANESWALKER, with loyalty: Some(N)
        power: Some(PtValue::Fixed(..)), toughness: Some(PtValue::Fixed(..)),
        ..Default::default()
    }},
    spell_ability: None,
}};
reg.register(
    CardDefinition::new(name, chars)
        .with_transform_back(back)
        // transform trigger / activated ability whose effect emits Effect::Transform {{ target: ctx.source }}
)
```

{cat}

=== TARGET CARD ===
{spec}

Generate the Rust source. Output only the file contents.",
        spec = card_spec(card),
        cat = effect_catalog("ctx"),
    )
}

/// Per-card prompt block for Saga (CR 716). Enchantment subtype Saga;
/// gains lore counters on ETB + first main phase; chapter abilities
/// trigger on lore-counter placement with the matching count.
fn user_saga(card: &Card) -> String {
    format!(
        "Generate a SAGA ENCHANTMENT (CR 716). A saga is an Enchantment — Saga that enters with one lore counter and adds another lore counter at the beginning of its controller's first main phase. Chapter abilities I, II, III ... fire when the matching N-th lore counter is placed (CR 716.5). The saga is sacrificed after the final chapter resolves (CR 716.6).

ENGINE STATUS — saga dispatch primitives are in place:
- `EntersWithSpec::Counters {{ kind: CounterKind::Lore, count: 1 }}` for the ETB lore counter (CR 716.2).
- A 'first main phase, add a lore counter' triggered ability you author with `TriggerCondition::PhaseBegins {{ phase: Phase::PreCombatMain, whose: ControllerConstraint::You }}` whose effect is `Effect::AddCounters {{ target: trig.source, kind: CounterKind::Lore, count: 1 }}` (CR 716.3).
- Chapter abilities: one `TriggeredAbilityDef` per chapter with `TriggerCondition::CounterAdded {{ on: TriggerSelf::Source, kind: Some(CounterKind::Lore), chapter: Some(N) }}` where N is the chapter number (1 for I, 2 for II, etc.). The chapter fires when the N-th lore counter is placed.
- FINAL-CHAPTER SACRIFICE (CR 716.5d) is now AUTOMATIC. The engine synthesizes the Saga's final chapter number at registration time by taking the MAX of the chapter numbers in your `CounterAdded` triggers, then a state-based action sacrifices the Saga once it has that many Lore counters AND no chapter ability is pending or on the stack. You do NOT author a self-sacrifice effect — emitting one would double-sacrifice. Just author each chapter's payload; the SBA handles the cleanup.

BUILD PATTERN (rough sketch — fill in details from the card spec):
```rust
let saga_sub = reg.interner_mut().intern(\"Saga\");
let mut subtypes = SubtypeSet::default();
subtypes.0.insert(saga_sub);
let chars = Characteristics {{
    name, mana_cost: ..., colors: ...,
    types: TypeLine::ENCHANTMENT.into(),
    subtypes,
    ..Default::default()
}};
reg.register(
    CardDefinition::new(name, chars)
        .with_enters_with(EntersWithSpec::Counters {{
            kind: CounterKind::Lore, count: 1,
        }})
        .with_triggered_ability(TriggeredAbilityDef {{
            id: 1,
            trigger_condition: TriggerCondition::PhaseBegins {{
                phase: Phase::PreCombatMain,
                whose: ControllerConstraint::You,
            }},
            intervening_if: None,
            effect: add_lore_counter,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }})
        .with_triggered_ability(TriggeredAbilityDef {{
            id: 2,
            trigger_condition: TriggerCondition::CounterAdded {{
                on: TriggerSelf::Source,
                kind: Some(CounterKind::Lore),
                chapter: Some(1),
            }},
            intervening_if: None,
            effect: chapter_i,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }})
        // ... one .with_triggered_ability per chapter
)
```

{cat}

=== TARGET CARD ===
{spec}

Generate the Rust source. Output only the file contents.",
        spec = card_spec(card),
        cat = effect_catalog("trig"),
    )
}

/// Per-card prompt block for ClassEnchantment (CR 717). Enchantment
/// subtype Class; starts at level 1 (CR 717.3); level-up activations
/// add Level counters; per-level granted abilities are continuous
/// effects gated on the counter count.
fn user_class_enchantment(card: &Card) -> String {
    format!(
        "Generate a CLASS ENCHANTMENT (CR 717). A Class is an Enchantment — [type] Class that starts at level 1 (one Level counter on ETB), levels up via activated abilities '{{cost}}: Level N' (sorcery speed; requires you've already reached level N-1), and grants additional abilities at each level.

ENGINE STATUS — Class dispatch primitives:
- `CounterKind::Level` is in the engine.
- `EntersWithSpec::Counters {{ kind: CounterKind::Level, count: 1 }}` gives the starting level (CR 717.3).
- Each level-up is a SORCERY-SPEED activated ability: `ActivatedAbilityDef`'s `is_instant_speed: false` (the default for non-mana activated abilities). The activation cost is the printed mana cost; the resolver's effect is `Effect::AddCounters {{ target: ctx.source, kind: CounterKind::Level, count: 1 }}`.
- LEVEL-PRECONDITION GATE (CR 717.5b) is now wired: set `cost.min_self_counters = Some((CounterKind::Level, N - 1))` on the Level-N activation. The engine's `legal_actions` filter consults this and only offers the activation when the Class has at least N-1 level counters. The field is a pure precondition — it does NOT remove counters (your effect does the +1 via `Effect::AddCounters`); it only gates legality.
- PER-LEVEL STATIC ABILITIES are now expressible via install-on-level-up. A Class level never decreases, so a static that is true \"while at level N or above\" is equivalent to installing a continuous effect WHEN the Level-N activation resolves, with `Duration::WhileSourceOnBattlefield`. Give the Level-N activation its own effect fn (e.g. `level_up_to_2`) that returns BOTH the `Effect::AddCounters` (Level +1) AND an `Effect::InstallContinuousEffect {{ effect: <builder> }}`. Builders available:
  - \"Creatures you control get +P/+T\": `ContinuousEffect::anthem(ctx.source, ctx.controller, P, T, Duration::WhileSourceOnBattlefield)`.
  - \"Creatures you control have [keyword]\" (menace, trample, …): `ContinuousEffect::keyword_anthem(ctx.source, ctx.controller, KeywordAbility::Menace, Duration::WhileSourceOnBattlefield)`.
  A level-1 (base) static instead installs from an ETB trigger (the static is on as soon as the Class enters).
- STILL DEFERRED engine debt: per-level abilities that are NOT a P/T or keyword anthem (e.g. \"you may play lands from your graveyard\", \"spells you cast cost less\", replacement effects). Document those as `// GAP: per-level granted ability deferred (continuous-effect engine subsystem)` in the doc comment.

BUILD PATTERN:
```rust
let class_sub = reg.interner_mut().intern(\"Class\");
let mut subtypes = SubtypeSet::default();
subtypes.0.insert(class_sub);
// Add any class type subtype (e.g. \"Wizard\") similarly.
let chars = Characteristics {{
    name, mana_cost: ..., colors: ...,
    types: TypeLine::ENCHANTMENT.into(),
    subtypes,
    ..Default::default()
}};
reg.register(
    CardDefinition::new(name, chars)
        .with_enters_with(EntersWithSpec::Counters {{
            kind: CounterKind::Level, count: 1,
        }})
        .with_activated_ability(ActivatedAbilityDef {{
            text: \"{{2}}{{R}}: Level 2.\".into(),
            cost: ActivationCost {{
                mana_cost: ManaCost::parse(\"{{2}}{{R}}\").unwrap(),
                // CR 717.5b — Level-2 requires you to already be at level 1.
                min_self_counters: Some((CounterKind::Level, 1)),
                ..ActivationCost::default()
            }},
            target_requirements: vec![],
            is_mana_ability: false,
            is_loyalty_ability: false,
            activation_zone: ActivationZone::Battlefield,
            is_instant_speed: false,
            face_gate: None,
            effect: level_up_to_2,
        }})
        .with_activated_ability(ActivatedAbilityDef {{
            text: \"{{4}}{{R}}: Level 3.\".into(),
            cost: ActivationCost {{
                mana_cost: ManaCost::parse(\"{{4}}{{R}}\").unwrap(),
                // Level-3 requires level 2.
                min_self_counters: Some((CounterKind::Level, 2)),
                ..ActivationCost::default()
            }},
            target_requirements: vec![],
            is_mana_ability: false,
            is_loyalty_ability: false,
            activation_zone: ActivationZone::Battlefield,
            is_instant_speed: false,
            face_gate: None,
            effect: level_up_to_3,
        }})
        // ... one .with_activated_ability per printed level-up.
        // Apply `min_self_counters: Some((CounterKind::Level, N - 1))`
        // for the Level-N activation.
)

// Level-2 effect fn: bump the counter AND install the level-2 static.
// Example for \"Level 2 — Creatures you control have menace\":
fn level_up_to_2(_s: &GameState, ctx: &ActivationContext, _r: &CardRegistry) -> Vec<Effect> {{
    vec![
        Effect::AddCounters {{ target: ctx.source, kind: CounterKind::Level, count: 1 }},
        Effect::InstallContinuousEffect {{
            effect: ContinuousEffect::keyword_anthem(
                ctx.source, ctx.controller,
                KeywordAbility::Menace, Duration::WhileSourceOnBattlefield),
        }},
    ]
}}
```

Per-level statics that are a P/T anthem or keyword anthem are modeled via install-on-level-up (above). Other per-level abilities remain continuous-effect engine debt; document those as GAP comments. The activation cost AND the level precondition are always modeled faithfully.

{cat}

=== TARGET CARD ===
{spec}

Generate the Rust source. Output only the file contents.",
        spec = card_spec(card),
        cat = effect_catalog("ctx"),
    )
}

/// Per-card prompt block for Battle (MOM mechanic). Battle is a
/// permanent type; battles enter with defense counters and are
/// "attacked" by their owner's opponents (planeswalker-style).
/// Defeat transforms the battle into its back face.
fn user_battle(card: &Card) -> String {
    format!(
        "Generate a BATTLE PERMANENT (MOM mechanic, CR 309). A Battle is a non-creature permanent with the Battle type (and typically a subtype like Siege) that enters with defense counters; opponents may attack it as a planeswalker-style defender; when its defense counters reach zero it transforms into a creature face the controller controls.

ENGINE STATUS — Battle combat now works:
- `TypeLine::BATTLE` is a permanent type; `CounterKind::Defense` exists.
- `EntersWithSpec::Counters {{ kind: CounterKind::Defense, count: N }}` gives the starting defense counters.
- A battle is ATTACKABLE: legal_actions offers it as a planeswalker-style defender (an opponent's battle can be attacked). Combat damage to a battle removes that many defense counters (CR 310.8) — not creature-style marked damage.
- DEFEAT is a state-based action (CR 704.5t): when a battle reaches 0 defense counters it is put into the graveyard automatically. You do NOT author a self-sacrifice for this.
- ETB / attack / 'when defeated' triggers are authored as usual with `TriggeredAbilityDef`.

DEFERRED engine debt (document as GAPs):
- 'When defeated, exile and cast the back face transformed' (CR 310.11) is NOT modeled — the battle just goes to the graveyard. If the card has a meaningful back face, author it via `.with_transform_back(CardFace {{ ... }})` (the transform machinery exists) and emit `// GAP: defeat→cast-back-face not auto-wired` for the on-defeat cast.
- The Siege protector-designation rule (choose an opponent to protect it) is simplified: any opponent's battle is attackable.

BUILD PATTERN:
```rust
let siege_sub = reg.interner_mut().intern(\"Siege\");
let mut subtypes = SubtypeSet::default();
subtypes.0.insert(siege_sub);
let chars = Characteristics {{
    name, mana_cost: ..., colors: ...,
    types: TypeLine::BATTLE.into(),
    subtypes,
    ..Default::default()
}};
reg.register(
    CardDefinition::new(name, chars)
        .with_enters_with(EntersWithSpec::Counters {{
            kind: CounterKind::Defense, count: N,
        }})
        // Any printed ETB or other triggers authored as usual.
)
```

{cat}

=== TARGET CARD ===
{spec}

Generate the Rust source. Output only the file contents.",
        spec = card_spec(card),
        cat = effect_catalog("entry"),
    )
}

/// Per-card prompt block for TriggeredEnchantment (Wave 1). A
/// non-Aura enchantment whose printed text is triggered abilities —
/// structurally identical to a triggered creature minus the P/T.
fn user_triggered_enchantment(card: &Card) -> String {
    format!(
        "Generate an ENCHANTMENT WITH TRIGGERED ABILITIES — a non-Aura enchantment `CardDefinition` carrying one `TriggeredAbilityDef` per trigger clause, plus a free `effect` fn (referenced as a fn pointer) for each. An enchantment's triggered ability is wired EXACTLY like a creature's (same `TriggeredAbilityDef`, same effect-fn signature) — only the characteristics differ: `types: TypeLine::ENCHANTMENT.into()` and NO power/toughness.

REFERENCE — Underworld Dreams ({{B}}{{B}}{{B}} enchantment, 'Whenever an opponent draws a card, ~ deals 1 damage to them' — the enchantment bones + a `CardDrawn` trigger. NOTE: this hand-written seed reads the drawing player off `trig.trigger_event`; in YOUR file prefer the typed `trig.*` accessors and NEVER invent a `GameEvent` variant — if no accessor fits, for 'that player' on an opponent-constrained trigger use `script::opponents(state, trig.controller).first().copied()` as the documented 2-player read):
```rust
{FS_UNDERWORLD_DREAMS}
```

REFERENCE — Glorious Anthem ({{1}}{{W}} enchantment, 'Creatures you control get +1/+1' — shows the ETB-install `ContinuousEffect` idiom. Use it ONLY when one line of the target's text is a static anthem alongside the trigger; a pure-trigger enchantment doesn't need it):
```rust
{FS_GLORIOUS_ANTHEM}
```

REFERENCE — Angelic Chorus ({{3}}{{W}}{{W}} enchantment, 'Whenever a creature you control enters, you gain life equal to its toughness' — a FILTERED `ZoneChange` trigger over OTHER objects entering, plus a dynamic amount read at resolution via `trig.entering_object()` + `script::toughness_of`. Adopt this pattern for any 'whenever a [filtered permanent] enters/dies/...' enchantment trigger):
```rust
{FS_ANGELIC_CHORUS}
```

REFERENCE — Phyrexian Tyranny ({{U}}{{B}}{{R}} enchantment, 'Whenever a player draws a card, that player loses 2 life unless they pay {{2}}' — an any-player `CardDrawn` trigger where 'that player' is read off the firing `GameEvent::DrawCard`, wrapped in `Effect::OptionalPayment` with the unless-pays polarity. Adopt this for 'whenever a player/each player does X, that player...' triggers):
```rust
{FS_PHYREXIAN_TYRANNY}
```

{trigcat}

{paccess}

BINDING — your effect fn's signature is `fn(_: &GameState, trig: &PendingTrigger, reg: &CardRegistry) -> Vec<Effect>` (see the references). The EFFECT CATALOG and CARD SCRIPTING sections below are shared with the other generators; YOUR binding is `trig` — it carries `trig.controller` (the enchantment's controller — use for 'you'), `trig.source` (this enchantment's `ObjectId`), `trig.targets` (declared targets), and the typed accessors listed just above.

{cat}

WORKED EFFECT FN — the trigger effect fn combining the catalog and the script prelude:
```rust
{worked}
```

=== TARGET CARD ===
{spec}

BONES ARE AUTHORITATIVE AND COME ONLY FROM THE TARGET CARD SPEC ABOVE — not from the reference cards (those are for code structure only). Transcribe verbatim, never infer from the card's name:
- `mana_cost`: the spec's `Mana cost` string EXACTLY into `ManaCost::parse(\"…\")`.
- `colors`: exactly the colored pips of that cost (combine with `|`); never add a color the cost lacks.
- `types`: exactly the spec's `Type line` (Enchantment → `TypeLine::ENCHANTMENT.into()`). Enchantments have NO power/toughness — leave both to `..Default::default()`.
- subtypes: any words after the `—` in the type line are interned subtypes (`let s = reg.interner_mut().intern(\"…\");` then insert into a `SubtypeSet`). Most plain enchantments have none.

Then build ONE `TriggeredAbilityDef` per trigger clause (ids 1, 2, … in printed order) whose `trigger_condition` matches the oracle's trigger and whose `effect` fn returns the `Effect`s for what follows it. If an effect genuinely cannot be expressed with any catalog variant, the effect fn returns `Vec::new()` with a `// GAP: <what is missing>` comment — never invent an `Effect` / `TriggerCondition` / `GameEvent` variant or a field not shown. Generate the Rust source. Output only the file contents.",
        spec = card_spec(card),
        trigcat = TRIGGER_CONDITION_CATALOG,
        paccess = TRIGGER_PENDING_ACCESSORS,
        cat = effect_catalog("trig"),
        worked = WORKED_TRIGGER_FN,
        FS_UNDERWORLD_DREAMS = FS_UNDERWORLD_DREAMS,
        FS_ANGELIC_CHORUS = FS_ANGELIC_CHORUS,
        FS_PHYREXIAN_TYRANNY = FS_PHYREXIAN_TYRANNY,
        FS_GLORIOUS_ANTHEM = FS_GLORIOUS_ANTHEM,
    )
}

/// Per-card prompt block for ActivatedArtifact (Wave 1). A
/// non-creature, non-Equipment artifact whose text is activated
/// abilities (mana rocks, utility activations), optionally plus a
/// triggered ability.
fn user_activated_artifact(card: &Card) -> String {
    format!(
        "Generate a NON-CREATURE ARTIFACT WITH ACTIVATED ABILITIES — a `CardDefinition` carrying one `ActivatedAbilityDef` per activated clause (plus a `TriggeredAbilityDef` if the text also has a trigger), each with a free `effect` fn referenced as a fn pointer. An artifact's abilities are wired EXACTLY like a creature's — only the characteristics differ: `types: TypeLine::ARTIFACT.into()`, usually `colors: ColorSet::new()` (colorless), and NO power/toughness.

REFERENCE — Mind Stone ({{1}} artifact, '{{T}}: Add {{C}}.' + '{{1}}, {{T}}, Sacrifice Mind Stone: Draw a card.' — the mana-rock archetype: a tap-only `is_mana_ability: true` colorless-mana ability CHAINED with a second utility activation whose cost combines mana + tap + sacrifice on one `ActivationCost`):
```rust
{FS_MIND_STONE}
```

{actcost}

ACTIVATED EFFECT FN BINDING — your effect fn's signature is `fn(_: &GameState, ctx: &ActivationContext, reg: &CardRegistry) -> Vec<Effect>`. `ctx` carries `ctx.controller` (the activator), `ctx.source` (this artifact's `ObjectId`), `ctx.targets` (the declared targets — read via `ctx.targets.targets.first()` + a `TargetChoice` match), `ctx.x_value: Option<u32>` (for X-cost activations), and `ctx.card_id`. Targets are read EXACTLY the same way as the spell program.

MANA ROCKS — '{{T}}: Add {{C}}' / 'Add {{G}}' etc. is a mana ability: `cost: ActivationCost::tap_only()`, `is_mana_ability: true`, effect `Effect::AddMana {{ player: ctx.controller, mana: vec![ManaUnit::plain(ManaColor::Colorless, ctx.source)] }}` (one `ManaUnit::plain(color, ctx.source)` per pip — `{{C}}{{C}}` is two units). For 'Add one mana of any color' or 'Add {{U}} or {{B}}': author ONE mana ability PER color, each `{{T}}: Add {{X}}.` — choosing which ability to activate IS the color choice (for any-color, five abilities, one per WUBRG color).

IF THE TEXT ALSO HAS A TRIGGERED ABILITY (\"Whenever …\", \"At the beginning of …\"), add a `TriggeredAbilityDef` exactly as the trigger catalog below describes — its effect fn binding is `trig: &PendingTrigger` (NOT `ctx`); chain `.with_triggered_ability(…)` alongside the `.with_activated_ability(…)` calls.

{trigcat}

{cat}

=== TARGET CARD ===
{spec}

BONES ARE AUTHORITATIVE AND COME ONLY FROM THE TARGET CARD SPEC ABOVE — not from the reference cards (those are for code structure only). Transcribe verbatim:
- `mana_cost`: exactly the spec's `Mana cost` string into `ManaCost::parse(\"…\")`.
- `colors`: exactly the colored pips of that cost — most artifacts have none → `ColorSet::new()`; never add a color the cost lacks.
- `types`: exactly the spec's `Type line` (Artifact → `TypeLine::ARTIFACT.into()`). NO power/toughness — leave both to `..Default::default()`.
- subtypes: any words after the `—` are interned subtypes inserted into a `SubtypeSet`.

Then build ONE `ActivatedAbilityDef` per activated clause (cost per the ACTIVATION COST CATALOG; `is_mana_ability: true` ONLY for pure add-mana abilities) and, if present, the triggered ability. If an effect genuinely cannot be expressed with any catalog variant, the effect fn returns `Vec::new()` with a `// GAP: <what is missing>` comment — never invent an `Effect` / `ActivationCost` field. Generate the Rust source. Output only the file contents.",
        spec = card_spec(card),
        actcost = ACTIVATION_COST_CATALOG,
        trigcat = TRIGGER_CONDITION_CATALOG,
        cat = effect_catalog("ctx"),
        FS_MIND_STONE = FS_MIND_STONE,
    )
}

/// Per-card prompt block for UtilityLand (Wave 1). A nonbasic land:
/// mana abilities (color choices = one ability per color),
/// enters-tapped, at most one extra simple activated ability.
fn user_utility_land(card: &Card) -> String {
    format!(
        "Generate a NONBASIC LAND — a `CardDefinition` whose characteristics have `mana_cost: None`, `colors: ColorSet::new()`, `types: TypeLine::LAND.into()`, and whose abilities are `ActivatedAbilityDef`s exactly like a creature's. Lands are PLAYED, not cast — there is NO `SpellAbilityDef` and NO mana cost; the engine's land-drop path keys purely on the LAND type.

REFERENCE — Rogue's Passage (land, '{{T}}: Add {{C}}.' + '{{4}}, {{T}}: Target creature can't be blocked this turn.' — the utility-land archetype: a colorless mana ability chained with one targeted utility activation):
```rust
{FS_ROGUES_PASSAGE}
```

REFERENCE — Dimir Guildgate (land — Gate, 'enters tapped' + '{{T}}: Add {{U}} or {{B}}.' — the tapland archetype: `EntersWithSpec::Tapped` plus a two-color mana CHOICE modeled as TWO separate mana abilities, one per color):
```rust
{FS_DIMIR_GUILDGATE}
```

LAND RULES — apply these exactly:
- '[This land] enters the battlefield tapped.' / 'enters tapped.' → chain `.with_enters_with(EntersWithSpec::Tapped)` (import `EntersWithSpec` from `arcana_core::registry`). Do NOT author a trigger for it.
- '{{T}}: Add {{G}}.' → one mana ability (`ActivationCost::tap_only()`, `is_mana_ability: true`, `Effect::AddMana` with `ManaUnit::plain(ManaColor::Green, ctx.source)`).
- '{{T}}: Add {{U}} or {{B}}.' → TWO mana abilities, '{{T}}: Add {{U}}.' and '{{T}}: Add {{B}}.' — choosing which to activate IS the color choice (the Dimir Guildgate reference). 'Add one mana of any color' → FIVE mana abilities, one per WUBRG color.
- '{{T}}: Add {{C}}{{C}}.' → ONE ability whose effect has two `ManaUnit::plain(ManaColor::Colorless, ctx.source)` entries in the `mana` vec.
- Conditional / restricted mana ('Spend this mana only to cast creature spells', '~ doesn't untap', 'Add {{B}} for each Swamp') — spend restrictions are NOT expressible: emit the plain mana ability and add `// GAP: <restriction>`.
- One extra utility activation (the Rogue's Passage shape) is wired per the ACTIVATION COST CATALOG below, `is_mana_ability: false`.

{actcost}

ACTIVATED EFFECT FN BINDING — your effect fn's signature is `fn(_: &GameState, ctx: &ActivationContext, reg: &CardRegistry) -> Vec<Effect>`. `ctx` carries `ctx.controller`, `ctx.source` (this land's `ObjectId`), `ctx.targets` (read via `ctx.targets.targets.first()` + a `TargetChoice` match), and `ctx.x_value`.

{cat}

=== TARGET CARD ===
{spec}

BONES ARE AUTHORITATIVE AND COME ONLY FROM THE TARGET CARD SPEC ABOVE — not from the reference cards (those are for code structure only). Transcribe verbatim:
- LANDS HAVE NO MANA COST: the spec has no `Mana cost` line — set `mana_cost: None` and `colors: ColorSet::new()`. (A land is colorless regardless of what colors of mana it produces.)
- `types`: exactly the spec's `Type line` (`TypeLine::LAND.into()`; a 'Legendary Land' adds `supertypes: SupertypeSet(SupertypeSet::LEGENDARY)`). NO power/toughness.
- subtypes: words after the `—` (Gate, Desert, Cave, Lair, …) are interned and inserted into a `SubtypeSet`. Basic-land TYPE words (Plains/Island/Swamp/Mountain/Forest) on a nonbasic land are subtypes too — intern them the same way.

Then wire each printed line per the LAND RULES above. If an effect genuinely cannot be expressed with any catalog variant, the effect fn returns `Vec::new()` with a `// GAP: <what is missing>` comment — never invent an `Effect` / `ActivationCost` field / `EntersWithSpec` variant. Generate the Rust source. Output only the file contents.",
        spec = card_spec(card),
        actcost = ACTIVATION_COST_CATALOG,
        cat = effect_catalog("ctx"),
        FS_ROGUES_PASSAGE = FS_ROGUES_PASSAGE,
        FS_DIMIR_GUILDGATE = FS_DIMIR_GUILDGATE,
    )
}

/// Per-card prompt block for Equipment (Wave 1). `with_equip(cost)`
/// + the ETB-installed `ContinuousEffect::attached_pt` static.
fn user_equipment(card: &Card) -> String {
    format!(
        "Generate an EQUIPMENT ARTIFACT (CR 702.6) — an artifact — Equipment `CardDefinition` built from exactly two pieces:
1. `.with_equip(ManaCost::parse(\"{{N}}\").expect(\"valid cost\"))` — synthesizes the canonical 'Equip {{N}}' activated ability (sorcery speed, target creature you control, attach on resolution). NEVER author the Equip activation by hand; the builder is the only path that wires attachment correctly.
2. The 'Equipped creature gets +P/+T' static: an ETB `TriggeredAbilityDef` whose effect installs `ContinuousEffect::attached_pt(trig.source, P, T, Duration::WhileSourceOnBattlefield)` via `Effect::InstallContinuousEffect`. The layer system dereferences the Equipment's `attached_to` dynamically, so the pump follows every re-equip and expires when the Equipment leaves.

REFERENCE — Bonesplitter ({{1}} artifact — Equipment, 'Equipped creature gets +2/+0. Equip {{1}}' — the COMPLETE pattern; your file should mirror it with only the numbers / cost changed):
```rust
{FS_BONESPLITTER}
```

EXPRESSIBLE vs GAP — the Equipment static surface is deliberately narrow:
- 'Equipped creature gets +P/+T.' → `ContinuousEffect::attached_pt(trig.source, P, T, Duration::WhileSourceOnBattlefield)`. Negative values are fine ('gets -1/-0' → `attached_pt(trig.source, -1, 0, …)`).
- 'Equipped creature is a [subtype] in addition to its other types.' → `ContinuousEffect::attached_subtypes(trig.source, subs, Duration::WhileSourceOnBattlefield)` where `subs` is built INSIDE the effect fn: `let mut subs = SubtypeSet::default(); if let Some(s) = reg.interner().lookup(\"Bird\") {{ subs.0.insert(s); }}` — intern 'Bird' in `register` first so the lookup finds it. ('Is EVERY creature type' — changeling CDA — is NOT expressible; GAP it.)
- '… and is [color] in addition to its other colors.' → `ContinuousEffect::attached_colors(trig.source, ColorSet::white(), Duration::WhileSourceOnBattlefield)`.
- 'Equipped creature is an artifact in addition to its other types.' → `ContinuousEffect::attached_types(trig.source, TypeLine::ARTIFACT.into(), Duration::WhileSourceOnBattlefield)`.
- 'Equipped creature is every creature type.' → `ContinuousEffect::attached_every_creature_type(trig.source, Duration::WhileSourceOnBattlefield)` (changeling templating — tribal filters see it).
- 'Target permanent becomes legendary in addition to its other supertypes.' → `ContinuousEffect::add_supertypes(0, target_id, SupertypeSet::new().with(SupertypeSet::LEGENDARY), <Duration>)`.
- 'Target creature becomes a [subtype] in addition to its other types.' → `ContinuousEffect::add_subtypes(0, target_id, subs, <Duration>)` (targeted sibling; build `subs` via interner lookup as above).
- 'Equipped creature loses [keyword].' → `ContinuousEffect::attached_loses_keyword(trig.source, KeywordAbility::Flying, Duration::WhileSourceOnBattlefield)`.
- 'Equipped creature gets +1/+1 for each [thing].' → `ContinuousEffect::attached_pt_dynamic(trig.source, compute_fn, Duration::WhileSourceOnBattlefield)` where `compute_fn` is a module-level `fn(&GameState, ObjectId) -> (i32, i32)` receiving the EQUIPMENT's id (read its controller via `state.objects.get(source)`, count whatever the card names, return the (power, toughness) delta — re-evaluated automatically as the board changes). Use this instead of GAP-ing dynamic equipped-creature pumps.
- 'Equipped creature gets +P/+T and has [keyword].' → install BOTH statics from the same ETB effect fn: `Effect::InstallContinuousEffect {{ effect: ContinuousEffect::attached_pt(trig.source, P, T, Duration::WhileSourceOnBattlefield) }}` and `Effect::InstallContinuousEffect {{ effect: ContinuousEffect::attached_keyword(trig.source, KeywordAbility::Flying, Duration::WhileSourceOnBattlefield) }}` (one per keyword for 'has flying and vigilance'). `attached_keyword` follows the attachment dynamically exactly like `attached_pt`. Do NOT use `ContinuousEffect::grant_keyword` (fixed target id). Only `KeywordAbility` variants demonstrated in this prompt are valid.
- 'Equipped creature has [keyword]' ONLY (no P/T) → install nothing; the ETB effect fn returns `Vec::new()` with the same GAP comment. Still emit `.with_equip(…)` — the Equip half is always real.
- Triggered riders ('Whenever equipped creature deals combat damage …') → author the `TriggeredAbilityDef` normally if its condition exists in the trigger catalog of the engine; otherwise GAP it.
- Equip cost: exactly the printed 'Equip {{N}}' mana cost. ('Equip — Sacrifice a creature' and other non-mana equip costs are NOT expressible — GAP the whole equip line and do not call `with_equip`.)

IMPORTS — mirror the reference exactly: `ContinuousEffect`, `Duration` from `arcana_core::layers`; `Effect` from `arcana_core::effects`; the trigger types from `arcana_core::triggers`.

=== TARGET CARD ===
{spec}

BONES ARE AUTHORITATIVE AND COME ONLY FROM THE TARGET CARD SPEC ABOVE — not from the reference card. Transcribe verbatim:
- `mana_cost`: exactly the spec's `Mana cost` string into `ManaCost::parse(\"…\")`.
- `colors`: exactly the colored pips of that cost — most Equipment is colorless → `ColorSet::new()`.
- `types`: `TypeLine::ARTIFACT.into()`; subtypes: intern \"Equipment\" (plus any other listed subtype) into a `SubtypeSet`. NO power/toughness.

Then `.with_equip(<printed equip cost>)` + the ETB install trigger per the rules above. If the static genuinely cannot be expressed, the effect fn returns `Vec::new()` with a `// GAP: <what is missing>` comment — never invent a `ContinuousEffect` builder or an `Effect` variant. Generate the Rust source. Output only the file contents.",
        spec = card_spec(card),
        FS_BONESPLITTER = FS_BONESPLITTER,
    )
}

/// Per-card prompt block for StaticEnchantment (Wave 1). The
/// Glorious Anthem ETB-install pattern for a single static
/// continuous effect.
fn user_static_enchantment(card: &Card) -> String {
    format!(
        "Generate a STATIC ENCHANTMENT — an enchantment whose single line of text is a static continuous ability ('Creatures you control get +1/+1', 'Creatures you control have flying'). The engine models this as an ETB-installed `ContinuousEffect` with `Duration::WhileSourceOnBattlefield`: a `TriggeredAbilityDef` on `TriggerCondition::SelfEntersBattlefield` whose effect fn returns ONE `Effect::InstallContinuousEffect`. The layer-cleanup pipeline auto-expires the effect when the enchantment leaves the battlefield, so the install is behaviorally identical to a true static for any permanent that doesn't blink.

REFERENCE — Glorious Anthem ({{1}}{{W}} enchantment, 'Creatures you control get +1/+1' — the COMPLETE pattern; your file should mirror it with only the builder swapped):
```rust
{FS_GLORIOUS_ANTHEM}
```

CONTINUOUS-EFFECT BUILDERS — the complete Wave-1 static surface. Each is constructed inside the ETB effect fn and wrapped in `Effect::InstallContinuousEffect {{ effect: <builder> }}`. Import `ContinuousEffect`, `Duration` from `arcana_core::layers`.
- 'Creatures you control get +P/+T.' → `ContinuousEffect::anthem(trig.source, trig.controller, P, T, Duration::WhileSourceOnBattlefield)`. Negative statics ('Creatures your opponents control get -1/-0') are NOT this builder — GAP them (anthem is controller-scoped, positive or negative P/T values but only for YOUR creatures; an opponent-scoped anthem is not expressible).
- 'Creatures you control have [keyword].' → `ContinuousEffect::keyword_anthem(trig.source, trig.controller, KeywordAbility::Flying, Duration::WhileSourceOnBattlefield)` — any `KeywordAbility` unit variant from the system-prompt list. For 'have [kw1] and [kw2]' return TWO `Effect::InstallContinuousEffect` values in the vec.
- 'Creatures you control get +P/+T and have [keyword].' → return BOTH installs (one `anthem`, one `keyword_anthem`) in the same vec.
- FILTERED statics ('Goblins you control get +1/+1', 'White creatures get -1/-0', 'Creatures with flying get +1/+1', 'all Zombies have menace') → `ContinuousEffect::filtered_pump(trig.source, <ObjectFilter>, P, T, Duration::WhileSourceOnBattlefield)` / `ContinuousEffect::filtered_keyword(trig.source, <ObjectFilter>, KeywordAbility::Menace, Duration::WhileSourceOnBattlefield)`. The filter is built with the usual `ObjectFilter` builders — `ObjectFilter::creature().controlled_by(ControllerConstraint::You)` (+ `.with_subtype_sym(goblin)` for tribal lords, `.with_colors(ColorSet::white())`, `.with_keyword(KeywordAbility::Flying)`, `.with_max_power(2)`). IMPORTANT: the filter is matched against PRINTED (base) characteristics — do not rely on it seeing other layer effects. Opponent-scoped statics use `.controlled_by(ControllerConstraint::Opponent)`; unscoped ('all creatures') omit the controller builder.
- '[filter] creatures can't attack.' → `ContinuousEffect::filtered_cant_attack(trig.source, <ObjectFilter>, Duration::WhileSourceOnBattlefield)`; '…can't block.' → `filtered_cant_block`. ('Creatures with power 2 or less can't block', Peacekeeper-class 'creatures can't attack' with `ObjectFilter::creature()`.)
- 'Creatures can't attack you [or planeswalkers you control] unless their controller pays {{N}} for each attacking creature.' (Ghostly Prison / Propaganda) → `ContinuousEffect::attack_tax(trig.source, N, Duration::WhileSourceOnBattlefield)`.
- 'If you would gain life, you gain twice that much instead.' (Boon Reflection) → `Effect::InstallReplacementEffect {{ effect: Box::new(ReplacementEffect {{ source: trig.source, id: 0, condition: ReplacementCondition::WouldGainLife {{ player: ControllerConstraint::You }}, kind: ReplacementKind::MultiplyLifeGain(2), is_self_replacement: false, duration: ReplacementDuration::WhileSourceOnBattlefield }}) }}` (import `ReplacementCondition`, `ReplacementEffect`, `ReplacementKind`, `ReplacementDuration` from `arcana_core::replacement`).
- '[Permanents] your opponents control enter the battlefield tapped.' (Kismet / Frozen Aether) → the same `Effect::InstallReplacementEffect` shape with `condition: ReplacementCondition::WouldEnterBattlefield {{ object_filter: <filter incl. .controlled_by(ControllerConstraint::Opponent)> }}, kind: ReplacementKind::EtbTapped`.
- 'If a [filter] would deal damage to a creature or player, it deals double that damage instead.' (Gratuitous Violence / Furnace of Rath class) → the same `Effect::InstallReplacementEffect` shape with `condition: ReplacementCondition::WouldDealDamage {{ source_filter: <ObjectFilter — e.g. creature().controlled_by(ControllerConstraint::You)>, target_filter: TargetFilter::AnyTarget }}, kind: ReplacementKind::DoubleDamage`.
- BACKGROUNDS / commander grants ('Commander creatures you own have <triggered ability>'): `ContinuousEffect::filtered_grant_triggered(trig.source, ObjectFilter::creature().commander_only().controlled_by(ControllerConstraint::You), <TriggeredAbilityDef>, Duration::WhileSourceOnBattlefield)` wrapped in `Effect::InstallContinuousEffect`. Build the granted `TriggeredAbilityDef` exactly like a printed one (module-level `fn` for its effect) BUT its `id` MUST be `arcana_core::triggers::GRANTED_TRIGGER_ID_BASE + 1`. 'you own' is approximated by controlled_by(You) (note it). Backgrounds' bones: Legendary supertype + an interned 'Background' subtype. The designation is inert outside commander games — that is the faithful reading, wire it anyway. Granted KEYWORD/P-T forms ('Commander creatures you own get +2/+2 / have ward {{1}}') use `filtered_pump` / `filtered_keyword` with the same commander filter.
- UNTAP restrictions: '[filter] don't untap during their controllers' untap steps' (Crackdown, Choke, Winter Orb) → `ContinuousEffect::filtered_dont_untap(trig.source, <ObjectFilter>, Duration::WhileSourceOnBattlefield)`; 'players can't untap more than N [filter] during their untap steps' (Damping Field, Smoke) → `ContinuousEffect::untap_cap(trig.source, <ObjectFilter>, N, Duration::WhileSourceOnBattlefield)`; '~ doesn't untap during its controller's untap step' (a single permanent) → `ContinuousEffect::dont_untap(trig.source, target_id, <Duration>)` — for 'during your NEXT untap step' one-shots use `Duration::UntilNextUpkeepOf(p)` (upkeep follows untap, so the window covers exactly one untap step; UntilYourNextTurn expires BEFORE the untap step — wrong).
- DAMAGE-PREVENTION statics, all via the same `Effect::InstallReplacementEffect` shape: 'Prevent all [combat / noncombat] damage that would be dealt to [filter/you]' → `condition: ReplacementCondition::WouldDealDamage {{ source_filter: <ObjectFilter>, target_filter: <TargetFilter — Permanent(creature filter) / Player / AnyTarget>, combat: None|Some(true)|Some(false) }}, kind: ReplacementKind::PreventAllDamage`; 'to YOU specifically' → `condition: ReplacementCondition::WouldDealDamageToSpecific {{ target: DamageTarget::Player(trig.controller) }}` (import DamageTarget from `arcana_core::events`). 'If a source would deal N or more damage…, it deals N instead' (Divine Presence) → `kind: ReplacementKind::CapDamageAt(N)`. Worship ('if you control a creature, damage that would reduce your life below 1 reduces it to 1') → `kind: ReplacementKind::DamageLifeFloor {{ floor: 1 }}` + `state_gate: Some(arcana_core::conditions::controls_a_creature)` + the ToSpecific condition above. 'During your turn, …' conditionals → `state_gate: Some(arcana_core::conditions::is_your_turn)`. Every ReplacementEffect literal needs `state_gate: None` when unconditional.
- TOKEN-scoped statics: `ObjectFilter` HAS a token predicate — `.tokens_only()` / `.nontoken()` ('Creature tokens you control get +1/+1 and have vigilance' → `filtered_pump(trig.source, ObjectFilter::creature().tokens_only().controlled_by(ControllerConstraint::You), 1, 1, ...)` + `filtered_keyword(...)`; Illness in the Ranks / Virulent Plague = opponent/unscoped token debuffs). Never GAP 'no token filter'.
- TOKEN DOUBLING ('If an effect would create one or more tokens [under your control], twice that many … instead' — Parallel Lives, Anointed Procession, Doubling Season's token half) → `Effect::InstallReplacementEffect` with `condition: ReplacementCondition::WouldCreateToken {{ token_filter: ObjectFilter::default().controlled_by(ControllerConstraint::You) }}, kind: ReplacementKind::MultiplyTokens(2)`. Token-IDENTITY replacement (Divine Visitation 'creates an Angel instead') is NOT expressible — GAP it.
- BLOCK CAPS ('each creature you control can't be blocked by more than one creature' — Familiar Ground) → `ContinuousEffect::filtered_max_blockers(trig.source, ObjectFilter::creature().controlled_by(ControllerConstraint::You), 1, Duration::WhileSourceOnBattlefield)`.
- GLOBAL keyword REMOVAL ('all creatures lose flying' — Gravity Sphere; Mystic Decree) → `ContinuousEffect::filtered_remove_keyword(trig.source, ObjectFilter::creature(), KeywordAbility::Flying, Duration::WhileSourceOnBattlefield)` — one install per removed keyword. (Landwalk loss and block-as-though-landwalk stay GAP'd; 'block an additional creature' multi-block is NOT expressible — GAP it.)
- COST MODIFIERS ('[filter] spells cost {{N}} more/less to cast' — Chill, Sphere of Resistance, Arcane Melee, Thalia-class) → `ContinuousEffect::spell_cost_modifier(trig.source, <spell ObjectFilter — types/colors only, e.g. ObjectFilter::new().with_colors(ColorSet::red())>, <ControllerConstraint — Any / You / Opponent for 'spells your opponents cast'>, N_or_negative_N, Duration::WhileSourceOnBattlefield)`. Adjusts the GENERIC component only (floor 0; colored pips never change — 'cost {{W}} more' stays GAP'd). 'Activated abilities of [filter] cost {{N}} less' (Training Grounds) → `ContinuousEffect::ability_cost_modifier(trig.source, <source-permanent ObjectFilter>, -N, ...)`.
- Counter-gated filters: `ObjectFilter` has `has_counter: Option<CounterKind>` ('each creature with a level counter' → `ObjectFilter {{ has_counter: Some(CounterKind::Level), ..ObjectFilter::creature() }}` — struct-update on a builder result is fine).
- Any other static ('Spells cost {{1}} more', 'Players can't gain life', 'nonbasic lands are Mountains', untap restrictions, targeting restrictions, token-scoped lords — there is NO token filter, alternative-cost statics) is still NOT expressible — the effect fn returns `Vec::new()` with a `// GAP: <which static is missing>` comment. The bones (name/cost/colors/types) are still valuable; emit them faithfully.

=== TARGET CARD ===
{spec}

BONES ARE AUTHORITATIVE AND COME ONLY FROM THE TARGET CARD SPEC ABOVE — not from the reference card. Transcribe verbatim, never infer from the card's name:
- `mana_cost`: exactly the spec's `Mana cost` string into `ManaCost::parse(\"…\")`.
- `colors`: exactly the colored pips of that cost (combine with `|`); never add a color the cost lacks.
- `types`: exactly the spec's `Type line` (Enchantment → `TypeLine::ENCHANTMENT.into()`). NO power/toughness.
- subtypes: any words after the `—` are interned subtypes in a `SubtypeSet`; most plain enchantments have none.

Then the ETB trigger (`id: 1`, `TriggerCondition::SelfEntersBattlefield`, `intervening_if: None`, `trigger_zones: vec![Zone::Battlefield]`, `frequency: TriggerFrequency::EachTime`, `target_requirements: Vec::new()`) whose effect fn installs the matching builder(s). Never invent a `ContinuousEffect` builder, an `Effect` variant, or a filter parameter that the builders above don't show. Generate the Rust source. Output only the file contents.",
        spec = card_spec(card),
        FS_GLORIOUS_ANTHEM = FS_GLORIOUS_ANTHEM,
    )
}

/// Per-card prompt block for Aura (Wave 2). An Aura enchants a
/// permanent on resolution and grants it an `attached_*` continuous
/// effect — the Equipment idiom, minus the Equip ability, plus the
/// engine-driven resolution attach.
fn user_aura(card: &Card) -> String {
    format!(
        "Generate an AURA — an enchantment that attaches to a permanent and modifies it ('Enchant creature. Enchanted creature gets +1/+2.', 'Enchant creature. Enchanted creature can't attack or block.'). The engine handles attachment for you: `CardDefinition::with_enchant(<TargetFilter>)` installs the spell's enchant target requirement, and on resolution the engine attaches the Aura to the chosen target (CR 303.4f) — you NEVER write an `Effect::Attach`. The Aura's grant is an ETB-installed `attached_*` `ContinuousEffect` that follows `source.attached_to`, so it lands on the enchanted permanent and auto-expires when the Aura leaves the battlefield. This is EXACTLY the Equipment pattern (Bonesplitter / Trusty Machete) with `with_equip` swapped for `with_enchant` and the Equipment subtype swapped for an interned `Aura` subtype.

REFERENCE 1 — Holy Strength ({{W}} Aura, 'Enchant creature. Enchanted creature gets +1/+2.' — the COMPLETE buff pattern; mirror it with only the numbers / builder swapped):
```rust
{FS_HOLY_STRENGTH}
```

REFERENCE 2 — Pacifism ({{1}}{{W}} Aura, 'Enchant creature. Enchanted creature can't attack or block.' — the restriction pattern; note TWO installs returned in the vec):
```rust
{FS_PACIFISM}
```

ENCHANT TARGET — `with_enchant(<TargetFilter>)` (import `TargetFilter` from `arcana_core::targets`):
- 'Enchant creature' → `TargetFilter::Creature` (the overwhelming majority).
- 'Enchant land' / 'Enchant artifact' / 'Enchant permanent' → `TargetFilter::Permanent(ObjectFilter::permanent().with_types(TypeLine::LAND.into()))` / `Permanent(ObjectFilter::permanent().with_types(TypeLine::ARTIFACT.into()))` / `Permanent(ObjectFilter::permanent())`. `ObjectFilter` has ONLY `creature()` and `permanent()` constructors — there is NO `ObjectFilter::land()` / `artifact()` / `enchantment()`; build those by `permanent().with_types(<TypeLine>)`. 'Enchant artifact or creature' has no disjunction filter — use `ObjectFilter::permanent()` (any) and add `// NOTE: artifact-or-creature widened to any permanent`.
- 'Enchant creature you control' / '… an opponent controls': use `TargetFilter::Creature` anyway — the caster chooses which creature to target, so the controller wording is honored in practice; add a `// NOTE: controller wording approximated by caster's choice` comment.
- 'Enchant PLAYER' (Curses) is NOT supported — the engine attaches to objects only. Emit the bones with `with_enchant(TargetFilter::Creature)` REMOVED and the effect fn returning `Vec::new()` with `// GAP: enchant player (Curse) — no player attachment`.

ATTACHED-GRANT BUILDERS — constructed inside the ETB effect fn, each wrapped in `Effect::InstallContinuousEffect {{ effect: <builder> }}`. Import `ContinuousEffect`, `Duration` from `arcana_core::layers`. `trig.source` is the Aura. ALWAYS `Duration::WhileSourceOnBattlefield`.
- 'Enchanted creature gets +P/+T.' (incl. negative, e.g. -2/-0) → `ContinuousEffect::attached_pt(trig.source, P, T, Duration::WhileSourceOnBattlefield)`.
- 'Enchanted creature gets +X/+Y, where X is [a board count].' → `ContinuousEffect::attached_pt_dynamic(trig.source, <fn(&GameState, ObjectId) -> (i32, i32)>, Duration::WhileSourceOnBattlefield)` — a module-level fn that reads the SOURCE (the Aura) to count, like Empyrial Armor ('+1/+1 for each card in your hand'). LIMITATION: the compute fn is a bare `fn(&GameState, ObjectId)` with NO `CardRegistry`/interner, so it CANNOT count by a NAMED subtype ('+1/+0 for each Forest you control', '+1/+1 for each Mountain') — the subtype symbol is unresolvable inside it. GAP those: `// GAP: dynamic P/T from a named-subtype count`. Counts of zone size / colorless categories (cards in hand, creatures you control via a type-only filter) are fine.
- 'Enchanted creature has [keyword].' → `ContinuousEffect::attached_keyword(trig.source, KeywordAbility::Flying, Duration::WhileSourceOnBattlefield)` — any `KeywordAbility` unit variant from the system-prompt list. 'has [kw1] and [kw2]' → TWO installs.
- 'Enchanted creature loses [keyword]' / 'can't have its abilities … ' (keyword removal) → `ContinuousEffect::attached_loses_keyword(trig.source, KeywordAbility::Flying, Duration::WhileSourceOnBattlefield)`.
- 'Enchanted creature can't attack.' → `ContinuousEffect::attached_cant_attack(trig.source, Duration::WhileSourceOnBattlefield)`; '… can't block.' → `attached_cant_block`; 'can't attack OR block' → BOTH (see Pacifism).
- 'Enchanted creature can't be blocked.' → `ContinuousEffect::attached_cant_be_blocked(trig.source, Duration::WhileSourceOnBattlefield)`. (Conditional 'can't be blocked except by [X]' / 'by more than one' stays GAP.)
- 'Enchanted creature doesn't untap during its controller's untap step.' → `ContinuousEffect::attached_dont_untap(trig.source, Duration::WhileSourceOnBattlefield)`.
- 'Enchanted creature has base power and toughness X/Y' / 'is a 0/1' / '~becomes an X/Y creature' (Zendikon animations, Lignify, Ensoul Artifact) → `ContinuousEffect::attached_set_pt(trig.source, X, Y, Duration::WhileSourceOnBattlefield)` — SETS base P/T (vs additive attached_pt). For an animation that also adds types/subtypes/color/keywords (e.g. Zendikon 'becomes a 4/2 red Elemental with haste'), return attached_set_pt + attached_types + attached_subtypes + attached_colors + attached_keyword installs together.
- 'Enchanted creature is [color] [in addition].' → `ContinuousEffect::attached_colors(trig.source, ColorSet::blue(), Duration::WhileSourceOnBattlefield)` (ADDITIVE).
- 'Enchanted creature is a(n) [subtype] [in addition to its other types].' → `ContinuousEffect::attached_subtypes(trig.source, {{ let mut s = SubtypeSet::default(); s.0.insert(reg.interner_mut().intern(\"Angel\")); s }}, Duration::WhileSourceOnBattlefield)`. (Intern subtypes BEFORE building the def, store the symbols, and capture them — you cannot call `reg` inside the effect fn. Follow the Holy Strength import/intern structure.) Adding a card TYPE ('is also an artifact') → `attached_types(trig.source, TypeLine::ARTIFACT.into(), Duration::WhileSourceOnBattlefield)`.
- 'Enchanted creature gets +P/+T AND has [keyword]' (and any combination) → return ALL the matching installs in one vec.

NOT EXPRESSIBLE — emit faithful bones (name/cost/colors/types + Aura subtype + `with_enchant`) and an effect fn returning `Vec::new()` with a `// GAP: <reason>` comment:
- CONTROL-CHANGE Auras ('You control enchanted creature' — Control Magic, Mind Control). // GAP: control-change aura.
- conditional 'as long as' clauses, 'enchant creature you don't control'-only-targeting restrictions, and any payoff that fires when the Aura itself enters/leaves. // GAP: <which>.

HOST-TRIGGER and HOST-ACTIVATED grants (NOW SUPPORTED — these are NOT GAPs):
- 'When/Whenever enchanted creature <does X>' (host trigger — 'When enchanted creature dies, …', 'Whenever enchanted creature attacks, …', 'Whenever enchanted creature becomes tapped, …', 'Whenever enchanted creature is dealt damage, …'): add a SECOND `TriggeredAbilityDef` to the card (alongside the fixed ETB one, OR as the only ability if the Aura's grant is purely this trigger), with `trigger_condition: TriggerCondition::AttachedCreatureDoes {{ condition: Box::new(TriggerCondition::SelfDies) }}` — wrap the matching `Self*` inner condition: SelfDies / SelfAttacks / SelfBecomesTapped / SelfBecomesBlocked / SelfBlocks / SelfIsDealtDamage {{ combat_only: false }} / SelfBecomesTarget {{ caster: ControllerConstraint::Opponent }}. Give this ability `id: 2`, `intervening_if: None`, `trigger_zones: vec![Zone::Battlefield]`, `frequency: TriggerFrequency::EachTime`, and a free effect fn that does the payoff (the effect runs with `trig.source` = the AURA; reach the host via `trig.dying_object()` / `trig.attacking_creature()` as the existing TriggeredEnchantment exemplars show). ONLY `Self*` conditions wrap correctly — a payoff keyed on 'deals COMBAT damage' (not a Self* condition) stays a `// GAP`.
- 'Enchanted creature has \"\\[cost\\]: \\[effect\\]\"' (host activated ability — Squirrel Nest's '{{T}}: create a Squirrel', Evanescent Intellect's '{{1}}{{U}}, {{T}}: mill 3'): in the ETB effect fn, `Effect::InstallContinuousEffect {{ effect: ContinuousEffect::attached_activated(trig.source, <ActivatedAbilityDef>, Duration::WhileSourceOnBattlefield) }}`. Build the `ActivatedAbilityDef` exactly like an ActivatedArtifact's (text, `cost: ActivationCost {{ mana_cost: ManaCost::parse(\"{{1}}{{U}}\").unwrap(), tap: true, ..Default::default() }}`, target_requirements, is_mana_ability:false, is_loyalty_ability:false, `activation_zone: ActivationZone::Battlefield`, is_instant_speed:false, face_gate:None, effect: <fn>). Its cost ({{T}} etc.) and effect run against the HOST. Import `ActivatedAbilityDef, ActivationCost, ActivationZone, ActivationContext` from `arcana_core::registry`.

=== TARGET CARD ===
{spec}

BONES ARE AUTHORITATIVE AND COME ONLY FROM THE TARGET CARD SPEC ABOVE — not from the reference cards. Transcribe verbatim:
- `mana_cost`: exactly the spec's `Mana cost` into `ManaCost::parse(\"…\")`.
- `colors`: exactly the colored pips of that cost; never add a color the cost lacks.
- `types`: `TypeLine::ENCHANTMENT.into()` (NO power/toughness). Intern an `Aura` subtype into the `SubtypeSet` (see the references).
- The ETB trigger is fixed boilerplate: `id: 1`, `TriggerCondition::SelfEntersBattlefield`, `intervening_if: None`, `trigger_zones: vec![Zone::Battlefield]`, `frequency: TriggerFrequency::EachTime`, `target_requirements: Vec::new()`.

Never invent a `ContinuousEffect` builder, an `Effect` variant, or a filter the lists above don't show. Generate the Rust source. Output only the file contents.",
        spec = card_spec(card),
        FS_HOLY_STRENGTH = FS_HOLY_STRENGTH,
        FS_PACIFISM = FS_PACIFISM,
    )
}

/// Per-card prompt block for Planeswalker (Wave 2). A PW with printed
/// starting loyalty and a set of CR 606 loyalty abilities.
fn user_planeswalker(card: &Card) -> String {
    format!(
        "Generate a PLANESWALKER — a permanent with printed starting loyalty and one or more loyalty abilities ('+1: …', '0: …', '−7: …'). The engine models each loyalty ability as an `ActivatedAbilityDef` with `is_loyalty_ability: true`; the cost is adding/removing Loyalty counters (CR 606.2). The engine ENFORCES the loyalty rules for you — sorcery-speed only, stack empty, controller-only, once per turn per planeswalker (CR 606.3), and the 0-loyalty state-based sacrifice (CR 704.5i). You only declare the abilities.

REFERENCE — Chandra, Pyromaster ({{2}}{{R}}{{R}} planeswalker, starting loyalty 4; the COMPLETE loyalty-ability pattern — mirror its structure):
```rust
{FS_CHANDRA_PYROMASTER}
```

BONES:
- `mana_cost`: exactly the spec's `Mana cost` into `ManaCost::parse(\"…\")`. `colors`: exactly its colored pips.
- `types: TypeLine::PLANESWALKER.into()`. NO power/toughness.
- `loyalty: Some(N)` — the spec's `Loyalty` value (printed starting loyalty). `after_enter_battlefield` places the counters (CR 113.3c).
- subtypes: the planeswalker's name-word is its subtype (Chandra → intern \"Chandra\"; Jace → \"Jace\"). Put it in a `SubtypeSet`.

LOYALTY ABILITIES — one `ActivatedAbilityDef` per printed loyalty line. ALL share: `is_loyalty_ability: true`, `activation_zone: arcana_core::registry::ActivationZone::Battlefield`, `is_instant_speed: false`, `face_gate: None`, and a free `effect` fn. The COST encodes the loyalty symbol:
- '+N: …' → `cost: ActivationCost {{ add_self_counter: Some((CounterKind::Loyalty, N)), ..ActivationCost::default() }}`.
- '−N: …' → `cost: ActivationCost {{ remove_self_counter: Some((CounterKind::Loyalty, N)), ..ActivationCost::default() }}` (the engine filters the ability out unless loyalty ≥ N, so ultimates are gated automatically).
- '0: …' → `cost: ActivationCost::default()` (no counter change).
The effect fn has the activated-ability signature `fn(&GameState, &ActivationContext, &CardRegistry) -> Vec<Effect>` — reach targets via `ctx.targets.targets.first()`, the source via `ctx.source` (exactly like Chandra). Use the full `Effect` catalog from the system prompt (DealDamage, DrawCards, CreateToken, DestroyPermanent, ExilePermanent, GainLife, AddCounters on a target, etc.) and `TargetRequirement` for any 'target …' clause.

GOTCHAS:
- `TypeLine::LAND` / `TypeLine::PLANESWALKER` / etc. are `u16` constants, NOT `TypeLine` values. In ANY `ObjectFilter` builder they need `.into()`: `ObjectFilter::permanent().without_types(TypeLine::LAND.into())`, `.with_types(TypeLine::PLANESWALKER.into())`. The bare constant is the #1 compile error.
- `−X` loyalty ('−X: …' where X is chosen) is NOT expressible — `remove_self_counter` is a fixed `u32`. GAP the ability (shell it with a small fixed cost is wrong; instead `// GAP: dynamic-X loyalty cost`, and omit the ability rather than mis-cost it).
- 'Target card in a graveyard' needs a concrete `Zone::Graveyard(player)` — there's no any-graveyard sentinel in the demonstrated surface, so GAP graveyard-targeting loyalty abilities.

NOT EXPRESSIBLE — for a loyalty ability whose effect can't be built from the demonstrated `Effect` surface, still declare the ability with its correct loyalty COST but make its effect fn return `Vec::new()` with a `// GAP: <reason>` comment. Common GAPs: emblems ('you get an emblem with …'), 'cast from exile / play this turn' riders, static/continuous abilities (no loyalty cost — those aren't loyalty abilities at all; if the PW's only text is a static, GAP the whole card), ultimates with bespoke one-shot effects, dynamic-X costs/effects. Declaring the ability shell (correct +/−/cost) is still valuable even when the effect is GAP'd. Emit AT LEAST the abilities you CAN express.

=== TARGET CARD ===
{spec}

Transcribe bones verbatim from the spec above, never from Chandra. Build each loyalty ability with the correct counter cost. Never invent an `Effect` variant, a cost field, or a builder the reference / system prompt doesn't show. Generate the Rust source. Output only the file contents.",
        spec = card_spec(card),
        FS_CHANDRA_PYROMASTER = FS_CHANDRA_PYROMASTER,
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
    fn t2_artifact_routes_to_activated_artifact() {
        // Wave 1: non-creature artifacts route to the
        // ActivatedArtifact pack instead of the legacy refusal.
        let c = mk_card(|c| {
            c.name = "Mystery Artifact".into();
            c.type_line = "Artifact".into();
            c.oracle_text = Some("Some static ability.".into());
        });
        let p = render_prompt(&c, Tier::Two).expect("artifact now renders");
        assert_eq!(p.shape, PromptShape::ActivatedArtifact);
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
    fn t3_activated_artifact_routes_to_activated_artifact() {
        // Wave 1: was NoFewShotForShape("non-creature permanent").
        let c = mk_card(|c| {
            c.name = "Icy Manipulator".into();
            c.type_line = "Artifact".into();
            c.oracle_text = Some("{1}, {T}: Tap target permanent.".into());
        });
        let p = render_prompt(&c, Tier::Three).expect("artifact now renders");
        assert_eq!(p.shape, PromptShape::ActivatedArtifact);
        assert!(p.user.contains("Mind Stone"), "Mind Stone few-shot");
        assert!(p.user.contains("ACTIVATION COST CATALOG"));
        assert!(p.user.contains("TRIGGER CONDITION CATALOG"));
        assert!(!p.user.contains("{BINDING}"), "BINDING marker substituted");
    }

    #[test]
    fn t3_activated_only_creature_routes_to_activated() {
        // A creature whose sole ability is activated (a mana dork)
        // routes to the ActivatedAbilityCreature few-shot pack
        // (Pass 2 — previously this was deferred as NoFewShotForShape).
        let c = mk_card(|c| {
            c.name = "Llanowar Elves".into();
            c.type_line = "Creature — Elf Druid".into();
            c.oracle_text = Some("{T}: Add {G}.".into());
            c.power = Some("1".into());
            c.toughness = Some("1".into());
        });
        let p = render_prompt(&c, Tier::Three)
            .expect("activated-only T3 creature now renders");
        assert_eq!(p.shape, PromptShape::ActivatedAbilityCreature);
        // The activated few-shots (Llanowar Elves, Prodigal Sorcerer)
        // must be embedded in the user prompt.
        assert!(p.user.contains("Llanowar Elves"));
        assert!(p.user.contains("Prodigal Sorcerer"));
        assert!(p.user.contains("ACTIVATION COST CATALOG"));
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
    fn spell_prompt_embeds_all_references_and_effect_catalog() {
        let c = mk_card(|c| {
            c.name = "Disenchant".into();
            c.type_line = "Instant".into();
            c.oracle_text = Some("Destroy target artifact or enchantment.".into());
        });
        let p = render_prompt(&c, Tier::Two).expect("ok");
        assert!(p.user.contains("Lightning Bolt"));
        assert!(p.user.contains("Murder"));
        assert!(p.user.contains("Counterspell"));
        assert!(p.user.contains("Preordain"));
        assert!(p.user.contains("Servo Exhibition"));
        // The engine-derived allowlist must be present with the
        // high-frequency variants the diagnostic flagged.
        assert!(p.user.contains("ENGINE EFFECT CATALOG"));
        for v in ["Effect::CreateToken", "Effect::Pump",
                  "Effect::ExilePermanent", "Effect::Discard",
                  "Effect::ReturnFromGraveyardToBattlefield",
                  "Effect::ForEach", "Effect::Fight"] {
            assert!(p.user.contains(v), "catalog must list {v}");
        }
        // Tier-1 card-scripting prelude must be exposed for
        // resolution-time computed amounts / filtered board sets.
        assert!(p.user.contains("CARD SCRIPTING"));
        for h in ["script::count_matching", "script::ids_matching",
                  "script::power_of", "script::hand_size",
                  "script::subtype_filter", ".with_max_cmc(",
                  "use arcana_core::script;"] {
            assert!(p.user.contains(h), "scripting block must list {h}");
        }
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
    fn triggered_prompt_embeds_trigger_and_effect_catalogs() {
        let c = mk_card(|c| {
            c.name = "Phantom Helper".into();
            c.type_line = "Creature — Spirit".into();
            c.oracle_text = Some("When Phantom Helper dies, draw a card.".into());
            c.power = Some("2".into());
            c.toughness = Some("2".into());
        });
        let p = render_prompt(&c, Tier::Three).expect("ok");
        // Trigger-condition catalog — so the model picks an existing
        // variant instead of inventing one (the dominant T3 L1 failure).
        assert!(p.user.contains("TRIGGER CONDITION CATALOG"));
        for v in ["TriggerCondition::SelfDies", "TriggerCondition::StepBegins",
                  "TriggerCondition::ZoneChange", "TriggerCondition::DamageDealt"] {
            assert!(p.user.contains(v), "trigger catalog must list {v}");
        }
        // Shared effect catalog + script prelude, binding-resolved to `trig`.
        assert!(p.user.contains("ENGINE EFFECT CATALOG"));
        assert!(p.user.contains("CARD SCRIPTING"));
        assert!(p.user.contains("Effect::CreateToken"));
        assert!(p.user.contains("script::count_matching"));
        assert!(p.user.contains("trig.controller"), "binding resolved to trig");
        // The {BINDING} marker must be fully substituted — no leak.
        assert!(!p.user.contains("{BINDING}"), "BINDING marker not substituted");
    }

    // --- retry prompt ---------------------------------------------------

    fn mk_error(file: &str, line: u32, col: u32, code: Option<&str>, level: &str, msg: &str) -> CompileError {
        CompileError {
            file: file.to_string(),
            line,
            column: col,
            level: level.to_string(),
            code: code.map(String::from),
            message: msg.to_string(),
        }
    }

    #[test]
    fn retry_prompt_embeds_previous_code_and_errors() {
        let c = mk_card(|c| {
            c.name = "Shock".into();
            c.type_line = "Instant".into();
            c.oracle_text = Some("Shock deals 2 damage to any target.".into());
        });
        let previous_code = "pub fn register() { /* broken */ }";
        let errors = [mk_error(
            "src/generated/_scratch/candidate.rs",
            3, 17,
            Some("E0425"),
            "error",
            "cannot find function `missing_fn` in this scope",
        )];
        let prev = PreviousAttempt { code: previous_code, errors: &errors };
        let p = render_retry_prompt(&c, Tier::Two, &prev).expect("ok");

        // Keeps original few-shot context.
        assert!(p.user.contains("Lightning Bolt"), "retry keeps original few-shots");
        assert!(p.user.contains("Shock"), "retry keeps target card");

        // Appends the retry block.
        assert!(p.user.contains("PREVIOUS ATTEMPT"));
        assert!(p.user.contains(previous_code));
        assert!(p.user.contains("COMPILE ERRORS"));
        assert!(p.user.contains("E0425"));
        assert!(p.user.contains("cannot find function `missing_fn`"));
        assert!(p.user.contains("Output only the Rust source"));
    }

    #[test]
    fn retry_prompt_handles_empty_error_list() {
        // Some failures (particularly syntax errors) surface zero
        // structured CompileError rows — verify returns FailedInCandidate
        // with an empty error vec is unlikely but possible. The retry
        // prompt should degrade gracefully, not panic or produce an
        // empty error block.
        let c = mk_card(|c| {
            c.name = "Shock".into();
            c.type_line = "Instant".into();
            c.oracle_text = Some("Shock deals 2 damage to any target.".into());
        });
        let prev = PreviousAttempt {
            code: "this is not valid rust }",
            errors: &[],
        };
        let p = render_retry_prompt(&c, Tier::Two, &prev).expect("ok");
        assert!(p.user.contains("no structured errors captured"));
        assert!(p.user.contains("unbalanced braces"));
    }

    #[test]
    fn retry_prompt_respects_unsupported_shapes() {
        // A retry for an out-of-scope card (Tier::Four/Five) is still
        // Unsupported. Use a TRANSFORM-layout planeswalker: single-faced
        // PWs now route to the Planeswalker shape, but transform/MDFC PW
        // faces stay refused (and Battle/Saga/Class are layout-dispatched
        // before the tier guard, so they'd route regardless of tier).
        let c = mk_card(|c| {
            c.name = "Some Planeswalker".into();
            c.type_line = "Legendary Planeswalker — Test".into();
            c.layout = "transform".into();
            c.loyalty = Some("4".into());
            c.oracle_text = Some(
                "+1: Do a thing.\n-2: Do another thing.".into());
        });
        let prev = PreviousAttempt { code: "", errors: &[] };
        assert!(render_retry_prompt(&c, Tier::Four, &prev).is_err());
    }

    #[test]
    fn format_compile_error_is_single_line() {
        let err = mk_error(
            "src/generated/_scratch/candidate.rs",
            14, 5,
            Some("E0308"),
            "error",
            "mismatched types",
        );
        let s = format_compile_error(&err);
        assert!(!s.contains('\n'), "formatter must produce exactly one line");
        assert!(s.contains("E0308"));
        assert!(s.contains("mismatched types"));
        assert!(s.contains("14:5"));
    }

    #[test]
    fn format_compile_error_handles_missing_code() {
        // Syntax errors often have no rustc error code. The formatter
        // should substitute a `-` sentinel, not panic.
        let err = mk_error(
            "src/generated/_scratch/candidate.rs",
            10, 1,
            None,
            "error",
            "expected `}`, found `fn`",
        );
        let s = format_compile_error(&err);
        assert!(s.contains("[-]"), "missing code should render as `-`; got: {s}");
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

    // --- Wave-1 shapes ----------------------------------------------

    #[test]
    fn triggered_enchantment_routes_and_renders() {
        let c = mk_card(|c| {
            c.name = "Nightmare Visions".into();
            c.mana_cost = Some("{2}{B}".into());
            c.type_line = "Enchantment".into();
            c.oracle_text = Some(
                "At the beginning of your upkeep, each opponent loses 1 life.".into(),
            );
            c.colors = vec!["B".into()];
        });
        let p = render_prompt(&c, Tier::Three).expect("triggered enchantment renders");
        assert_eq!(p.shape, PromptShape::TriggeredEnchantment);
        assert!(p.user.contains("Underworld Dreams"), "seed few-shot embedded");
        assert!(p.user.contains("Glorious Anthem"), "anthem reference embedded");
        assert!(p.user.contains("TRIGGER CONDITION CATALOG"));
        assert!(p.user.contains("ENGINE EFFECT CATALOG"));
        assert!(p.user.contains("Nightmare Visions"), "target card in spec");
        assert!(p.user.contains("each opponent loses 1 life"), "oracle in spec");
        assert!(!p.user.contains("{BINDING}"), "BINDING marker substituted");
    }

    #[test]
    fn activated_artifact_routes_and_renders() {
        let c = mk_card(|c| {
            c.name = "Hedron Battery".into();
            c.mana_cost = Some("{2}".into());
            c.type_line = "Artifact".into();
            c.oracle_text = Some(
                "{T}: Add {C}.\n{3}, {T}, Sacrifice Hedron Battery: Draw two cards.".into(),
            );
        });
        // The two-line mana-rock carve-out classifies this T3.
        assert_eq!(
            crate::classifier::classify(&c).tier,
            Tier::Three,
            "mana rock classifies T3 via the Wave-1 carve-out"
        );
        let p = render_prompt(&c, Tier::Three).expect("mana rock renders");
        assert_eq!(p.shape, PromptShape::ActivatedArtifact);
        assert!(p.user.contains("Mind Stone"));
        assert!(p.user.contains("ACTIVATION COST CATALOG"));
        assert!(p.user.contains("Hedron Battery"));
        assert!(!p.user.contains("{BINDING}"));
    }

    #[test]
    fn utility_land_routes_and_renders() {
        let c = mk_card(|c| {
            c.name = "Shadowy Backstreet".into();
            c.mana_cost = None;
            c.type_line = "Land — Gate".into();
            c.oracle_text = Some(
                "Shadowy Backstreet enters tapped.\n{T}: Add {U} or {B}.".into(),
            );
        });
        assert_eq!(
            crate::classifier::classify(&c).tier,
            Tier::Three,
            "tapland classifies T3 via the Wave-1 carve-out"
        );
        let p = render_prompt(&c, Tier::Three).expect("land renders");
        assert_eq!(p.shape, PromptShape::UtilityLand);
        assert!(p.user.contains("Rogue's Passage"), "utility-land seed");
        assert!(p.user.contains("Dimir Guildgate"), "tapland seed");
        assert!(p.user.contains("EntersWithSpec::Tapped"));
        assert!(p.user.contains("mana_cost: None"), "land bones taught");
        assert!(p.user.contains("Shadowy Backstreet"));
        assert!(!p.user.contains("{BINDING}"));
    }

    #[test]
    fn equipment_routes_and_renders() {
        let c = mk_card(|c| {
            c.name = "Spiked Cleaver".into();
            c.mana_cost = Some("{1}".into());
            c.type_line = "Artifact — Equipment".into();
            c.oracle_text =
                Some("Equipped creature gets +2/+0.\nEquip {1}".into());
            c.keywords = vec!["Equip".into()];
        });
        assert_eq!(
            crate::classifier::classify(&c).tier,
            Tier::Three,
            "Equipment classifies T3 via the Wave-1 carve-out"
        );
        let p = render_prompt(&c, Tier::Three).expect("equipment renders");
        assert_eq!(p.shape, PromptShape::Equipment);
        assert!(p.user.contains("Bonesplitter"), "Bonesplitter few-shot");
        assert!(p.user.contains("with_equip"));
        assert!(p.user.contains("attached_pt"));
        assert!(p.user.contains("Spiked Cleaver"));
    }

    #[test]
    fn static_enchantment_routes_and_renders() {
        let c = mk_card(|c| {
            c.name = "Banner of Valor".into();
            c.mana_cost = Some("{1}{W}".into());
            c.type_line = "Enchantment".into();
            c.oracle_text = Some("Creatures you control get +1/+1.".into());
            c.colors = vec!["W".into()];
        });
        assert_eq!(
            crate::classifier::classify(&c).tier,
            Tier::Two,
            "single-line static enchantment classifies T2"
        );
        let p = render_prompt(&c, Tier::Two).expect("static enchantment renders");
        assert_eq!(p.shape, PromptShape::StaticEnchantment);
        assert!(p.user.contains("Glorious Anthem"));
        assert!(p.user.contains("InstallContinuousEffect"));
        assert!(p.user.contains("keyword_anthem"));
        assert!(p.user.contains("Banner of Valor"));
    }

    #[test]
    fn planeswalker_routes_to_pw_shape() {
        let c = mk_card(|c| {
            c.name = "Jaya, Fiery Negotiator".into();
            c.mana_cost = Some("{2}{R}".into());
            c.type_line = "Legendary Planeswalker — Jaya".into();
            c.loyalty = Some("4".into());
            c.colors = vec!["R".into()];
            c.oracle_text = Some(
                "+1: Add {R}{R}{R}.\n+1: Jaya deals 2 damage to target creature.\n\
                 -8: You get an emblem.".into());
        });
        let p = render_prompt(&c, Tier::Three).expect("PW yields a prompt");
        assert_eq!(p.shape, PromptShape::Planeswalker);
        assert!(p.user.contains("is_loyalty_ability")
            && p.user.contains("add_self_counter"),
            "PW pack teaches the loyalty-ability idiom");
    }

    #[test]
    fn aura_routes_to_aura_shape() {
        // Wave-2: resolution-time attach landed, so Auras route to the
        // dedicated Aura pack instead of being refused.
        let c = mk_card(|c| {
            c.name = "Arcane Binding".into();
            c.mana_cost = Some("{1}{W}".into());
            c.type_line = "Enchantment — Aura".into();
            c.oracle_text =
                Some("Enchant creature. Enchanted creature gets +2/+2.".into());
        });
        let p = render_prompt(&c, Tier::Two).expect("Aura yields a prompt");
        assert_eq!(p.shape, PromptShape::Aura);
        assert!(p.user.contains("with_enchant"),
            "Aura pack teaches the with_enchant idiom");
    }

    #[test]
    fn activated_enchantment_is_refused() {
        // Shrines / activated enchantments have no Wave-1 pack.
        let c = mk_card(|c| {
            c.name = "Strange Shrine".into();
            c.type_line = "Enchantment — Shrine".into();
            c.oracle_text =
                Some("{2}, Sacrifice Strange Shrine: Draw a card.".into());
        });
        match render_prompt(&c, Tier::Three).unwrap_err() {
            Unsupported::NoFewShotForShape { detail, .. } => {
                assert!(detail.contains("activated"), "got detail {detail}");
            }
            other => panic!("expected NoFewShotForShape, got {other:?}"),
        }
    }

    #[test]
    fn basic_land_still_refused_not_utility_land() {
        // The UtilityLand route must not swallow basic lands — they
        // stay hand-written helpers.
        let c = mk_card(|c| {
            c.name = "Island".into();
            c.type_line = "Basic Land — Island".into();
            c.oracle_text = Some("({T}: Add {U}.)".into());
            c.mana_cost = None;
        });
        assert_eq!(
            render_prompt(&c, Tier::One).unwrap_err(),
            Unsupported::BasicLand
        );
    }

    #[test]
    fn saga_layout_still_beats_wave1_enchantment_routing() {
        // Saga/Class dispatch on layout BEFORE the Wave-1 type
        // routing; an enchantment — Saga must not become a
        // TriggeredEnchantment.
        let c = mk_card(|c| {
            c.name = "Epic Tale".into();
            c.type_line = "Enchantment — Saga".into();
            c.layout = "saga".into();
            c.oracle_text = Some(
                "I — Draw a card.\nII — Draw a card.\nIII — Discard your hand.".into(),
            );
        });
        let p = render_prompt(&c, Tier::Three).expect("saga renders");
        assert_eq!(p.shape, PromptShape::Saga);
    }

    /// Shape census over the live (cached) Scryfall oracle pool —
    /// prints how many cards route to each PromptShape and each
    /// Unsupported bucket. Ignored by default (needs the pool cache);
    /// run with `cargo test -p arcana-gen --lib -- --ignored
    /// --nocapture shape_census`.
    #[test]
    #[ignore]
    fn shape_census_live_pool() {
        use crate::scryfall::ScryfallPool;
        use std::collections::BTreeMap;
        let pool = ScryfallPool::load_default().expect("cached pool");
        let mut shapes: BTreeMap<String, usize> = BTreeMap::new();
        let mut refused: BTreeMap<String, usize> = BTreeMap::new();
        for card in pool.iter() {
            let tier = crate::classifier::classify(card).tier;
            match select_shape(card, tier) {
                Ok(s) => *shapes.entry(format!("{s:?}")).or_default() += 1,
                Err(e) => *refused.entry(e.to_string()).or_default() += 1,
            }
        }
        eprintln!("== shapes ==");
        for (s, n) in &shapes {
            eprintln!("  {s}: {n}");
        }
        eprintln!("== refused ==");
        for (s, n) in &refused {
            eprintln!("  {s}: {n}");
        }
    }

    /// Dump one rendered prompt per Wave-1 shape for human
    /// inspection. Ignored by default (writes outside the target
    /// dir); run explicitly with:
    /// `cargo test -p arcana-gen --lib -- --ignored dump_wave1`
    #[test]
    #[ignore]
    fn dump_wave1_sample_prompts() {
        let dir = std::path::Path::new("/tmp/wave1_prompts");
        std::fs::create_dir_all(dir).expect("mkdir");
        let cases: Vec<(&str, Card, Tier)> = vec![
            (
                "triggered_enchantment",
                mk_card(|c| {
                    c.name = "Underworld Dreams".into();
                    c.mana_cost = Some("{B}{B}{B}".into());
                    c.type_line = "Enchantment".into();
                    c.oracle_text = Some(
                        "Whenever an opponent draws a card, Underworld Dreams deals 1 damage to them.".into());
                    c.colors = vec!["B".into()];
                }),
                Tier::Three,
            ),
            (
                "activated_artifact",
                mk_card(|c| {
                    c.name = "Mind Stone".into();
                    c.mana_cost = Some("{1}".into());
                    c.type_line = "Artifact".into();
                    c.oracle_text = Some(
                        "{T}: Add {C}.\n{1}, {T}, Sacrifice Mind Stone: Draw a card.".into());
                }),
                Tier::Three,
            ),
            (
                "utility_land",
                mk_card(|c| {
                    c.name = "Rogue's Passage".into();
                    c.mana_cost = None;
                    c.type_line = "Land".into();
                    c.oracle_text = Some(
                        "{T}: Add {C}.\n{4}, {T}: Target creature can't be blocked this turn.".into());
                }),
                Tier::Three,
            ),
            (
                "equipment",
                mk_card(|c| {
                    c.name = "Bonesplitter".into();
                    c.mana_cost = Some("{1}".into());
                    c.type_line = "Artifact — Equipment".into();
                    c.oracle_text =
                        Some("Equipped creature gets +2/+0.\nEquip {1}".into());
                    c.keywords = vec!["Equip".into()];
                }),
                Tier::Three,
            ),
            (
                "static_enchantment",
                mk_card(|c| {
                    c.name = "Glorious Anthem".into();
                    c.mana_cost = Some("{1}{W}".into());
                    c.type_line = "Enchantment".into();
                    c.oracle_text =
                        Some("Creatures you control get +1/+1.".into());
                    c.colors = vec!["W".into()];
                }),
                Tier::Two,
            ),
        ];
        for (slug, card, tier) in cases {
            let p = render_prompt(&card, tier).expect("renders");
            let body = format!(
                "=== SHAPE: {:?} ===\n\n=== SYSTEM ===\n{}\n\n=== USER ===\n{}\n",
                p.shape, p.system, p.user,
            );
            std::fs::write(dir.join(format!("{slug}.txt")), body).expect("write");
        }
    }

}
