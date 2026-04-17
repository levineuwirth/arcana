//! Targeting system: `TargetFilter`, `TargetRequirement`, `ObjectFilter`,
//! and the validation logic that underpins CR 608.2b (target recheck at
//! resolution).
//!
//! Addendum Section 7, Phase 1 Task #9. Depends on tasks 4 (objects),
//! 6 (state), and transitively on 1 (types).
//!
//! **Design overview**
//!
//! Three interlocking concepts:
//!
//! - [`ObjectFilter`] is a structured predicate over a single [`GameObject`].
//!   Every field is an `Option<_>`; a `None` field is "don't care". All
//!   specified fields are ANDed together.
//! - [`TargetFilter`] is the kind-level shape of a targeting clause
//!   ("target creature", "target spell", "target card in any graveyard
//!   with CMC ≤ 3"). Several variants carry an `ObjectFilter` for the
//!   fine-grained characteristics.
//! - [`TargetRequirement`] bundles a filter with a `count` (`Exactly`,
//!   `UpTo`, `Any`, `X`) and an optional outer `controller` constraint.
//!   The engine holds one `TargetRequirement` per targeting clause in the
//!   card's text.
//!
//! **CR 608.2b** — when a spell or ability resolves, every target is
//! rechecked. Targets that are still legal keep their effects; targets
//! that have become illegal are ignored. If *every* target has become
//! illegal, the spell or ability doesn't resolve — it's countered by the
//! rules. [`validate_targets_on_resolution`] performs this recheck.
//!
//! **Why P/T comes from `raw_*_with_counters` for now**: computed P/T is
//! the layer system's job (Task #17). The current stub delegates to the
//! object's base + counter math, which is correct in the absence of
//! static P/T-modifying effects. The target validator itself won't need
//! changes once layers land — it'll call `state.computed_power(id)` in
//! place of the raw helper.

use serde::{Serialize, Deserialize};

use crate::objects::{GameObject, ObjectId};
use crate::state::GameState;
use crate::types::*;
use crate::zones::Zone;

// =============================================================================
// TargetSelection / TargetChoice — the chosen targets
// =============================================================================

/// A complete targeting selection for a spell or ability.
/// One entry per targeting clause in the oracle text.
#[derive(Clone, Debug, Default, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct TargetSelection {
    pub targets: Vec<TargetChoice>,
}

impl TargetSelection {
    pub fn new() -> Self { Self::default() }
    pub fn is_empty(&self) -> bool { self.targets.is_empty() }
    pub fn len(&self) -> usize { self.targets.len() }
}

/// A single target choice for one targeting clause.
#[derive(Clone, Debug, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub enum TargetChoice {
    Object(ObjectId),
    Player(PlayerId),
    /// For "target creature or player" / "any target".
    ObjectOrPlayer(ObjectOrPlayer),
}

#[derive(Clone, Debug, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub enum ObjectOrPlayer {
    Object(ObjectId),
    Player(PlayerId),
}

// =============================================================================
// TargetRequirement — filter + count + outer controller constraint
// =============================================================================

/// Defines what is a legal target for a single targeting clause. A spell
/// like "destroy two target creatures you control" is one
/// `TargetRequirement` with `filter = Permanent(..creature..)`,
/// `count = Exactly(2)`, `controller = Some(You)`.
#[derive(Clone, Debug)]
pub struct TargetRequirement {
    pub filter: TargetFilter,
    pub count: TargetCount,
    /// Outer controller constraint — applies to targets produced by the
    /// filter. Useful for `TargetFilter` variants without a sub-filter
    /// (e.g. `Creature`). When using `Permanent(ObjectFilter)`, prefer
    /// setting the controller inside the inner filter.
    pub controller: Option<ControllerConstraint>,
}

impl TargetRequirement {
    /// Convenience: "target creature" with no special constraints.
    pub fn target_creature() -> Self {
        Self {
            filter: TargetFilter::Creature,
            count: TargetCount::Exactly(1),
            controller: None,
        }
    }

    /// Convenience: "target player".
    pub fn target_player() -> Self {
        Self {
            filter: TargetFilter::Player,
            count: TargetCount::Exactly(1),
            controller: None,
        }
    }

    /// Convenience: "any target" (creature, player, or planeswalker).
    pub fn any_target() -> Self {
        Self {
            filter: TargetFilter::AnyTarget,
            count: TargetCount::Exactly(1),
            controller: None,
        }
    }

    /// Is every choice in `selection` legal under this requirement, and
    /// is the number of choices consistent with `count`?
    ///
    /// This is the *full* check used when validating a proposed
    /// selection (e.g. when an agent submits an `Action::CastSpell`).
    /// For the CR 608.2b resolution-time recheck, use
    /// [`validate_targets_on_resolution`] which handles the
    /// partial-legality case.
    pub fn is_satisfied(
        &self,
        selection: &TargetSelection,
        state: &GameState,
        source_controller: PlayerId,
        x_value: Option<u32>,
    ) -> bool {
        if !self.count.is_valid_count(selection.len() as u32, x_value) {
            return false;
        }
        selection.targets.iter().all(|c|
            self.matches_choice(c, state, source_controller))
    }

    /// Legality of a single choice under this requirement (filter AND
    /// outer controller). Exposed so the legal-action enumerator can
    /// build up selections one target at a time.
    pub fn matches_choice(
        &self,
        choice: &TargetChoice,
        state: &GameState,
        source_controller: PlayerId,
    ) -> bool {
        if !self.filter.matches(choice, state, source_controller) {
            return false;
        }
        if let Some(ctrl) = &self.controller {
            // Controller constraint only meaningful for object targets.
            if let Some(id) = choice.object_id() {
                if let Some(obj) = state.objects.get(id) {
                    if !ctrl.matches(obj.controller, source_controller) {
                        return false;
                    }
                }
            }
        }
        // CR 702.11b — Hexproof: can't be the target of spells or
        // abilities your opponents control.
        //
        // CR 702.21a — Ward: target legality is NOT affected. Ward is a
        // triggered ability that fires on being targeted; the caster
        // pays or the spell is countered at resolution. We resolve the
        // Ward prompt in `engine::collect_ward_queue` + `begin_ward_check`
        // before running the spell's effects (Phase 2-A stopgap —
        // TODO(phase-2b): route Ward as a real trigger so it can be
        // Stifled).
        if let Some(id) = choice.object_id() {
            if let Some(obj) = state.objects.get(id) {
                if obj.controller != source_controller
                    && state.has_keyword(id, &crate::effects::KeywordAbility::Hexproof)
                {
                    return false;
                }
                // CR 702.16e — Protection: rejects being the target of
                // matching sources. The "source" for targeting purposes
                // is the spell's characteristics (or the activated
                // ability's source). For Phase 1 we reject if any
                // object in the arena belonging to source_controller
                // with the outer-filter matching qualities would match;
                // practically, the spell resolves from controller's
                // library/hand/stack so we use their color identity
                // later. Conservative Phase 1 check: reject only when
                // Protection::Everything or Protection::AnyColor is
                // present (broad shields); fine-grained source-color
                // matching is TODO until the targeting API carries the
                // originating object.
                use crate::effects::{KeywordAbility, ProtectionQuality};
                if obj.controller != source_controller
                    && state.effective_keywords(id).iter().any(|kw| matches!(kw,
                        KeywordAbility::Protection(ProtectionQuality::Everything)))
                {
                    return false;
                }
            }
        }
        true
    }
}

#[derive(Clone, Copy, Debug)]
pub enum TargetCount {
    Exactly(u32),
    UpTo(u32),
    /// "Any number of" — including zero.
    Any,
    /// X from the spell's X value; the caller supplies the `x_value`.
    X,
}

impl TargetCount {
    /// Is `n` a permissible number of chosen targets for this count?
    /// For the `X` variant, `x_value` must be provided.
    pub fn is_valid_count(&self, n: u32, x_value: Option<u32>) -> bool {
        match self {
            Self::Exactly(k) => n == *k,
            Self::UpTo(k)    => n <= *k,
            Self::Any        => true,
            Self::X          => x_value.map_or(false, |x| n == x),
        }
    }
}

// =============================================================================
// TargetFilter — kind-level targeting shape
// =============================================================================

// TODO(serialize): `TargetFilter::Custom` carries a bare `fn` pointer.
// Migrate per Section 12 in Phase 3.
#[derive(Clone, Debug)]
pub enum TargetFilter {
    Creature,
    Player,
    CreatureOrPlayer,
    /// CR 115.4 "any target" — creature, player, or planeswalker.
    AnyTarget,
    Permanent(ObjectFilter),
    Spell(ObjectFilter),
    /// "Target card in [zone]" — e.g. target card in a graveyard.
    Card { zone: Zone, filter: ObjectFilter },
    Custom(fn(&GameObject, &GameState) -> bool),
}

impl TargetFilter {
    /// Is `choice` a legal target under this filter, given current state?
    ///
    /// Pure filter check — does not consult any outer controller
    /// constraint (see [`TargetRequirement::matches_choice`] for that).
    pub fn matches(
        &self,
        choice: &TargetChoice,
        state: &GameState,
        source_controller: PlayerId,
    ) -> bool {
        match (self, choice) {
            // --- Creature: battlefield, creature type ---
            (TargetFilter::Creature, TargetChoice::Object(id)) => {
                state.objects.get(*id).is_some_and(|o|
                    o.zone.is_battlefield() && o.is_creature())
            }

            // --- Player: a live player ---
            (TargetFilter::Player, TargetChoice::Player(p)) => {
                (*p as usize) < state.players.len()
                    && state.player(*p).is_alive()
            }

            // --- Creature-or-player: either branch legal ---
            (TargetFilter::CreatureOrPlayer, TargetChoice::ObjectOrPlayer(oop))
            | (TargetFilter::AnyTarget, TargetChoice::ObjectOrPlayer(oop)) => {
                match oop {
                    ObjectOrPlayer::Object(id) => {
                        state.objects.get(*id).is_some_and(|o|
                            o.zone.is_battlefield()
                                && (o.is_creature()
                                    || (matches!(self, TargetFilter::AnyTarget)
                                        && o.is_planeswalker())))
                    }
                    ObjectOrPlayer::Player(p) => {
                        (*p as usize) < state.players.len()
                            && state.player(*p).is_alive()
                    }
                }
            }

            // --- Permanent: battlefield + inner filter ---
            (TargetFilter::Permanent(f), TargetChoice::Object(id)) => {
                state.objects.get(*id).is_some_and(|o|
                    o.is_permanent_on_battlefield()
                        && f.matches(o, state, source_controller))
            }

            // --- Spell: on the stack + inner filter ---
            (TargetFilter::Spell(f), TargetChoice::Object(id)) => {
                state.objects.get(*id).is_some_and(|o|
                    o.zone == Zone::Stack
                        && f.matches(o, state, source_controller))
            }

            // --- Card in a specified zone kind ---
            (TargetFilter::Card { zone, filter }, TargetChoice::Object(id)) => {
                state.objects.get(*id).is_some_and(|o|
                    o.zone.same_kind(*zone)
                        && filter.matches(o, state, source_controller))
            }

            // --- Custom closure ---
            (TargetFilter::Custom(f), TargetChoice::Object(id)) => {
                state.objects.get(*id).is_some_and(|o| f(o, state))
            }

            // --- Any other pairing is a type mismatch (e.g. picking a
            // Player for TargetFilter::Creature) ---
            _ => false,
        }
    }

    /// Enumerate every legal `TargetChoice` for this filter in the current
    /// state. Drives the legal-action enumerator.
    ///
    /// For `Player` and player-inclusive filters, players are yielded in
    /// id order. For object filters, iteration order follows the arena's
    /// `HashMap`, which is nondeterministic — callers that need a
    /// reproducible order should sort by id.
    pub fn enumerate_legal<'a>(
        &'a self,
        state: &'a GameState,
        source_controller: PlayerId,
    ) -> Vec<TargetChoice> {
        let mut out = Vec::new();
        match self {
            TargetFilter::Creature => {
                for o in state.objects.objects_in_zone(Zone::Battlefield) {
                    if o.is_creature() {
                        out.push(TargetChoice::Object(o.id));
                    }
                }
            }
            TargetFilter::Player => {
                for p in 0..state.num_players() {
                    if state.player(p).is_alive() {
                        out.push(TargetChoice::Player(p));
                    }
                }
            }
            TargetFilter::CreatureOrPlayer | TargetFilter::AnyTarget => {
                for o in state.objects.objects_in_zone(Zone::Battlefield) {
                    let legal = o.is_creature()
                        || (matches!(self, TargetFilter::AnyTarget)
                            && o.is_planeswalker());
                    if legal {
                        out.push(TargetChoice::ObjectOrPlayer(
                            ObjectOrPlayer::Object(o.id)));
                    }
                }
                for p in 0..state.num_players() {
                    if state.player(p).is_alive() {
                        out.push(TargetChoice::ObjectOrPlayer(
                            ObjectOrPlayer::Player(p)));
                    }
                }
            }
            TargetFilter::Permanent(f) => {
                for o in state.objects.objects_in_zone(Zone::Battlefield) {
                    if o.is_permanent_on_battlefield()
                        && f.matches(o, state, source_controller)
                    {
                        out.push(TargetChoice::Object(o.id));
                    }
                }
            }
            TargetFilter::Spell(f) => {
                for o in state.objects.objects_in_zone(Zone::Stack) {
                    if f.matches(o, state, source_controller) {
                        out.push(TargetChoice::Object(o.id));
                    }
                }
            }
            TargetFilter::Card { zone, filter } => {
                for o in state.objects.objects_in_zone_kind(zone.kind()) {
                    if filter.matches(o, state, source_controller) {
                        out.push(TargetChoice::Object(o.id));
                    }
                }
            }
            TargetFilter::Custom(f) => {
                for o in state.objects.iter() {
                    if f(o, state) {
                        out.push(TargetChoice::Object(o.id));
                    }
                }
            }
        }
        out
    }
}

// =============================================================================
// ObjectFilter — structured predicate over a GameObject
// =============================================================================

/// Generic filter over game objects. Every field is a conjunct: all set
/// fields must match. A freshly-`Default`ed filter matches every object.
///
/// Subtypes are stored as `SmallString` (interner handles). Filters must
/// be built with the same interner as the objects they're applied to;
/// `SmallString` equality across interners is meaningless.
// TODO(serialize): `ObjectFilter.custom` is an `Option<fn>` pointer.
// Migrate per Section 12 in Phase 3.
#[derive(Clone, Debug, Default)]
pub struct ObjectFilter {
    /// Every bit in this `TypeLine` must be set on the object (AND).
    pub types: Option<TypeLine>,
    /// No bit in this `TypeLine` may be set on the object.
    pub not_types: Option<TypeLine>,
    /// Every color in this set must be in the object's colors (AND).
    /// Use [`Self::exact_colors`] if you need strict equality instead.
    pub colors: Option<ColorSet>,
    /// Every subtype here must be on the object.
    pub subtypes: Option<Vec<SmallString>>,
    pub controller: Option<ControllerConstraint>,
    pub cmc_condition: Option<CmcCondition>,
    pub power_condition: Option<PtCondition>,
    pub toughness_condition: Option<PtCondition>,
    pub name: Option<SmallString>,
    pub is_token: Option<bool>,
    pub has_counter: Option<CounterKind>,
    pub custom: Option<fn(&GameObject, &GameState) -> bool>,
}

impl ObjectFilter {
    pub fn new() -> Self { Self::default() }

    /// Shorthand: "a creature". The spec-preferred builder entry point.
    pub fn creature() -> Self {
        Self { types: Some(TypeLine::CREATURE.into()), ..Self::default() }
    }

    /// Shorthand: "a permanent of any type".
    pub fn permanent() -> Self { Self::default() }

    /// Builder: require type bits.
    pub fn with_types(mut self, tl: TypeLine) -> Self {
        self.types = Some(tl);
        self
    }

    /// Builder: exclude type bits.
    pub fn without_types(mut self, tl: TypeLine) -> Self {
        self.not_types = Some(tl);
        self
    }

    /// Builder: require a controller constraint.
    pub fn controlled_by(mut self, c: ControllerConstraint) -> Self {
        self.controller = Some(c);
        self
    }

    /// Builder: require a color mask (all listed colors must be present).
    pub fn with_colors(mut self, colors: ColorSet) -> Self {
        self.colors = Some(colors);
        self
    }

    /// Does `obj` match this filter?
    ///
    /// `source_controller` disambiguates [`ControllerConstraint::You`] and
    /// [`ControllerConstraint::Opponent`]: "you" means
    /// `obj.controller == source_controller`.
    pub fn matches(
        &self,
        obj: &GameObject,
        state: &GameState,
        source_controller: PlayerId,
    ) -> bool {
        // --- type bits: all required, none forbidden ---
        if let Some(required) = self.types {
            if (obj.characteristics.types.0 & required.0) != required.0 {
                return false;
            }
        }
        if let Some(forbidden) = self.not_types {
            if obj.characteristics.types.0 & forbidden.0 != 0 {
                return false;
            }
        }

        // --- colors: all colors in the filter must be in the object ---
        if let Some(required) = self.colors {
            if (obj.characteristics.colors.0 & required.0) != required.0 {
                return false;
            }
        }

        // --- subtypes: all required subtypes must be present ---
        if let Some(subs) = &self.subtypes {
            for s in subs {
                if !obj.characteristics.subtypes.contains(*s) {
                    return false;
                }
            }
        }

        // --- controller ---
        if let Some(ctrl) = &self.controller {
            if !ctrl.matches(obj.controller, source_controller) {
                return false;
            }
        }

        // --- CMC ---
        if let Some(cond) = &self.cmc_condition {
            if !cond.matches(obj.characteristics.mana_value()) {
                return false;
            }
        }

        // --- power / toughness (raw; layer system will replace later) ---
        if let Some(cond) = &self.power_condition {
            match obj.raw_power_with_counters(None) {
                Some(p) if cond.matches(p) => {}
                _ => return false,
            }
        }
        if let Some(cond) = &self.toughness_condition {
            match obj.raw_toughness_with_counters(None) {
                Some(t) if cond.matches(t) => {}
                _ => return false,
            }
        }

        // --- name ---
        if let Some(name) = self.name {
            if obj.characteristics.name != name {
                return false;
            }
        }

        // --- is_token: TODO(tokens) — GameObject doesn't track token
        // status yet. When tokens land, check obj.is_token here. For now
        // the field is accepted but not enforced.
        let _ = self.is_token;

        // --- has_counter ---
        if let Some(kind) = &self.has_counter {
            if !obj.has_counter(kind.clone()) {
                return false;
            }
        }

        // --- custom escape hatch ---
        if let Some(f) = self.custom {
            if !f(obj, state) {
                return false;
            }
        }

        true
    }
}

// =============================================================================
// Controller / numeric constraints
// =============================================================================

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ControllerConstraint {
    You,
    Opponent,
    Any,
    Player(PlayerId),
}

impl ControllerConstraint {
    /// Is `obj_controller` permitted by this constraint, given that the
    /// source of the filter is controlled by `source_controller`?
    pub fn matches(&self, obj_controller: PlayerId, source_controller: PlayerId) -> bool {
        match self {
            Self::You        => obj_controller == source_controller,
            Self::Opponent   => obj_controller != source_controller,
            Self::Any        => true,
            Self::Player(p)  => obj_controller == *p,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CmcCondition {
    Eq(u32), Le(u32), Ge(u32), Lt(u32), Gt(u32),
}

impl CmcCondition {
    pub fn matches(&self, cmc: u32) -> bool {
        match *self {
            Self::Eq(n) => cmc == n,
            Self::Le(n) => cmc <= n,
            Self::Ge(n) => cmc >= n,
            Self::Lt(n) => cmc <  n,
            Self::Gt(n) => cmc >  n,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PtCondition {
    Eq(i32), Le(i32), Ge(i32), Lt(i32), Gt(i32),
}

impl PtCondition {
    pub fn matches(&self, pt: i32) -> bool {
        match *self {
            Self::Eq(n) => pt == n,
            Self::Le(n) => pt <= n,
            Self::Ge(n) => pt >= n,
            Self::Lt(n) => pt <  n,
            Self::Gt(n) => pt >  n,
        }
    }
}

// =============================================================================
// CR 608.2b — target recheck at resolution
// =============================================================================

/// Outcome of rechecking a single target at resolution time.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TargetLegality {
    /// Still a legal target — apply effects normally.
    Legal,
    /// No longer legal (moved zone, gained hexproof, lost creature type,
    /// etc.). Per CR 608.2b, the effect is not applied to this target.
    Illegal,
}

/// Per CR 608.2b: when a spell or ability begins to resolve, recheck
/// each target. Returns one [`TargetLegality`] per entry in `selection`,
/// in the same order.
///
/// The caller (resolution pipeline in stack.rs, Task #11) uses this to:
/// - Apply each effect only to targets still marked `Legal`.
/// - If *every* target is `Illegal` *and* the spell or ability had at
///   least one target, the spell/ability fails to resolve and is
///   countered by the rules (CR 608.2b, last sentence).
///
/// `x_value` is required when the spell had an `X` in its targeting
/// count; it's ignored otherwise. The count itself is *not* rechecked
/// here — CR 608.2b concerns individual target legality.
pub fn validate_targets_on_resolution(
    requirement: &TargetRequirement,
    selection: &TargetSelection,
    state: &GameState,
    source_controller: PlayerId,
) -> Vec<TargetLegality> {
    selection.targets.iter().map(|c| {
        if requirement.matches_choice(c, state, source_controller) {
            TargetLegality::Legal
        } else {
            TargetLegality::Illegal
        }
    }).collect()
}

/// Convenience: `true` if *all* chosen targets are still legal.
pub fn all_targets_still_legal(
    requirement: &TargetRequirement,
    selection: &TargetSelection,
    state: &GameState,
    source_controller: PlayerId,
) -> bool {
    validate_targets_on_resolution(requirement, selection, state, source_controller)
        .iter()
        .all(|l| *l == TargetLegality::Legal)
}

/// Convenience: `true` if the spell/ability should be countered by CR
/// 608.2b — i.e. it had at least one chosen target and none remain
/// legal. A clause with zero chosen targets (some "up to" clauses) is
/// not counter-worthy.
pub fn should_counter_due_to_illegal_targets(
    requirement: &TargetRequirement,
    selection: &TargetSelection,
    state: &GameState,
    source_controller: PlayerId,
) -> bool {
    if selection.is_empty() {
        return false;
    }
    validate_targets_on_resolution(requirement, selection, state, source_controller)
        .iter()
        .all(|l| *l == TargetLegality::Illegal)
}

// =============================================================================
// TargetChoice utility
// =============================================================================

impl TargetChoice {
    /// The `ObjectId` this choice refers to, if any. Player-only choices
    /// return `None`.
    pub fn object_id(&self) -> Option<ObjectId> {
        match self {
            Self::Object(id) => Some(*id),
            Self::ObjectOrPlayer(ObjectOrPlayer::Object(id)) => Some(*id),
            Self::Player(_)
            | Self::ObjectOrPlayer(ObjectOrPlayer::Player(_)) => None,
        }
    }

    /// The `PlayerId` this choice refers to, if any.
    pub fn player_id(&self) -> Option<PlayerId> {
        match self {
            Self::Player(p) => Some(*p),
            Self::ObjectOrPlayer(ObjectOrPlayer::Player(p)) => Some(*p),
            Self::Object(_)
            | Self::ObjectOrPlayer(ObjectOrPlayer::Object(_)) => None,
        }
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mana::ManaCost;
    use crate::objects::{Characteristics, GameObject};
    use crate::state::GameState;

    fn creature_chars(p: i32, t: i32) -> Characteristics {
        Characteristics {
            mana_cost: Some(ManaCost::parse("{1}{G}").unwrap()),
            colors: ColorSet::green(),
            types: TypeLine::CREATURE.into(),
            power: Some(PtValue::Fixed(p)),
            toughness: Some(PtValue::Fixed(t)),
            ..Default::default()
        }
    }

    fn put_creature(
        state: &mut GameState,
        owner: PlayerId,
        controller: PlayerId,
        zone: Zone,
        p: i32,
        t: i32,
    ) -> ObjectId {
        let id = state.allocate_object_id();
        let mut obj = GameObject::new(id, owner, zone, /*card_id=*/ 1, creature_chars(p, t));
        obj.controller = controller;
        state.objects.insert(obj);
        id
    }

    fn put_sorcery(state: &mut GameState, owner: PlayerId, zone: Zone) -> ObjectId {
        let id = state.allocate_object_id();
        let chars = Characteristics {
            mana_cost: Some(ManaCost::parse("{2}{R}").unwrap()),
            colors: ColorSet::red(),
            types: TypeLine::SORCERY.into(),
            ..Default::default()
        };
        state.objects.insert(GameObject::new(id, owner, zone, /*card_id=*/ 2, chars));
        id
    }

    fn put_planeswalker(state: &mut GameState, controller: PlayerId) -> ObjectId {
        let id = state.allocate_object_id();
        let chars = Characteristics {
            types: TypeLine::PLANESWALKER.into(),
            loyalty: Some(3),
            ..Default::default()
        };
        let mut obj = GameObject::new(id, controller, Zone::Battlefield, 3, chars);
        obj.controller = controller;
        state.objects.insert(obj);
        id
    }

    // --- CmcCondition / PtCondition ------------------------------------------

    #[test]
    fn cmc_condition_matches_each_variant() {
        assert!(CmcCondition::Eq(3).matches(3));
        assert!(!CmcCondition::Eq(3).matches(4));
        assert!(CmcCondition::Le(3).matches(3));
        assert!(CmcCondition::Le(3).matches(0));
        assert!(!CmcCondition::Le(3).matches(4));
        assert!(CmcCondition::Ge(3).matches(3));
        assert!(!CmcCondition::Ge(3).matches(2));
        assert!(CmcCondition::Lt(3).matches(2));
        assert!(!CmcCondition::Lt(3).matches(3));
        assert!(CmcCondition::Gt(3).matches(4));
        assert!(!CmcCondition::Gt(3).matches(3));
    }

    #[test]
    fn pt_condition_handles_negative() {
        assert!(PtCondition::Le(0).matches(-1));
        assert!(PtCondition::Le(0).matches(0));
        assert!(!PtCondition::Le(0).matches(1));
        assert!(PtCondition::Gt(-1).matches(0));
    }

    // --- ControllerConstraint ------------------------------------------------

    #[test]
    fn controller_you_and_opponent() {
        assert!(ControllerConstraint::You.matches(0, 0));
        assert!(!ControllerConstraint::You.matches(1, 0));
        assert!(ControllerConstraint::Opponent.matches(1, 0));
        assert!(!ControllerConstraint::Opponent.matches(0, 0));
    }

    #[test]
    fn controller_any_and_specific_player() {
        assert!(ControllerConstraint::Any.matches(0, 0));
        assert!(ControllerConstraint::Any.matches(1, 0));
        assert!(ControllerConstraint::Player(2).matches(2, 0));
        assert!(!ControllerConstraint::Player(2).matches(1, 0));
    }

    // --- TargetCount ---------------------------------------------------------

    #[test]
    fn target_count_exactly_and_up_to() {
        assert!(TargetCount::Exactly(2).is_valid_count(2, None));
        assert!(!TargetCount::Exactly(2).is_valid_count(1, None));
        assert!(TargetCount::UpTo(3).is_valid_count(0, None));
        assert!(TargetCount::UpTo(3).is_valid_count(3, None));
        assert!(!TargetCount::UpTo(3).is_valid_count(4, None));
        assert!(TargetCount::Any.is_valid_count(99, None));
    }

    #[test]
    fn target_count_x_requires_x_value() {
        assert!(!TargetCount::X.is_valid_count(3, None));
        assert!(TargetCount::X.is_valid_count(3, Some(3)));
        assert!(!TargetCount::X.is_valid_count(3, Some(2)));
    }

    // --- ObjectFilter.matches ------------------------------------------------

    #[test]
    fn object_filter_default_matches_everything() {
        let mut s = GameState::new(2, 0);
        let id = put_creature(&mut s, 0, 0, Zone::Battlefield, 2, 2);
        let obj = s.objects.get(id).unwrap();
        let f = ObjectFilter::new();
        assert!(f.matches(obj, &s, 0));
    }

    #[test]
    fn object_filter_types_required_bits() {
        let mut s = GameState::new(2, 0);
        let c_id = put_creature(&mut s, 0, 0, Zone::Battlefield, 2, 2);
        let p_id = put_planeswalker(&mut s, 0);

        let creature_only = ObjectFilter::creature();
        assert!(creature_only.matches(s.objects.get(c_id).unwrap(), &s, 0));
        assert!(!creature_only.matches(s.objects.get(p_id).unwrap(), &s, 0));
    }

    #[test]
    fn object_filter_not_types_excludes() {
        let mut s = GameState::new(2, 0);
        let c_id = put_creature(&mut s, 0, 0, Zone::Battlefield, 2, 2);

        // "noncreature" — exclude CREATURE bit
        let f = ObjectFilter::new().without_types(TypeLine::CREATURE.into());
        assert!(!f.matches(s.objects.get(c_id).unwrap(), &s, 0));
    }

    #[test]
    fn object_filter_colors_requires_all() {
        let mut s = GameState::new(2, 0);
        let id = put_creature(&mut s, 0, 0, Zone::Battlefield, 2, 2);
        // creature is green
        let green = ObjectFilter::new().with_colors(ColorSet::green());
        let red   = ObjectFilter::new().with_colors(ColorSet::red());
        assert!( green.matches(s.objects.get(id).unwrap(), &s, 0));
        assert!(!red.matches(s.objects.get(id).unwrap(), &s, 0));
    }

    #[test]
    fn object_filter_controller_you_vs_opponent() {
        let mut s = GameState::new(2, 0);
        let mine   = put_creature(&mut s, 0, 0, Zone::Battlefield, 2, 2);
        let theirs = put_creature(&mut s, 1, 1, Zone::Battlefield, 2, 2);

        let you = ObjectFilter::new().controlled_by(ControllerConstraint::You);
        let opp = ObjectFilter::new().controlled_by(ControllerConstraint::Opponent);

        assert!( you.matches(s.objects.get(mine).unwrap(),   &s, 0));
        assert!(!you.matches(s.objects.get(theirs).unwrap(), &s, 0));
        assert!(!opp.matches(s.objects.get(mine).unwrap(),   &s, 0));
        assert!( opp.matches(s.objects.get(theirs).unwrap(), &s, 0));
    }

    #[test]
    fn object_filter_cmc_condition() {
        let mut s = GameState::new(2, 0);
        let id = put_creature(&mut s, 0, 0, Zone::Battlefield, 2, 2);
        // creature has CMC 2 from {1}{G}

        let le2 = ObjectFilter { cmc_condition: Some(CmcCondition::Le(2)), ..Default::default() };
        let eq3 = ObjectFilter { cmc_condition: Some(CmcCondition::Eq(3)), ..Default::default() };
        assert!( le2.matches(s.objects.get(id).unwrap(), &s, 0));
        assert!(!eq3.matches(s.objects.get(id).unwrap(), &s, 0));
    }

    #[test]
    fn object_filter_power_toughness_conditions() {
        let mut s = GameState::new(2, 0);
        let c = put_creature(&mut s, 0, 0, Zone::Battlefield, 3, 4);

        let p_ge3 = ObjectFilter {
            power_condition: Some(PtCondition::Ge(3)),
            ..Default::default()
        };
        let t_lt3 = ObjectFilter {
            toughness_condition: Some(PtCondition::Lt(3)),
            ..Default::default()
        };
        assert!( p_ge3.matches(s.objects.get(c).unwrap(), &s, 0));
        assert!(!t_lt3.matches(s.objects.get(c).unwrap(), &s, 0));
    }

    #[test]
    fn object_filter_pt_fails_on_non_creature() {
        // A sorcery has no power; a power condition should reject it.
        let mut s = GameState::new(2, 0);
        let id = put_sorcery(&mut s, 0, Zone::Hand(0));

        let f = ObjectFilter {
            power_condition: Some(PtCondition::Ge(0)),
            ..Default::default()
        };
        assert!(!f.matches(s.objects.get(id).unwrap(), &s, 0));
    }

    #[test]
    fn object_filter_has_counter() {
        let mut s = GameState::new(2, 0);
        let id = put_creature(&mut s, 0, 0, Zone::Battlefield, 2, 2);
        s.objects.get_mut(id).unwrap()
            .add_counters(CounterKind::PlusOnePlusOne, 1);

        let with = ObjectFilter {
            has_counter: Some(CounterKind::PlusOnePlusOne),
            ..Default::default()
        };
        let with_other = ObjectFilter {
            has_counter: Some(CounterKind::Loyalty),
            ..Default::default()
        };
        assert!( with.matches(s.objects.get(id).unwrap(), &s, 0));
        assert!(!with_other.matches(s.objects.get(id).unwrap(), &s, 0));
    }

    #[test]
    fn object_filter_custom_predicate() {
        fn is_even_id(o: &GameObject, _: &GameState) -> bool { o.id % 2 == 0 }
        let mut s = GameState::new(2, 0);
        let odd_id  = put_creature(&mut s, 0, 0, Zone::Battlefield, 1, 1);
        // force an even id
        let even_id = put_creature(&mut s, 0, 0, Zone::Battlefield, 1, 1);
        assert!(odd_id % 2 == 1 && even_id % 2 == 0,
            "test setup expects ids 1 and 2");

        let f = ObjectFilter { custom: Some(is_even_id), ..Default::default() };
        assert!(!f.matches(s.objects.get(odd_id).unwrap(),  &s, 0));
        assert!( f.matches(s.objects.get(even_id).unwrap(), &s, 0));
    }

    // --- TargetFilter.matches ------------------------------------------------

    #[test]
    fn target_filter_creature_only_legal_for_battlefield_creatures() {
        let mut s = GameState::new(2, 0);
        let on_bf = put_creature(&mut s, 0, 0, Zone::Battlefield, 2, 2);
        let in_gy = put_creature(&mut s, 0, 0, Zone::Graveyard(0), 2, 2);

        let f = TargetFilter::Creature;
        assert!( f.matches(&TargetChoice::Object(on_bf), &s, 0));
        assert!(!f.matches(&TargetChoice::Object(in_gy), &s, 0));
    }

    #[test]
    fn target_filter_creature_rejects_player_choice() {
        let s = GameState::new(2, 0);
        let f = TargetFilter::Creature;
        assert!(!f.matches(&TargetChoice::Player(0), &s, 0));
    }

    #[test]
    fn target_filter_player_rejects_out_of_range_or_dead() {
        let mut s = GameState::new(2, 0);
        assert!(TargetFilter::Player.matches(&TargetChoice::Player(0), &s, 0));

        s.player_mut(1).has_lost = true;
        assert!(!TargetFilter::Player.matches(&TargetChoice::Player(1), &s, 0));
        // Out-of-range
        assert!(!TargetFilter::Player.matches(&TargetChoice::Player(9), &s, 0));
    }

    #[test]
    fn target_filter_creature_or_player_accepts_either() {
        let mut s = GameState::new(2, 0);
        let c = put_creature(&mut s, 0, 0, Zone::Battlefield, 2, 2);
        let f = TargetFilter::CreatureOrPlayer;

        assert!(f.matches(&TargetChoice::ObjectOrPlayer(
            ObjectOrPlayer::Object(c)), &s, 0));
        assert!(f.matches(&TargetChoice::ObjectOrPlayer(
            ObjectOrPlayer::Player(1)), &s, 0));
    }

    #[test]
    fn target_filter_any_target_includes_planeswalkers() {
        let mut s = GameState::new(2, 0);
        let pw = put_planeswalker(&mut s, 0);

        assert!(TargetFilter::AnyTarget.matches(
            &TargetChoice::ObjectOrPlayer(ObjectOrPlayer::Object(pw)), &s, 0));
        // Creature-or-player rejects the bare planeswalker
        assert!(!TargetFilter::CreatureOrPlayer.matches(
            &TargetChoice::ObjectOrPlayer(ObjectOrPlayer::Object(pw)), &s, 0));
    }

    #[test]
    fn target_filter_spell_requires_stack_zone() {
        let mut s = GameState::new(2, 0);
        let in_hand  = put_sorcery(&mut s, 0, Zone::Hand(0));
        let on_stack = put_sorcery(&mut s, 0, Zone::Stack);

        let f = TargetFilter::Spell(ObjectFilter::new());
        assert!(!f.matches(&TargetChoice::Object(in_hand),  &s, 0));
        assert!( f.matches(&TargetChoice::Object(on_stack), &s, 0));
    }

    #[test]
    fn target_filter_card_in_graveyard_any_owner() {
        let mut s = GameState::new(2, 0);
        let mine   = put_creature(&mut s, 0, 0, Zone::Graveyard(0), 2, 2);
        let theirs = put_creature(&mut s, 1, 1, Zone::Graveyard(1), 2, 2);
        let on_bf  = put_creature(&mut s, 0, 0, Zone::Battlefield, 2, 2);

        // "target creature card in a graveyard"
        let f = TargetFilter::Card {
            zone: Zone::Graveyard(0), // any owner thanks to same_kind
            filter: ObjectFilter::creature(),
        };
        assert!( f.matches(&TargetChoice::Object(mine),   &s, 0));
        assert!( f.matches(&TargetChoice::Object(theirs), &s, 0));
        assert!(!f.matches(&TargetChoice::Object(on_bf),  &s, 0));
    }

    // --- TargetFilter.enumerate_legal ---------------------------------------

    #[test]
    fn enumerate_legal_creature_battlefield_only() {
        let mut s = GameState::new(2, 0);
        put_creature(&mut s, 0, 0, Zone::Battlefield, 1, 1);
        put_creature(&mut s, 0, 0, Zone::Battlefield, 2, 2);
        put_creature(&mut s, 0, 0, Zone::Graveyard(0), 3, 3);

        let legals = TargetFilter::Creature.enumerate_legal(&s, 0);
        assert_eq!(legals.len(), 2);
    }

    #[test]
    fn enumerate_legal_players_excludes_dead() {
        let mut s = GameState::new(3, 0);
        s.player_mut(1).has_lost = true;
        let legals = TargetFilter::Player.enumerate_legal(&s, 0);
        let ids: Vec<_> = legals.iter().filter_map(|c| c.player_id()).collect();
        assert_eq!(ids, vec![0, 2]);
    }

    // --- TargetRequirement --------------------------------------------------

    /// Ward does NOT affect target legality (CR 702.21a makes it a
    /// triggered ability). The Ward prompt is resolved at spell
    /// resolution in [`crate::engine::begin_ward_check`]. End-to-end
    /// pay-vs-decline tests live in
    /// `engine::resolution_choice_framework_tests::ward_*`.
    #[test]
    fn ward_does_not_block_targeting() {
        use crate::effects::KeywordAbility;
        use crate::mana::ManaCost;
        let mut s = GameState::new(2, 0);
        let theirs = put_creature(&mut s, 1, 1, Zone::Battlefield, 2, 2);
        s.objects.get_mut(theirs).unwrap().characteristics.keywords
            .push(KeywordAbility::Ward(ManaCost::parse("{2}").unwrap()));

        let req = TargetRequirement::target_creature();
        assert!(req.matches_choice(&TargetChoice::Object(theirs), &s, 0),
            "Ward must not short-circuit targeting — spell can be cast, \
             Ward fires at resolution");
        assert!(req.matches_choice(&TargetChoice::Object(theirs), &s, 1));
    }

    #[test]
    fn hexproof_blocks_opponent_targeting() {
        use crate::effects::KeywordAbility;
        let mut s = GameState::new(2, 0);
        let theirs = put_creature(&mut s, 1, 1, Zone::Battlefield, 2, 2);
        s.objects.get_mut(theirs).unwrap().characteristics.keywords
            .push(KeywordAbility::Hexproof);

        let req = TargetRequirement::target_creature();
        // From player 0's spell (opponent): rejected.
        assert!(!req.matches_choice(&TargetChoice::Object(theirs), &s, 0));
        // From the creature's own controller (player 1): still OK.
        assert!(req.matches_choice(&TargetChoice::Object(theirs), &s, 1));
    }

    #[test]
    fn target_requirement_outer_controller_overrides() {
        // "target creature you control" using the outer controller field.
        let mut s = GameState::new(2, 0);
        let mine   = put_creature(&mut s, 0, 0, Zone::Battlefield, 2, 2);
        let theirs = put_creature(&mut s, 1, 1, Zone::Battlefield, 2, 2);

        let req = TargetRequirement {
            filter: TargetFilter::Creature,
            count: TargetCount::Exactly(1),
            controller: Some(ControllerConstraint::You),
        };
        assert!( req.matches_choice(&TargetChoice::Object(mine),   &s, 0));
        assert!(!req.matches_choice(&TargetChoice::Object(theirs), &s, 0));
    }

    #[test]
    fn target_requirement_is_satisfied_count_and_filter() {
        let mut s = GameState::new(2, 0);
        let a = put_creature(&mut s, 0, 0, Zone::Battlefield, 2, 2);
        let b = put_creature(&mut s, 0, 0, Zone::Battlefield, 2, 2);

        let req = TargetRequirement::target_creature();
        let ok = TargetSelection { targets: vec![TargetChoice::Object(a)] };
        let wrong_count = TargetSelection {
            targets: vec![TargetChoice::Object(a), TargetChoice::Object(b)],
        };

        assert!(req.is_satisfied(&ok, &s, 0, None));
        assert!(!req.is_satisfied(&wrong_count, &s, 0, None));
    }

    #[test]
    fn target_requirement_x_count_matches_x_value() {
        let mut s = GameState::new(2, 0);
        let a = put_creature(&mut s, 0, 0, Zone::Battlefield, 2, 2);
        let b = put_creature(&mut s, 0, 0, Zone::Battlefield, 2, 2);

        // "X target creatures" with X=2
        let req = TargetRequirement {
            filter: TargetFilter::Creature,
            count: TargetCount::X,
            controller: None,
        };
        let sel = TargetSelection {
            targets: vec![TargetChoice::Object(a), TargetChoice::Object(b)],
        };
        assert!(req.is_satisfied(&sel, &s, 0, Some(2)));
        assert!(!req.is_satisfied(&sel, &s, 0, Some(1)));
    }

    // --- CR 608.2b: target recheck at resolution ----------------------------

    #[test]
    fn resolution_recheck_all_legal_returns_all_legal() {
        let mut s = GameState::new(2, 0);
        let c = put_creature(&mut s, 0, 0, Zone::Battlefield, 2, 2);
        let req = TargetRequirement::target_creature();
        let sel = TargetSelection { targets: vec![TargetChoice::Object(c)] };

        let legals = validate_targets_on_resolution(&req, &sel, &s, 0);
        assert_eq!(legals, vec![TargetLegality::Legal]);
        assert!(all_targets_still_legal(&req, &sel, &s, 0));
        assert!(!should_counter_due_to_illegal_targets(&req, &sel, &s, 0));
    }

    #[test]
    fn resolution_recheck_target_leaves_battlefield() {
        // Classic scenario: Lightning Bolt the creature, it gets blinked
        // with Ephemerate in response. By resolution time the target is
        // a different object at a different zone — illegal.
        let mut s = GameState::new(2, 0);
        let c = put_creature(&mut s, 0, 0, Zone::Battlefield, 2, 2);
        let req = TargetRequirement::target_creature();
        let sel = TargetSelection { targets: vec![TargetChoice::Object(c)] };

        // Move it to exile mid-resolution.
        s.objects.get_mut(c).unwrap().zone = Zone::Exile;

        let legals = validate_targets_on_resolution(&req, &sel, &s, 0);
        assert_eq!(legals, vec![TargetLegality::Illegal]);
        assert!(should_counter_due_to_illegal_targets(&req, &sel, &s, 0));
    }

    #[test]
    fn resolution_recheck_mixed_legal_and_illegal() {
        // A multi-target spell where some targets are still legal.
        let mut s = GameState::new(2, 0);
        let a = put_creature(&mut s, 0, 0, Zone::Battlefield, 2, 2);
        let b = put_creature(&mut s, 0, 0, Zone::Battlefield, 2, 2);

        let req = TargetRequirement {
            filter: TargetFilter::Creature,
            count: TargetCount::Exactly(2),
            controller: None,
        };
        let sel = TargetSelection {
            targets: vec![TargetChoice::Object(a), TargetChoice::Object(b)],
        };

        // Only `b` becomes illegal.
        s.objects.get_mut(b).unwrap().zone = Zone::Graveyard(0);
        let legals = validate_targets_on_resolution(&req, &sel, &s, 0);
        assert_eq!(legals, vec![TargetLegality::Legal, TargetLegality::Illegal]);

        // With at least one legal target remaining, the spell resolves
        // (it only skips the illegal ones).
        assert!(!all_targets_still_legal(&req, &sel, &s, 0));
        assert!(!should_counter_due_to_illegal_targets(&req, &sel, &s, 0));
    }

    #[test]
    fn resolution_recheck_empty_selection_is_not_counter() {
        // "Up to N" clauses with zero chosen targets — CR 608.2b counter
        // rule applies only when there was at least one target chosen.
        let s = GameState::new(2, 0);
        let req = TargetRequirement {
            filter: TargetFilter::Creature,
            count: TargetCount::UpTo(3),
            controller: None,
        };
        let empty = TargetSelection::new();
        assert!(!should_counter_due_to_illegal_targets(&req, &empty, &s, 0));
    }

    // --- TargetChoice utility -----------------------------------------------

    #[test]
    fn target_choice_object_and_player_accessors() {
        assert_eq!(TargetChoice::Object(7).object_id(), Some(7));
        assert_eq!(TargetChoice::Object(7).player_id(), None);
        assert_eq!(TargetChoice::Player(1).object_id(), None);
        assert_eq!(TargetChoice::Player(1).player_id(), Some(1));

        let oop = TargetChoice::ObjectOrPlayer(ObjectOrPlayer::Object(4));
        assert_eq!(oop.object_id(), Some(4));
        assert_eq!(oop.player_id(), None);

        let oop = TargetChoice::ObjectOrPlayer(ObjectOrPlayer::Player(2));
        assert_eq!(oop.object_id(), None);
        assert_eq!(oop.player_id(), Some(2));
    }

    // --- Serde roundtrip for the serializable types -------------------------

    #[test]
    fn target_selection_roundtrip() {
        let sel = TargetSelection {
            targets: vec![
                TargetChoice::Object(1),
                TargetChoice::Player(0),
                TargetChoice::ObjectOrPlayer(ObjectOrPlayer::Object(3)),
                TargetChoice::ObjectOrPlayer(ObjectOrPlayer::Player(1)),
            ],
        };
        let json = serde_json::to_string(&sel).unwrap();
        let back: TargetSelection = serde_json::from_str(&json).unwrap();
        assert_eq!(sel, back);
    }
}
