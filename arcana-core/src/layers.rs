//! Continuous effects and the CR 613 layer system.
//!
//! Addendum Section 11 / Phase 1 Task #17. Depends on tasks 4
//! (objects), 6 (state), 13 (effects).
//!
//! # Model (CR 613)
//!
//! Continuous effects — pump spells, anthems, control-changers, static
//! abilities — apply in a canonical order. For each object, the
//! engine computes its current characteristics by walking seven
//! layers in sequence:
//!
//! ```text
//!   1. Copy effects
//!   2. Control-changing effects
//!   3. Text-changing effects
//!   4. Type-changing effects
//!   5. Color-changing effects
//!   6. Ability-adding / -removing effects
//!   7. Power/toughness:
//!      7a. Characteristic-defining abilities
//!      7b. Set-to-specific-value effects
//!      7c. Modify-by-delta effects
//!      7d. +1/+1 and -1/-1 counters
//!      7e. Switch-power-and-toughness effects
//! ```
//!
//! Within each layer, effects are sorted by **timestamp** (the order
//! they were created). A full implementation also solves a
//! **dependency** graph per CR 613.8 — effects that change whether
//! *another* effect applies at all. Phase 1 sorts by (layer,
//! timestamp) only and stubs dependency handling.
//!
//! # Scope
//!
//! - [`GameState::compute_characteristics`] walks the layer pipeline
//!   and returns the computed characteristics of a single object.
//!   This is **the** authoritative answer to "what is this object
//!   right now?" — the stubs here replaced in Phase 1 Task #17 are
//!   now wired through to it.
//! - [`GameState::add_continuous_effect`] assigns a monotonic
//!   timestamp and pushes the effect.
//! - [`GameState::expire_end_of_turn_effects`] /
//!   [`GameState::expire_effects_from_source`] are the cleanup hooks
//!   that the engine invokes at step/zone transitions.
//! - Layer 7d (counter math) is applied inline by reading the
//!   object's own `CounterMap` — no `ContinuousEffect` entry is
//!   needed.
//!
//! # Fn-pointer policy
//!
//! [`ContinuousEffectKind`] is a sum of the common cases
//! (pump/set-PT/anthem/grant-keyword) plus a `Custom` variant whose
//! fn pointer is the escape hatch. Concrete variants don't need
//! fn pointers; they match cleanly. Serde can roundtrip everything
//! except `Custom` (same `ConditionFnId` migration as elsewhere).

use serde::{Deserialize, Serialize};

use crate::effects::KeywordAbility;
use crate::objects::{Characteristics, ObjectId};
use crate::state::GameState;
use crate::types::*;

// =============================================================================
// Layer + Duration
// =============================================================================

/// The 7 layers (with sublayers for 7a-7e). Ordering is the
/// application order (CR 613.1): variants earlier in the enum apply
/// first.
#[derive(Clone, Copy, Debug, PartialOrd, Ord, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Layer {
    L1Copy,
    L2Control,
    L3Text,
    L4Type,
    L5Color,
    L6Ability,
    L7aPTCharacteristicDefining,
    L7bPTSetting,
    L7cPTModifying,
    L7dPTCounters,
    L7ePTSwitching,
}

impl Layer {
    /// All layers in application order.
    pub fn all_in_order() -> [Layer; 11] {
        [
            Layer::L1Copy,
            Layer::L2Control,
            Layer::L3Text,
            Layer::L4Type,
            Layer::L5Color,
            Layer::L6Ability,
            Layer::L7aPTCharacteristicDefining,
            Layer::L7bPTSetting,
            Layer::L7cPTModifying,
            Layer::L7dPTCounters,
            Layer::L7ePTSwitching,
        ]
    }
}

/// How long a continuous effect lasts.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Duration {
    EndOfTurn,
    UntilYourNextTurn(crate::types::PlayerId),
    /// "Until your next upkeep" (Halfdane's copy, Erhnam Djinn's
    /// forestwalk grant): expires when that player's upkeep begins.
    UntilNextUpkeepOf(crate::types::PlayerId),
    /// FACE-GATED static (transforming DFC back-face statics —
    /// Belenon War Anthem): live only while the source shows `face`
    /// AND is on the battlefield. A LIVE check, not removal — the
    /// effect dims when the permanent transforms away and lights
    /// back up when it returns; removed for real when the source
    /// leaves the battlefield.
    WhileSourceShowsFace(u8),
    WhileSourceOnBattlefield,
    /// "During your turn" — a static that is LIVE only while the
    /// source is on the battlefield AND its controller is the active
    /// player (street_riot: "During your turn, creatures you control
    /// get +1/+0"; raph's bravado). A LIVE check like
    /// [`Self::WhileSourceShowsFace`]: dims off-turn and relights
    /// on-turn; removed for real when the source leaves the
    /// battlefield (via [`GameState::expire_effects_from_source`]).
    WhileControllerTurn,
    WhileCondition(crate::types::ConditionId),
    WhileExiled(ObjectId),
    Permanent,
    /// Apply once and discard. Should never appear in `continuous_effects`
    /// at rest.
    Instant,
}

// =============================================================================
// ContinuousEffect
// =============================================================================

// TODO(serialize): `ContinuousEffectKind::Custom` carries a bare fn
// pointer. Migrate per addendum Section 12 in Phase 3.
/// A continuous effect in mid-flight. Applied to every object by the
/// [`GameState::compute_characteristics`] pipeline, filtered by
/// [`ContinuousEffectKind::applies_to`].
#[derive(Clone, Debug)]
pub struct ContinuousEffect {
    pub source: ObjectId,
    pub layer: Layer,
    /// Monotonic timestamp from [`GameState::next_timestamp`]. Within
    /// a layer, lower timestamps apply first.
    pub timestamp: u64,
    pub duration: Duration,
    pub dependency: Option<DependencyInfo>,
    pub kind: ContinuousEffectKind,
}

impl ContinuousEffect {
    /// Build a standard pump effect ("+P/+T to target"), layer 7c.
    pub fn pump(source: ObjectId, target: ObjectId, power: i32, toughness: i32,
                duration: Duration) -> Self {
        Self {
            source,
            layer: Layer::L7cPTModifying,
            timestamp: 0, // overwritten by `add_continuous_effect`
            duration,
            dependency: None,
            kind: ContinuousEffectKind::PumpTarget { target, power, toughness },
        }
    }

    /// Build a "creatures you control get +P/+T" anthem, layer 7c.
    pub fn anthem(source: ObjectId, controller: PlayerId, power: i32, toughness: i32,
                  duration: Duration) -> Self {
        Self {
            source,
            layer: Layer::L7cPTModifying,
            timestamp: 0,
            duration,
            dependency: None,
            kind: ContinuousEffectKind::AnthemForController {
                controller, power, toughness,
            },
        }
    }

    /// Build a "target becomes P/T" effect, layer 7b.
    pub fn set_pt(source: ObjectId, target: ObjectId, power: i32, toughness: i32,
                  duration: Duration) -> Self {
        Self {
            source,
            layer: Layer::L7bPTSetting,
            timestamp: 0,
            duration,
            dependency: None,
            kind: ContinuousEffectKind::SetPt { target, power, toughness },
        }
    }

    /// Build a "target gets [keyword]" grant effect, layer 6.
    pub fn grant_keyword(source: ObjectId, target: ObjectId,
                         keyword: KeywordAbility,
                         duration: Duration) -> Self {
        Self {
            source,
            layer: Layer::L6Ability,
            timestamp: 0,
            duration,
            dependency: None,
            kind: ContinuousEffectKind::GrantKeywordTarget { target, keyword },
        }
    }

    /// Build a "creatures you control have [keyword]" controller-wide
    /// keyword anthem, layer 6 — the keyword analogue of
    /// [`Self::anthem`]. Used by Class level statics ("Creatures you
    /// control have menace") and lord effects; pair with
    /// [`Duration::WhileSourceOnBattlefield`] so it auto-expires when
    /// the source leaves play (and, for Classes, only ever installed
    /// once the level-up that grants it resolves).
    pub fn keyword_anthem(source: ObjectId, controller: PlayerId,
                          keyword: KeywordAbility,
                          duration: Duration) -> Self {
        Self {
            source,
            layer: Layer::L6Ability,
            timestamp: 0,
            duration,
            dependency: None,
            kind: ContinuousEffectKind::GrantKeywordToController { controller, keyword },
        }
    }

    /// Build a "target is goaded by `goader`" effect (CR 701.38). Lives
    /// at Layer 6 alongside ability-granting effects.
    pub fn goad(source: ObjectId, target: ObjectId, goader: PlayerId,
                duration: Duration) -> Self {
        Self {
            source,
            layer: Layer::L6Ability,
            timestamp: 0,
            duration,
            dependency: None,
            kind: ContinuousEffectKind::Goaded { target, goader },
        }
    }

    /// Build an Equipment-style "equipped creature gets +P/+T" effect,
    /// layer 7c. The buff follows `source`'s attachment — whichever
    /// creature `source` is currently attached to receives the pump.
    /// Typically paired with [`Duration::WhileSourceOnBattlefield`]
    /// so the effect auto-expires when the Equipment leaves play.
    pub fn attached_pt(source: ObjectId, power: i32, toughness: i32,
                       duration: Duration) -> Self {
        Self {
            source,
            layer: Layer::L7cPTModifying,
            timestamp: 0,
            duration,
            dependency: None,
            kind: ContinuousEffectKind::AttachedCreatureGetsPt { power, toughness },
        }
    }

    /// Build an Equipment/Aura "equipped creature gets +X/+Y for each
    /// …" effect, layer 7c — the dynamic sibling of
    /// [`Self::attached_pt`]. `compute` receives the game state and
    /// the SOURCE (the Equipment/Aura, so its controller and
    /// attachment are readable) and returns the (power, toughness)
    /// delta, re-evaluated every layer application. Pair with
    /// [`Duration::WhileSourceOnBattlefield`].
    pub fn attached_pt_dynamic(source: ObjectId,
                               compute: fn(&GameState, ObjectId) -> (i32, i32),
                               duration: Duration) -> Self {
        Self {
            source,
            layer: Layer::L7cPTModifying,
            timestamp: 0,
            duration,
            dependency: None,
            kind: ContinuousEffectKind::AttachedCreatureGetsPtDynamic { compute },
        }
    }

    /// Build a "+P/+T for each [filter] permanent" attached pump
    /// (Blanchwood Armor). `filter` is matched from the host's
    /// controller's perspective at apply time.
    pub fn attached_pt_per_match(source: ObjectId,
                                 filter: crate::targets::ObjectFilter,
                                 per_power: i32, per_toughness: i32,
                                 duration: Duration) -> Self {
        Self {
            source,
            layer: Layer::L7cPTModifying,
            timestamp: 0,
            duration,
            dependency: None,
            kind: ContinuousEffectKind::AttachedCreatureGetsPtPerMatch {
                filter, per_power, per_toughness },
        }
    }

    /// Build an "equipped/enchanted creature has [keyword]" effect —
    /// the keyword sibling of [`Self::attached_pt`]: whatever creature
    /// `source` is currently attached to gains the keyword (Layer 6).
    /// Inert while unattached; pair with
    /// [`Duration::WhileSourceOnBattlefield`].
    pub fn attached_keyword(source: ObjectId, keyword: KeywordAbility,
                            duration: Duration) -> Self {
        Self {
            source,
            layer: Layer::L6Ability,
            timestamp: 0,
            duration,
            dependency: None,
            kind: ContinuousEffectKind::AttachedCreatureGainsKeyword { keyword },
        }
    }

    /// Build a "target creature loses [keyword]" effect (Layer 6
    /// removal). Within-layer timestamps settle removal-vs-grant
    /// races (CR 613.7). Typically `Duration::EndOfTurn`.
    pub fn remove_keyword(source: ObjectId, target: ObjectId,
                          keyword: KeywordAbility,
                          duration: Duration) -> Self {
        Self {
            source,
            layer: Layer::L6Ability,
            timestamp: 0,
            duration,
            dependency: None,
            kind: ContinuousEffectKind::RemoveKeywordTarget { target, keyword },
        }
    }

    /// Build an "equipped creature loses [keyword]" effect — the
    /// removal sibling of [`Self::attached_keyword`]; follows the
    /// attachment, inert while unattached.
    pub fn attached_loses_keyword(source: ObjectId, keyword: KeywordAbility,
                                  duration: Duration) -> Self {
        Self {
            source,
            layer: Layer::L6Ability,
            timestamp: 0,
            duration,
            dependency: None,
            kind: ContinuousEffectKind::AttachedCreatureLosesKeyword { keyword },
        }
    }

    /// Build an "equipped/enchanted creature is a [subtype] in
    /// addition to its other types" effect (Layer 4, additive) —
    /// follows the attachment like [`Self::attached_pt`]. Pair with
    /// [`Duration::WhileSourceOnBattlefield`]. Subtype-reading
    /// `ObjectFilter` predicates are layer-aware, so tribal counts
    /// and lords see the granted subtype.
    pub fn attached_subtypes(source: ObjectId,
                             subtypes: crate::types::SubtypeSet,
                             duration: Duration) -> Self {
        Self {
            source,
            layer: Layer::L4Type,
            timestamp: 0,
            duration,
            dependency: None,
            kind: ContinuousEffectKind::AttachedCreatureAddSubtypes { subtypes },
        }
    }

    /// Build a "TARGET creature becomes a [subtype] in addition to
    /// its other types" effect (Layer 4, additive) — the targeted
    /// sibling of [`Self::attached_subtypes`].
    pub fn add_subtypes(source: ObjectId, target: ObjectId,
                        subtypes: crate::types::SubtypeSet,
                        duration: Duration) -> Self {
        Self {
            source,
            layer: Layer::L4Type,
            timestamp: 0,
            duration,
            dependency: None,
            kind: ContinuousEffectKind::AddSubtypesTarget { target, subtypes },
        }
    }

    /// Build a "target becomes [supertype] in addition" effect
    /// (Layer 4, additive) — "becomes legendary".
    pub fn add_supertypes(source: ObjectId, target: ObjectId,
                          supertypes: crate::types::SupertypeSet,
                          duration: Duration) -> Self {
        Self {
            source,
            layer: Layer::L4Type,
            timestamp: 0,
            duration,
            dependency: None,
            kind: ContinuousEffectKind::AddSupertypesTarget { target, supertypes },
        }
    }

    /// Build an "equipped creature is an artifact in addition to its
    /// other types" effect (Layer 4) — attached CARD-TYPE add
    /// (Silverskin Armor).
    pub fn attached_types(source: ObjectId, types: crate::types::TypeLine,
                          duration: Duration) -> Self {
        Self {
            source,
            layer: Layer::L4Type,
            timestamp: 0,
            duration,
            dependency: None,
            kind: ContinuousEffectKind::AttachedCreatureAddTypes { types },
        }
    }

    /// Build an "equipped creature is every creature type" effect
    /// (Layer 4 changeling templating — Runed Stalactite).
    pub fn attached_every_creature_type(source: ObjectId,
                                        duration: Duration) -> Self {
        Self {
            source,
            layer: Layer::L4Type,
            timestamp: 0,
            duration,
            dependency: None,
            kind: ContinuousEffectKind::AttachedCreatureEveryCreatureType,
        }
    }

    /// Build an "… and is [color] in addition to its other colors"
    /// effect (Layer 5, ADDITIVE — unlike `SetColor`). Follows the
    /// attachment; pair with [`Duration::WhileSourceOnBattlefield`].
    pub fn attached_colors(source: ObjectId,
                           colors: crate::types::ColorSet,
                           duration: Duration) -> Self {
        Self {
            source,
            layer: Layer::L5Color,
            timestamp: 0,
            duration,
            dependency: None,
            kind: ContinuousEffectKind::AttachedCreatureAddColors { colors },
        }
    }

    /// Build an "enchanted/equipped creature can't attack" marker
    /// (Pacifism's attack half). Follows `source.attached_to`; pair
    /// with [`Duration::WhileSourceOnBattlefield`]. Layer is irrelevant
    /// (markers don't enter the characteristic pipeline) but recorded
    /// as L6 for consistency with the other attached-grant effects.
    pub fn attached_cant_attack(source: ObjectId, duration: Duration) -> Self {
        Self {
            source,
            layer: Layer::L6Ability,
            timestamp: 0,
            duration,
            dependency: None,
            kind: ContinuousEffectKind::AttachedCreatureCantAttack,
        }
    }

    /// Build an "enchanted/equipped creature can't block" marker
    /// (Pacifism's block half). Sibling of [`Self::attached_cant_attack`].
    pub fn attached_cant_block(source: ObjectId, duration: Duration) -> Self {
        Self {
            source,
            layer: Layer::L6Ability,
            timestamp: 0,
            duration,
            dependency: None,
            kind: ContinuousEffectKind::AttachedCreatureCantBlock,
        }
    }

    /// Build an "enchanted/equipped creature has '\[cost\]: \[effect\]'"
    /// grant. The `ability`'s cost and effect run against the HOST when
    /// the host's controller activates it. Pair with
    /// [`Duration::WhileSourceOnBattlefield`].
    pub fn attached_activated(source: ObjectId,
                              ability: crate::registry::ActivatedAbilityDef,
                              duration: Duration) -> Self {
        Self {
            source,
            layer: Layer::L6Ability,
            timestamp: 0,
            duration,
            dependency: None,
            kind: ContinuousEffectKind::AttachedCreatureGrantsActivated { ability },
        }
    }

    /// Build an "enchanted/equipped creature doesn't untap" marker
    /// (Glimmerdust Nap). Follows `source.attached_to`.
    pub fn attached_dont_untap(source: ObjectId, duration: Duration) -> Self {
        Self {
            source,
            layer: Layer::L6Ability,
            timestamp: 0,
            duration,
            dependency: None,
            kind: ContinuousEffectKind::AttachedCreatureDontUntap,
        }
    }

    /// Build an "enchanted/equipped creature can't be blocked" marker
    /// (Aqueous Form). Follows `source.attached_to`.
    pub fn attached_cant_be_blocked(source: ObjectId, duration: Duration) -> Self {
        Self {
            source,
            layer: Layer::L6Ability,
            timestamp: 0,
            duration,
            dependency: None,
            kind: ContinuousEffectKind::AttachedCreatureCantBeBlocked,
        }
    }

    /// Build an "enchanted creature has base P/T X/Y" / "becomes an
    /// X/Y creature" effect (Layer 7b, SETS base P/T). Follows
    /// `source.attached_to`. Pair with [`Duration::WhileSourceOnBattlefield`].
    pub fn attached_set_pt(source: ObjectId, power: i32, toughness: i32,
                           duration: Duration) -> Self {
        Self {
            source,
            layer: Layer::L7bPTSetting,
            timestamp: 0,
            duration,
            dependency: None,
            kind: ContinuousEffectKind::AttachedCreatureSetsPt { power, toughness },
        }
    }

    /// Build a GLOBAL filtered pump ("[filter] creatures get +P/+T"),
    /// layer 7c. Filter is matched against BASE characteristics from
    /// the source controller's perspective.
    pub fn filtered_pump(source: ObjectId,
                         filter: crate::targets::ObjectFilter,
                         power: i32, toughness: i32,
                         duration: Duration) -> Self {
        Self {
            source,
            layer: Layer::L7cPTModifying,
            timestamp: 0,
            duration,
            dependency: None,
            kind: ContinuousEffectKind::FilteredPump { filter, power, toughness },
        }
    }

    /// Build a GLOBAL DYNAMIC filtered pump ("[filter] creatures get
    /// +X/+X, where X is …"), layer 7c — the global sibling of
    /// [`Self::attached_pt_dynamic`] and the dynamic sibling of
    /// [`Self::filtered_pump`]. `filter` selects WHICH creatures get the
    /// buff (matched against BASE characteristics from the source
    /// controller's perspective); `compute` receives the game state and
    /// the SOURCE and returns the (power, toughness) delta applied to
    /// each, re-evaluated every layer application. RECURSION RULE:
    /// `compute` must read game-state SCALARS (cards drawn this turn,
    /// hand size, a cast count) or count via BASE characteristics — never
    /// a layer-computed P/T of a creature this effect buffs.
    /// (knowledge_is_power: +X/+X where X = cards drawn; meishin: -X/-0
    /// where X = your hand size; commander's insignia: +1/+1 per cast.)
    pub fn filtered_pump_dynamic(source: ObjectId,
                                 filter: crate::targets::ObjectFilter,
                                 compute: fn(&GameState, ObjectId) -> (i32, i32),
                                 duration: Duration) -> Self {
        Self {
            source,
            layer: Layer::L7cPTModifying,
            timestamp: 0,
            duration,
            dependency: None,
            kind: ContinuousEffectKind::FilteredPumpDynamic { filter, compute },
        }
    }

    /// Build a GLOBAL PER-MATCH filtered pump ("[filter] creatures get
    /// +`per_power`/+`per_toughness` for each [count_filter] permanent"),
    /// layer 7c — the global sibling of [`Self::attached_pt_per_match`].
    /// `filter` selects which creatures get buffed; `count_filter` is the
    /// thing counted (e.g. Gates you control). BOTH are matched against
    /// BASE characteristics from the source controller's perspective —
    /// the count is recursion-proof by construction (it never re-enters
    /// the layer system, unlike `script::count_matching`). (hold_the_gates:
    /// +0/+1 for each Gate you control.)
    pub fn filtered_pump_per_match(source: ObjectId,
                                   filter: crate::targets::ObjectFilter,
                                   count_filter: crate::targets::ObjectFilter,
                                   per_power: i32, per_toughness: i32,
                                   duration: Duration) -> Self {
        Self {
            source,
            layer: Layer::L7cPTModifying,
            timestamp: 0,
            duration,
            dependency: None,
            kind: ContinuousEffectKind::FilteredPumpPerMatch {
                filter, count_filter, per_power, per_toughness },
        }
    }

    /// Build a SELF characteristic-defining P/T ability (CR 604.3 / Layer
    /// 7a): "[this creature]'s power and toughness are each equal to …" /
    /// "*/* where * is …" (Tarmogoyf, Nightmare, Lhurgoyf, Mortivore,
    /// Veteran Warleader). `compute(state, source)` returns the (power,
    /// toughness) the `*` resolves to, re-evaluated every layer pass. The
    /// effect SETS the source's base P/T at 7a (resolving the `Star`), so
    /// later +N/+N pumps (7c) and counters (7d) add on top, and a 7b
    /// "becomes 1/1" (Humility) still overrides it. Install via a
    /// `SelfEntersBattlefield` trigger with `Duration::WhileSourceOnBattlefield`
    /// (the value is only needed while on the battlefield). RECURSION RULE:
    /// `compute` must read BASE characteristics / state scalars / zone
    /// counts — NEVER a layer-computed P/T (it would re-enter Layer 7).
    pub fn self_pt_cda(source: ObjectId,
                       compute: fn(&GameState, ObjectId) -> (i32, i32),
                       duration: Duration) -> Self {
        Self {
            source,
            layer: Layer::L7aPTCharacteristicDefining,
            timestamp: 0,
            duration,
            dependency: None,
            kind: ContinuousEffectKind::SetBasePtFromCda { compute },
        }
    }

    /// Build a SELF CDA whose `*` is the COUNT of a `count_filter` (Layer
    /// 7a) — "[this]'s power and toughness are each equal to the number of
    /// [filter]" (Lumra: Forests+Plains you control; Kodama: Forests;
    /// Multani: lands you control + in your graveyard via a fn variant).
    /// The filter is built in `register()` (so it can carry interned
    /// SUBTYPE symbols the no-registry `compute` fn can't resolve) and
    /// counted at apply via `matches_base` — recursion-proof, no registry
    /// needed. SETS base P/T at 7a (pumps/counters stack on top). Pair with
    /// `Duration::WhileSourceOnBattlefield`.
    pub fn self_pt_from_match(source: ObjectId,
                              count_filter: crate::targets::ObjectFilter,
                              duration: Duration) -> Self {
        Self {
            source,
            layer: Layer::L7aPTCharacteristicDefining,
            timestamp: 0,
            duration,
            dependency: None,
            kind: ContinuousEffectKind::SetBasePtFromMatch { count_filter },
        }
    }

    /// Build an ASYMMETRIC self CDA (Layer 7a) where ONE axis is the count
    /// of `count_filter` and the OTHER is a fixed value — the missing case
    /// for "0/*" / "*/N" CDAs whose `*` is a SUBTYPE count (Traproot Kami
    /// 0/* = Forests; Uchuulon */4 = Crabs/Oozes/Horrors; Wintermoor 2/* =
    /// Knights; Namor */4 = Merfolk). `self_pt_from_match` is symmetric (no
    /// good for `*/N`) and `self_pt_cda`'s compute has no registry to name
    /// subtypes — this carries the interner-built filter AND the asymmetry.
    /// `count_is_power`: the count fills power (toughness = `other_fixed`)
    /// when true, else toughness (power = `other_fixed`). Counted via
    /// `matches_base` — recursion-proof.
    pub fn self_pt_from_match_asym(source: ObjectId,
                                   count_filter: crate::targets::ObjectFilter,
                                   count_is_power: bool,
                                   other_fixed: i32,
                                   duration: Duration) -> Self {
        Self {
            source,
            layer: Layer::L7aPTCharacteristicDefining,
            timestamp: 0,
            duration,
            dependency: None,
            kind: ContinuousEffectKind::SetBasePtFromMatchAsym {
                count_filter, count_is_power, other_fixed },
        }
    }

    /// Build a GLOBAL filtered keyword grant ("all [filter] have
    /// [keyword]"), layer 6.
    pub fn filtered_keyword(source: ObjectId,
                            filter: crate::targets::ObjectFilter,
                            keyword: KeywordAbility,
                            duration: Duration) -> Self {
        Self {
            source,
            layer: Layer::L6Ability,
            timestamp: 0,
            duration,
            dependency: None,
            kind: ContinuousEffectKind::FilteredGrantKeyword { filter, keyword },
        }
    }

    /// Build a "[filter] creatures can't attack" restriction.
    pub fn filtered_cant_attack(source: ObjectId,
                                filter: crate::targets::ObjectFilter,
                                duration: Duration) -> Self {
        Self {
            source,
            layer: Layer::L6Ability,
            timestamp: 0,
            duration,
            dependency: None,
            kind: ContinuousEffectKind::FilteredCantAttack { filter },
        }
    }

    /// Build a "[filter] creatures can't block" restriction.
    pub fn filtered_cant_block(source: ObjectId,
                               filter: crate::targets::ObjectFilter,
                               duration: Duration) -> Self {
        Self {
            source,
            layer: Layer::L6Ability,
            timestamp: 0,
            duration,
            dependency: None,
            kind: ContinuousEffectKind::FilteredCantBlock { filter },
        }
    }

    /// Build a STATIC "[filter] creatures have '<triggered
    /// ability>'" grant (Background statics). The def's `id` should
    /// use the granted range
    /// (`crate::triggers::GRANTED_TRIGGER_ID_BASE + n`); its effect
    /// fn dispatches via the pending trigger's effect_override.
    pub fn filtered_grant_triggered(
        source: ObjectId,
        filter: crate::targets::ObjectFilter,
        ability: crate::triggers::TriggeredAbilityDef,
        duration: Duration,
    ) -> Self {
        Self {
            source,
            layer: Layer::L6Ability,
            timestamp: 0,
            duration,
            dependency: None,
            kind: ContinuousEffectKind::FilteredGrantTriggeredAbility {
                filter, ability: Box::new(ability),
            },
        }
    }

    /// Build a "[this] doesn't untap during its controller's untap
    /// step" restriction.
    pub fn dont_untap(source: ObjectId, target: ObjectId,
                      duration: Duration) -> Self {
        Self {
            source,
            layer: Layer::L6Ability,
            timestamp: 0,
            duration,
            dependency: None,
            kind: ContinuousEffectKind::DontUntapTarget { target },
        }
    }

    /// Build a "[filter] don't untap during their controllers' untap
    /// steps" restriction (Crackdown / Choke / Winter Orb class).
    pub fn filtered_dont_untap(source: ObjectId,
                               filter: crate::targets::ObjectFilter,
                               duration: Duration) -> Self {
        Self {
            source,
            layer: Layer::L6Ability,
            timestamp: 0,
            duration,
            dependency: None,
            kind: ContinuousEffectKind::FilteredDontUntap { filter },
        }
    }

    /// Build a "players can't untap more than `max` [filter] during
    /// their untap steps" cap (Damping Field / Smoke class).
    pub fn untap_cap(source: ObjectId,
                     filter: crate::targets::ObjectFilter,
                     max: u32,
                     duration: Duration) -> Self {
        Self {
            source,
            layer: Layer::L6Ability,
            timestamp: 0,
            duration,
            dependency: None,
            kind: ContinuousEffectKind::UntapCap { filter, max },
        }
    }

    /// Build a spell cost modifier ("[filter] spells cost {N}
    /// more/less" — positive delta = tax, negative = reduction).
    pub fn spell_cost_modifier(source: ObjectId,
                               spell_filter: crate::targets::ObjectFilter,
                               caster: crate::targets::ControllerConstraint,
                               generic_delta: i32,
                               duration: Duration) -> Self {
        Self {
            source,
            layer: Layer::L6Ability,
            timestamp: 0,
            duration,
            dependency: None,
            kind: ContinuousEffectKind::SpellCostModifier {
                spell_filter, caster, generic_delta,
            },
        }
    }

    /// Build an activated-ability cost modifier (Training Grounds).
    pub fn ability_cost_modifier(source: ObjectId,
                                 source_filter: crate::targets::ObjectFilter,
                                 generic_delta: i32,
                                 duration: Duration) -> Self {
        Self {
            source,
            layer: Layer::L6Ability,
            timestamp: 0,
            duration,
            dependency: None,
            kind: ContinuousEffectKind::AbilityCostModifier {
                source_filter, generic_delta,
            },
        }
    }

    /// Build a "[filter] creatures can't be blocked by more than
    /// `max` creature(s)" cap (Familiar Ground).
    pub fn filtered_max_blockers(source: ObjectId,
                                 filter: crate::targets::ObjectFilter,
                                 max: u32,
                                 duration: Duration) -> Self {
        Self {
            source,
            layer: Layer::L6Ability,
            timestamp: 0,
            duration,
            dependency: None,
            kind: ContinuousEffectKind::FilteredMaxBlockers { filter, max },
        }
    }

    /// Build a GLOBAL "[filter] creatures lose [keyword]" removal
    /// (Gravity Sphere) — the filtered sibling of
    /// [`Self::remove_keyword`].
    pub fn filtered_remove_keyword(source: ObjectId,
                                   filter: crate::targets::ObjectFilter,
                                   keyword: KeywordAbility,
                                   duration: Duration) -> Self {
        Self {
            source,
            layer: Layer::L6Ability,
            timestamp: 0,
            duration,
            dependency: None,
            kind: ContinuousEffectKind::FilteredRemoveKeyword { filter, keyword },
        }
    }

    /// Build a Ghostly Prison / Propaganda attack tax protecting the
    /// source's controller.
    pub fn attack_tax(source: ObjectId, generic: u32,
                      duration: Duration) -> Self {
        Self {
            source,
            layer: Layer::L6Ability,
            timestamp: 0,
            duration,
            dependency: None,
            kind: ContinuousEffectKind::AttackTax { generic },
        }
    }

    /// Build a "target can't attack" effect (Pacifism-style).
    pub fn cant_attack(source: ObjectId, target: ObjectId,
                       duration: Duration) -> Self {
        Self {
            source,
            layer: Layer::L6Ability,
            timestamp: 0,
            duration,
            dependency: None,
            kind: ContinuousEffectKind::CantAttack { target },
        }
    }

    /// "Target attacks each combat if able" (CR 508.1a). Mirror of
    /// [`Self::cant_attack`]; consumed by `legal_actions` attacker enumeration.
    pub fn must_attack(source: ObjectId, target: ObjectId,
                       duration: Duration) -> Self {
        Self {
            source, layer: Layer::L6Ability, timestamp: 0, duration,
            dependency: None,
            kind: ContinuousEffectKind::MustAttack { target },
        }
    }

    /// "[filter] creatures attack each combat if able" — board-wide must-attack.
    pub fn filtered_must_attack(source: ObjectId,
                                filter: crate::targets::ObjectFilter,
                                duration: Duration) -> Self {
        Self {
            source, layer: Layer::L6Ability, timestamp: 0, duration,
            dependency: None,
            kind: ContinuousEffectKind::FilteredMustAttack { filter },
        }
    }

    /// Build a "target can't be blocked" effect. Mirror of
    /// [`Self::cant_attack`]; consumed by `combat::block_constraints`
    /// (sets the attacker's max blockers to 0). Typically installed
    /// with [`Duration::EndOfTurn`] for "~ can't be blocked this turn".
    pub fn cant_be_blocked(source: ObjectId, target: ObjectId,
                           duration: Duration) -> Self {
        Self {
            source,
            layer: Layer::L6Ability,
            timestamp: 0,
            duration,
            dependency: None,
            kind: ContinuousEffectKind::CantBeBlocked { target },
        }
    }

    /// "Target creature can't block [this turn]", Layer 6. Mirror of
    /// [`Self::cant_attack`]; consumed by `combat::blocker_eligible`.
    pub fn cant_block(source: ObjectId, target: ObjectId,
                      duration: Duration) -> Self {
        Self {
            source, layer: Layer::L6Ability, timestamp: 0, duration,
            dependency: None,
            kind: ContinuousEffectKind::CantBlock { target },
        }
    }

    /// "Target loses all abilities", Layer 6.
    pub fn lose_all_abilities(source: ObjectId, target: ObjectId,
                              duration: Duration) -> Self {
        Self {
            source, layer: Layer::L6Ability, timestamp: 0, duration,
            dependency: None,
            kind: ContinuousEffectKind::LoseAllAbilities { target },
        }
    }

    /// "[filter] permanents lose all abilities" (Layer 6) — the board-wide
    /// sibling of [`Self::lose_all_abilities`]. Strips keyword abilities from
    /// every battlefield object matching `filter` (base-characteristics,
    /// recursion-proof). Registry-defined activated/triggered abilities
    /// dispatch off card_id and aren't reachable here (documented partial,
    /// same as the single-target form).
    pub fn filtered_lose_abilities(source: ObjectId,
                                   filter: crate::targets::ObjectFilter,
                                   duration: Duration) -> Self {
        Self {
            source, layer: Layer::L6Ability, timestamp: 0, duration,
            dependency: None,
            kind: ContinuousEffectKind::FilteredLoseAllAbilities { filter },
        }
    }

    /// "[filter] permanents have base power and toughness N/M" (Layer 7b) —
    /// the board-wide sibling of `SetPt`. Sets (not adds) base P/T on every
    /// battlefield object matching `filter`; Layer 7c pumps and 7d counters
    /// stack on top per CR 613.
    pub fn filtered_set_base_pt(source: ObjectId,
                                filter: crate::targets::ObjectFilter,
                                power: i32, toughness: i32,
                                duration: Duration) -> Self {
        Self {
            source, layer: Layer::L7bPTSetting, timestamp: 0, duration,
            dependency: None,
            kind: ContinuousEffectKind::FilteredSetBasePt { filter, power, toughness },
        }
    }

    /// "Target becomes a/an [types] in addition", Layer 4.
    pub fn add_type(source: ObjectId, target: ObjectId,
                    types: crate::types::TypeLine, duration: Duration) -> Self {
        Self {
            source, layer: Layer::L4Type, timestamp: 0, duration,
            dependency: None,
            kind: ContinuousEffectKind::AddType { target, types },
        }
    }

    /// "Target becomes [colors]", Layer 5 (replaces the color set).
    pub fn set_color(source: ObjectId, target: ObjectId,
                     colors: crate::types::ColorSet, duration: Duration) -> Self {
        Self {
            source, layer: Layer::L5Color, timestamp: 0, duration,
            dependency: None,
            kind: ContinuousEffectKind::SetColor { target, colors },
        }
    }
}

/// The concrete kind of continuous effect. Most cards fit one of the
/// named variants; `Custom` is the escape hatch for exotic effects
/// that don't fit the common shapes.
// TODO(serialize): `Custom` carries a bare fn pointer.
#[derive(Clone, Debug)]
pub enum ContinuousEffectKind {
    /// "Target object gets +P/+T until end of turn" (Giant Growth).
    PumpTarget { target: ObjectId, power: i32, toughness: i32 },
    /// "Creatures you control get +P/+T" (Crusade, Glorious Anthem).
    AnthemForController { controller: PlayerId, power: i32, toughness: i32 },
    /// "Target object becomes P/T" (Humility-style 1/1).
    SetPt { target: ObjectId, power: i32, toughness: i32 },
    /// "Target gains [keyword] until end of turn" (Swiftfoot Boots).
    GrantKeywordTarget { target: ObjectId, keyword: KeywordAbility },
    /// "Creatures you control have [keyword]" (lord effects, Class
    /// level statics like "Creatures you control have menace"). The
    /// keyword analogue of [`Self::AnthemForController`].
    GrantKeywordToController { controller: PlayerId, keyword: KeywordAbility },
    /// CR 701.38 — Goad. "That creature attacks each combat if able
    /// and attacks a player other than `goader` if able." Doesn't
    /// modify characteristics; consumed by the legal-action enumerator.
    Goaded { target: ObjectId, goader: PlayerId },
    /// "Target creature can't attack" (Pacifism-style). Doesn't
    /// modify characteristics; consumed by [`crate::legal_actions`].
    CantAttack { target: ObjectId },
    /// "Target creature attacks each combat if able" (CR 508.1a — Juggernaut /
    /// Ulamog's Crusher self-must-attack). A combat REQUIREMENT, not a
    /// characteristic change; consumed by [`crate::legal_actions`] (the
    /// attacker enumerator filters to declarations that include every able
    /// must-attack creature). Mirror of [`Self::CantAttack`].
    MustAttack { target: ObjectId },
    /// "[filter] creatures attack each combat if able" (board-wide must-attack —
    /// Grizzled Anglerfish "{6}: opponents' creatures attack", Instigator).
    /// Filtered sibling of [`Self::MustAttack`].
    FilteredMustAttack { filter: crate::targets::ObjectFilter },
    /// "Target creature can't be blocked [this turn]." Doesn't modify
    /// characteristics; consumed by [`crate::combat`]'s
    /// `block_constraints` (caps the attacker's blockers at 0).
    CantBeBlocked { target: ObjectId },
    /// "Target creature can't block [this turn]" (Goblin Diplomats,
    /// Dragon Hatchling-style). Mirror of [`Self::CantAttack`];
    /// consumed by [`crate::combat::GameState::blocker_eligible`].
    /// Doesn't modify characteristics.
    CantBlock { target: ObjectId },
    /// Layer 6 — "Target loses all abilities" (Ovinize / Humility-style,
    /// the per-target form). Clears the in-flight `keywords`. NOTE:
    /// registry-defined activated/triggered abilities are dispatched
    /// off `card_id`, not these characteristics, so this clears
    /// KEYWORD abilities (flying, etc.) but not registry abilities —
    /// a documented partial until abilities migrate into characteristics.
    LoseAllAbilities { target: ObjectId },
    /// Board-wide "[filter] permanents lose all abilities" (Layer 6).
    FilteredLoseAllAbilities { filter: crate::targets::ObjectFilter },
    /// Board-wide "[filter] permanents have base power/toughness N/M"
    /// (Layer 7b) — sets, doesn't add.
    FilteredSetBasePt {
        filter: crate::targets::ObjectFilter,
        power: i32,
        toughness: i32,
    },
    /// Layer 4 — "Target is a/an [types] in addition to its other
    /// types" (Ardenvale Tactician's land animation, "becomes an
    /// artifact", "is also a creature"). ORs `types` into the
    /// in-flight type line (additive — does not remove existing types).
    AddType { target: ObjectId, types: crate::types::TypeLine },
    /// Layer 5 — "Target becomes [colors]" (becomes black, etc.).
    /// REPLACES the in-flight color set (CR 613.3e: a "becomes" color
    /// effect sets, it doesn't add — use the full intended set, e.g.
    /// black+green for "becomes black and green").
    SetColor { target: ObjectId, colors: crate::types::ColorSet },
    /// CR 702.6 — "Equipped creature gets +P/+T" (Bonesplitter,
    /// Sword of Fire and Ice, etc.). The buff applies to whatever
    /// creature the effect's source (the Equipment) is currently
    /// attached to, looked up dynamically at apply-time via
    /// `state.objects.get(source).attached_to`. When the Equipment
    /// is unattached, the effect is inert. Paired with
    /// [`Duration::WhileSourceOnBattlefield`] the effect auto-
    /// expires when the Equipment leaves play.
    AttachedCreatureGetsPt { power: i32, toughness: i32 },
    /// "Equipped/enchanted creature has [keyword]" — the Layer-6
    /// sibling of [`Self::AttachedCreatureGetsPt`]; follows
    /// `source.attached_to` dynamically, inert while unattached.
    AttachedCreatureGainsKeyword { keyword: KeywordAbility },
    /// Layer 6 — "Target creature loses [keyword] [until end of
    /// turn]" (Radjan Spirit, Crash Landing prep, Cephalid Snitch's
    /// protection strip). REMOVAL: within-layer timestamp order (CR
    /// 613.7) decides against later grants — a grant installed after
    /// this removal re-adds the keyword, one installed before stays
    /// removed.
    RemoveKeywordTarget { target: ObjectId, keyword: KeywordAbility },
    /// Layer 6 — "Equipped creature loses [keyword]" (Colossus Hammer
    /// "loses flying", Executioner's Hood kin). The removal sibling of
    /// [`Self::AttachedCreatureGainsKeyword`]; follows `attached_to`.
    AttachedCreatureLosesKeyword { keyword: KeywordAbility },
    /// Layer 4 — "Equipped/enchanted creature is a [subtype] in
    /// addition to its other types" (Raven Wings "is a Bird", Angelic
    /// Armaments "is an Angel"). ADDITIVE; follows `attached_to` like
    /// the other Attached* kinds. ("Is EVERY creature type" — Runed
    /// Stalactite — is [`Self::AttachedCreatureEveryCreatureType`].)
    AttachedCreatureAddSubtypes { subtypes: crate::types::SubtypeSet },
    /// Layer 4 — "TARGET creature becomes a [subtype] in addition to
    /// its other types" (Vault 87's Mutant, Xu-Ifit's Skeleton): the
    /// targeted sibling of [`Self::AttachedCreatureAddSubtypes`].
    AddSubtypesTarget { target: ObjectId, subtypes: crate::types::SubtypeSet },
    /// Layer 4 — "TARGET permanent becomes legendary in addition to
    /// its other supertypes" (Origin of Spider-Man's chapter; the
    /// supertype sibling of [`Self::AddSubtypesTarget`]). ADDITIVE.
    AddSupertypesTarget {
        target: ObjectId,
        supertypes: crate::types::SupertypeSet,
    },
    /// Layer 4 — "Equipped creature is an artifact in addition to its
    /// other types" (Silverskin Armor): attached CARD-TYPE add, the
    /// attached sibling of [`Self::AddType`].
    AttachedCreatureAddTypes { types: crate::types::TypeLine },
    /// Layer 4 — "Equipped creature is every creature type"
    /// (Runed Stalactite, Amorphous Axe — changeling templating).
    /// Sets [`crate::objects::Characteristics::every_creature_type`];
    /// layer-aware subtype predicates then treat the creature as
    /// having every subtype (type constraints on filters keep
    /// land/Equipment subtype filters honest in practice).
    AttachedCreatureEveryCreatureType,
    /// Layer 5 — "… and is [color] in addition to its other colors"
    /// (Angelic Armaments' white half). ADDITIVE — contrast
    /// [`Self::SetColor`], which replaces per CR 613.3e.
    AttachedCreatureAddColors { colors: crate::types::ColorSet },
    /// MARKER — "Enchanted/equipped creature can't attack" (Pacifism's
    /// attack half, Faith's Fetters, Bound in Silence). Follows
    /// `source.attached_to` dynamically (the dynamic sibling of
    /// [`Self::CantAttack`], whose `target` is fixed at install time);
    /// consumed by [`GameState::cant_attack`]. Inert while unattached.
    AttachedCreatureCantAttack,
    /// MARKER — "Enchanted/equipped creature can't block" (Pacifism's
    /// block half, Pillory of the Sleepless). Dynamic sibling of
    /// [`Self::CantBlock`]; follows `source.attached_to`; consumed by
    /// [`GameState::cant_block`]. Inert while unattached.
    AttachedCreatureCantBlock,
    /// MARKER — "Enchanted/equipped creature has '\[cost\]: \[effect\]'"
    /// (Evanescent Intellect, Squirrel Nest, Murderous Betrayal). Grants
    /// the held [`ActivatedAbilityDef`] to `source.attached_to`; gathered
    /// by [`GameState::granted_activated_for`] and chained into the
    /// host's activated-ability enumeration (the cost — e.g. `{T}` —
    /// applies to the HOST). Inert while unattached; auto-expires when
    /// the Aura/Equipment leaves (the effect is dropped from
    /// `continuous_effects`).
    AttachedCreatureGrantsActivated {
        ability: crate::registry::ActivatedAbilityDef,
    },
    /// MARKER — "Enchanted/equipped creature doesn't untap during its
    /// controller's untap step" (Glimmerdust Nap, Ice Over, Encrust).
    /// Dynamic sibling of [`Self::DontUntapTarget`]; follows
    /// `source.attached_to`; consumed by [`GameState::skips_untap`].
    AttachedCreatureDontUntap,
    /// MARKER — "Enchanted/equipped creature can't be blocked"
    /// (Aqueous Form, Aether Tunnel, Cloak of Mists). Dynamic sibling
    /// of [`Self::CantBeBlocked`]; follows `source.attached_to`;
    /// consumed by [`GameState::cant_be_blocked`].
    AttachedCreatureCantBeBlocked,
    /// Layer 7b — "Enchanted creature has base power and toughness
    /// X/Y" / Zendikon-style "becomes an X/Y creature" (Ensoul
    /// Artifact, Lignify, Guardian Zendikon). SETS base P/T (contrast
    /// the ADDITIVE [`Self::AttachedCreatureGetsPt`]); the targeted
    /// sibling of [`Self::SetPt`]. Follows `source.attached_to`.
    AttachedCreatureSetsPt { power: i32, toughness: i32 },
    /// "Equipped/enchanted creature gets +X/+Y where X/Y depend on
    /// board state" (Blackblade Reforged "+1/+1 for each land you
    /// control", Empyrial Armor "+1/+1 for each card in your hand").
    /// The dynamic sibling of [`Self::AttachedCreatureGetsPt`]:
    /// follows `source.attached_to`, and `compute` is called at
    /// apply-time with the SOURCE (the Equipment/Aura) so it can read
    /// the source's controller and count whatever the card names.
    AttachedCreatureGetsPtDynamic {
        compute: fn(&GameState, ObjectId) -> (i32, i32),
    },
    /// "Enchanted/equipped creature gets +`per_power`/+`per_toughness`
    /// for each [filter] permanent" — Blanchwood Armor ("+1/+1 for each
    /// Forest you control"), Aspect of Wolf, Nightmare. The
    /// registry-free counterpart of [`Self::AttachedCreatureGetsPtDynamic`]
    /// for the common board-count case: `filter` is matched against the
    /// battlefield from the SOURCE's controller's perspective (so
    /// `controlled_by(You)` resolves correctly), the count multiplies
    /// the per-match P/T, and the total is added to the host. Follows
    /// `source.attached_to`. Layer 7c, additive.
    AttachedCreatureGetsPtPerMatch {
        filter: crate::targets::ObjectFilter,
        per_power: i32,
        per_toughness: i32,
    },
    /// Layer 7c — GLOBAL FILTERED pump: "[filter] creatures get
    /// +P/+T" beyond the controller-anthem ("creatures with flying
    /// you control get +1/+1", "all Goblins get +1/+0", "white
    /// creatures get -1/-1"). The filter is evaluated against BASE
    /// characteristics (`ObjectFilter::matches_base`) from the
    /// SOURCE's controller's perspective — layer-aware predicates
    /// here would recurse.
    FilteredPump {
        filter: crate::targets::ObjectFilter,
        power: i32,
        toughness: i32,
    },
    /// Layer 7c — GLOBAL DYNAMIC filtered pump: "[filter] creatures get
    /// +X/+X, where X is …" (knowledge_is_power, meishin). `filter`
    /// selects the buffed creatures (BASE-characteristics match);
    /// `compute(state, source)` yields the per-creature delta. The
    /// dynamic sibling of [`Self::FilteredPump`] / global sibling of
    /// [`Self::AttachedCreatureGetsPtDynamic`]. `compute` must read
    /// state scalars / base counts — a layer-computed read recurses.
    FilteredPumpDynamic {
        filter: crate::targets::ObjectFilter,
        compute: fn(&GameState, ObjectId) -> (i32, i32),
    },
    /// Layer 7c — GLOBAL PER-MATCH filtered pump: "[filter] creatures
    /// get +`per_power`/+`per_toughness` for each [count_filter]
    /// permanent" (hold_the_gates). Both filters match against BASE
    /// characteristics from the source controller's perspective; the
    /// count is taken with `matches_base`, so it NEVER re-enters the
    /// layer system (recursion-proof by construction).
    FilteredPumpPerMatch {
        filter: crate::targets::ObjectFilter,
        count_filter: crate::targets::ObjectFilter,
        per_power: i32,
        per_toughness: i32,
    },
    /// Layer 7a — a SELF characteristic-defining P/T ability: the
    /// source's `*`/`*+N` base P/T resolves to `compute(state, source)`.
    /// SETS (not adds) the in-flight P/T to the computed Fixed values, so
    /// 7c pumps / 7d counters stack on top and a 7b "becomes N/N" still
    /// wins. `compute` must read base characteristics only (Layer-7
    /// recursion guard). Applies only to its own source.
    SetBasePtFromCda { compute: fn(&GameState, ObjectId) -> (i32, i32) },
    /// Layer 7a — a SELF CDA whose `*` is the COUNT of `count_filter`
    /// (matched via `matches_base` from the source's controller's
    /// perspective). SETS the in-flight P/T to (count, count). The filter
    /// variant of [`Self::SetBasePtFromCda`] for the common "equal to the
    /// number of [filter]" case — carries interned subtype symbols the
    /// no-registry compute fn can't, and the base-count never re-enters
    /// Layer 7 (recursion-proof). Applies only to its own source.
    SetBasePtFromMatch { count_filter: crate::targets::ObjectFilter },
    /// Layer 7a — ASYMMETRIC self CDA: one axis = count of `count_filter`
    /// (via `matches_base`), the other = `other_fixed`. `count_is_power`
    /// routes the count to power (else toughness). The asymmetric sibling
    /// of [`Self::SetBasePtFromMatch`] for "0/*" / "*/N" subtype-count CDAs
    /// the symmetric variant and the registry-less compute fn can't express.
    SetBasePtFromMatchAsym {
        count_filter: crate::targets::ObjectFilter,
        count_is_power: bool,
        other_fixed: i32,
    },
    /// Layer 6 — global filtered keyword grant ("all Zombies have
    /// menace", "creatures with power 2 or less have shroud"). Same
    /// base-characteristics filter posture as
    /// [`Self::FilteredPump`].
    FilteredGrantKeyword {
        filter: crate::targets::ObjectFilter,
        keyword: KeywordAbility,
    },
    /// Marker — "[filter] creatures can't attack" (Moat templating
    /// via without-flying filters; Peacekeeper-class with default
    /// filter). Consumed by [`GameState::cant_attack`].
    FilteredCantAttack { filter: crate::targets::ObjectFilter },
    /// Marker — "[filter] creatures can't block" (Goblin War Drums
    /// kin, "creatures with power 2 or less can't block"). Consumed
    /// by [`GameState::cant_block`].
    FilteredCantBlock { filter: crate::targets::ObjectFilter },
    /// Marker — STATIC ability grant to a filtered class: "[filter]
    /// creatures have '<triggered ability>'" (Backgrounds' "Commander
    /// creatures you own have …", Aether Charge-kin). Consumed by the
    /// trigger-collection scan in `engine::collect_pending_triggers`:
    /// each battlefield object matching `filter` (base
    /// characteristics, source-controller perspective) gets the
    /// ability checked against every event while this effect is
    /// live; fires carry the def's effect fn via
    /// `PendingTrigger::effect_override` (no registry entry exists
    /// for granted ids — same dispatch as delayed triggers).
    FilteredGrantTriggeredAbility {
        filter: crate::targets::ObjectFilter,
        ability: Box<crate::triggers::TriggeredAbilityDef>,
    },
    /// Marker — "[this permanent] doesn't untap during its
    /// controller's untap step" (tapped-for-good costs, Exhaust-kin).
    /// Consumed by [`GameState::skips_untap`] in the untap step.
    DontUntapTarget { target: ObjectId },
    /// Marker — "[filter] don't untap during their controllers'
    /// untap steps" (Crackdown, Choke's Islands, Winter Orb / Stasis
    /// templating with broad filters). Filter is matched on BASE
    /// characteristics from the source controller's perspective.
    FilteredDontUntap { filter: crate::targets::ObjectFilter },
    /// Marker — "players can't untap more than `max` [filter] during
    /// their untap steps" (Damping Field, Smoke, Mungha Wurm).
    /// Enforced in the untap step in deterministic ascending-id order
    /// (the player-choice ordering is a documented stand-in).
    UntapCap { filter: crate::targets::ObjectFilter, max: u32 },
    /// Marker — SPELL cost modifier: "[filter] spells [cast by
    /// caster] cost {N} more/less to cast" (Chill +1 on red spells,
    /// Sphere of Resistance +1 all, Arcane Melee -2 on instants/
    /// sorceries, Thalia-class taxes). `generic_delta` adjusts the
    /// GENERIC component only (colored pips never change; floor 0 —
    /// CR 601.2f). The spell filter matches the CAST FACE's
    /// characteristics; `caster` resolves against the modifier
    /// source's controller. Consumed by
    /// [`GameState::spell_cost_delta`] at cast-cost computation.
    SpellCostModifier {
        spell_filter: crate::targets::ObjectFilter,
        caster: crate::targets::ControllerConstraint,
        generic_delta: i32,
    },
    /// Marker — ACTIVATED-ABILITY cost modifier: "activated abilities
    /// of [filter] permanents [you control] cost {N} less to
    /// activate" (Training Grounds, Heartstone class). The filter
    /// matches the ability's SOURCE permanent. Consumed by
    /// [`GameState::ability_cost_delta`].
    AbilityCostModifier {
        source_filter: crate::targets::ObjectFilter,
        generic_delta: i32,
    },
    /// Marker — "each [filter] creature can't be blocked by more
    /// than `max` creature(s)" (Familiar Ground class). Consumed by
    /// [`crate::combat`]'s `block_constraints` (sets max_blockers).
    FilteredMaxBlockers { filter: crate::targets::ObjectFilter, max: u32 },
    /// Layer 6 — GLOBAL filtered keyword removal: "[filter] creatures
    /// lose [keyword]" (Gravity Sphere "all creatures lose flying",
    /// Mystic Decree). The filtered sibling of
    /// [`Self::RemoveKeywordTarget`]; same base-characteristics
    /// filter posture as [`Self::FilteredPump`].
    FilteredRemoveKeyword {
        filter: crate::targets::ObjectFilter,
        keyword: KeywordAbility,
    },
    /// Marker — Ghostly Prison / Propaganda: "creatures can't attack
    /// [the source's controller] unless their controller pays
    /// {generic} for each attacking creature". Consumed by
    /// [`GameState::attack_tax_total`]; payment is taken from the
    /// attacker's FLOATED mana pool at declaration (documented
    /// strictness: no float, no attack).
    AttackTax { generic: u32 },
    /// Custom. Called with the object id under consideration, its
    /// in-flight characteristics, and the game state.
    Custom(fn(ObjectId, &mut Characteristics, &GameState)),
}

impl ContinuousEffect {
    /// Is this effect currently LIVE? `WhileSourceShowsFace` dims
    /// while the source shows a different face (and while off the
    /// battlefield); every other duration is live until removed by
    /// its expiry hook.
    pub fn is_live(&self, state: &GameState) -> bool {
        match self.duration {
            Duration::WhileSourceShowsFace(face) => {
                state.objects.get(self.source).is_some_and(|o|
                    o.zone.is_battlefield() && o.visible_face == face)
            }
            Duration::WhileControllerTurn => {
                state.objects.get(self.source).is_some_and(|o|
                    o.zone.is_battlefield()
                        && o.controller == state.active_player())
            }
            _ => true,
        }
    }
}

impl ContinuousEffectKind {
    /// Does this effect variant apply to `object_id`? `source` is the
    /// installer (the permanent that produced the effect) — needed by
    /// variants whose target depends on live state referencing the
    /// source, such as
    /// [`ContinuousEffectKind::AttachedCreatureGetsPt`].
    pub fn applies_to(
        &self,
        object_id: ObjectId,
        source: ObjectId,
        state: &GameState,
    ) -> bool {
        match self {
            Self::PumpTarget { target, .. }
            | Self::SetPt { target, .. }
            | Self::GrantKeywordTarget { target, .. }
            | Self::Goaded { target, .. }
            | Self::CantAttack { target }
            | Self::MustAttack { target }
            | Self::CantBeBlocked { target }
            | Self::CantBlock { target }
            | Self::LoseAllAbilities { target }
            | Self::AddType { target, .. }
            | Self::RemoveKeywordTarget { target, .. }
            | Self::AddSubtypesTarget { target, .. }
            | Self::AddSupertypesTarget { target, .. }
            | Self::SetColor { target, .. } => *target == object_id,
            Self::AnthemForController { controller, .. }
            | Self::GrantKeywordToController { controller, .. } => {
                state.objects.get(object_id).is_some_and(|o|
                    o.is_creature()
                    && o.zone.is_battlefield()
                    && o.controller == *controller)
            }
            Self::AttachedCreatureGetsPt { .. }
            | Self::AttachedCreatureGainsKeyword { .. }
            | Self::AttachedCreatureGetsPtDynamic { .. }
            | Self::AttachedCreatureGetsPtPerMatch { .. }
            | Self::AttachedCreatureAddSubtypes { .. }
            | Self::AttachedCreatureAddColors { .. }
            | Self::AttachedCreatureAddTypes { .. }
            | Self::AttachedCreatureEveryCreatureType
            | Self::AttachedCreatureSetsPt { .. }
            | Self::AttachedCreatureLosesKeyword { .. } => {
                state.objects.get(source)
                    .and_then(|src| src.attached_to)
                    == Some(object_id)
            }
            // Self characteristic-defining ability — applies only to its
            // own source (CR 604.3).
            Self::SetBasePtFromCda { .. }
            | Self::SetBasePtFromMatch { .. }
            | Self::SetBasePtFromMatchAsym { .. } => object_id == source,
            Self::FilteredPump { filter, .. }
            | Self::FilteredPumpDynamic { filter, .. }
            | Self::FilteredPumpPerMatch { filter, .. }
            | Self::FilteredRemoveKeyword { filter, .. }
            | Self::FilteredLoseAllAbilities { filter }
            | Self::FilteredSetBasePt { filter, .. }
            | Self::FilteredGrantKeyword { filter, .. } => {
                // Battlefield-only, base-characteristics filter from
                // the source controller's perspective.
                let Some(src_ctrl) = state.objects.get(source)
                    .map(|s| s.controller) else { return false; };
                state.objects.get(object_id).is_some_and(|o|
                    o.zone.is_battlefield()
                        && filter.matches_base(o, state, src_ctrl))
            }
            // Markers: consumed by their scanners, never applied to
            // characteristics.
            Self::FilteredCantAttack { .. }
            | Self::FilteredMustAttack { .. }
            | Self::FilteredCantBlock { .. }
            | Self::AttachedCreatureCantAttack
            | Self::AttachedCreatureCantBlock
            | Self::AttachedCreatureDontUntap
            | Self::AttachedCreatureCantBeBlocked
            | Self::AttachedCreatureGrantsActivated { .. }
            | Self::FilteredGrantTriggeredAbility { .. }
            | Self::DontUntapTarget { .. }
            | Self::FilteredDontUntap { .. }
            | Self::UntapCap { .. }
            | Self::FilteredMaxBlockers { .. }
            | Self::SpellCostModifier { .. }
            | Self::AbilityCostModifier { .. }
            | Self::AttackTax { .. } => false,
            Self::Custom(_) => true, // Custom fn decides internally
        }
    }

    /// Apply this effect to `chars` (the in-flight characteristics
    /// of `object_id`). `source` is the installer (the permanent
    /// that produced the effect) — needed by variants whose value is
    /// computed relative to the source, such as
    /// [`Self::AttachedCreatureGetsPtDynamic`].
    pub fn apply(
        &self,
        object_id: ObjectId,
        source: ObjectId,
        chars: &mut Characteristics,
        state: &GameState,
    ) {
        match self {
            Self::PumpTarget { power, toughness, .. }
            | Self::AnthemForController { power, toughness, .. }
            | Self::AttachedCreatureGetsPt { power, toughness } => {
                add_to_pt(chars, *power, *toughness);
            }
            Self::AttachedCreatureGetsPtDynamic { compute } => {
                let (power, toughness) = compute(state, source);
                add_to_pt(chars, power, toughness);
            }
            Self::AttachedCreatureGetsPtPerMatch { filter, per_power, per_toughness } => {
                let who = state.objects.get(source).map(|s| s.controller).unwrap_or(0);
                let n = crate::script::count_matching(state, filter, who) as i32;
                add_to_pt(chars, per_power * n, per_toughness * n);
            }
            Self::AttachedCreatureAddSubtypes { subtypes }
            | Self::AddSubtypesTarget { subtypes, .. } => {
                chars.subtypes.0.extend(subtypes.0.iter().copied());
            }
            Self::AttachedCreatureAddTypes { types } => {
                chars.types = crate::types::TypeLine(chars.types.0 | types.0);
            }
            Self::AddSupertypesTarget { supertypes, .. } => {
                chars.supertypes = crate::types::SupertypeSet(
                    chars.supertypes.0 | supertypes.0);
            }
            Self::AttachedCreatureEveryCreatureType => {
                chars.every_creature_type = true;
            }
            Self::RemoveKeywordTarget { keyword, .. }
            | Self::AttachedCreatureLosesKeyword { keyword } => {
                chars.keywords.retain(|k| k != keyword);
            }
            Self::FilteredPump { power, toughness, .. } => {
                add_to_pt(chars, *power, *toughness);
            }
            Self::FilteredPumpDynamic { compute, .. } => {
                let (power, toughness) = compute(state, source);
                add_to_pt(chars, power, toughness);
            }
            Self::SetBasePtFromCda { compute } => {
                // CR 604.3 / Layer 7a — resolve the `*` to a Fixed value
                // (SET, not add) so 7c pumps / 7d counters stack on top.
                let (power, toughness) = compute(state, source);
                chars.power = Some(PtValue::Fixed(power));
                chars.toughness = Some(PtValue::Fixed(toughness));
            }
            Self::SetBasePtFromMatch { count_filter } => {
                let who = state.objects.get(source)
                    .map(|s| s.controller).unwrap_or(0);
                // BASE-characteristics count — recursion-proof at Layer 7a.
                let n = state.objects
                    .objects_in_zone(crate::zones::Zone::Battlefield)
                    .filter(|o| count_filter.matches_base(o, state, who))
                    .count() as i32;
                chars.power = Some(PtValue::Fixed(n));
                chars.toughness = Some(PtValue::Fixed(n));
            }
            Self::SetBasePtFromMatchAsym { count_filter, count_is_power, other_fixed } => {
                let who = state.objects.get(source)
                    .map(|s| s.controller).unwrap_or(0);
                let n = state.objects
                    .objects_in_zone(crate::zones::Zone::Battlefield)
                    .filter(|o| count_filter.matches_base(o, state, who))
                    .count() as i32;
                let (p, t) = if *count_is_power { (n, *other_fixed) }
                             else { (*other_fixed, n) };
                chars.power = Some(PtValue::Fixed(p));
                chars.toughness = Some(PtValue::Fixed(t));
            }
            Self::FilteredPumpPerMatch {
                count_filter, per_power, per_toughness, ..
            } => {
                let who = state.objects.get(source)
                    .map(|s| s.controller).unwrap_or(0);
                // Count via BASE characteristics — recursion-proof: never
                // re-enters the layer system (cf. `script::count_matching`,
                // which uses layer-aware `matches` and could recurse).
                let n = state.objects
                    .objects_in_zone(crate::zones::Zone::Battlefield)
                    .filter(|o| count_filter.matches_base(o, state, who))
                    .count() as i32;
                add_to_pt(chars, per_power * n, per_toughness * n);
            }
            Self::FilteredGrantKeyword { keyword, .. } => {
                if !chars.keywords.contains(keyword) {
                    chars.keywords.push(keyword.clone());
                }
            }
            Self::FilteredRemoveKeyword { keyword, .. } => {
                chars.keywords.retain(|k| k != keyword);
            }
            Self::FilteredCantAttack { .. }
            | Self::FilteredMustAttack { .. }
            | Self::FilteredCantBlock { .. }
            | Self::AttachedCreatureCantAttack
            | Self::AttachedCreatureCantBlock
            | Self::AttachedCreatureDontUntap
            | Self::AttachedCreatureCantBeBlocked
            | Self::AttachedCreatureGrantsActivated { .. }
            | Self::FilteredGrantTriggeredAbility { .. }
            | Self::DontUntapTarget { .. }
            | Self::FilteredDontUntap { .. }
            | Self::UntapCap { .. }
            | Self::FilteredMaxBlockers { .. }
            | Self::SpellCostModifier { .. }
            | Self::AbilityCostModifier { .. }
            | Self::AttackTax { .. } => {} // markers
            Self::AttachedCreatureAddColors { colors } => {
                chars.colors = crate::types::ColorSet(chars.colors.0 | colors.0);
            }
            Self::SetPt { power, toughness, .. }
            | Self::AttachedCreatureSetsPt { power, toughness } => {
                chars.power = Some(PtValue::Fixed(*power));
                chars.toughness = Some(PtValue::Fixed(*toughness));
            }
            Self::GrantKeywordTarget { keyword, .. }
            | Self::GrantKeywordToController { keyword, .. }
            | Self::AttachedCreatureGainsKeyword { keyword } => {
                if !chars.keywords.contains(keyword) {
                    chars.keywords.push(keyword.clone());
                }
            }
            Self::Goaded { .. }
            | Self::CantAttack { .. }
            | Self::MustAttack { .. }
            | Self::CantBeBlocked { .. }
            | Self::CantBlock { .. } => {
                // No characteristic modification — these are
                // combat-time modifiers consumed by `legal_actions`
                // (attack) / `combat` (block).
            }
            Self::LoseAllAbilities { .. }
            | Self::FilteredLoseAllAbilities { .. } => {
                // Layer 6 — strip keyword abilities. Registry-defined
                // activated/triggered abilities dispatch off card_id and
                // aren't reachable here (documented partial).
                chars.keywords.clear();
            }
            Self::FilteredSetBasePt { power, toughness, .. } => {
                // Layer 7b — SET base P/T (7c pumps / 7d counters stack on top).
                chars.power = Some(PtValue::Fixed(*power));
                chars.toughness = Some(PtValue::Fixed(*toughness));
            }
            Self::AddType { types, .. } => {
                // Layer 4 — additive: OR the new type bits in.
                chars.types.0 |= types.0;
            }
            Self::SetColor { colors, .. } => {
                // Layer 5 — "becomes [colors]" replaces the color set.
                chars.colors = *colors;
            }
            Self::Custom(f) => f(object_id, chars, state),
        }
    }
}

/// Add a P/T delta to fixed-value characteristics. Leaves
/// `PtValue::Star`/`StarPlus` alone (those are CDA territory, Layer 7a).
fn add_to_pt(chars: &mut Characteristics, p: i32, t: i32) {
    if p != 0 {
        if let Some(PtValue::Fixed(n)) = chars.power.as_mut() { *n += p; }
    }
    if t != 0 {
        if let Some(PtValue::Fixed(n)) = chars.toughness.as_mut() { *n += t; }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct DependencyInfo {
    pub depends_on: Vec<ObjectId>,
}

// =============================================================================
// DelayedTrigger re-export
// =============================================================================

// DelayedTrigger is defined canonically in `triggers.rs` now that it
// carries effect fn pointers. Re-export here for backwards-compat with
// callers that imported via `layers::DelayedTrigger`.
pub use crate::triggers::DelayedTrigger;

// =============================================================================
// GameState integration — the layer pipeline
// =============================================================================

impl GameState {
    /// Allocate a fresh monotonic timestamp. Called by
    /// [`Self::add_continuous_effect`] and by any other code that
    /// needs to stamp events for CR 613 ordering.
    pub fn next_timestamp(&mut self) -> u64 {
        let t = self.timestamp_counter;
        self.timestamp_counter = self.timestamp_counter
            .checked_add(1)
            .expect("timestamp_counter overflow");
        t
    }

    /// Register a continuous effect. Overwrites `effect.timestamp`
    /// with a fresh value so callers don't need to manage it.
    pub fn add_continuous_effect(&mut self, mut effect: ContinuousEffect) {
        effect.timestamp = self.next_timestamp();
        self.continuous_effects.push(effect);
    }

    /// Remove every continuous effect matching `pred`. Returns the
    /// number removed.
    pub fn remove_continuous_effects<F>(&mut self, pred: F) -> usize
    where F: FnMut(&ContinuousEffect) -> bool,
    {
        let before = self.continuous_effects.len();
        let mut keep = Vec::with_capacity(before);
        let mut pred = pred;
        for e in self.continuous_effects.drain(..) {
            if pred(&e) {
                // `pred(&e)` returns true for "remove" — discard.
            } else {
                keep.push(e);
            }
        }
        self.continuous_effects = keep;
        before - self.continuous_effects.len()
    }

    /// Expire all `Duration::EndOfTurn` continuous effects. Called by
    /// the engine at the cleanup step (CR 514.2).
    pub fn expire_end_of_turn_effects(&mut self) {
        self.remove_continuous_effects(|e|
            matches!(e.duration, Duration::EndOfTurn));
    }

    /// Expire "until your next turn" effects as `player`'s turn
    /// begins (engine turn-start hook). Without this they lasted
    /// forever — Goblin Racketeer's goad never lapsed.
    pub fn expire_until_next_turn_effects(&mut self, player: crate::types::PlayerId) {
        self.remove_continuous_effects(|e|
            matches!(e.duration, Duration::UntilYourNextTurn(p) if p == player));
    }

    /// Expire "until your next upkeep" effects as `player`'s upkeep
    /// begins (engine upkeep-tick hook).
    pub fn expire_until_next_upkeep_effects(&mut self, player: crate::types::PlayerId) {
        self.remove_continuous_effects(|e|
            matches!(e.duration, Duration::UntilNextUpkeepOf(p) if p == player));
    }

    /// Expire continuous effects sourced from `source_id` whose
    /// `duration == WhileSourceOnBattlefield`. Called when an object
    /// leaves the battlefield (engine hook).
    pub fn expire_effects_from_source(&mut self, source_id: ObjectId) {
        self.remove_continuous_effects(|e|
            e.source == source_id
            && matches!(e.duration,
                Duration::WhileSourceOnBattlefield
                | Duration::WhileSourceShowsFace(_)
                | Duration::WhileControllerTurn));
    }

    /// Run the CR 613 layer pipeline for `object_id` and return its
    /// current computed characteristics. Returns `None` if the object
    /// isn't in the arena.
    pub fn compute_characteristics(&self, object_id: ObjectId) -> Option<Characteristics> {
        let obj = self.objects.get(object_id)?;
        let mut chars = obj.characteristics.clone();

        // CR 702.73a — Changeling is a characteristic-defining
        // ability: a printed Changeling keyword means "this is every
        // creature type" in every zone, before any layer applies.
        if chars.keywords.contains(&crate::effects::KeywordAbility::Changeling) {
            chars.every_creature_type = true;
        }

        for &layer in Layer::all_in_order().iter() {
            if layer == Layer::L7dPTCounters {
                // Inline: fold the object's own +1/+1 and -1/-1
                // counters. Each pair contributes (+1, +1) / (-1, -1)
                // to P and T respectively.
                let plus = obj.count_counters(CounterKind::PlusOnePlusOne) as i32;
                let minus = obj.count_counters(CounterKind::MinusOneMinusOne) as i32;
                let delta = plus - minus;
                add_to_pt(&mut chars, delta, delta);
                continue;
            }
            // Gather effects in this layer applicable to this object,
            // sorted by timestamp. (TODO(613.8): full dependency
            // analysis — we only honor timestamps for now.)
            let mut effects: Vec<&ContinuousEffect> = self.continuous_effects.iter()
                .filter(|e| e.layer == layer
                    && e.is_live(self)
                    && e.kind.applies_to(object_id, e.source, self))
                .collect();
            effects.sort_by_key(|e| e.timestamp);
            for e in effects {
                e.kind.apply(object_id, e.source, &mut chars, self);
            }
        }

        Some(chars)
    }

    // --- Replaced stubs: computed P/T / lethal damage ---------------------

    /// Effective power for `object_id`, accounting for the full layer
    /// system (CR 613). Returns `None` when the object has no base
    /// power (not a creature), doesn't exist, or its base P is a
    /// still-unresolved CDA (`PtValue::Star`).
    pub fn computed_power(&self, object_id: ObjectId) -> Option<i32> {
        let chars = self.compute_characteristics(object_id)?;
        match chars.power? {
            PtValue::Fixed(n) => Some(n),
            // Star / StarPlus require the CDA to have been resolved
            // at Layer 7a. For Phase 1 we return None and let callers
            // decide.
            _ => None,
        }
    }

    /// Effective toughness for `object_id`. See [`Self::computed_power`].
    pub fn computed_toughness(&self, object_id: ObjectId) -> Option<i32> {
        let chars = self.compute_characteristics(object_id)?;
        match chars.toughness? {
            PtValue::Fixed(n) => Some(n),
            _ => None,
        }
    }

    /// CR 704.5g predicate using computed toughness. Returns `false`
    /// when toughness is 0 or negative — that's CR 704.5f territory,
    /// handled by a separate SBA.
    pub fn has_lethal_damage(&self, object_id: ObjectId) -> bool {
        let Some(t) = self.computed_toughness(object_id) else { return false; };
        if t <= 0 { return false; }
        self.objects.get(object_id)
            .is_some_and(|o| (o.damage_marked as i32) >= t)
    }

    // --- Keyword queries --------------------------------------------------

    /// Every keyword on `object_id` after the layer system — base
    /// keywords plus anything granted by Layer 6 continuous effects.
    /// Returns an empty vector if the object doesn't exist.
    pub fn effective_keywords(&self, object_id: ObjectId) -> Vec<KeywordAbility> {
        self.compute_characteristics(object_id)
            .map(|c| c.keywords)
            .unwrap_or_default()
    }

    /// Does `object_id` have the given keyword (base or granted)?
    pub fn has_keyword(&self, object_id: ObjectId, kw: &KeywordAbility) -> bool {
        self.compute_characteristics(object_id)
            .is_some_and(|c| c.keywords.contains(kw))
    }

    /// Is `object_id` goaded? Returns the first goading player if so
    /// (CR 701.38a — a creature can be goaded by multiple players; the
    /// aggregate restriction is "can't attack any of them", handled by
    /// [`Self::goaders_of`]).
    pub fn goaders_of(&self, object_id: ObjectId) -> Vec<PlayerId> {
        self.continuous_effects.iter()
            .filter_map(|e| match &e.kind {
                ContinuousEffectKind::Goaded { target, goader }
                    if *target == object_id => Some(*goader),
                _ => None,
            })
            .collect()
    }

    /// Does `object_id` have an active "can't attack" restriction?
    pub fn cant_attack(&self, object_id: ObjectId) -> bool {
        self.continuous_effects.iter().any(|e| match &e.kind {
            ContinuousEffectKind::CantAttack { target } => *target == object_id,
            ContinuousEffectKind::AttachedCreatureCantAttack =>
                e.is_live(self)
                    && self.objects.get(e.source)
                        .and_then(|s| s.attached_to) == Some(object_id),
            ContinuousEffectKind::FilteredCantAttack { filter } => {
                e.is_live(self)
                    && self.objects.get(e.source)
                        .map(|s| s.controller)
                        .zip(self.objects.get(object_id))
                        .is_some_and(|(ctrl, o)|
                            filter.matches_base(o, self, ctrl))
            }
            _ => false,
        })
    }

    /// CR 508.1a — does `object_id` have a "must attack if able" requirement?
    /// Sources: [`ContinuousEffectKind::MustAttack`] (self, Juggernaut-style),
    /// [`ContinuousEffectKind::FilteredMustAttack`] (board-wide), and Goad
    /// (CR 701.38a — a goaded creature attacks if able). Consumed by the
    /// attacker enumerator. Whether the requirement can actually be met is the
    /// enumerator's concern ("able" = has a legal defender).
    pub fn must_attack(&self, object_id: ObjectId) -> bool {
        if !self.goaders_of(object_id).is_empty() { return true; }
        self.continuous_effects.iter().any(|e| match &e.kind {
            ContinuousEffectKind::MustAttack { target } => *target == object_id,
            ContinuousEffectKind::FilteredMustAttack { filter } => {
                e.is_live(self)
                    && self.objects.get(e.source)
                        .map(|s| s.controller)
                        .zip(self.objects.get(object_id))
                        .is_some_and(|(ctrl, o)|
                            filter.matches_base(o, self, ctrl))
            }
            _ => false,
        })
    }

    /// Does `object_id` skip its controller's untap step? Consumed
    /// by `engine::untap_step` (CR 502.1 "doesn't untap" effects).
    pub fn skips_untap(&self, object_id: ObjectId) -> bool {
        self.continuous_effects.iter().any(|e| match &e.kind {
            ContinuousEffectKind::DontUntapTarget { target } =>
                e.is_live(self) && *target == object_id,
            ContinuousEffectKind::AttachedCreatureDontUntap =>
                e.is_live(self)
                    && self.objects.get(e.source)
                        .and_then(|s| s.attached_to) == Some(object_id),
            ContinuousEffectKind::FilteredDontUntap { filter } => {
                e.is_live(self)
                    && self.objects.get(e.source)
                        .map(|s| s.controller)
                        .zip(self.objects.get(object_id))
                        .is_some_and(|(ctrl, o)|
                            filter.matches_base(o, self, ctrl))
            }
            _ => None::<()>.is_some(),
        })
    }

    /// Live untap caps: (filter, max, source controller) triples —
    /// "players can't untap more than `max` [filter] during their
    /// untap steps". Consumed by `engine::untap_step`.
    pub fn untap_caps(&self)
        -> Vec<(crate::targets::ObjectFilter, u32, PlayerId)>
    {
        self.continuous_effects.iter().filter_map(|e| match &e.kind {
            ContinuousEffectKind::UntapCap { filter, max }
                if e.is_live(self) =>
                    self.objects.get(e.source).map(|s|
                        (filter.clone(), *max, s.controller)),
            _ => None,
        }).collect()
    }

    /// Net generic-cost delta for casting a spell whose CAST-FACE
    /// characteristics are `chars`, by `caster` (CR 601.2f — sum of
    /// live SpellCostModifier deltas whose filter matches and whose
    /// caster constraint accepts `caster`). The spell filter is
    /// evaluated against a transient object built from the face
    /// characteristics (the spell may not be a battlefield object).
    pub fn spell_cost_delta(
        &self,
        chars: &crate::objects::Characteristics,
        caster: PlayerId,
    ) -> i32 {
        let probe = crate::objects::GameObject::new(
            crate::objects::NULL_OBJECT_ID, caster,
            crate::zones::Zone::Stack, 0, chars.clone());
        self.continuous_effects.iter().filter_map(|e| match &e.kind {
            ContinuousEffectKind::SpellCostModifier {
                spell_filter, caster: who, generic_delta,
            } if e.is_live(self) => {
                let src_ctrl = self.objects.get(e.source)
                    .map(|s| s.controller)?;
                (who.matches(caster, src_ctrl)
                    && spell_filter.matches_base(&probe, self, src_ctrl))
                    .then_some(*generic_delta)
            }
            _ => None,
        }).sum()
    }

    /// Net generic-cost delta for activating an ability of
    /// `ability_source` (Training Grounds class).
    pub fn ability_cost_delta(
        &self,
        ability_source: ObjectId,
    ) -> i32 {
        self.continuous_effects.iter().filter_map(|e| match &e.kind {
            ContinuousEffectKind::AbilityCostModifier {
                source_filter, generic_delta,
            } if e.is_live(self) => {
                let src_ctrl = self.objects.get(e.source)
                    .map(|s| s.controller)?;
                let m = self.objects.get(ability_source).is_some_and(|o|
                    source_filter.matches_base(o, self, src_ctrl));
                m.then_some(*generic_delta)
            }
            _ => None,
        }).sum()
    }

    /// Total generic attack tax protecting `defender` (Ghostly
    /// Prison / Propaganda class): the sum over live [`
    /// ContinuousEffectKind::AttackTax`] effects whose SOURCE is
    /// controlled by the defender. Paid per attacking creature.
    pub fn attack_tax_total(&self, defender: PlayerId) -> u32 {
        self.continuous_effects.iter().filter_map(|e| match &e.kind {
            ContinuousEffectKind::AttackTax { generic }
                if e.is_live(self)
                    && self.objects.get(e.source)
                        .is_some_and(|s| s.controller == defender) =>
                Some(*generic),
            _ => None,
        }).sum()
    }

    /// Does `object_id` have an active "can't be blocked" restriction?
    /// Consumed by [`crate::combat`]'s `block_constraints`.
    pub fn cant_be_blocked(&self, object_id: ObjectId) -> bool {
        self.continuous_effects.iter().any(|e| match &e.kind {
            ContinuousEffectKind::CantBeBlocked { target } => *target == object_id,
            ContinuousEffectKind::AttachedCreatureCantBeBlocked =>
                e.is_live(self)
                    && self.objects.get(e.source)
                        .and_then(|s| s.attached_to) == Some(object_id),
            _ => false,
        })
    }

    /// Does `object_id` have an active "can't block" restriction?
    /// Activated abilities granted to `object_id` by an attached
    /// Aura/Equipment (CR 303.4 / 702.6 "enchanted creature has
    /// '\[cost\]: …'"). Gathers every live
    /// [`ContinuousEffectKind::AttachedCreatureGrantsActivated`] whose
    /// source is attached to `object_id`, in `continuous_effects`
    /// order. The SAME order is used by both the legal-action
    /// enumerator and `lookup_activated_ability`, so the flat ability
    /// index stays consistent between enumeration and resolution.
    pub fn granted_activated_for(&self, object_id: ObjectId)
        -> Vec<&crate::registry::ActivatedAbilityDef>
    {
        self.continuous_effects.iter().filter_map(|e| match &e.kind {
            ContinuousEffectKind::AttachedCreatureGrantsActivated { ability }
                if e.is_live(self)
                    && self.objects.get(e.source)
                        .and_then(|s| s.attached_to) == Some(object_id) =>
                Some(ability),
            _ => None,
        }).collect()
    }

    /// Consumed by [`crate::combat::GameState::blocker_eligible`].
    pub fn cant_block(&self, object_id: ObjectId) -> bool {
        self.continuous_effects.iter().any(|e| match &e.kind {
            ContinuousEffectKind::CantBlock { target } => *target == object_id,
            ContinuousEffectKind::AttachedCreatureCantBlock =>
                e.is_live(self)
                    && self.objects.get(e.source)
                        .and_then(|s| s.attached_to) == Some(object_id),
            ContinuousEffectKind::FilteredCantBlock { filter } => {
                e.is_live(self)
                    && self.objects.get(e.source)
                        .map(|s| s.controller)
                        .zip(self.objects.get(object_id))
                        .is_some_and(|(ctrl, o)|
                            filter.matches_base(o, self, ctrl))
            }
            _ => false,
        })
    }

    /// Every active Protection quality on `object_id`. Reads from the
    /// post-layer characteristics so granted protections count.
    pub fn protections_on(&self, object_id: ObjectId)
        -> Vec<crate::effects::ProtectionQuality>
    {
        let Some(chars) = self.compute_characteristics(object_id) else {
            return Vec::new();
        };
        chars.keywords.iter().filter_map(|kw| match kw {
            KeywordAbility::Protection(q) => Some(q.clone()),
            _ => None,
        }).collect()
    }

    /// CR 702.16e — does `target` have Protection that matches a source
    /// with `source_chars`?
    pub fn is_protected_from(
        &self,
        target: ObjectId,
        source_chars: &Characteristics,
    ) -> bool {
        self.protections_on(target).iter()
            .any(|q| q.matches_source(source_chars))
    }

    /// CR 702.16e — would `target` reject being attached by `attacher`?
    /// Reads `attacher`'s characteristics and runs the usual check.
    pub fn is_protected_from_attachment(
        &self,
        target: ObjectId,
        attacher: ObjectId,
    ) -> bool {
        let Some(src_chars) = self.compute_characteristics(attacher) else {
            return false;
        };
        self.is_protected_from(target, &src_chars)
    }

    /// CR 702.16e — does Protection on `target` reject being targeted
    /// by `source` object? Wrapper around [`Self::is_protected_from`].
    pub fn is_protected_target_of(
        &self,
        target: ObjectId,
        source: ObjectId,
    ) -> bool {
        let Some(src_chars) = self.compute_characteristics(source) else {
            return false;
        };
        self.is_protected_from(target, &src_chars)
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::effects::Effect;
    use crate::mana::ManaCost;
    use crate::objects::{Characteristics, GameObject};
    use crate::zones::Zone;

    fn creature_chars(p: i32, t: i32) -> Characteristics {
        Characteristics {
            mana_cost: Some(ManaCost::parse("{G}").unwrap()),
            colors: ColorSet::green(),
            types: TypeLine::CREATURE.into(),
            power: Some(PtValue::Fixed(p)),
            toughness: Some(PtValue::Fixed(t)),
            ..Default::default()
        }
    }

    fn put_creature(s: &mut GameState, owner: PlayerId, p: i32, t: i32) -> ObjectId {
        let id = s.allocate_object_id();
        let mut obj = GameObject::new(id, owner, Zone::Battlefield, 1, creature_chars(p, t));
        obj.controller = owner;
        s.objects.insert(obj);
        id
    }

    // --- Layer order --------------------------------------------------------

    #[test]
    fn layer_order_is_canonical() {
        let layers = Layer::all_in_order();
        assert_eq!(layers.len(), 11);
        assert_eq!(layers[0], Layer::L1Copy);
        assert_eq!(layers[6], Layer::L7aPTCharacteristicDefining);
        assert_eq!(layers[9], Layer::L7dPTCounters);
        assert_eq!(layers[10], Layer::L7ePTSwitching);
    }

    // --- compute_characteristics baseline ----------------------------------

    #[test]
    fn computes_base_characteristics_unchanged() {
        let mut s = GameState::new(2, 0);
        let c = put_creature(&mut s, 0, 2, 2);
        let chars = s.compute_characteristics(c).unwrap();
        assert_eq!(chars.power, Some(PtValue::Fixed(2)));
        assert_eq!(chars.toughness, Some(PtValue::Fixed(2)));
    }

    #[test]
    fn computed_pt_matches_base_with_no_effects() {
        let mut s = GameState::new(2, 0);
        let c = put_creature(&mut s, 0, 3, 4);
        assert_eq!(s.computed_power(c), Some(3));
        assert_eq!(s.computed_toughness(c), Some(4));
    }

    // --- Layer 7c pump ------------------------------------------------------

    #[test]
    fn pump_adds_to_power_and_toughness() {
        let mut s = GameState::new(2, 0);
        let c = put_creature(&mut s, 0, 2, 2);
        s.add_continuous_effect(
            ContinuousEffect::pump(999, c, 3, 3, Duration::EndOfTurn));
        assert_eq!(s.computed_power(c), Some(5));
        assert_eq!(s.computed_toughness(c), Some(5));
    }

    #[test]
    fn multiple_pumps_stack() {
        let mut s = GameState::new(2, 0);
        let c = put_creature(&mut s, 0, 1, 1);
        s.add_continuous_effect(
            ContinuousEffect::pump(999, c, 1, 1, Duration::EndOfTurn));
        s.add_continuous_effect(
            ContinuousEffect::pump(999, c, 2, 0, Duration::EndOfTurn));
        assert_eq!(s.computed_power(c), Some(4));
        assert_eq!(s.computed_toughness(c), Some(2));
    }

    #[test]
    fn pump_on_other_object_does_not_leak() {
        let mut s = GameState::new(2, 0);
        let c1 = put_creature(&mut s, 0, 2, 2);
        let c2 = put_creature(&mut s, 0, 2, 2);
        s.add_continuous_effect(
            ContinuousEffect::pump(999, c1, 3, 3, Duration::EndOfTurn));
        assert_eq!(s.computed_power(c1), Some(5));
        assert_eq!(s.computed_power(c2), Some(2));
    }

    // --- Layer 7b set-P/T ---------------------------------------------------

    #[test]
    fn set_pt_overrides_base_value() {
        let mut s = GameState::new(2, 0);
        let c = put_creature(&mut s, 0, 5, 5);
        s.add_continuous_effect(
            ContinuousEffect::set_pt(999, c, 1, 1, Duration::Permanent));
        assert_eq!(s.computed_power(c), Some(1));
        assert_eq!(s.computed_toughness(c), Some(1));
    }

    #[test]
    fn set_pt_then_pump_stacks_correctly() {
        // L7b applies first, so SetPT forces to 1/1, then L7c pump
        // adds +2/+2 → final 3/3.
        let mut s = GameState::new(2, 0);
        let c = put_creature(&mut s, 0, 5, 5);
        s.add_continuous_effect(
            ContinuousEffect::set_pt(999, c, 1, 1, Duration::Permanent));
        s.add_continuous_effect(
            ContinuousEffect::pump(999, c, 2, 2, Duration::EndOfTurn));
        assert_eq!(s.computed_power(c), Some(3));
        assert_eq!(s.computed_toughness(c), Some(3));
    }

    // --- Layer 7d counters --------------------------------------------------

    #[test]
    fn plus_one_counters_apply_in_layer_7d() {
        let mut s = GameState::new(2, 0);
        let c = put_creature(&mut s, 0, 2, 2);
        s.objects.get_mut(c).unwrap()
            .add_counters(CounterKind::PlusOnePlusOne, 3);
        assert_eq!(s.computed_power(c), Some(5));
        assert_eq!(s.computed_toughness(c), Some(5));
    }

    #[test]
    fn minus_one_counters_reduce_pt() {
        let mut s = GameState::new(2, 0);
        let c = put_creature(&mut s, 0, 3, 3);
        s.objects.get_mut(c).unwrap()
            .add_counters(CounterKind::MinusOneMinusOne, 1);
        assert_eq!(s.computed_power(c), Some(2));
        assert_eq!(s.computed_toughness(c), Some(2));
    }

    #[test]
    fn pump_then_counters_stack() {
        // Pump +1/+1 (L7c) then three +1/+1 counters (L7d).
        let mut s = GameState::new(2, 0);
        let c = put_creature(&mut s, 0, 1, 1);
        s.add_continuous_effect(
            ContinuousEffect::pump(999, c, 1, 1, Duration::EndOfTurn));
        s.objects.get_mut(c).unwrap()
            .add_counters(CounterKind::PlusOnePlusOne, 3);
        assert_eq!(s.computed_power(c), Some(5));
    }

    // --- Anthem -------------------------------------------------------------

    #[test]
    fn anthem_buffs_all_controller_creatures() {
        let mut s = GameState::new(2, 0);
        let mine1 = put_creature(&mut s, 0, 1, 1);
        let mine2 = put_creature(&mut s, 0, 2, 2);
        let theirs = put_creature(&mut s, 1, 3, 3);

        s.add_continuous_effect(
            ContinuousEffect::anthem(999, /*ctrl=*/ 0, 1, 1, Duration::Permanent));

        assert_eq!(s.computed_power(mine1), Some(2));
        assert_eq!(s.computed_power(mine2), Some(3));
        assert_eq!(s.computed_power(theirs), Some(3)); // unchanged
    }

    #[test]
    fn attached_cant_attack_block_follow_the_host() {
        // Pacifism: an Aura whose can't-attack/block markers resolve
        // through `source.attached_to`, so the restriction lands on the
        // enchanted creature, not the Aura.
        let mut s = GameState::new(2, 0);
        let creature = put_creature(&mut s, 0, 2, 2);
        let aura = s.allocate_object_id();
        s.objects.insert(GameObject::new(
            aura, 0, Zone::Battlefield, 1, Characteristics {
                types: TypeLine::ENCHANTMENT.into(),
                ..Default::default()
            }));

        s.add_continuous_effect(
            ContinuousEffect::attached_cant_attack(aura, Duration::WhileSourceOnBattlefield));
        s.add_continuous_effect(
            ContinuousEffect::attached_cant_block(aura, Duration::WhileSourceOnBattlefield));

        // Unattached: inert.
        assert!(!s.cant_attack(creature));
        assert!(!s.cant_block(creature));

        // Attach the Aura; the markers now bite the enchanted creature.
        s.objects.get_mut(aura).unwrap().attached_to = Some(creature);
        s.objects.get_mut(creature).unwrap().attachments.push(aura);
        assert!(s.cant_attack(creature), "enchanted creature can't attack");
        assert!(s.cant_block(creature), "enchanted creature can't block");
        // The Aura itself is unaffected.
        assert!(!s.cant_attack(aura));
    }

    #[test]
    fn aura_grants_protection_via_attached_keyword() {
        // "Enchanted creature has protection from red" — attached_keyword
        // carries the Protection keyword to the host; the existing
        // protection enforcement (is_protected_from) then applies. No
        // new engine surface — pure pack idiom.
        use crate::effects::{KeywordAbility, ProtectionQuality};
        use crate::types::Color;
        let mut s = GameState::new(2, 0);
        let host = put_creature(&mut s, 0, 2, 2);
        let aura = s.allocate_object_id();
        s.objects.insert(GameObject::new(
            aura, 0, Zone::Battlefield, 1, Characteristics {
                types: TypeLine::ENCHANTMENT.into(), ..Default::default() }));
        s.objects.get_mut(aura).unwrap().attached_to = Some(host);
        s.add_continuous_effect(ContinuousEffect::attached_keyword(
            aura, KeywordAbility::Protection(ProtectionQuality::Color(Color::Red)),
            Duration::WhileSourceOnBattlefield));

        let red = Characteristics { colors: ColorSet::red(), ..Default::default() };
        let blue = Characteristics { colors: ColorSet::blue(), ..Default::default() };
        assert!(s.is_protected_from(host, &red), "protected from red source");
        assert!(!s.is_protected_from(host, &blue), "not from blue");
    }

    #[test]
    fn attached_pt_per_match_counts_from_host_controllers_board() {
        // Blanchwood Armor: "+1/+1 for each Forest you control" — modeled
        // as a per-match attached pump counted from the source (aura)
        // controller's perspective.
        use crate::targets::{ControllerConstraint, ObjectFilter};
        let mut s = GameState::new(2, 0);
        let host = put_creature(&mut s, 0, 2, 2);
        // Two more creatures controlled by player 0 (the count target).
        put_creature(&mut s, 0, 1, 1);
        put_creature(&mut s, 0, 1, 1);
        let aura = s.allocate_object_id();
        s.objects.insert(GameObject::new(
            aura, 0, Zone::Battlefield, 1, Characteristics {
                types: TypeLine::ENCHANTMENT.into(), ..Default::default() }));
        s.objects.get_mut(aura).unwrap().attached_to = Some(host);

        // +1/+1 for each creature you control. Host (2/2) + 2 others = 3
        // creatures controlled by player 0 → +3/+3.
        s.add_continuous_effect(ContinuousEffect::attached_pt_per_match(
            aura,
            ObjectFilter::creature().controlled_by(ControllerConstraint::You),
            1, 1, Duration::WhileSourceOnBattlefield));
        assert_eq!(s.computed_power(host), Some(2 + 3));
        assert_eq!(s.computed_toughness(host), Some(2 + 3));
    }

    #[test]
    fn attached_dont_untap_cant_be_blocked_set_pt_follow_the_host() {
        // The Wave-2 easy-win attached markers/effects: each resolves
        // through `source.attached_to` like the cant-attack/block pair.
        let mut s = GameState::new(2, 0);
        let creature = put_creature(&mut s, 0, 3, 3);
        let aura = s.allocate_object_id();
        s.objects.insert(GameObject::new(
            aura, 0, Zone::Battlefield, 1, Characteristics {
                types: TypeLine::ENCHANTMENT.into(),
                ..Default::default()
            }));

        s.add_continuous_effect(
            ContinuousEffect::attached_dont_untap(aura, Duration::WhileSourceOnBattlefield));
        s.add_continuous_effect(
            ContinuousEffect::attached_cant_be_blocked(aura, Duration::WhileSourceOnBattlefield));
        s.add_continuous_effect(
            ContinuousEffect::attached_set_pt(aura, 0, 1, Duration::WhileSourceOnBattlefield));

        // Unattached: inert (P/T unchanged, no restrictions).
        assert!(!s.skips_untap(creature));
        assert!(!s.cant_be_blocked(creature));
        assert_eq!(s.computed_power(creature), Some(3));

        // Attach: all three bite the host.
        s.objects.get_mut(aura).unwrap().attached_to = Some(creature);
        s.objects.get_mut(creature).unwrap().attachments.push(aura);
        assert!(s.skips_untap(creature), "enchanted creature doesn't untap");
        assert!(s.cant_be_blocked(creature), "enchanted creature can't be blocked");
        assert_eq!(s.computed_power(creature), Some(0), "base P/T set to 0/1");
        assert_eq!(s.computed_toughness(creature), Some(1));
        // The Aura itself is unaffected.
        assert!(!s.skips_untap(aura));
    }

    #[test]
    fn lose_all_abilities_clears_keywords() {
        use crate::effects::KeywordAbility;
        let mut s = GameState::new(2, 0);
        let c = put_creature(&mut s, 0, 2, 2);
        s.objects.get_mut(c).unwrap().characteristics.keywords =
            vec![KeywordAbility::Flying, KeywordAbility::Trample];
        s.add_continuous_effect(
            ContinuousEffect::lose_all_abilities(999, c, Duration::EndOfTurn));
        assert!(s.effective_keywords(c).is_empty(), "all keyword abilities stripped");
    }

    #[test]
    fn add_type_and_set_color_alter_characteristics() {
        use crate::types::{TypeLine, ColorSet};
        let mut s = GameState::new(2, 0);
        let c = put_creature(&mut s, 0, 2, 2); // green creature
        // "becomes an artifact in addition" — additive type.
        s.add_continuous_effect(
            ContinuousEffect::add_type(999, c, TypeLine::ARTIFACT.into(), Duration::Permanent));
        // "becomes black".
        s.add_continuous_effect(
            ContinuousEffect::set_color(999, c, ColorSet::black(), Duration::Permanent));
        let cc = s.compute_characteristics(c).unwrap();
        assert!(cc.types.is_creature() && cc.types.is_artifact(),
            "artifact added without losing creature");
        assert_eq!(cc.colors, ColorSet::black(), "color set replaced to black");
    }

    #[test]
    fn cant_block_marker_blocks_blocker_eligibility() {
        let mut s = GameState::new(2, 0);
        let c = put_creature(&mut s, 1, 2, 2);
        assert!(!s.cant_block(c));
        s.add_continuous_effect(
            ContinuousEffect::cant_block(999, c, Duration::EndOfTurn));
        assert!(s.cant_block(c), "can't-block marker is queryable by combat");
    }

    #[test]
    fn keyword_anthem_grants_to_all_controller_creatures_only() {
        // "Creatures you control have menace" — the Class level-2 shape.
        let mut s = GameState::new(2, 0);
        let mine1 = put_creature(&mut s, 0, 1, 1);
        let mine2 = put_creature(&mut s, 0, 2, 2);
        let theirs = put_creature(&mut s, 1, 3, 3);

        s.add_continuous_effect(ContinuousEffect::keyword_anthem(
            999, /*ctrl=*/ 0, KeywordAbility::Menace, Duration::WhileSourceOnBattlefield));

        assert!(s.has_keyword(mine1, &KeywordAbility::Menace));
        assert!(s.has_keyword(mine2, &KeywordAbility::Menace));
        assert!(!s.has_keyword(theirs, &KeywordAbility::Menace)); // opponent's, unchanged
    }

    // --- has_lethal_damage uses the pipeline -------------------------------

    #[test]
    fn lethal_damage_with_pump_needs_more_damage() {
        let mut s = GameState::new(2, 0);
        let c = put_creature(&mut s, 0, 2, 2);
        s.objects.get_mut(c).unwrap().mark_damage(2);
        assert!(s.has_lethal_damage(c));

        // Pump +0/+1 → toughness 3, no longer lethal at 2 damage.
        s.add_continuous_effect(
            ContinuousEffect::pump(999, c, 0, 1, Duration::EndOfTurn));
        assert!(!s.has_lethal_damage(c));
    }

    // --- Timestamp ordering within a layer --------------------------------

    #[test]
    fn set_pt_with_later_timestamp_wins_on_same_layer() {
        // Two SetPT effects: latest-timestamp applies last (both are L7b).
        let mut s = GameState::new(2, 0);
        let c = put_creature(&mut s, 0, 5, 5);
        s.add_continuous_effect(
            ContinuousEffect::set_pt(1, c, 1, 1, Duration::Permanent));
        s.add_continuous_effect(
            ContinuousEffect::set_pt(2, c, 4, 4, Duration::Permanent));
        assert_eq!(s.computed_power(c), Some(4));
        assert_eq!(s.computed_toughness(c), Some(4));
    }

    #[test]
    fn next_timestamp_is_monotonic() {
        let mut s = GameState::new(2, 0);
        let t1 = s.next_timestamp();
        let t2 = s.next_timestamp();
        let t3 = s.next_timestamp();
        assert!(t1 < t2);
        assert!(t2 < t3);
    }

    // --- Duration / expiry -------------------------------------------------

    #[test]
    fn expire_end_of_turn_removes_eot_effects() {
        let mut s = GameState::new(2, 0);
        let c = put_creature(&mut s, 0, 2, 2);
        s.add_continuous_effect(
            ContinuousEffect::pump(999, c, 3, 3, Duration::EndOfTurn));
        s.add_continuous_effect(
            ContinuousEffect::pump(999, c, 1, 1, Duration::Permanent));
        assert_eq!(s.computed_power(c), Some(6)); // 2+3+1

        s.expire_end_of_turn_effects();
        assert_eq!(s.continuous_effects.len(), 1);
        assert_eq!(s.computed_power(c), Some(3)); // 2+1
    }

    #[test]
    fn expire_effects_from_source_respects_while_source_on_battlefield() {
        let mut s = GameState::new(2, 0);
        let src = put_creature(&mut s, 0, 1, 1);
        let target = put_creature(&mut s, 0, 2, 2);
        s.add_continuous_effect(ContinuousEffect::anthem(
            src, 0, 1, 1, Duration::WhileSourceOnBattlefield));
        assert_eq!(s.computed_power(target), Some(3));

        // Source leaves.
        s.expire_effects_from_source(src);
        assert_eq!(s.computed_power(target), Some(2));
    }

    #[test]
    fn expire_effects_from_source_leaves_others() {
        let mut s = GameState::new(2, 0);
        let src_a = put_creature(&mut s, 0, 1, 1);
        let src_b = put_creature(&mut s, 0, 1, 1);
        let target = put_creature(&mut s, 0, 2, 2);
        s.add_continuous_effect(ContinuousEffect::anthem(
            src_a, 0, 1, 0, Duration::WhileSourceOnBattlefield));
        s.add_continuous_effect(ContinuousEffect::anthem(
            src_b, 0, 0, 1, Duration::WhileSourceOnBattlefield));

        s.expire_effects_from_source(src_a);
        // src_b's anthem still applies.
        assert_eq!(s.computed_power(target), Some(2)); // base, no power
        assert_eq!(s.computed_toughness(target), Some(3)); // +1 from src_b
    }

    // --- Effect::Pump integration ------------------------------------------

    #[test]
    fn effect_pump_pushes_continuous_effect() {
        let mut s = GameState::new(2, 0);
        let c = put_creature(&mut s, 0, 2, 2);
        Effect::Pump {
            target: c,
            power: 3,
            toughness: 3,
            duration: Duration::EndOfTurn,
            keywords: vec![],
        }.execute(&mut s);
        // The pump registered a continuous effect; computed P/T reflects it.
        assert_eq!(s.computed_power(c), Some(5));
        assert_eq!(s.computed_toughness(c), Some(5));
    }

    // --- Defensive: missing object, no effects ----------------------------

    #[test]
    fn compute_characteristics_missing_object_returns_none() {
        let s = GameState::new(2, 0);
        assert!(s.compute_characteristics(999).is_none());
    }

    // --- Keyword grants (Layer 6) -----------------------------------------

    #[test]
    fn attached_keyword_follows_the_attachment() {
        let mut s = GameState::new(2, 0);
        let equipment = put_creature(&mut s, 0, 1, 1); // stands in for the Equipment
        let bearer_a = put_creature(&mut s, 0, 2, 2);
        let bearer_b = put_creature(&mut s, 0, 2, 2);
        s.add_continuous_effect(ContinuousEffect::attached_keyword(
            equipment, KeywordAbility::Vigilance,
            Duration::WhileSourceOnBattlefield,
        ));
        // Unattached: inert.
        assert!(!s.has_keyword(bearer_a, &KeywordAbility::Vigilance));
        // Attach to A: A has it, B doesn't.
        s.objects.get_mut(equipment).unwrap().attached_to = Some(bearer_a);
        assert!( s.has_keyword(bearer_a, &KeywordAbility::Vigilance));
        assert!(!s.has_keyword(bearer_b, &KeywordAbility::Vigilance));
        // Move to B: follows dynamically.
        s.objects.get_mut(equipment).unwrap().attached_to = Some(bearer_b);
        assert!(!s.has_keyword(bearer_a, &KeywordAbility::Vigilance));
        assert!( s.has_keyword(bearer_b, &KeywordAbility::Vigilance));
    }

    #[test]
    fn targeted_subtype_add_and_attached_type_add() {
        let mut s = GameState::new(2, 0);
        let equipment = put_creature(&mut s, 0, 0, 0);
        let bearer = put_creature(&mut s, 0, 2, 2);
        let other = put_creature(&mut s, 0, 2, 2);
        // Targeted: `other` becomes subtype-7 ("Mutant").
        s.add_continuous_effect(ContinuousEffect::add_subtypes(
            0, other, {
                let mut set = crate::types::SubtypeSet::new();
                set.0.insert(7);
                set
            }, Duration::Permanent,
        ));
        assert!( s.compute_characteristics(other).unwrap().subtypes.contains(7));
        assert!(!s.compute_characteristics(bearer).unwrap().subtypes.contains(7));
        // Attached card-type add: bearer becomes an ARTIFACT creature.
        s.add_continuous_effect(ContinuousEffect::attached_types(
            equipment, crate::types::TypeLine::ARTIFACT.into(),
            Duration::WhileSourceOnBattlefield,
        ));
        s.objects.get_mut(equipment).unwrap().attached_to = Some(bearer);
        let chars = s.compute_characteristics(bearer).unwrap();
        assert!(chars.types.is_artifact());
        assert!(chars.types.is_creature()); // additive, not replacing
    }

    #[test]
    fn filtered_pump_and_keyword_hit_the_matching_class_only() {
        let mut s = GameState::new(2, 0);
        let anthem_src = put_creature(&mut s, 0, 0, 0);
        let my_flyer = {
            let id = s.allocate_object_id();
            let mut chars = creature_chars(2, 2);
            chars.keywords.push(KeywordAbility::Flying);
            let mut o = GameObject::new(id, 0, Zone::Battlefield, 0, chars);
            o.controller = 0;
            s.objects.insert(o);
            id
        };
        let my_grounded = put_creature(&mut s, 0, 2, 2);
        let their_flyer = {
            let id = s.allocate_object_id();
            let mut chars = creature_chars(2, 2);
            chars.keywords.push(KeywordAbility::Flying);
            let mut o = GameObject::new(id, 1, Zone::Battlefield, 0, chars);
            o.controller = 1;
            s.objects.insert(o);
            id
        };
        // "Creatures with flying you control get +1/+1 and have
        // vigilance" — perspective = the SOURCE's controller (P0).
        let filter = crate::targets::ObjectFilter::creature()
            .with_keyword(KeywordAbility::Flying)
            .controlled_by(crate::targets::ControllerConstraint::You);
        s.add_continuous_effect(ContinuousEffect::filtered_pump(
            anthem_src, filter.clone(), 1, 1,
            Duration::WhileSourceOnBattlefield));
        s.add_continuous_effect(ContinuousEffect::filtered_keyword(
            anthem_src, filter, KeywordAbility::Vigilance,
            Duration::WhileSourceOnBattlefield));
        assert_eq!(s.computed_power(my_flyer), Some(3));
        assert!(s.has_keyword(my_flyer, &KeywordAbility::Vigilance));
        assert_eq!(s.computed_power(my_grounded), Some(2));
        assert_eq!(s.computed_power(their_flyer), Some(2));
        assert!(!s.has_keyword(their_flyer, &KeywordAbility::Vigilance));
    }

    #[test]
    fn filtered_lose_abilities_and_set_base_pt() {
        use crate::targets::{ControllerConstraint, ObjectFilter};
        // Poppet Factory: "Creature tokens you control lose all abilities and
        // have base power and toughness 3/3."
        let mut s = GameState::new(2, 0);
        let src = put_creature(&mut s, 0, 0, 0);
        // A creature token you control: Flying, base 5/5.
        let token = {
            let id = s.allocate_object_id();
            let mut chars = creature_chars(5, 5);
            chars.keywords.push(KeywordAbility::Flying);
            let mut o = GameObject::new(id, 0, Zone::Battlefield, 0, chars);
            o.controller = 0;
            o.is_token = true;
            s.objects.insert(o);
            id
        };
        // A non-token creature you control (must be untouched).
        let nontoken = put_creature(&mut s, 0, 4, 4);
        s.objects.get_mut(nontoken).unwrap()
            .characteristics.keywords.push(KeywordAbility::Trample);

        let tok_filter = ObjectFilter {
            is_token: Some(true),
            ..ObjectFilter::creature().controlled_by(ControllerConstraint::You)
        };
        s.add_continuous_effect(ContinuousEffect::filtered_lose_abilities(
            src, tok_filter.clone(), Duration::WhileSourceOnBattlefield));
        s.add_continuous_effect(ContinuousEffect::filtered_set_base_pt(
            src, tok_filter, 3, 3, Duration::WhileSourceOnBattlefield));

        // Token: abilities stripped, base 3/3.
        assert_eq!(s.computed_power(token), Some(3));
        assert_eq!(s.computed_toughness(token), Some(3));
        assert!(!s.has_keyword(token, &KeywordAbility::Flying));
        // Non-token: untouched.
        assert_eq!(s.computed_power(nontoken), Some(4));
        assert!(s.has_keyword(nontoken, &KeywordAbility::Trample));
    }

    #[test]
    fn spell_and_ability_cost_deltas() {
        use crate::targets::{ControllerConstraint, ObjectFilter};
        let mut s = GameState::new(2, 0);
        let chill = put_creature(&mut s, 0, 0, 4); // stands in for Chill
        // "Red spells cost {1} more to cast" (any caster).
        s.add_continuous_effect(ContinuousEffect::spell_cost_modifier(
            chill,
            ObjectFilter::new().with_colors(crate::types::ColorSet::red()),
            ControllerConstraint::Any, 1,
            Duration::WhileSourceOnBattlefield));
        // "Spells YOU cast cost {2} less" (controller-scoped).
        s.add_continuous_effect(ContinuousEffect::spell_cost_modifier(
            chill, ObjectFilter::default(),
            ControllerConstraint::You, -2,
            Duration::WhileSourceOnBattlefield));
        let red_spell = Characteristics {
            colors: crate::types::ColorSet::red(),
            types: crate::types::TypeLine::INSTANT.into(),
            ..Default::default()
        };
        // P0 (the modifier controller): +1 tax, -2 reduction = -1 net.
        assert_eq!(s.spell_cost_delta(&red_spell, 0), -1);
        // P1: only the unscoped tax applies.
        assert_eq!(s.spell_cost_delta(&red_spell, 1), 1);
        // The generic floor: {R} (no generic) taxed +1 grows a {1};
        // reduced -2 stays {R}.
        let r_cost = crate::mana::ManaCost::parse("{R}").unwrap();
        assert_eq!(r_cost.with_generic_delta(1),
                   crate::mana::ManaCost::parse("{1}{R}").unwrap());
        assert_eq!(r_cost.with_generic_delta(-2), r_cost);

        // Training Grounds: abilities of creatures you control cost
        // {2} less.
        let dork = put_creature(&mut s, 0, 1, 1);
        s.add_continuous_effect(ContinuousEffect::ability_cost_modifier(
            chill,
            ObjectFilter::creature().controlled_by(ControllerConstraint::You),
            -2, Duration::WhileSourceOnBattlefield));
        assert_eq!(s.ability_cost_delta(dork), -2);
    }

    #[test]
    fn filtered_max_blockers_and_filtered_keyword_removal() {
        let mut s = GameState::new(2, 0);
        let ground = put_creature(&mut s, 0, 0, 4);
        let mine = put_creature(&mut s, 0, 2, 2);
        let theirs = put_creature(&mut s, 1, 2, 2);
        // Familiar Ground: my creatures can't be blocked by >1.
        s.add_continuous_effect(ContinuousEffect::filtered_max_blockers(
            ground,
            crate::targets::ObjectFilter::creature()
                .controlled_by(crate::targets::ControllerConstraint::You),
            1, Duration::WhileSourceOnBattlefield));
        assert_eq!(s.block_constraints(mine).max_blockers, Some(1));
        assert_eq!(s.block_constraints(theirs).max_blockers, None);

        // Gravity Sphere: all creatures lose flying.
        s.objects.get_mut(theirs).unwrap().characteristics.keywords
            .push(KeywordAbility::Flying);
        assert!(s.has_keyword(theirs, &KeywordAbility::Flying));
        s.add_continuous_effect(ContinuousEffect::filtered_remove_keyword(
            ground, crate::targets::ObjectFilter::creature(),
            KeywordAbility::Flying, Duration::WhileSourceOnBattlefield));
        assert!(!s.has_keyword(theirs, &KeywordAbility::Flying));
    }

    #[test]
    fn filtered_cant_block_and_attack_tax_scanners() {
        let mut s = GameState::new(2, 0);
        let prison = put_creature(&mut s, 0, 0, 4);
        let weenie = put_creature(&mut s, 1, 1, 1);
        let big = put_creature(&mut s, 1, 5, 5);
        // "Creatures with power 2 or less can't block."
        s.add_continuous_effect(ContinuousEffect::filtered_cant_block(
            prison,
            crate::targets::ObjectFilter::creature().with_max_power(2),
            Duration::WhileSourceOnBattlefield));
        assert!( s.cant_block(weenie));
        assert!(!s.cant_block(big));
        // Ghostly Prison protecting P0 (the prison's controller).
        s.add_continuous_effect(ContinuousEffect::attack_tax(
            prison, 2, Duration::WhileSourceOnBattlefield));
        assert_eq!(s.attack_tax_total(0), 2);
        assert_eq!(s.attack_tax_total(1), 0);
    }

    #[test]
    fn filtered_pump_dynamic_scales_only_matching_creatures() {
        // "Creatures you control get +X/+X, where X is the number of
        // creatures you control" (knowledge_is_power class).
        fn per_you_creature(s: &GameState, source: ObjectId) -> (i32, i32) {
            let Some(src) = s.objects.get(source) else { return (0, 0); };
            let n = s.objects.iter()
                .filter(|o| o.zone.is_battlefield() && o.is_creature()
                    && o.controller == src.controller)
                .count() as i32;
            (n, n)
        }
        let mut s = GameState::new(2, 0);
        let src = put_creature(&mut s, 0, 0, 0);
        let mine = put_creature(&mut s, 0, 2, 2);
        let theirs = put_creature(&mut s, 1, 2, 2);
        let filter = crate::targets::ObjectFilter::creature()
            .controlled_by(crate::targets::ControllerConstraint::You);
        s.add_continuous_effect(ContinuousEffect::filtered_pump_dynamic(
            src, filter, per_you_creature, Duration::WhileSourceOnBattlefield));
        // P0 controls 2 creatures (src + mine) → +2/+2 to mine.
        assert_eq!(s.computed_power(mine), Some(4));
        assert_eq!(s.computed_toughness(mine), Some(4));
        // Opponent's creature is unaffected (filter is controlled_by You).
        assert_eq!(s.computed_power(theirs), Some(2));
        // Board grows → recomputes.
        let _extra = put_creature(&mut s, 0, 1, 1);
        assert_eq!(s.computed_power(mine), Some(5));
    }

    #[test]
    fn filtered_pump_per_match_is_recursion_proof() {
        // STRESS: the buffed creatures THEMSELVES match count_filter, so a
        // layer-aware count would re-enter and overflow; matches_base does not.
        // "Creatures you control get +0/+1 for each creature you control."
        let mut s = GameState::new(2, 0);
        let src = put_creature(&mut s, 0, 0, 0);
        let a = put_creature(&mut s, 0, 2, 2);
        let _b = put_creature(&mut s, 0, 1, 1);
        let you = crate::targets::ObjectFilter::creature()
            .controlled_by(crate::targets::ControllerConstraint::You);
        s.add_continuous_effect(ContinuousEffect::filtered_pump_per_match(
            src, you.clone(), you, 0, 1, Duration::WhileSourceOnBattlefield));
        // P0 controls 3 creatures → +0/+3 on each of its creatures.
        assert_eq!(s.computed_power(a), Some(2));
        assert_eq!(s.computed_toughness(a), Some(5));
    }

    #[test]
    fn self_pt_cda_resolves_star_and_stacks_with_pumps_and_counters() {
        // "*/* where * = creatures you control" (Veteran Warleader class).
        fn my_creatures(s: &GameState, src: ObjectId) -> (i32, i32) {
            let who = s.objects.get(src).map(|o| o.controller).unwrap_or(0);
            let n = s.objects.iter()
                .filter(|o| o.zone.is_battlefield() && o.is_creature()
                    && o.controller == who)
                .count() as i32;
            (n, n)
        }
        let mut s = GameState::new(2, 0);
        let goyf = {
            let id = s.allocate_object_id();
            let mut chars = creature_chars(0, 0);
            chars.power = Some(PtValue::Star);
            chars.toughness = Some(PtValue::Star);
            let mut o = GameObject::new(id, 0, Zone::Battlefield, 0, chars);
            o.controller = 0;
            s.objects.insert(o);
            id
        };
        // Unresolved `*` before the CDA installs.
        assert_eq!(s.computed_power(goyf), None);
        s.add_continuous_effect(ContinuousEffect::self_pt_cda(
            goyf, my_creatures, Duration::WhileSourceOnBattlefield));
        // Only goyf → */* = 1/1.
        assert_eq!(s.computed_power(goyf), Some(1));
        assert_eq!(s.computed_toughness(goyf), Some(1));
        // Board grows → recomputes (2 creatures).
        let _other = put_creature(&mut s, 0, 2, 2);
        assert_eq!(s.computed_power(goyf), Some(2));
        // +1/+1 counter (7d) stacks on the CDA: 2/2 → 3/3.
        s.objects.get_mut(goyf).unwrap().add_counters(CounterKind::PlusOnePlusOne, 1);
        assert_eq!(s.computed_power(goyf), Some(3));
        // A controller anthem +1/+0 (7c) also stacks. src2 is a 3rd
        // creature, so CDA = 3, +counter = 4, +anthem(+1/+0) = 5/4.
        let src2 = put_creature(&mut s, 0, 0, 0);
        s.add_continuous_effect(ContinuousEffect::filtered_pump(
            src2,
            crate::targets::ObjectFilter::creature()
                .controlled_by(crate::targets::ControllerConstraint::You),
            1, 0, Duration::WhileSourceOnBattlefield));
        assert_eq!(s.computed_power(goyf), Some(5));
        assert_eq!(s.computed_toughness(goyf), Some(4));
    }

    #[test]
    fn self_pt_from_match_counts_base_and_is_recursion_proof() {
        // "*/* = number of creatures you control" — STRESS: goyf itself
        // matches the count_filter, so a layer-aware count would re-enter
        // 7a and overflow; matches_base does not.
        let mut s = GameState::new(2, 0);
        let goyf = {
            let id = s.allocate_object_id();
            let mut chars = creature_chars(0, 0);
            chars.power = Some(PtValue::Star);
            chars.toughness = Some(PtValue::Star);
            let mut o = GameObject::new(id, 0, Zone::Battlefield, 0, chars);
            o.controller = 0;
            s.objects.insert(o);
            id
        };
        let you = crate::targets::ObjectFilter::creature()
            .controlled_by(crate::targets::ControllerConstraint::You);
        s.add_continuous_effect(ContinuousEffect::self_pt_from_match(
            goyf, you, Duration::WhileSourceOnBattlefield));
        assert_eq!(s.computed_power(goyf), Some(1));
        assert_eq!(s.computed_toughness(goyf), Some(1));
        let _other = put_creature(&mut s, 0, 2, 2);
        assert_eq!(s.computed_power(goyf), Some(2));
    }

    #[test]
    fn self_pt_from_match_asym_counts_one_axis_fixes_other() {
        // "0/* = number of creatures you control" (Traproot-class, asymmetric):
        // toughness = the count, power fixed 0.
        let mut s = GameState::new(2, 0);
        let wall = {
            let id = s.allocate_object_id();
            let mut chars = creature_chars(0, 0);
            chars.power = Some(PtValue::Fixed(0));
            chars.toughness = Some(PtValue::Star);
            let mut o = GameObject::new(id, 0, Zone::Battlefield, 0, chars);
            o.controller = 0;
            s.objects.insert(o);
            id
        };
        let you = crate::targets::ObjectFilter::creature()
            .controlled_by(crate::targets::ControllerConstraint::You);
        s.add_continuous_effect(ContinuousEffect::self_pt_from_match_asym(
            wall, you, /*count_is_power=*/ false, /*other_fixed=*/ 0,
            Duration::WhileSourceOnBattlefield));
        // 1 creature (wall) → 0/1; power stays fixed 0.
        assert_eq!(s.computed_power(wall), Some(0));
        assert_eq!(s.computed_toughness(wall), Some(1));
        let _other = put_creature(&mut s, 0, 2, 2);
        assert_eq!(s.computed_power(wall), Some(0));
        assert_eq!(s.computed_toughness(wall), Some(2));
    }

    #[test]
    fn self_pt_cda_is_overridden_by_layer_7b_set() {
        let mut s = GameState::new(2, 0);
        let goyf = {
            let id = s.allocate_object_id();
            let mut chars = creature_chars(0, 0);
            chars.power = Some(PtValue::Star);
            chars.toughness = Some(PtValue::Star);
            let mut o = GameObject::new(id, 0, Zone::Battlefield, 0, chars);
            o.controller = 0;
            s.objects.insert(o);
            id
        };
        s.add_continuous_effect(ContinuousEffect::self_pt_cda(
            goyf, |_, _| (5, 5), Duration::WhileSourceOnBattlefield));
        assert_eq!(s.computed_power(goyf), Some(5));
        // Humility "becomes 1/1" (7b) overrides the 7a CDA.
        let humility = put_creature(&mut s, 0, 0, 0);
        s.add_continuous_effect(ContinuousEffect::set_pt(
            humility, goyf, 1, 1, Duration::WhileSourceOnBattlefield));
        assert_eq!(s.computed_power(goyf), Some(1));
        assert_eq!(s.computed_toughness(goyf), Some(1));
    }

    #[test]
    fn while_controller_turn_dims_off_turn() {
        let mut s = GameState::new(2, 0);
        let src = put_creature(&mut s, 0, 0, 0);
        let mine = put_creature(&mut s, 0, 2, 2);
        // "During your turn, creatures you control get +1/+0" (street_riot).
        let you = crate::targets::ObjectFilter::creature()
            .controlled_by(crate::targets::ControllerConstraint::You);
        s.add_continuous_effect(ContinuousEffect::filtered_pump(
            src, you, 1, 0, Duration::WhileControllerTurn));
        // P0's turn (active_player = 0): live.
        assert_eq!(s.computed_power(mine), Some(3));
        // Opponent's turn: dims (not removed).
        s.turn.active_player = 1;
        assert_eq!(s.computed_power(mine), Some(2));
        // Back to P0's turn: relights.
        s.turn.active_player = 0;
        assert_eq!(s.computed_power(mine), Some(3));
    }


    #[test]
    fn while_source_shows_face_dims_with_the_face() {
        let mut s = GameState::new(2, 0);
        let dfc = put_creature(&mut s, 0, 2, 2);
        let c = put_creature(&mut s, 0, 2, 2);
        // Back-face anthem: live only while the source shows face 1.
        s.add_continuous_effect(ContinuousEffect {
            source: dfc,
            layer: Layer::L7cPTModifying,
            timestamp: 0,
            duration: Duration::WhileSourceShowsFace(1),
            dependency: None,
            kind: ContinuousEffectKind::AnthemForController {
                controller: 0, power: 1, toughness: 1 },
        });
        // Front face: dim.
        assert_eq!(s.computed_power(c), Some(2));
        // Transformed to the back face: live.
        s.objects.get_mut(dfc).unwrap().visible_face = 1;
        assert_eq!(s.computed_power(c), Some(3));
        // Back to front: dims again (not removed).
        s.objects.get_mut(dfc).unwrap().visible_face = 0;
        assert_eq!(s.computed_power(c), Some(2));
    }

    #[test]
    fn targeted_supertype_add_is_additive() {
        let mut s = GameState::new(2, 0);
        let c = put_creature(&mut s, 0, 2, 2);
        s.add_continuous_effect(ContinuousEffect::add_supertypes(
            0, c,
            crate::types::SupertypeSet::new()
                .with(crate::types::SupertypeSet::LEGENDARY),
            Duration::Permanent,
        ));
        let chars = s.compute_characteristics(c).unwrap();
        assert!(chars.supertypes.0 & crate::types::SupertypeSet::LEGENDARY != 0);
        assert!(chars.types.is_creature());
    }

    #[test]
    fn every_creature_type_flag_satisfies_subtype_filters() {
        let mut s = GameState::new(2, 0);
        let stalactite = put_creature(&mut s, 0, 0, 0);
        let bearer = put_creature(&mut s, 0, 2, 2);
        s.add_continuous_effect(ContinuousEffect::attached_every_creature_type(
            stalactite, Duration::WhileSourceOnBattlefield,
        ));
        s.objects.get_mut(stalactite).unwrap().attached_to = Some(bearer);
        assert!(s.compute_characteristics(bearer).unwrap().every_creature_type);
        // Any positive subtype filter matches; exclusions reject.
        let obj = s.objects.get(bearer).unwrap();
        let goblin = crate::targets::ObjectFilter::new().with_subtype_sym(42);
        assert!(goblin.matches(obj, &s, 0));
        let non_goblin = crate::targets::ObjectFilter::new().without_subtype_sym(42);
        assert!(!non_goblin.matches(obj, &s, 0));
    }

    #[test]
    fn changeling_keyword_is_a_cda() {
        let mut s = GameState::new(2, 0);
        let c = put_creature(&mut s, 0, 2, 2);
        s.objects.get_mut(c).unwrap().characteristics.keywords
            .push(crate::effects::KeywordAbility::Changeling);
        assert!(s.compute_characteristics(c).unwrap().every_creature_type);
        let obj = s.objects.get(c).unwrap();
        let any_tribe = crate::targets::ObjectFilter::new().with_subtype_sym(13);
        assert!(any_tribe.matches(obj, &s, 0));
    }

    #[test]
    fn until_next_turn_and_upkeep_durations_expire_on_their_hooks() {
        let mut s = GameState::new(2, 0);
        let c = put_creature(&mut s, 0, 2, 2);
        s.add_continuous_effect(ContinuousEffect::grant_keyword(
            0, c, KeywordAbility::Flying, Duration::UntilYourNextTurn(1),
        ));
        s.add_continuous_effect(ContinuousEffect::grant_keyword(
            0, c, KeywordAbility::Haste, Duration::UntilNextUpkeepOf(0),
        ));
        assert!(s.has_keyword(c, &KeywordAbility::Flying));
        assert!(s.has_keyword(c, &KeywordAbility::Haste));
        // Player 0's turn/upkeep: only the upkeep-bound effect ends.
        s.expire_until_next_turn_effects(0);
        s.expire_until_next_upkeep_effects(0);
        assert!( s.has_keyword(c, &KeywordAbility::Flying));
        assert!(!s.has_keyword(c, &KeywordAbility::Haste));
        // Player 1's turn begins: the until-your-next-turn effect ends.
        s.expire_until_next_turn_effects(1);
        assert!(!s.has_keyword(c, &KeywordAbility::Flying));
    }

    #[test]
    fn keyword_removal_strips_base_and_races_grants_by_timestamp() {
        let mut s = GameState::new(2, 0);
        let c = put_creature(&mut s, 0, 2, 2);
        s.objects.get_mut(c).unwrap().characteristics.keywords
            .push(KeywordAbility::Flying);
        assert!(s.has_keyword(c, &KeywordAbility::Flying));
        // Removal strips the printed keyword.
        s.add_continuous_effect(ContinuousEffect::remove_keyword(
            0, c, KeywordAbility::Flying, Duration::EndOfTurn,
        ));
        assert!(!s.has_keyword(c, &KeywordAbility::Flying));
        // A LATER grant out-timestamps the removal (CR 613.7).
        s.add_continuous_effect(ContinuousEffect::grant_keyword(
            0, c, KeywordAbility::Flying, Duration::EndOfTurn,
        ));
        assert!(s.has_keyword(c, &KeywordAbility::Flying));
    }

    #[test]
    fn attached_loses_keyword_follows_attachment() {
        let mut s = GameState::new(2, 0);
        let hood = put_creature(&mut s, 0, 0, 0); // stands in for the Equipment
        let bearer = put_creature(&mut s, 0, 2, 2);
        s.objects.get_mut(bearer).unwrap().characteristics.keywords
            .push(KeywordAbility::Flying);
        s.add_continuous_effect(ContinuousEffect::attached_loses_keyword(
            hood, KeywordAbility::Flying, Duration::WhileSourceOnBattlefield,
        ));
        // Unattached: inert.
        assert!(s.has_keyword(bearer, &KeywordAbility::Flying));
        // Attached: keyword stripped.
        s.objects.get_mut(hood).unwrap().attached_to = Some(bearer);
        assert!(!s.has_keyword(bearer, &KeywordAbility::Flying));
    }

    #[test]
    fn attached_subtype_and_color_grants_follow_attachment() {
        let mut s = GameState::new(2, 0);
        let armaments = put_creature(&mut s, 0, 0, 0); // stands in for the Equipment
        let bearer = put_creature(&mut s, 0, 2, 2);
        let angel = {
            // Intern via a throwaway interner-free path: build the set
            // directly from a raw symbol id.
            let mut set = crate::types::SubtypeSet::new();
            set.0.insert(7);
            set
        };
        s.add_continuous_effect(ContinuousEffect::attached_subtypes(
            armaments, angel.clone(), Duration::WhileSourceOnBattlefield,
        ));
        s.add_continuous_effect(ContinuousEffect::attached_colors(
            armaments, crate::types::ColorSet::white(),
            Duration::WhileSourceOnBattlefield,
        ));
        // Unattached: inert.
        let chars = s.compute_characteristics(bearer).unwrap();
        assert!(!chars.subtypes.contains(7));
        // Attached: bearer is an Angel and white, additively.
        s.objects.get_mut(armaments).unwrap().attached_to = Some(bearer);
        let chars = s.compute_characteristics(bearer).unwrap();
        assert!(chars.subtypes.contains(7));
        assert!(chars.colors.0 & crate::types::ColorSet::white().0 != 0);
        // Layer-aware ObjectFilter sees the granted subtype.
        let f = crate::targets::ObjectFilter::new().with_subtype_sym(7);
        let obj = s.objects.get(bearer).unwrap();
        assert!(f.matches(obj, &s, 0));
    }

    #[test]
    fn attached_pt_dynamic_recomputes_from_board_state() {
        // "+1/+1 for each creature its controller controls" stand-in.
        fn per_controlled_creature(s: &GameState, source: ObjectId) -> (i32, i32) {
            let Some(src) = s.objects.get(source) else { return (0, 0); };
            let n = s.objects.iter()
                .filter(|o| o.zone.is_battlefield()
                    && o.is_creature()
                    && o.controller == src.controller)
                .count() as i32;
            (n, n)
        }
        let mut s = GameState::new(2, 0);
        let equipment = put_creature(&mut s, 0, 1, 1); // stands in for the Equipment
        let bearer = put_creature(&mut s, 0, 2, 2);
        s.add_continuous_effect(ContinuousEffect::attached_pt_dynamic(
            equipment, per_controlled_creature,
            Duration::WhileSourceOnBattlefield,
        ));
        // Unattached: inert.
        assert_eq!(s.computed_power(bearer), Some(2));
        // Attached: 2 creatures on board → +2/+2.
        s.objects.get_mut(equipment).unwrap().attached_to = Some(bearer);
        assert_eq!(s.computed_power(bearer), Some(4));
        assert_eq!(s.computed_toughness(bearer), Some(4));
        // Board changes re-evaluate: add a third creature → +3/+3.
        let _third = put_creature(&mut s, 0, 1, 1);
        assert_eq!(s.computed_power(bearer), Some(5));
    }

    #[test]
    fn grant_keyword_target_folds_into_effective_keywords() {
        let mut s = GameState::new(2, 0);
        let c = put_creature(&mut s, 0, 2, 2);
        s.add_continuous_effect(ContinuousEffect::grant_keyword(
            /*source=*/ 0, c, KeywordAbility::Flying, Duration::EndOfTurn,
        ));
        assert!(s.has_keyword(c, &KeywordAbility::Flying));
        assert_eq!(s.effective_keywords(c), vec![KeywordAbility::Flying]);
    }

    #[test]
    fn grant_keyword_is_idempotent_with_base_keyword() {
        let mut s = GameState::new(2, 0);
        let c = put_creature(&mut s, 0, 2, 2);
        // Base creature already has Trample.
        s.objects.get_mut(c).unwrap().characteristics.keywords
            .push(KeywordAbility::Trample);
        s.add_continuous_effect(ContinuousEffect::grant_keyword(
            0, c, KeywordAbility::Trample, Duration::EndOfTurn,
        ));
        let kws = s.effective_keywords(c);
        // Trample appears once, not twice.
        assert_eq!(kws.iter().filter(|k| **k == KeywordAbility::Trample).count(), 1);
    }

    #[test]
    fn has_keyword_reads_base_characteristics() {
        let mut s = GameState::new(2, 0);
        let c = put_creature(&mut s, 0, 2, 2);
        s.objects.get_mut(c).unwrap().characteristics.keywords
            .push(KeywordAbility::Vigilance);
        assert!(s.has_keyword(c, &KeywordAbility::Vigilance));
        assert!(!s.has_keyword(c, &KeywordAbility::Flying));
    }

    #[test]
    fn custom_effect_escape_hatch() {
        fn grow(oid: ObjectId, chars: &mut Characteristics, _: &GameState) {
            if oid == 1 { add_to_pt(chars, 2, 2); }
        }
        let mut s = GameState::new(2, 0);
        let c = put_creature(&mut s, 0, 2, 2);
        assert_eq!(c, 1); // test relies on first allocated id = 1
        s.add_continuous_effect(ContinuousEffect {
            source: 0,
            layer: Layer::L7cPTModifying,
            timestamp: 0,
            duration: Duration::Permanent,
            dependency: None,
            kind: ContinuousEffectKind::Custom(grow),
        });
        assert_eq!(s.computed_power(c), Some(4));
    }
}
