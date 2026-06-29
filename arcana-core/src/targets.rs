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

use crate::effects::KeywordAbility;
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
        source: ObjectId,
        source_controller: PlayerId,
        x_value: Option<u32>,
    ) -> bool {
        if !self.count.is_valid_count(selection.len() as u32, x_value) {
            return false;
        }
        selection.targets.iter().all(|c|
            self.matches_choice(c, state, source, source_controller))
    }

    /// Legality of a single choice under this requirement (filter AND
    /// outer controller). Exposed so the legal-action enumerator can
    /// build up selections one target at a time.
    pub fn matches_choice(
        &self,
        choice: &TargetChoice,
        state: &GameState,
        source: ObjectId,
        source_controller: PlayerId,
    ) -> bool {
        if !self.filter.matches(choice, state, source, source_controller) {
            return false;
        }
        if let Some(ctrl) = &self.controller {
            // The constraint applies to the target's controller. For a PLAYER
            // target the player IS that controller (so "target opponent" can't
            // pick yourself — Soldevi Steam Beast); for an object target it's the
            // object's controller.
            let target_controller = choice.player_id().or_else(|| {
                choice.object_id()
                    .and_then(|id| state.objects.get(id))
                    .map(|obj| obj.controller)
            });
            if let Some(pc) = target_controller {
                if !ctrl.matches(pc, source_controller) {
                    return false;
                }
            }
        }
        // CR 702.11b — Hexproof: can't be the target of spells or
        // abilities your opponents control.
        //
        // CR 702.21a — Ward: target legality is NOT affected. Ward is a
        // triggered ability that fires on being targeted; the caster
        // pays or the spell is countered when the Ward trigger
        // resolves. Synthesized as [`engine::WARD_TRIGGER_ID`] from
        // [`GameEvent::BecomesTarget`] events emitted by
        // [`engine::apply_cast_spell`] and
        // [`engine::apply_activate_ability`].
        if let Some(id) = choice.object_id() {
            if let Some(obj) = state.objects.get(id) {
                if obj.controller != source_controller
                    && state.has_keyword(id, &crate::effects::KeywordAbility::Hexproof)
                {
                    return false;
                }
                // CR 702.16e — Protection: can't be the target of a
                // spell/ability whose SOURCE matches the quality. NOT
                // controller-gated (unlike Hexproof) — protection from red
                // stops a red source whoever controls it. The `source`
                // ObjectId is the targeting spell/ability's source, so we
                // can do fine-grained color/subtype matching against its
                // characteristics (the prior Phase-1 stub only handled the
                // source-agnostic Protection::Everything shield).
                use crate::effects::{KeywordAbility, ProtectionQuality};
                if state.objects.get(source).is_some() {
                    if state.is_protected_target_of(id, source) {
                        return false;
                    }
                } else if state.effective_keywords(id).iter().any(|kw| matches!(kw,
                    KeywordAbility::Protection(ProtectionQuality::Everything)))
                {
                    // Source object unavailable (synthetic/no-source path):
                    // only the source-agnostic "everything" shield applies.
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
    /// "Counter target activated ability" / "target activated or
    /// triggered ability [from a noncreature source]" (Cursecatcher's
    /// ability-level kin: Rimewind Cryomancer, Emerald Dragon, Tale's
    /// End class). Ability STACK ENTRIES have no stack-zone
    /// `GameObject`, so [`Self::Spell`] can't see them — this variant
    /// matches `state.stack` entries by id instead. `activated` /
    /// `triggered` select which kinds qualify; `source_filter` (when
    /// set) is applied to the ability's SOURCE permanent ("from a
    /// noncreature source"). `Effect::Counter` already handles
    /// ability entries at resolution.
    AbilityOnStack {
        activated: bool,
        triggered: bool,
        source_filter: Option<ObjectFilter>,
    },
    /// "Target card in [zone]" — e.g. target card in a graveyard.
    Card { zone: Zone, filter: ObjectFilter },
    /// "Target creature blocking [this creature]" — source-relative
    /// combat pairing (Knight of Dusk, Godo's Irregulars, Flowstone
    /// Salamander). Read from [`GameState::combat`]'s blocker list
    /// against the targeting ability's SOURCE; matches nothing outside
    /// combat or when the source is unblocked.
    CreatureBlockingSource,
    /// "Target creature blocking or blocked by [this creature]"
    /// (Lesser Werewolf): the candidate blocks the source OR the
    /// source blocks the candidate.
    CreatureBlockingOrBlockedBySource,
    Custom(fn(&GameObject, &GameState) -> bool),
}

/// Does the combat pairing for [`TargetFilter::CreatureBlockingSource`]
/// / [`TargetFilter::CreatureBlockingOrBlockedBySource`] hold between
/// `candidate` and `source`?
fn blocking_pairing(
    state: &GameState,
    candidate: ObjectId,
    source: ObjectId,
    include_blocked_by: bool,
) -> bool {
    state.combat.as_ref().is_some_and(|c|
        c.blockers.iter().any(|b|
            (b.object_id == candidate && b.blocking == source)
                || (include_blocked_by
                    && b.object_id == source && b.blocking == candidate)))
}

impl TargetFilter {
    /// Is `choice` a legal target under this filter, given current state?
    ///
    /// Pure filter check — does not consult any outer controller
    /// constraint (see [`TargetRequirement::matches_choice`] for that).
    /// `source` is the targeting spell/ability's source object — the
    /// referent of source-relative filters ("creature blocking THIS
    /// creature"); pass [`crate::objects::NULL_OBJECT_ID`] when there
    /// is no meaningful source (those filters then match nothing).
    pub fn matches(
        &self,
        choice: &TargetChoice,
        state: &GameState,
        source: ObjectId,
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

            // --- Creature-or-player: bare Object/Player choices are
            // accepted too (the replacement pipeline and deterministic
            // agents construct those shapes directly) ---
            (TargetFilter::CreatureOrPlayer, TargetChoice::Player(p))
            | (TargetFilter::AnyTarget, TargetChoice::Player(p)) => {
                (*p as usize) < state.players.len()
                    && state.player(*p).is_alive()
            }
            (TargetFilter::CreatureOrPlayer, TargetChoice::Object(id))
            | (TargetFilter::AnyTarget, TargetChoice::Object(id)) => {
                state.objects.get(*id).is_some_and(|o|
                    o.zone.is_battlefield()
                        && (o.is_creature()
                            || (matches!(self, TargetFilter::AnyTarget)
                                && o.is_planeswalker())))
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

            // --- Ability on the stack (entry id, not an object) ---
            (TargetFilter::AbilityOnStack { activated, triggered, source_filter },
             TargetChoice::Object(id)) => {
                state.stack.iter().any(|e| e.id == *id
                    && match &e.kind {
                        crate::stack::StackEntryKind::ActivatedAbility { .. } =>
                            *activated,
                        crate::stack::StackEntryKind::TriggeredAbility { .. } =>
                            *triggered,
                        crate::stack::StackEntryKind::Spell { .. } => false,
                    }
                    && source_filter.as_ref().is_none_or(|f|
                        state.objects.get(e.source).is_some_and(|o|
                            f.matches(o, state, source_controller))))
            }

            // --- Card in a specified zone kind ---
            (TargetFilter::Card { zone, filter }, TargetChoice::Object(id)) => {
                state.objects.get(*id).is_some_and(|o|
                    o.zone.same_kind(*zone)
                        && filter.matches(o, state, source_controller))
            }

            // --- Source-relative combat pairings ---
            (TargetFilter::CreatureBlockingSource, TargetChoice::Object(id)) => {
                state.objects.get(*id).is_some_and(|o|
                    o.zone.is_battlefield() && o.is_creature())
                    && blocking_pairing(state, *id, source, false)
            }
            (TargetFilter::CreatureBlockingOrBlockedBySource, TargetChoice::Object(id)) => {
                state.objects.get(*id).is_some_and(|o|
                    o.zone.is_battlefield() && o.is_creature())
                    && blocking_pairing(state, *id, source, true)
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
        source: ObjectId,
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
            TargetFilter::AbilityOnStack { activated, triggered, source_filter } => {
                for e in &state.stack {
                    let kind_ok = match &e.kind {
                        crate::stack::StackEntryKind::ActivatedAbility { .. } =>
                            *activated,
                        crate::stack::StackEntryKind::TriggeredAbility { .. } =>
                            *triggered,
                        crate::stack::StackEntryKind::Spell { .. } => false,
                    };
                    let src_ok = source_filter.as_ref().is_none_or(|f|
                        state.objects.get(e.source).is_some_and(|o|
                            f.matches(o, state, source_controller)));
                    if kind_ok && src_ok {
                        out.push(TargetChoice::Object(e.id));
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
            TargetFilter::CreatureBlockingSource
            | TargetFilter::CreatureBlockingOrBlockedBySource => {
                let both = matches!(self,
                    TargetFilter::CreatureBlockingOrBlockedBySource);
                for o in state.objects.objects_in_zone(Zone::Battlefield) {
                    if o.is_creature() && blocking_pairing(state, o.id, source, both) {
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
    /// At least one bit in this `TypeLine` must be set on the object (OR).
    /// Use for "instant or sorcery", "creature or planeswalker", etc.,
    /// where the primary [`Self::types`] AND-mask can't express the
    /// disjunction. An empty (`0`) mask matches no object.
    pub types_any: Option<TypeLine>,
    /// No bit in this `TypeLine` may be set on the object.
    pub not_types: Option<TypeLine>,
    /// Every color in this set must be in the object's colors (AND).
    /// Use [`Self::exact_colors`] if you need strict equality instead.
    pub colors: Option<ColorSet>,
    /// No color in this set may be in the object's colors. "nonblack
    /// creature" = `not_colors: ColorSet::black()`. Colorless objects
    /// (no color bits) pass any `not_colors`.
    pub not_colors: Option<ColorSet>,
    /// Tap-state constraint: `Some(true)` = only tapped permanents,
    /// `Some(false)` = only untapped. `None` = either.
    pub tapped: Option<bool>,
    /// Every subtype here must be on the object.
    pub subtypes: Option<Vec<SmallString>>,
    /// At least one subtype here must be on the object (OR). Use for
    /// "Human or Warrior creature", "Spirit or Arcane spell" — where
    /// the AND-only [`Self::subtypes`] can't express the disjunction.
    /// An empty Vec matches no object.
    pub subtypes_any: Option<Vec<SmallString>>,
    /// No subtype here may be on the object. "non-Human creature" =
    /// one entry; Power Word Kill's "non-Angel, non-Demon, non-Devil,
    /// non-Dragon creature" pushes four.
    pub not_subtypes: Option<Vec<SmallString>>,
    /// Every supertype here must be set on the object (AND). Use the
    /// bit constants on [`SupertypeSet`]: `SupertypeSet::LEGENDARY`,
    /// `SupertypeSet::BASIC`, `SupertypeSet::SNOW`, `SupertypeSet::WORLD`.
    pub supertypes: Option<SupertypeSet>,
    /// No supertype bit here may be set on the object. "nonlegendary
    /// creature" = `not_supertypes: SupertypeSet(SupertypeSet::LEGENDARY)`.
    pub not_supertypes: Option<SupertypeSet>,
    /// Every keyword here must be on the object (AND). Layer-aware:
    /// checked via [`GameState::has_keyword`], so Layer-6 grants
    /// ("target creature gains flying") and removals ("loses all
    /// abilities") are respected. Payload keywords (`Ward`,
    /// `Landwalk`) match by full equality including the payload.
    pub keywords: Option<Vec<KeywordAbility>>,
    /// At least one keyword here must be on the object (OR). Use for
    /// "creature with deathtouch, hexproof, reach, or trample" — where
    /// the AND-only [`Self::keywords`] can't express the disjunction.
    /// An empty Vec matches no object.
    pub keywords_any: Option<Vec<KeywordAbility>>,
    /// No keyword here may be on the object. "creature without flying"
    /// = `not_keywords: Some(vec![KeywordAbility::Flying])`.
    pub not_keywords: Option<Vec<KeywordAbility>>,
    /// Combat-state constraint ("target attacking creature", "each
    /// blocking creature", "creature attacking you"). Read from
    /// [`GameState::combat`]; outside combat nothing is attacking or
    /// blocking, so every variant except `NotAttacking` matches no
    /// object.
    pub combat_status: Option<CombatStatusFilter>,
    pub controller: Option<ControllerConstraint>,
    pub cmc_condition: Option<CmcCondition>,
    pub power_condition: Option<PtCondition>,
    pub toughness_condition: Option<PtCondition>,
    /// Compare the object's own power vs its own toughness (CR 208) —
    /// e.g. "creatures with toughness greater than power".
    pub pt_compare: Option<PtCompare>,
    /// CR 712 — "a double-faced card" (a transforming DFC). Read from
    /// the object's seeded `back_face_characteristics` (set at
    /// instantiation for transform backs, preserved across re-ids), so
    /// no registry lookup is needed. `Some(true)` = transforming DFC
    /// only; `Some(false)` = single-faced only. MDFC/Adventure/Split
    /// don't carry a transform back, so they read as not double-faced.
    pub is_double_faced: Option<bool>,
    pub name: Option<SmallString>,
    pub is_token: Option<bool>,
    pub has_counter: Option<CounterKind>,
    /// `Some(true)` = only commanders (CR 903.3 designation —
    /// Background statics); `Some(false)` = only non-commanders.
    pub is_commander: Option<bool>,
    /// Match a predefined commodity token by kind (Treasure / Clue /
    /// Food / Powerstone / Incubator / Blood / Map). These artifact
    /// subtypes are token-only in real Magic and are minted engine-side
    /// without an interner, so they carry a [`CommodityToken`] marker
    /// (`GameObject::commodity`) rather than a subtype symbol. "sacrifice
    /// a Treasure" / "for each Clue you control" filters match on this.
    pub commodity: Option<crate::effects::CommodityToken>,
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

    /// Builder: require at least one of the given type bits (OR).
    /// E.g. `with_types_any(INSTANT | SORCERY)` for Young Pyromancer-
    /// style "instant or sorcery" triggers.
    pub fn with_types_any(mut self, tl: TypeLine) -> Self {
        self.types_any = Some(tl);
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

    // --- interner-free numeric / boolean refinements (Tier 1.2) -----
    // These let a generated resolver narrow a board-wide set without
    // touching the string interner, e.g. "destroy each creature with
    // mana value 3 or less", "each token", "each colorless creature".

    /// Builder: mana value ≤ `n`.
    pub fn with_max_cmc(mut self, n: u32) -> Self {
        self.cmc_condition = Some(CmcCondition::Le(n));
        self
    }
    /// Builder: mana value ≥ `n`.
    pub fn with_min_cmc(mut self, n: u32) -> Self {
        self.cmc_condition = Some(CmcCondition::Ge(n));
        self
    }
    /// Builder: mana value exactly `n`.
    pub fn with_exact_cmc(mut self, n: u32) -> Self {
        self.cmc_condition = Some(CmcCondition::Eq(n));
        self
    }
    /// Builder: power ≥ `n`.
    pub fn with_min_power(mut self, n: i32) -> Self {
        self.power_condition = Some(PtCondition::Ge(n));
        self
    }
    /// Builder: power ≤ `n`.
    pub fn with_max_power(mut self, n: i32) -> Self {
        self.power_condition = Some(PtCondition::Le(n));
        self
    }
    /// Builder: compare the object's own power vs toughness (CR 208) —
    /// e.g. `with_pt_compare(PtCompare::ToughnessGreater)` for "toughness
    /// greater than power".
    pub fn with_pt_compare(mut self, cmp: PtCompare) -> Self {
        self.pt_compare = Some(cmp);
        self
    }
    /// Builder: only transforming double-faced cards (CR 712).
    pub fn double_faced(mut self) -> Self {
        self.is_double_faced = Some(true);
        self
    }
    /// Builder: toughness ≤ `n`.
    pub fn with_max_toughness(mut self, n: i32) -> Self {
        self.toughness_condition = Some(PtCondition::Le(n));
        self
    }
    /// Builder: only tokens.
    pub fn tokens_only(mut self) -> Self {
        self.is_token = Some(true);
        self
    }
    /// Builder: only nontoken (card) permanents.
    pub fn nontoken(mut self) -> Self {
        self.is_token = Some(false);
        self
    }
    /// Builder: require an already-interned subtype symbol. Prefer
    /// [`crate::script::subtype_filter`] from a generated resolver —
    /// it resolves the symbol totally without interner access.
    pub fn with_subtype_sym(mut self, sym: SmallString) -> Self {
        self.subtypes.get_or_insert_with(Vec::new).push(sym);
        self
    }
    /// Builder: accept any of the given subtypes (OR). For
    /// "Human or Warrior creature" pass `vec![human_sym, warrior_sym]`.
    /// "Commander creatures" (CR 903.3 designation — Background
    /// statics). Inert in non-commander games (nothing carries the
    /// designation), which is the faithful reading.
    pub fn commander_only(mut self) -> Self {
        self.is_commander = Some(true);
        self
    }

    pub fn with_subtypes_any(mut self, syms: Vec<SmallString>) -> Self {
        self.subtypes_any = Some(syms);
        self
    }
    /// Builder: exclude a subtype (AND if called repeatedly).
    /// "non-Human creature" = `creature().without_subtype_sym(human)`;
    /// chain for "non-Angel, non-Demon, non-Devil, non-Dragon".
    pub fn without_subtype_sym(mut self, sym: SmallString) -> Self {
        self.not_subtypes.get_or_insert_with(Vec::new).push(sym);
        self
    }
    /// Builder: require supertype bits (AND). "Legendary creature" =
    /// `creature().with_supertypes(SupertypeSet(SupertypeSet::LEGENDARY))`.
    pub fn with_supertypes(mut self, st: SupertypeSet) -> Self {
        self.supertypes = Some(st);
        self
    }
    /// Builder: forbid supertype bits. "nonlegendary creature" =
    /// `creature().without_supertypes(SupertypeSet(SupertypeSet::LEGENDARY))`.
    pub fn without_supertypes(mut self, st: SupertypeSet) -> Self {
        self.not_supertypes = Some(st);
        self
    }
    /// Builder: exclude a color ("nonblack creature" =
    /// `creature().without_colors(ColorSet::black())`).
    pub fn without_colors(mut self, colors: ColorSet) -> Self {
        self.not_colors = Some(colors);
        self
    }
    /// Builder: require a keyword (AND if called repeatedly). "creature
    /// with flying" = `creature().with_keyword(KeywordAbility::Flying)`.
    /// Layer-aware — granted keywords count, removed ones don't.
    pub fn with_keyword(mut self, kw: KeywordAbility) -> Self {
        self.keywords.get_or_insert_with(Vec::new).push(kw);
        self
    }
    /// Builder: accept any of the given keywords (OR). For "creature
    /// with deathtouch, hexproof, reach, or trample" pass all four.
    pub fn with_keywords_any(mut self, kws: Vec<KeywordAbility>) -> Self {
        self.keywords_any = Some(kws);
        self
    }
    /// Builder: exclude a keyword ("creature without flying" =
    /// `creature().without_keyword(KeywordAbility::Flying)`).
    pub fn without_keyword(mut self, kw: KeywordAbility) -> Self {
        self.not_keywords.get_or_insert_with(Vec::new).push(kw);
        self
    }
    /// Builder: only attacking creatures ("target attacking creature").
    pub fn attacking_only(mut self) -> Self {
        self.combat_status = Some(CombatStatusFilter::Attacking);
        self
    }
    /// Builder: only blocking creatures ("target blocking creature").
    pub fn blocking_only(mut self) -> Self {
        self.combat_status = Some(CombatStatusFilter::Blocking);
        self
    }
    /// Builder: attacking or blocking ("target attacking or blocking
    /// creature").
    pub fn attacking_or_blocking_only(mut self) -> Self {
        self.combat_status = Some(CombatStatusFilter::AttackingOrBlocking);
        self
    }
    /// Builder: attacking the filter's source controller ("creature
    /// attacking you").
    pub fn attacking_you_only(mut self) -> Self {
        self.combat_status = Some(CombatStatusFilter::AttackingYou);
        self
    }
    /// Builder: not attacking ("nonattacking creature").
    pub fn nonattacking_only(mut self) -> Self {
        self.combat_status = Some(CombatStatusFilter::NotAttacking);
        self
    }
    /// Builder: only *blocked* attackers ("target blocked creature").
    pub fn blocked_only(mut self) -> Self {
        self.combat_status = Some(CombatStatusFilter::Blocked);
        self
    }
    /// Builder: match a commodity token of the given kind ("sacrifice a
    /// Treasure", "Food you control"). See [`Self::commodity`].
    pub fn commodity_kind(mut self, kind: crate::effects::CommodityToken) -> Self {
        self.commodity = Some(kind);
        self
    }
    /// Builder: only tapped permanents.
    pub fn tapped_only(mut self) -> Self {
        self.tapped = Some(true);
        self
    }
    /// Builder: only untapped permanents.
    pub fn untapped_only(mut self) -> Self {
        self.tapped = Some(false);
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
        self.matches_inner(obj, state, source_controller, false)
    }

    /// BASE-characteristics variant of [`Self::matches`]: subtype /
    /// color / keyword predicates read the object's printed
    /// characteristics instead of the layer-computed ones. REQUIRED
    /// for filters evaluated INSIDE the layer pipeline (the filtered
    /// global statics' `applies_to`) — a layer-aware read there would
    /// recurse into `compute_characteristics`.
    pub fn matches_base(
        &self,
        obj: &GameObject,
        state: &GameState,
        source_controller: PlayerId,
    ) -> bool {
        self.matches_inner(obj, state, source_controller, true)
    }

    fn matches_inner(
        &self,
        obj: &GameObject,
        state: &GameState,
        source_controller: PlayerId,
        base_only: bool,
    ) -> bool {
        // --- type bits: all required, at least one of any, none forbidden ---
        if let Some(required) = self.types {
            if (obj.characteristics.types.0 & required.0) != required.0 {
                return false;
            }
        }
        if let Some(any) = self.types_any {
            if obj.characteristics.types.0 & any.0 == 0 {
                return false;
            }
        }
        if let Some(forbidden) = self.not_types {
            if obj.characteristics.types.0 & forbidden.0 != 0 {
                return false;
            }
        }

        // --- colors: layer-aware when the object is in the state
        // (Layer-5 SetColor / AttachedCreatureAddColors count); base
        // characteristics otherwise. ---
        if self.colors.is_some() || self.not_colors.is_some() {
            let colors = if base_only { None } else {
                state.objects.get(obj.id)
                    .and_then(|_| state.compute_characteristics(obj.id))
                    .map(|c| c.colors)
            }.unwrap_or(obj.characteristics.colors);
            // All colors in the filter must be in the object.
            if let Some(required) = self.colors {
                if (colors.0 & required.0) != required.0 {
                    return false;
                }
            }
            // Color exclusion: none of these colors may be present.
            if let Some(excluded) = self.not_colors {
                if colors.0 & excluded.0 != 0 {
                    return false;
                }
            }
        }
        // --- tap state ---
        if let Some(want_tapped) = self.tapped {
            if obj.is_tapped() != want_tapped {
                return false;
            }
        }
        // --- commander designation (CR 903.3) — object-level flag,
        // not a layer property, so it reads the same in base mode ---
        if let Some(want_cmd) = self.is_commander {
            if obj.is_commander != want_cmd {
                return false;
            }
        }

        // --- subtypes: layer-aware when the object is in the state
        // (Layer-4 grants like AttachedCreatureAddSubtypes count for
        // tribal filters); base characteristics otherwise. Computed
        // once for all three predicate shapes. ---
        if self.subtypes.is_some() || self.subtypes_any.is_some()
            || self.not_subtypes.is_some()
        {
            let computed = if base_only { None } else {
                state.objects.get(obj.id)
                    .and_then(|_| state.compute_characteristics(obj.id))
            };
            // CR 702.73a — "is every creature type": the flagged
            // object satisfies any positive subtype requirement and
            // fails any subtype exclusion. (Filters' type constraints
            // keep land/Equipment subtype reads honest — see the
            // flag's doc on Characteristics.)
            let every = computed.as_ref()
                .map(|c| c.every_creature_type)
                .unwrap_or(obj.characteristics.every_creature_type);
            let computed_subs = computed.map(|c| c.subtypes);
            let subtypes = computed_subs.as_ref()
                .unwrap_or(&obj.characteristics.subtypes);
            // All required subtypes must be present.
            if let Some(subs) = &self.subtypes {
                if !every {
                    for s in subs {
                        if !subtypes.contains(*s) {
                            return false;
                        }
                    }
                }
            }
            // subtypes_any: at least one must be present (OR). Empty
            // Vec matches no object (consistent with types_any=0).
            if let Some(any) = &self.subtypes_any {
                if !every && !any.iter().any(|s| subtypes.contains(*s)) {
                    return false;
                }
            }
            // Subtype exclusion: none of these may be present — an
            // every-creature-type object IS each of them, so it fails.
            if let Some(excluded) = &self.not_subtypes {
                if every
                    || excluded.iter().any(|s| subtypes.contains(*s))
                {
                    return false;
                }
            }
        }
        // --- supertypes: every required supertype bit must be set ---
        if let Some(required) = self.supertypes {
            if (obj.characteristics.supertypes.0 & required.0) != required.0 {
                return false;
            }
        }
        // --- supertype exclusion: none of these bits may be set ---
        if let Some(forbidden) = self.not_supertypes {
            if obj.characteristics.supertypes.0 & forbidden.0 != 0 {
                return false;
            }
        }

        // --- keywords: layer-aware via the state when the object is
        // registered there (grants and removals respected); base
        // characteristics otherwise (e.g. a hypothetical object not
        // yet in the state's object table). ---
        if self.keywords.is_some() || self.keywords_any.is_some() || self.not_keywords.is_some() {
            let has = |kw: &KeywordAbility| -> bool {
                if !base_only && state.objects.get(obj.id).is_some() {
                    state.has_keyword(obj.id, kw)
                } else {
                    obj.characteristics.keywords.contains(kw)
                }
            };
            if let Some(kws) = &self.keywords {
                if !kws.iter().all(&has) {
                    return false;
                }
            }
            // Empty Vec matches no object (consistent with subtypes_any).
            if let Some(any) = &self.keywords_any {
                if !any.iter().any(&has) {
                    return false;
                }
            }
            if let Some(excluded) = &self.not_keywords {
                if excluded.iter().any(&has) {
                    return false;
                }
            }
        }

        // --- combat status (CR 506.2: attacking/blocking only have
        // meaning during combat; state.combat is None otherwise) ---
        if let Some(cs) = self.combat_status {
            let combat = state.combat.as_ref();
            let attacking = combat.is_some_and(|c| c.is_attacker(obj.id));
            let ok = match cs {
                CombatStatusFilter::Attacking => attacking,
                CombatStatusFilter::Blocking =>
                    combat.is_some_and(|c| c.is_blocker(obj.id)),
                CombatStatusFilter::AttackingOrBlocking =>
                    attacking || combat.is_some_and(|c| c.is_blocker(obj.id)),
                CombatStatusFilter::AttackingYou => combat
                    .and_then(|c| c.attacker(obj.id))
                    .is_some_and(|a| a.defending_player == source_controller),
                CombatStatusFilter::NotAttacking => !attacking,
                CombatStatusFilter::Blocked => combat
                    .and_then(|c| c.attacker(obj.id))
                    .is_some_and(|a| a.is_blocked),
            };
            if !ok {
                return false;
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
        // --- power vs toughness (same-object compare; CR 208) ---
        if let Some(cmp) = &self.pt_compare {
            match (obj.raw_power_with_counters(None),
                   obj.raw_toughness_with_counters(None)) {
                (Some(p), Some(t)) if cmp.matches(p, t) => {}
                _ => return false,
            }
        }

        // --- name ---
        if let Some(name) = self.name {
            if obj.characteristics.name != name {
                return false;
            }
        }

        // --- is_token: CR 111 — "nontoken creature" and "target
        // token" filters read the object's token flag. Set by
        // `Effect::CreateToken` / `Effect::CopyPermanent`.
        if let Some(required) = self.is_token {
            if obj.is_token != required {
                return false;
            }
        }

        // --- is_double_faced: CR 712 — a transforming DFC, marked by a
        // seeded back face (no registry lookup needed).
        if let Some(required) = self.is_double_faced {
            if obj.back_face_characteristics.is_some() != required {
                return false;
            }
        }

        // --- has_counter ---
        if let Some(kind) = &self.has_counter {
            if !obj.has_counter(kind.clone()) {
                return false;
            }
        }

        // --- commodity token (Treasure / Clue / Food / …): minted
        // engine-side with no interner, so matched by the kind marker
        // rather than a subtype symbol. ---
        if let Some(kind) = self.commodity {
            if obj.commodity != Some(kind) {
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
// Combat-state constraint
// =============================================================================

/// Combat-state filter for [`ObjectFilter::combat_status`]. One enum
/// (not separate bools) because "attacking or blocking" is a
/// disjunction the AND-only field set can't express.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CombatStatusFilter {
    /// Declared as an attacker in the current combat.
    Attacking,
    /// Declared as a blocker in the current combat.
    Blocking,
    /// Either of the above ("target attacking or blocking creature").
    AttackingOrBlocking,
    /// Attacking the filter's source controller (or a planeswalker /
    /// battle they control — CR 508.1: the defending player is who
    /// the attack was declared against). "creature attacking you".
    AttackingYou,
    /// NOT declared as an attacker ("nonattacking creature"). Matches
    /// everything outside combat.
    NotAttacking,
    /// A *blocked* attacker — declared as an attacker AND assigned at
    /// least one blocker (CR 509.1h). "target blocked creature" /
    /// "whenever a creature you control becomes blocked" effects that
    /// read state. Distinct from `Blocking`, which is the blocker side.
    Blocked,
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

/// Compare a creature's OWN power against its OWN toughness (CR 208/107) —
/// "creatures with toughness greater than power", "power greater than
/// toughness", etc. Uses the same raw-power/raw-toughness-with-counters
/// read as [`PtCondition`] (the layer-aware upgrade will replace both at
/// once). Non-creatures (no power/toughness) never match.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PtCompare {
    /// power > toughness
    PowerGreater,
    /// toughness > power
    ToughnessGreater,
    /// power == toughness
    Equal,
}

impl PtCompare {
    pub fn matches(&self, power: i32, toughness: i32) -> bool {
        match self {
            Self::PowerGreater => power > toughness,
            Self::ToughnessGreater => toughness > power,
            Self::Equal => power == toughness,
        }
    }
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
    source: ObjectId,
    source_controller: PlayerId,
) -> Vec<TargetLegality> {
    selection.targets.iter().map(|c| {
        if requirement.matches_choice(c, state, source, source_controller) {
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
    source: ObjectId,
    source_controller: PlayerId,
) -> bool {
    validate_targets_on_resolution(requirement, selection, state, source, source_controller)
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
    source: ObjectId,
    source_controller: PlayerId,
) -> bool {
    if selection.is_empty() {
        return false;
    }
    validate_targets_on_resolution(requirement, selection, state, source, source_controller)
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

    /// "target opponent" (e.g. Soldevi Steam Beast) must reject targeting
    /// yourself — the outer controller constraint applies to PLAYER targets, not
    /// just object targets.
    #[test]
    fn player_target_honors_opponent_controller_constraint() {
        let s = GameState::new(2, 0);
        let req = TargetRequirement {
            filter: TargetFilter::Player,
            count: TargetCount::Exactly(1),
            controller: Some(ControllerConstraint::Opponent),
        };
        let src = crate::objects::NULL_OBJECT_ID;
        assert!(!req.matches_choice(&TargetChoice::Player(0), &s, src, 0),
            "opponent-only must reject targeting yourself");
        assert!(req.matches_choice(&TargetChoice::Player(1), &s, src, 0),
            "opponent-only must accept an opponent");
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
        // Direct arena insert bypasses after_enter_battlefield, so
        // place the PW's loyalty counters here to match the post-ETB
        // state the real pipeline would produce.
        state.objects.get_mut(id).unwrap()
            .add_counters(crate::types::CounterKind::Loyalty, 3);
        id
    }

    // --- CmcCondition / PtCondition ------------------------------------------

    #[test]
    fn ability_on_stack_targets_ability_entries_only() {
        let mut s = GameState::new(2, 0);
        // A creature source on the battlefield + its activated ability
        // on the stack (entry id distinct from any object).
        let src = put_creature(&mut s, 1, 1, Zone::Battlefield, 2, 2);
        let entry_id = s.allocate_object_id();
        s.stack.push(crate::stack::StackEntry::new_activated_ability(
            entry_id, src, 1, /*card_id=*/ 0, /*ability_id=*/ 0,
            "test ability".into(), TargetSelection::new(), Vec::new(), None,
        ));

        let activated_only = TargetFilter::AbilityOnStack {
            activated: true, triggered: false, source_filter: None,
        };
        let choice = TargetChoice::Object(entry_id);
        assert!(activated_only.matches(&choice, &s, crate::objects::NULL_OBJECT_ID, 0));
        // Triggered-only filter rejects an activated entry.
        let triggered_only = TargetFilter::AbilityOnStack {
            activated: false, triggered: true, source_filter: None,
        };
        assert!(!triggered_only.matches(&choice, &s, crate::objects::NULL_OBJECT_ID, 0));
        // Source filter: "from a noncreature source" rejects it.
        let noncreature_src = TargetFilter::AbilityOnStack {
            activated: true, triggered: false,
            source_filter: Some(ObjectFilter::new()
                .without_types(TypeLine::CREATURE.into())),
        };
        assert!(!noncreature_src.matches(&choice, &s, crate::objects::NULL_OBJECT_ID, 0));
        // Candidate enumeration finds the entry id.
        let cands = activated_only.enumerate_legal(&s, crate::objects::NULL_OBJECT_ID, 0);
        assert_eq!(cands, vec![TargetChoice::Object(entry_id)]);
    }

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
    fn object_filter_pt_compare_power_vs_toughness() {
        let mut s = GameState::new(2, 0);
        let wide = put_creature(&mut s, 0, 0, Zone::Battlefield, 4, 2); // power > toughness
        let tall = put_creature(&mut s, 0, 0, Zone::Battlefield, 2, 5); // toughness > power
        let sq = put_creature(&mut s, 0, 0, Zone::Battlefield, 3, 3);   // equal
        let sorc = put_sorcery(&mut s, 0, Zone::Hand(0));               // no P/T

        let tough = ObjectFilter::default().with_pt_compare(PtCompare::ToughnessGreater);
        let pow = ObjectFilter::default().with_pt_compare(PtCompare::PowerGreater);
        let eq = ObjectFilter::default().with_pt_compare(PtCompare::Equal);

        assert!( tough.matches(s.objects.get(tall).unwrap(), &s, 0));
        assert!(!tough.matches(s.objects.get(wide).unwrap(), &s, 0));
        assert!(!tough.matches(s.objects.get(sq).unwrap(), &s, 0));
        assert!( pow.matches(s.objects.get(wide).unwrap(), &s, 0));
        assert!( eq.matches(s.objects.get(sq).unwrap(), &s, 0));
        // Non-creatures (no power/toughness) never match a pt compare.
        assert!(!tough.matches(s.objects.get(sorc).unwrap(), &s, 0));
    }

    #[test]
    fn object_filter_double_faced() {
        let mut s = GameState::new(2, 0);
        let dfc = put_creature(&mut s, 0, 0, Zone::Battlefield, 2, 2);
        s.objects.get_mut(dfc).unwrap().back_face_characteristics =
            Some(Characteristics::default());
        let plain = put_creature(&mut s, 0, 0, Zone::Battlefield, 2, 2);

        let f = ObjectFilter::creature().double_faced();
        assert!( f.matches(s.objects.get(dfc).unwrap(), &s, 0));
        assert!(!f.matches(s.objects.get(plain).unwrap(), &s, 0));
        // Some(false) = single-faced only.
        let single = ObjectFilter { is_double_faced: Some(false), ..Default::default() };
        assert!(!single.matches(s.objects.get(dfc).unwrap(), &s, 0));
        assert!( single.matches(s.objects.get(plain).unwrap(), &s, 0));
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
    fn object_filter_types_any_or_mask() {
        // "instant or sorcery" — a type-OR the primary types mask
        // (AND) can't express. Young Pyromancer's trigger filter
        // rides on this field.
        let mut s = GameState::new(2, 0);
        let sorc = put_sorcery(&mut s, 0, Zone::Stack);
        let creature = put_creature(&mut s, 0, 0, Zone::Battlefield, 2, 2);
        let instant_id = {
            let id = s.allocate_object_id();
            let chars = Characteristics {
                mana_cost: Some(ManaCost::parse("{R}").unwrap()),
                colors: ColorSet::red(),
                types: TypeLine::INSTANT.into(),
                ..Default::default()
            };
            s.objects.insert(GameObject::new(id, 0, Zone::Stack, 3, chars));
            id
        };

        let instant_or_sorcery = ObjectFilter::new()
            .with_types_any(TypeLine(TypeLine::INSTANT | TypeLine::SORCERY));
        assert!( instant_or_sorcery.matches(s.objects.get(sorc).unwrap(), &s, 0));
        assert!( instant_or_sorcery.matches(s.objects.get(instant_id).unwrap(), &s, 0));
        assert!(!instant_or_sorcery.matches(s.objects.get(creature).unwrap(), &s, 0),
            "creature has neither instant nor sorcery bit");
    }

    #[test]
    fn object_filter_types_any_composes_with_types_and_not_types() {
        // AND + OR + NOT all compose: require CREATURE, at least one
        // of CREATURE|ARTIFACT, exclude LAND. A plain creature passes
        // all three conjuncts.
        let mut s = GameState::new(2, 0);
        let creature = put_creature(&mut s, 0, 0, Zone::Battlefield, 2, 2);
        let sorc = put_sorcery(&mut s, 0, Zone::Stack);

        let f = ObjectFilter::new()
            .with_types(TypeLine::CREATURE.into())
            .with_types_any(TypeLine(TypeLine::CREATURE | TypeLine::ARTIFACT))
            .without_types(TypeLine::LAND.into());
        assert!(f.matches(s.objects.get(creature).unwrap(), &s, 0));
        // Sorcery fails the CREATURE AND-mask first; separately it
        // would also fail the types_any OR-mask.
        assert!(!f.matches(s.objects.get(sorc).unwrap(), &s, 0));
    }

    #[test]
    fn object_filter_types_any_empty_mask_matches_nothing() {
        // types_any of 0 bits has no way to succeed — matches no
        // object. Regression guard: the check uses `& mask != 0`, not
        // `& mask == mask`.
        let mut s = GameState::new(2, 0);
        let creature = put_creature(&mut s, 0, 0, Zone::Battlefield, 2, 2);

        let f = ObjectFilter { types_any: Some(TypeLine(0)), ..Default::default() };
        assert!(!f.matches(s.objects.get(creature).unwrap(), &s, 0));
    }

    #[test]
    fn object_filter_subtypes_any_disjunction() {
        // "Human or Warrior creature" — neither subtype alone is a
        // requirement; the object must have AT LEAST one.
        let mut s = GameState::new(2, 0);
        let mut reg = crate::registry::CardRegistry::new();
        let human   = reg.interner_mut().intern("Human");
        let warrior = reg.interner_mut().intern("Warrior");
        let elf     = reg.interner_mut().intern("Elf");

        let mk = |subs: &[SmallString], state: &mut GameState| -> ObjectId {
            let id = state.allocate_object_id();
            let mut subtypes = crate::types::SubtypeSet::default();
            for s in subs { subtypes.0.insert(*s); }
            let chars = Characteristics {
                types: TypeLine::CREATURE.into(),
                subtypes,
                power: Some(crate::types::PtValue::Fixed(1)),
                toughness: Some(crate::types::PtValue::Fixed(1)),
                ..Default::default()
            };
            state.objects.insert(GameObject::new(id, 0, Zone::Battlefield, 1, chars));
            id
        };
        let h_only = mk(&[human], &mut s);
        let w_only = mk(&[warrior], &mut s);
        let e_only = mk(&[elf], &mut s);

        let f = ObjectFilter::creature()
            .with_subtypes_any(vec![human, warrior]);
        assert!( f.matches(s.objects.get(h_only).unwrap(), &s, 0));
        assert!( f.matches(s.objects.get(w_only).unwrap(), &s, 0));
        assert!(!f.matches(s.objects.get(e_only).unwrap(), &s, 0),
            "elf has neither subtype in the disjunction");
    }

    #[test]
    fn object_filter_supertypes_legendary_and_excluded() {
        let mut s = GameState::new(2, 0);
        let mk = |state: &mut GameState, legendary: bool| -> ObjectId {
            let id = state.allocate_object_id();
            let mut sts = SupertypeSet::default();
            if legendary { sts = SupertypeSet::new().with(SupertypeSet::LEGENDARY); }
            let chars = Characteristics {
                types: TypeLine::CREATURE.into(),
                supertypes: sts,
                power: Some(crate::types::PtValue::Fixed(1)),
                toughness: Some(crate::types::PtValue::Fixed(1)),
                ..Default::default()
            };
            state.objects.insert(GameObject::new(id, 0, Zone::Battlefield, 1, chars));
            id
        };
        let legendary_id = mk(&mut s, true);
        let mundane_id   = mk(&mut s, false);

        let legendary_only = ObjectFilter::creature()
            .with_supertypes(SupertypeSet::new().with(SupertypeSet::LEGENDARY));
        assert!( legendary_only.matches(s.objects.get(legendary_id).unwrap(), &s, 0));
        assert!(!legendary_only.matches(s.objects.get(mundane_id).unwrap(), &s, 0));

        let nonlegendary_only = ObjectFilter::creature()
            .without_supertypes(SupertypeSet::new().with(SupertypeSet::LEGENDARY));
        assert!(!nonlegendary_only.matches(s.objects.get(legendary_id).unwrap(), &s, 0));
        assert!( nonlegendary_only.matches(s.objects.get(mundane_id).unwrap(), &s, 0));
    }

    #[test]
    fn object_filter_subtype_exclusion() {
        let mut s = GameState::new(2, 0);
        // SmallString ids are arbitrary in a bare test state; 7 = "Human",
        // 9 = "Wizard" by fiat.
        let human: SmallString = 7;
        let wizard: SmallString = 9;
        let mk = |state: &mut GameState, subs: &[SmallString]| -> ObjectId {
            let id = state.allocate_object_id();
            let mut st = SubtypeSet::default();
            for s in subs { st.0.insert(*s); }
            let chars = Characteristics {
                types: TypeLine::CREATURE.into(),
                subtypes: st,
                ..Default::default()
            };
            state.objects.insert(GameObject::new(id, 0, Zone::Battlefield, 1, chars));
            id
        };
        let human_wizard = mk(&mut s, &[human, wizard]);
        let plain_wizard = mk(&mut s, &[wizard]);
        let untyped      = mk(&mut s, &[]);

        let non_human = ObjectFilter::creature().without_subtype_sym(human);
        assert!(!non_human.matches(s.objects.get(human_wizard).unwrap(), &s, 0));
        assert!( non_human.matches(s.objects.get(plain_wizard).unwrap(), &s, 0));
        assert!( non_human.matches(s.objects.get(untyped).unwrap(),      &s, 0));

        // Chained exclusions AND together (Power Word Kill shape).
        let neither = ObjectFilter::creature()
            .without_subtype_sym(human)
            .without_subtype_sym(wizard);
        assert!(!neither.matches(s.objects.get(plain_wizard).unwrap(), &s, 0));
        assert!( neither.matches(s.objects.get(untyped).unwrap(),      &s, 0));

        // Composes with a positive subtype requirement: "non-Human Wizard".
        let nonhuman_wizard = ObjectFilter::creature()
            .with_subtype_sym(wizard)
            .without_subtype_sym(human);
        assert!(!nonhuman_wizard.matches(s.objects.get(human_wizard).unwrap(), &s, 0));
        assert!( nonhuman_wizard.matches(s.objects.get(plain_wizard).unwrap(), &s, 0));
        assert!(!nonhuman_wizard.matches(s.objects.get(untyped).unwrap(),      &s, 0));
    }

    #[test]
    fn object_filter_keywords_layer_aware_and_excluded() {
        use crate::effects::KeywordAbility;
        let mut s = GameState::new(2, 0);
        let flyer    = put_creature(&mut s, 0, 0, Zone::Battlefield, 1, 1);
        let grounded = put_creature(&mut s, 0, 0, Zone::Battlefield, 1, 1);
        s.objects.get_mut(flyer).unwrap()
            .characteristics.keywords.push(KeywordAbility::Flying);

        let with_flying = ObjectFilter::creature().with_keyword(KeywordAbility::Flying);
        assert!( with_flying.matches(s.objects.get(flyer).unwrap(),    &s, 0));
        assert!(!with_flying.matches(s.objects.get(grounded).unwrap(), &s, 0));

        // "without flying" — the exclusion mirror.
        let without_flying = ObjectFilter::creature().without_keyword(KeywordAbility::Flying);
        assert!(!without_flying.matches(s.objects.get(flyer).unwrap(),    &s, 0));
        assert!( without_flying.matches(s.objects.get(grounded).unwrap(), &s, 0));

        // Layer-aware: a Layer-6 grant makes the grounded creature match.
        s.add_continuous_effect(crate::layers::ContinuousEffect::grant_keyword(
            0, grounded, KeywordAbility::Flying, crate::layers::Duration::EndOfTurn,
        ));
        assert!( with_flying.matches(s.objects.get(grounded).unwrap(),    &s, 0));
        assert!(!without_flying.matches(s.objects.get(grounded).unwrap(), &s, 0));
    }

    #[test]
    fn object_filter_keywords_any_disjunction() {
        use crate::effects::KeywordAbility;
        let mut s = GameState::new(2, 0);
        let trampler = put_creature(&mut s, 0, 0, Zone::Battlefield, 1, 1);
        let vanilla  = put_creature(&mut s, 0, 0, Zone::Battlefield, 1, 1);
        s.objects.get_mut(trampler).unwrap()
            .characteristics.keywords.push(KeywordAbility::Trample);

        // Mwonvuli Beast Tracker — "deathtouch, hexproof, reach, or trample".
        let f = ObjectFilter::creature().with_keywords_any(vec![
            KeywordAbility::Deathtouch, KeywordAbility::Hexproof,
            KeywordAbility::Reach, KeywordAbility::Trample,
        ]);
        assert!( f.matches(s.objects.get(trampler).unwrap(), &s, 0));
        assert!(!f.matches(s.objects.get(vanilla).unwrap(),  &s, 0));

        // Empty disjunction matches nothing (consistent with subtypes_any).
        let empty = ObjectFilter::creature().with_keywords_any(vec![]);
        assert!(!empty.matches(s.objects.get(trampler).unwrap(), &s, 0));
    }

    #[test]
    fn target_filter_blocking_pairing_variants() {
        use crate::combat::{AttackerInfo, BlockerInfo, CombatState};
        let mut s = GameState::new(2, 0);
        // P0's `knight` attacks; P1 blocks with `blocker_a`. P1's
        // `other_atk` attacks separately, blocked by P0's `mine`.
        let knight    = put_creature(&mut s, 0, 0, Zone::Battlefield, 2, 2);
        let blocker_a = put_creature(&mut s, 1, 1, Zone::Battlefield, 2, 2);
        let other_atk = put_creature(&mut s, 1, 1, Zone::Battlefield, 2, 2);
        let mine      = put_creature(&mut s, 0, 0, Zone::Battlefield, 2, 2);
        let bystander = put_creature(&mut s, 1, 1, Zone::Battlefield, 2, 2);

        let blocking = TargetFilter::CreatureBlockingSource;
        let either   = TargetFilter::CreatureBlockingOrBlockedBySource;

        // Outside combat: nothing pairs.
        assert!(!blocking.matches(&TargetChoice::Object(blocker_a), &s, knight, 0));

        let mut combat = CombatState::new();
        combat.attackers.push(AttackerInfo {
            object_id: knight, defending_player: 1,
            defending_planeswalker: None,
            blocked_by: vec![blocker_a], is_blocked: true,
        });
        combat.attackers.push(AttackerInfo {
            object_id: other_atk, defending_player: 0,
            defending_planeswalker: None,
            blocked_by: vec![mine], is_blocked: true,
        });
        combat.blockers.push(BlockerInfo { object_id: blocker_a, blocking: knight });
        combat.blockers.push(BlockerInfo { object_id: mine, blocking: other_atk });
        s.combat = Some(combat);

        // "Target creature blocking [knight]": only its own blocker.
        assert!( blocking.matches(&TargetChoice::Object(blocker_a), &s, knight, 0));
        assert!(!blocking.matches(&TargetChoice::Object(mine),      &s, knight, 0));
        assert!(!blocking.matches(&TargetChoice::Object(bystander), &s, knight, 0));

        // "Blocking or blocked by [mine]" (Lesser Werewolf on a blocker):
        // the attacker it blocks qualifies; unrelated combatants don't.
        assert!( either.matches(&TargetChoice::Object(other_atk), &s, mine, 0));
        assert!(!either.matches(&TargetChoice::Object(knight),    &s, mine, 0));
        assert!(!either.matches(&TargetChoice::Object(blocker_a), &s, mine, 0));

        // enumerate_legal agrees with matches.
        let found = blocking.enumerate_legal(&s, knight, 0);
        assert_eq!(found, vec![TargetChoice::Object(blocker_a)]);

        // Script accessors expose the same pairing.
        assert_eq!(crate::script::blockers_of(&s, knight), vec![blocker_a]);
        assert_eq!(crate::script::attackers_blocked_by(&s, mine), vec![other_atk]);
    }

    #[test]
    fn object_filter_combat_status_variants() {
        use crate::combat::{AttackerInfo, BlockerInfo, CombatState};
        let mut s = GameState::new(2, 0);
        // P0 attacks P1 with two creatures; P1 blocks one and keeps
        // one home.
        let atk_vs_p1  = put_creature(&mut s, 0, 0, Zone::Battlefield, 2, 2);
        let atk_vs_p0_ = put_creature(&mut s, 0, 0, Zone::Battlefield, 2, 2);
        let blocker    = put_creature(&mut s, 1, 1, Zone::Battlefield, 2, 2);
        let bystander  = put_creature(&mut s, 1, 1, Zone::Battlefield, 2, 2);

        let attacking    = ObjectFilter::creature().attacking_only();
        let blocking     = ObjectFilter::creature().blocking_only();
        let in_combat    = ObjectFilter::creature().attacking_or_blocking_only();
        let attacking_me = ObjectFilter::creature().attacking_you_only();
        let nonattacking = ObjectFilter::creature().nonattacking_only();
        let blocked      = ObjectFilter::creature().blocked_only();

        // Outside combat: nothing attacks/blocks; NotAttacking matches all.
        assert!(!attacking.matches(s.objects.get(atk_vs_p1).unwrap(), &s, 1));
        assert!(!in_combat.matches(s.objects.get(blocker).unwrap(), &s, 1));
        assert!(nonattacking.matches(s.objects.get(atk_vs_p1).unwrap(), &s, 1));

        let mut combat = CombatState::new();
        combat.attackers.push(AttackerInfo {
            object_id: atk_vs_p1, defending_player: 1,
            defending_planeswalker: None,
            blocked_by: vec![blocker], is_blocked: true,
        });
        combat.attackers.push(AttackerInfo {
            object_id: atk_vs_p0_, defending_player: 1,
            defending_planeswalker: None,
            blocked_by: Vec::new(), is_blocked: false,
        });
        combat.blockers.push(BlockerInfo { object_id: blocker, blocking: atk_vs_p1 });
        s.combat = Some(combat);

        assert!( attacking.matches(s.objects.get(atk_vs_p1).unwrap(), &s, 1));
        assert!(!attacking.matches(s.objects.get(blocker).unwrap(),   &s, 1));
        assert!(!attacking.matches(s.objects.get(bystander).unwrap(), &s, 1));

        assert!( blocking.matches(s.objects.get(blocker).unwrap(),   &s, 1));
        assert!(!blocking.matches(s.objects.get(atk_vs_p1).unwrap(), &s, 1));

        assert!( in_combat.matches(s.objects.get(atk_vs_p1).unwrap(), &s, 1));
        assert!( in_combat.matches(s.objects.get(blocker).unwrap(),   &s, 1));
        assert!(!in_combat.matches(s.objects.get(bystander).unwrap(), &s, 1));

        // "attacking you": true for P1 (the defender), false for P0.
        assert!( attacking_me.matches(s.objects.get(atk_vs_p1).unwrap(), &s, 1));
        assert!(!attacking_me.matches(s.objects.get(atk_vs_p1).unwrap(), &s, 0));

        assert!(!nonattacking.matches(s.objects.get(atk_vs_p1).unwrap(), &s, 1));
        assert!( nonattacking.matches(s.objects.get(bystander).unwrap(), &s, 1));

        // "blocked": true only for the attacker that has a blocker assigned.
        assert!( blocked.matches(s.objects.get(atk_vs_p1).unwrap(),  &s, 1),
            "atk_vs_p1 is blocked by `blocker`");
        assert!(!blocked.matches(s.objects.get(atk_vs_p0_).unwrap(), &s, 1),
            "atk_vs_p0_ attacks unblocked");
        assert!(!blocked.matches(s.objects.get(blocker).unwrap(),    &s, 1),
            "the blocker itself is not a *blocked* creature");
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
        assert!( f.matches(&TargetChoice::Object(on_bf), &s, crate::objects::NULL_OBJECT_ID, 0));
        assert!(!f.matches(&TargetChoice::Object(in_gy), &s, crate::objects::NULL_OBJECT_ID, 0));
    }

    #[test]
    fn target_filter_creature_rejects_player_choice() {
        let s = GameState::new(2, 0);
        let f = TargetFilter::Creature;
        assert!(!f.matches(&TargetChoice::Player(0), &s, crate::objects::NULL_OBJECT_ID, 0));
    }

    #[test]
    fn target_filter_player_rejects_out_of_range_or_dead() {
        let mut s = GameState::new(2, 0);
        assert!(TargetFilter::Player.matches(&TargetChoice::Player(0), &s, crate::objects::NULL_OBJECT_ID, 0));

        s.player_mut(1).has_lost = true;
        assert!(!TargetFilter::Player.matches(&TargetChoice::Player(1), &s, crate::objects::NULL_OBJECT_ID, 0));
        // Out-of-range
        assert!(!TargetFilter::Player.matches(&TargetChoice::Player(9), &s, crate::objects::NULL_OBJECT_ID, 0));
    }

    #[test]
    fn target_filter_creature_or_player_accepts_either() {
        let mut s = GameState::new(2, 0);
        let c = put_creature(&mut s, 0, 0, Zone::Battlefield, 2, 2);
        let f = TargetFilter::CreatureOrPlayer;

        assert!(f.matches(&TargetChoice::ObjectOrPlayer(
            ObjectOrPlayer::Object(c)), &s, crate::objects::NULL_OBJECT_ID, 0));
        assert!(f.matches(&TargetChoice::ObjectOrPlayer(
            ObjectOrPlayer::Player(1)), &s, crate::objects::NULL_OBJECT_ID, 0));
    }

    #[test]
    fn target_filter_any_target_includes_planeswalkers() {
        let mut s = GameState::new(2, 0);
        let pw = put_planeswalker(&mut s, 0);

        assert!(TargetFilter::AnyTarget.matches(
            &TargetChoice::ObjectOrPlayer(ObjectOrPlayer::Object(pw)), &s,
            crate::objects::NULL_OBJECT_ID, 0));
        // Creature-or-player rejects the bare planeswalker
        assert!(!TargetFilter::CreatureOrPlayer.matches(
            &TargetChoice::ObjectOrPlayer(ObjectOrPlayer::Object(pw)), &s,
            crate::objects::NULL_OBJECT_ID, 0));
    }

    #[test]
    fn target_filter_spell_requires_stack_zone() {
        let mut s = GameState::new(2, 0);
        let in_hand  = put_sorcery(&mut s, 0, Zone::Hand(0));
        let on_stack = put_sorcery(&mut s, 0, Zone::Stack);

        let f = TargetFilter::Spell(ObjectFilter::new());
        assert!(!f.matches(&TargetChoice::Object(in_hand),  &s, crate::objects::NULL_OBJECT_ID, 0));
        assert!( f.matches(&TargetChoice::Object(on_stack), &s, crate::objects::NULL_OBJECT_ID, 0));
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
        assert!( f.matches(&TargetChoice::Object(mine),   &s, crate::objects::NULL_OBJECT_ID, 0));
        assert!( f.matches(&TargetChoice::Object(theirs), &s, crate::objects::NULL_OBJECT_ID, 0));
        assert!(!f.matches(&TargetChoice::Object(on_bf),  &s, crate::objects::NULL_OBJECT_ID, 0));
    }

    // --- TargetFilter.enumerate_legal ---------------------------------------

    #[test]
    fn enumerate_legal_creature_battlefield_only() {
        let mut s = GameState::new(2, 0);
        put_creature(&mut s, 0, 0, Zone::Battlefield, 1, 1);
        put_creature(&mut s, 0, 0, Zone::Battlefield, 2, 2);
        put_creature(&mut s, 0, 0, Zone::Graveyard(0), 3, 3);

        let legals = TargetFilter::Creature.enumerate_legal(&s, crate::objects::NULL_OBJECT_ID, 0);
        assert_eq!(legals.len(), 2);
    }

    #[test]
    fn enumerate_legal_players_excludes_dead() {
        let mut s = GameState::new(3, 0);
        s.player_mut(1).has_lost = true;
        let legals = TargetFilter::Player.enumerate_legal(&s, crate::objects::NULL_OBJECT_ID, 0);
        let ids: Vec<_> = legals.iter().filter_map(|c| c.player_id()).collect();
        assert_eq!(ids, vec![0, 2]);
    }

    // --- TargetRequirement --------------------------------------------------

    /// Ward does NOT affect target legality (CR 702.21a makes it a
    /// triggered ability). The Ward trigger synthesizes from the
    /// [`GameEvent::BecomesTarget`] event emitted at cast/activate
    /// time and resolves as its own stack object. End-to-end
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
        assert!(req.matches_choice(&TargetChoice::Object(theirs), &s, crate::objects::NULL_OBJECT_ID, 0),
            "Ward must not short-circuit targeting — spell can be cast, \
             Ward fires at resolution");
        assert!(req.matches_choice(&TargetChoice::Object(theirs), &s, crate::objects::NULL_OBJECT_ID, 1));
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
        assert!(!req.matches_choice(&TargetChoice::Object(theirs), &s, crate::objects::NULL_OBJECT_ID, 0));
        // From the creature's own controller (player 1): still OK.
        assert!(req.matches_choice(&TargetChoice::Object(theirs), &s, crate::objects::NULL_OBJECT_ID, 1));
    }

    #[test]
    fn protection_blocks_matching_source_targeting_uncontroller_gated() {
        use crate::effects::{KeywordAbility, ProtectionQuality};
        use crate::types::{Color, ColorSet};
        let mut s = GameState::new(2, 0);
        // A creature with protection from red.
        let pro = put_creature(&mut s, 1, 1, Zone::Battlefield, 2, 2);
        s.objects.get_mut(pro).unwrap().characteristics.keywords
            .push(KeywordAbility::Protection(ProtectionQuality::Color(Color::Red)));
        // A red and a green source object (stand-ins for spells on the stack).
        let red_src = put_creature(&mut s, 0, 0, Zone::Stack, 1, 1);
        s.objects.get_mut(red_src).unwrap().characteristics.colors = ColorSet::red();
        let green_src = put_creature(&mut s, 0, 0, Zone::Stack, 1, 1);
        s.objects.get_mut(green_src).unwrap().characteristics.colors = ColorSet::green();

        let req = TargetRequirement::target_creature();
        // Red source → rejected (fine-grained color match, was a TODO).
        assert!(!req.matches_choice(&TargetChoice::Object(pro), &s, red_src, 0));
        // Green source → allowed (protection from red doesn't match green).
        assert!(req.matches_choice(&TargetChoice::Object(pro), &s, green_src, 0));
        // NOT controller-gated: the protected creature's OWN controller's
        // red source is rejected too (CR 702.16e, unlike Hexproof).
        assert!(!req.matches_choice(&TargetChoice::Object(pro), &s, red_src, 1));
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
        assert!( req.matches_choice(&TargetChoice::Object(mine),   &s, crate::objects::NULL_OBJECT_ID, 0));
        assert!(!req.matches_choice(&TargetChoice::Object(theirs), &s, crate::objects::NULL_OBJECT_ID, 0));
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

        assert!(req.is_satisfied(&ok, &s, crate::objects::NULL_OBJECT_ID, 0, None));
        assert!(!req.is_satisfied(&wrong_count, &s, crate::objects::NULL_OBJECT_ID, 0, None));
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
        assert!(req.is_satisfied(&sel, &s, crate::objects::NULL_OBJECT_ID, 0, Some(2)));
        assert!(!req.is_satisfied(&sel, &s, crate::objects::NULL_OBJECT_ID, 0, Some(1)));
    }

    // --- CR 608.2b: target recheck at resolution ----------------------------

    #[test]
    fn resolution_recheck_all_legal_returns_all_legal() {
        let mut s = GameState::new(2, 0);
        let c = put_creature(&mut s, 0, 0, Zone::Battlefield, 2, 2);
        let req = TargetRequirement::target_creature();
        let sel = TargetSelection { targets: vec![TargetChoice::Object(c)] };

        let legals = validate_targets_on_resolution(&req, &sel, &s, crate::objects::NULL_OBJECT_ID, 0);
        assert_eq!(legals, vec![TargetLegality::Legal]);
        assert!(all_targets_still_legal(&req, &sel, &s, crate::objects::NULL_OBJECT_ID, 0));
        assert!(!should_counter_due_to_illegal_targets(&req, &sel, &s, crate::objects::NULL_OBJECT_ID, 0));
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

        let legals = validate_targets_on_resolution(&req, &sel, &s, crate::objects::NULL_OBJECT_ID, 0);
        assert_eq!(legals, vec![TargetLegality::Illegal]);
        assert!(should_counter_due_to_illegal_targets(&req, &sel, &s, crate::objects::NULL_OBJECT_ID, 0));
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
        let legals = validate_targets_on_resolution(&req, &sel, &s, crate::objects::NULL_OBJECT_ID, 0);
        assert_eq!(legals, vec![TargetLegality::Legal, TargetLegality::Illegal]);

        // With at least one legal target remaining, the spell resolves
        // (it only skips the illegal ones).
        assert!(!all_targets_still_legal(&req, &sel, &s, crate::objects::NULL_OBJECT_ID, 0));
        assert!(!should_counter_due_to_illegal_targets(&req, &sel, &s, crate::objects::NULL_OBJECT_ID, 0));
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
        assert!(!should_counter_due_to_illegal_targets(&req, &empty, &s, crate::objects::NULL_OBJECT_ID, 0));
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
