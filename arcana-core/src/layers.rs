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
    WhileSourceOnBattlefield,
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
    /// CR 701.38 — Goad. "That creature attacks each combat if able
    /// and attacks a player other than `goader` if able." Doesn't
    /// modify characteristics; consumed by the legal-action enumerator.
    Goaded { target: ObjectId, goader: PlayerId },
    /// "Target creature can't attack" (Pacifism-style). Doesn't
    /// modify characteristics; consumed by [`crate::legal_actions`].
    CantAttack { target: ObjectId },
    /// Custom. Called with the object id under consideration, its
    /// in-flight characteristics, and the game state.
    Custom(fn(ObjectId, &mut Characteristics, &GameState)),
}

impl ContinuousEffectKind {
    /// Does this effect variant apply to `object_id`?
    pub fn applies_to(&self, object_id: ObjectId, state: &GameState) -> bool {
        match self {
            Self::PumpTarget { target, .. }
            | Self::SetPt { target, .. }
            | Self::GrantKeywordTarget { target, .. }
            | Self::Goaded { target, .. }
            | Self::CantAttack { target } => *target == object_id,
            Self::AnthemForController { controller, .. } => {
                state.objects.get(object_id).is_some_and(|o|
                    o.is_creature()
                    && o.zone.is_battlefield()
                    && o.controller == *controller)
            }
            Self::Custom(_) => true, // Custom fn decides internally
        }
    }

    /// Apply this effect to `chars` (the in-flight characteristics
    /// of `object_id`).
    pub fn apply(
        &self,
        object_id: ObjectId,
        chars: &mut Characteristics,
        state: &GameState,
    ) {
        match self {
            Self::PumpTarget { power, toughness, .. }
            | Self::AnthemForController { power, toughness, .. } => {
                add_to_pt(chars, *power, *toughness);
            }
            Self::SetPt { power, toughness, .. } => {
                chars.power = Some(PtValue::Fixed(*power));
                chars.toughness = Some(PtValue::Fixed(*toughness));
            }
            Self::GrantKeywordTarget { keyword, .. } => {
                if !chars.keywords.contains(keyword) {
                    chars.keywords.push(keyword.clone());
                }
            }
            Self::Goaded { .. } | Self::CantAttack { .. } => {
                // No characteristic modification — these are
                // attack-time modifiers consumed by `legal_actions`.
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

    /// Expire continuous effects sourced from `source_id` whose
    /// `duration == WhileSourceOnBattlefield`. Called when an object
    /// leaves the battlefield (engine hook).
    pub fn expire_effects_from_source(&mut self, source_id: ObjectId) {
        self.remove_continuous_effects(|e|
            e.source == source_id
            && matches!(e.duration, Duration::WhileSourceOnBattlefield));
    }

    /// Run the CR 613 layer pipeline for `object_id` and return its
    /// current computed characteristics. Returns `None` if the object
    /// isn't in the arena.
    pub fn compute_characteristics(&self, object_id: ObjectId) -> Option<Characteristics> {
        let obj = self.objects.get(object_id)?;
        let mut chars = obj.characteristics.clone();

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
                    && e.kind.applies_to(object_id, self))
                .collect();
            effects.sort_by_key(|e| e.timestamp);
            for e in effects {
                e.kind.apply(object_id, &mut chars, self);
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
        self.continuous_effects.iter().any(|e| matches!(&e.kind,
            ContinuousEffectKind::CantAttack { target } if *target == object_id))
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
