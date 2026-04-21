//! [`Effect`] — the primitive operations cards are built from — and
//! their execution on [`GameState`].
//!
//! Addendum Section 6 / Phase 1 Task #13. Depends on tasks 4 (objects),
//! 5 (events), 6 (state), 9 (targets), 10 (priority), 11 (stack).
//!
//! # Design
//!
//! A card's behavior is expressed as a sequence of `Effect`s. The card
//! registry's effect callback builds the concrete effects (with
//! targets and choices already resolved) and the engine's resolution
//! pipeline (Task #11) hands each one to [`Effect::execute`]. The
//! execute implementation is pure in the sense that it takes
//! `&mut GameState` and mutates exactly what the effect says — no
//! extra bookkeeping, no hidden state.
//!
//! # Scope and stubs
//!
//! Variants that demand a mid-resolution **player decision** (Search a
//! library, Discard opponent-chooses, Sacrifice with a choice,
//! Counterspell-style targeting, copies-with-new-targets) cannot run
//! as a single `execute` call — the engine needs to yield back to the
//! agent. Those variants here apply a **deterministic first-match**
//! action so the shape of the mutation is right for integration
//! testing, and carry a `TODO(decision)` marker so the engine
//! (Task #20) knows to re-route them through a proper decision
//! yield.
//!
//! # Serialization
//!
//! [`Condition::Custom`] is a bare `fn` pointer, so [`Condition`] and
//! transitively [`Effect::Conditional`] cannot derive `Serialize`/
//! `Deserialize` yet. Migration to `ConditionFnId` is scheduled for
//! Phase 3 (addendum Section 12).

use crate::events::{DamageTarget, GameEvent, MoveCause};
use crate::layers::Duration;
use crate::objects::{Characteristics, GameObject, ObjectId};
use crate::state::GameState;
use crate::targets::{CmcCondition, ObjectFilter};
use crate::types::*;
use crate::zones::Zone;

// =============================================================================
// Effect
// =============================================================================

// TODO(serialize): `Effect::Conditional` holds a `Condition` which may
// contain a bare `fn` pointer (`Condition::Custom`). Migrate to
// `ConditionFnId` per addendum Section 12 in Phase 3.
/// Core effect primitives. All card effects are composed from these.
#[derive(Clone, Debug)]
pub enum Effect {
    // --- damage / life -----------------------------------------------------
    DealDamage { source: ObjectId, target: DamageTarget, amount: u32 },
    GainLife { player: PlayerId, amount: u32 },
    LoseLife { player: PlayerId, amount: u32 },
    SetLifeTotal { player: PlayerId, amount: u32 },
    /// CR 615 — Install a prevention shield on `target`. `amount` of
    /// `None` prevents all damage; `Some(n)` prevents up to `n`
    /// damage (Healing Salve-style). The shield lasts for `duration`.
    PreventDamage {
        target: DamageTarget,
        amount: Option<u32>,
        duration: crate::replacement::ReplacementDuration,
    },
    /// CR 614.9 — Redirect damage that would hit `from` onto `to`
    /// instead. The redirect applies to every qualifying damage event
    /// until `duration` expires. No source filtering (agents can
    /// install a second shield to narrow the source).
    RedirectDamage {
        from: DamageTarget,
        to: DamageTarget,
        duration: crate::replacement::ReplacementDuration,
    },
    /// CR 701.25 — Install a regenerate shield on `target`. The next
    /// time `target` would die this turn, it's saved instead: damage
    /// cleared, tapped, removed from combat. Shield is consumed on
    /// fire. Stacks: multiple calls install multiple shields.
    Regenerate { target: ObjectId },
    /// CR 701.32 — Manifest the top card of `player`'s library: put
    /// it onto the battlefield face-down as a 2/2 creature. Until
    /// turned face-up it has no name, no characteristics beyond
    /// those base 2/2. The arena object retains its card id and
    /// original characteristics for when it's flipped.
    Manifest { player: PlayerId },

    // --- card flow ---------------------------------------------------------
    DrawCards { player: PlayerId, count: u32 },
    Discard { player: PlayerId, count: u32, choice: DiscardChoice },
    Mill { player: PlayerId, count: u32 },

    // --- zone manipulation -------------------------------------------------
    DestroyPermanent { target: ObjectId },
    ExilePermanent { target: ObjectId },
    ReturnToHand { target: ObjectId },
    PutOnTopOfLibrary { target: ObjectId },
    PutOnBottomOfLibrary { target: ObjectId },
    /// Reanimate a specific card from a graveyard (spec card, not
    /// by filter). No-op if `target` isn't currently in a graveyard.
    ReturnFromGraveyardToBattlefield { target: ObjectId },
    /// Regrowth-style: return a specific graveyard card to its
    /// owner's hand. No-op if `target` isn't in a graveyard.
    ReturnFromGraveyardToHand { target: ObjectId },
    /// Exile a specific card from a graveyard (Bojuka Bog,
    /// graveyard-hate). No-op if `target` isn't in a graveyard.
    ExileFromGraveyard { target: ObjectId },

    // --- library manipulation ---------------------------------------------
    /// CR 103.2 — shuffle a player's library.
    Shuffle { player: PlayerId },
    /// CR 701.19 — look at top N cards of library; put any number on
    /// bottom (in any order), rest on top. Phase 1 policy: keep
    /// everything on top (deterministic no-op); agent-decision variant
    /// deferred. Emits [`GameEvent::Scry`].
    Scry { player: PlayerId, count: u32 },
    /// CR 701.45 — look at top N; put any number in the graveyard,
    /// rest on top. Phase 1 policy: mill everything looked at.
    /// Emits [`GameEvent::Surveil`].
    Surveil { player: PlayerId, count: u32 },

    // --- tokens / copies ---------------------------------------------------
    CreateToken { controller: PlayerId, token: TokenDefinition },
    CopySpell { target: ObjectId },
    CopyPermanent { target: ObjectId },

    // --- cascade (CR 702.85) -----------------------------------------------
    /// Exile cards off the top of `controller`'s library until a
    /// nonland card with mana value strictly less than `source`'s
    /// mana value appears (or the library runs out). Prompt a may-cast
    /// for the hit (if any) via a [`crate::actions::ChoiceKind::YesNo`];
    /// on yes, cast the hit for free. All non-cast exiled cards go to
    /// the bottom of the library in seeded-random order.
    Cascade { source: ObjectId, controller: PlayerId },

    // --- counters ----------------------------------------------------------
    AddCounters { target: ObjectId, kind: CounterKind, count: u32 },
    RemoveCounters { target: ObjectId, kind: CounterKind, count: u32 },
    /// CR 701.25 — Proliferate. Phase 1 policy: apply to every
    /// eligible permanent and player (greedy maximum). Each chosen
    /// permanent gains one counter of each kind already on it; each
    /// chosen player gains one of each counter type they have.
    /// Agent-choice variant is deferred.
    Proliferate,
    /// Move up to `count` counters of `kind` from `from` to `to`.
    /// Only the actually-present count is moved — no counter is
    /// created out of thin air if `from` has fewer.
    MoveCounter { from: ObjectId, to: ObjectId, kind: CounterKind, count: u32 },

    // --- P/T and keywords (static until `duration`) ------------------------
    Pump {
        target: ObjectId,
        power: i32,
        toughness: i32,
        duration: Duration,
        keywords: Vec<KeywordAbility>,
    },
    /// "Creatures `controller` controls get +P/+T" (Glorious Anthem,
    /// Crusade). Layer 7c. `source` on the underlying
    /// [`crate::layers::ContinuousEffect`] is the sentinel
    /// [`crate::objects::NULL_OBJECT_ID`] — for permanent-based anthems
    /// that should expire when the emitter leaves the battlefield, the
    /// card's builder should register the continuous effect directly
    /// with the real source id and
    /// [`Duration::WhileSourceOnBattlefield`].
    Anthem {
        controller: PlayerId,
        power: i32,
        toughness: i32,
        duration: Duration,
    },
    /// "Target gains `keyword` until `duration`" (Swiftfoot Boots,
    /// Heroic Intervention's Indestructible grant). Layer 6.
    GrantKeyword {
        target: ObjectId,
        keyword: KeywordAbility,
        duration: Duration,
    },
    /// Install an arbitrary [`crate::layers::ContinuousEffect`] on
    /// the state. The escape hatch for permanent-based anthems and
    /// other static-ability effects whose source must be a specific
    /// object id (so cleanup at source-leave-battlefield works
    /// correctly). Prefer the narrower `Anthem`, `GrantKeyword`,
    /// etc. variants when they fit; reach for this only when you
    /// need [`Duration::WhileSourceOnBattlefield`] bound to a real
    /// permanent id.
    InstallContinuousEffect { effect: crate::layers::ContinuousEffect },

    /// Snapcaster-style: push a [`crate::actions::ChoiceKind::PickCards`]
    /// listing every instant or sorcery card currently in any
    /// graveyard, and grant Flashback (cost = printed mana cost) to
    /// whichever one the `controller` picks. Zero-candidate case
    /// degrades gracefully — the choice presents an empty list and
    /// the pick resolves to nothing. A choosing player may also
    /// decline to pick (min = 0) when the trigger is resolving with
    /// no legal target and the game has no effect to apply.
    GrantFlashbackToInstantOrSorceryInGraveyard {
        source: ObjectId,
        controller: PlayerId,
        duration: crate::layers::Duration,
    },

    /// "Target becomes P/T" (Humility, Turn to Frog, Song of the
    /// Damned). Layer 7b — overrides the base characteristic.
    SetBasePT {
        target: ObjectId,
        power: i32,
        toughness: i32,
        duration: Duration,
    },

    // --- stack manipulation ------------------------------------------------
    /// Counter the spell or ability with the given stack-entry id.
    Counter { target: ObjectId },
    /// Cast `target` from its controller's hand without paying mana
    /// costs (Cascade, Discover). No-op if `target` isn't in the
    /// expected hand. Phase 1: no targets/modes chosen
    /// (TODO(decision)) — suitable for non-targeted spells; spells
    /// requiring targets resolve with empty selections and will
    /// CR 608.2b-counter themselves.
    CastFromHandFree {
        player: PlayerId,
        target: ObjectId,
    },
    /// Cast `target` from a graveyard (Flashback, Yawgmoth's Will,
    /// Snapcaster Mage enabler). No-op if `target` isn't in a
    /// graveyard. Same Phase 1 target/mode limitation as
    /// [`Self::CastFromHandFree`].
    CastFromGraveyard {
        player: PlayerId,
        target: ObjectId,
    },

    // --- state flip --------------------------------------------------------
    ChangeControl { target: ObjectId, new_controller: PlayerId },
    Transform { target: ObjectId },
    Tap { target: ObjectId },
    Untap { target: ObjectId },
    /// CR 701.2 — Attach `equipment_or_aura` to `target`. Removes
    /// any prior attachment and wires the new one. No-op if either
    /// object is missing or if `target` has protection that the
    /// attacher's characteristics don't overcome.
    Attach { equipment_or_aura: ObjectId, target: ObjectId },

    /// CR 702.21a — Ward trigger resolution: push a
    /// [`crate::actions::ChoiceKind::PayOrDecline`] to `caster`. On
    /// decline, `counter_target` (the targeting spell/ability's stack
    /// entry id) is countered. Emitted by the synthesized Ward trigger
    /// handler ([`crate::engine::WARD_TRIGGER_ID`] dispatch).
    WardPrompt {
        caster: PlayerId,
        cost: crate::mana::ManaCost,
        counter_target: ObjectId,
    },

    // --- mana / phases -----------------------------------------------------
    AddMana { player: PlayerId, mana: Vec<crate::mana::ManaUnit> },
    ExtraTurn { player: PlayerId },
    AdditionalCombatPhase,
    SkipNextPhase { player: PlayerId, phase: crate::turn::Phase },
    /// Azusa / Exploration — grant `player` `amount` additional land
    /// plays this turn (adds to both current remaining and per-turn
    /// base).
    PlayExtraLand { player: PlayerId, amount: u32 },
    /// Empty `player`'s mana pool immediately (CR 106.4 at phase
    /// boundaries; used here for card effects that dump pools).
    EmptyManaPool { player: PlayerId },
    /// Planeswalker ultimates and similar: create an emblem in the
    /// command zone carrying `emblem`'s abilities. Owner and
    /// controller are both `controller`.
    CreateEmblem { controller: PlayerId, emblem: EmblemDefinition },

    // --- choice-requiring (see module doc) ---------------------------------
    Search { player: PlayerId, zone: Zone, filter: ObjectFilter,
             destination: Zone, reveal: bool },
    Reanimate { player: PlayerId, filter: ObjectFilter, from_zone: Zone },
    Sacrifice { player: PlayerId, filter: ObjectFilter, count: u32 },
    /// CR 701.20a — Search `player`'s library for a matching card, put
    /// it into hand, then shuffle. `reveal` adds a public-info mark.
    /// Phase 1 picks the first matching id (deterministic); TODO
    /// upgrade to an agent decision.
    TutorToHand {
        player: PlayerId,
        filter: ObjectFilter,
        reveal: bool,
    },
    /// Natural Order-style: search library for a matching card, put
    /// it onto the battlefield (optionally tapped), then shuffle.
    TutorToBattlefield {
        player: PlayerId,
        filter: ObjectFilter,
        tapped: bool,
    },

    // --- combat-like -------------------------------------------------------
    Fight { a: ObjectId, b: ObjectId },
    /// CR 701.38 — Goad `target`. Until `duration` expires, the
    /// creature must attack each combat if able and must attack a
    /// player other than `goader` if able. Phase 1 honors the
    /// "can't attack goader" restriction in the legal-action
    /// enumerator; the "must attack if able" requirement is a
    /// TODO(agent-hint) until the decision layer surfaces it.
    Goad { target: ObjectId, goader: PlayerId, duration: Duration },
    /// Pacifism-style restriction: `target` can't attack until
    /// `duration` expires.
    ForbidAttacking { target: ObjectId, duration: Duration },

    // --- composites --------------------------------------------------------
    /// Apply `effect` once per id in `targets`, substituting the id as
    /// the primary target where the inner effect names one.
    /// Substitution is handled by the caller at construction time —
    /// this variant simply iterates.
    ForEach { targets: Vec<ObjectId>, effect: Box<Effect> },
    Conditional { condition: Condition, then: Box<Effect>,
                  otherwise: Option<Box<Effect>> },
    Sequence(Vec<Effect>),
}

// =============================================================================
// Execute
// =============================================================================

impl Effect {
    /// Apply this effect to `state`.
    ///
    /// Defensive against missing objects: if a named target has left
    /// the arena or changed zones, the effect silently no-ops on that
    /// target rather than panicking. This matches CR 608.2b's
    /// "illegal targets are skipped" principle — resolution-time
    /// target recheck happens in [`crate::stack`], but execute still
    /// needs to tolerate late-breaking state changes (e.g. another
    /// effect earlier in a Sequence removed this one's target).
    pub fn execute(&self, state: &mut GameState) {
        match self {
            // --- damage / life -------------------------------------------
            Effect::DealDamage { source, target, amount } => {
                state.deal_damage(*source, *target, *amount, /*combat=*/ false);
            }
            Effect::GainLife { player, amount } => {
                if !valid_player(state, *player) || *amount == 0 { return; }
                state.player_mut(*player).life += *amount as i32;
                state.emit(GameEvent::LifeGained { player: *player, amount: *amount });
            }
            Effect::LoseLife { player, amount } => {
                lose_life(state, *player, *amount);
            }
            Effect::SetLifeTotal { player, amount } => {
                if !valid_player(state, *player) { return; }
                let old = state.player(*player).life;
                state.player_mut(*player).life = *amount as i32;
                state.emit(GameEvent::LifeSet {
                    player: *player, old, new_total: *amount as i32,
                });
            }
            Effect::PreventDamage { target, amount, duration } => {
                use crate::replacement::{
                    ReplacementCondition, ReplacementEffect, ReplacementKind,
                };
                let kind = match amount {
                    None => ReplacementKind::PreventAllDamage,
                    Some(n) => ReplacementKind::PreventDamageUpTo(*n),
                };
                state.add_replacement_effect(ReplacementEffect {
                    source: crate::objects::NULL_OBJECT_ID,
                    id: 0,
                    condition: ReplacementCondition::WouldDealDamageToSpecific {
                        target: *target,
                    },
                    kind,
                    is_self_replacement: false,
                    duration: *duration,
                });
            }
            Effect::RedirectDamage { from, to, duration } => {
                use crate::replacement::{
                    ReplacementCondition, ReplacementEffect, ReplacementKind,
                };
                state.add_replacement_effect(ReplacementEffect {
                    source: crate::objects::NULL_OBJECT_ID,
                    id: 0,
                    condition: ReplacementCondition::WouldDealDamageToSpecific {
                        target: *from,
                    },
                    kind: ReplacementKind::RedirectDamageTo(*to),
                    is_self_replacement: false,
                    duration: *duration,
                });
            }
            Effect::Regenerate { target } => {
                install_regenerate_shield(state, *target);
            }
            Effect::Manifest { player } => {
                manifest_top_of_library(state, *player);
            }

            // --- card flow -----------------------------------------------
            Effect::DrawCards { player, count } => {
                if !valid_player(state, *player) { return; }
                for _ in 0..*count { state.draw_one_card(*player); }
            }
            Effect::Discard { player, count, choice } => {
                discard_cards(state, *player, *count, choice);
            }
            Effect::Mill { player, count } => {
                for _ in 0..*count { mill_one_card(state, *player); }
            }

            // --- zone changes --------------------------------------------
            Effect::DestroyPermanent { target } => {
                // CR 701.7b / 702.12b — Indestructible permanents
                // ignore "destroy" effects.
                if state.has_keyword(*target, &KeywordAbility::Indestructible) {
                    return;
                }
                if let Some(owner) = owner_of(state, *target) {
                    state.move_object_to_zone(*target, Zone::Graveyard(owner),
                        MoveCause::SpellResolution);
                }
            }
            Effect::ExilePermanent { target } => {
                state.move_object_to_zone(*target, Zone::Exile, MoveCause::SpellResolution);
            }
            Effect::ReturnToHand { target } => {
                if let Some(owner) = owner_of(state, *target) {
                    state.move_object_to_zone(*target, Zone::Hand(owner),
                        MoveCause::SpellResolution);
                }
            }
            Effect::PutOnTopOfLibrary { target } => {
                if let Some(owner) = owner_of(state, *target) {
                    // move_object_to_zone re-ids the object and
                    // appends it to the library's bottom; lift it
                    // back to the top using the NEW id it returns.
                    if let Some(new_id) = state.move_object_to_zone(
                        *target, Zone::Library(owner),
                        MoveCause::SpellResolution)
                    {
                        state.put_on_top_of_library(new_id, owner);
                    }
                }
            }
            Effect::PutOnBottomOfLibrary { target } => {
                if let Some(owner) = owner_of(state, *target) {
                    // move_object_to_zone appends to the library's
                    // bottom under a fresh id — that's what we want.
                    state.move_object_to_zone(*target, Zone::Library(owner),
                        MoveCause::SpellResolution);
                }
            }
            Effect::ReturnFromGraveyardToBattlefield { target } => {
                let Some(obj) = state.objects.get(*target) else { return; };
                if !matches!(obj.zone, Zone::Graveyard(_)) { return; }
                state.move_object_to_zone(
                    *target, Zone::Battlefield, MoveCause::SpellResolution);
            }
            Effect::ReturnFromGraveyardToHand { target } => {
                let Some(obj) = state.objects.get(*target) else { return; };
                if !matches!(obj.zone, Zone::Graveyard(_)) { return; }
                let owner = obj.owner;
                state.move_object_to_zone(
                    *target, Zone::Hand(owner), MoveCause::SpellResolution);
            }
            Effect::ExileFromGraveyard { target } => {
                let Some(obj) = state.objects.get(*target) else { return; };
                if !matches!(obj.zone, Zone::Graveyard(_)) { return; }
                state.move_object_to_zone(
                    *target, Zone::Exile, MoveCause::SpellResolution);
            }

            // --- library manipulation ------------------------------------
            Effect::Shuffle { player } => {
                if !valid_player(state, *player) { return; }
                state.shuffle_library(*player);
            }
            Effect::Scry { player, count } => {
                if !valid_player(state, *player) || *count == 0 { return; }
                let lib_len = state.player(*player).library_top_to_bottom.len();
                let actual = (*count as usize).min(lib_len);
                if actual == 0 {
                    // Empty library — emit Scry(0) for observability
                    // and skip the choice entirely (no cards to order).
                    state.emit(GameEvent::Scry { player: *player, count: 0 });
                    return;
                }
                let cards: Vec<ObjectId> = state.player(*player)
                    .library_top_to_bottom
                    .iter()
                    .take(actual)
                    .copied()
                    .collect();
                state.emit(GameEvent::Scry {
                    player: *player, count: actual as u32,
                });
                let stack_entry = state.currently_resolving
                    .expect("Effect::Scry: no currently_resolving stack \
                             entry — Scry must execute inside a stack \
                             resolution (set state.currently_resolving \
                             = Some(entry.id) before calling execute)");
                state.push_pending_choice(
                    *player,
                    crate::actions::ChoiceContext::ResolvingStack(stack_entry),
                    crate::actions::ChoiceKind::OrderCards {
                        cards,
                        allowed: vec![
                            crate::actions::CardDestination::TopOfLibrary,
                            crate::actions::CardDestination::BottomOfLibrary,
                        ],
                    },
                );
            }
            Effect::Surveil { player, count } => {
                if !valid_player(state, *player) || *count == 0 { return; }
                let lib_len = state.player(*player).library_top_to_bottom.len();
                let actual = (*count as usize).min(lib_len);
                if actual == 0 {
                    state.emit(GameEvent::Surveil { player: *player, count: 0 });
                    return;
                }
                let cards: Vec<ObjectId> = state.player(*player)
                    .library_top_to_bottom
                    .iter()
                    .take(actual)
                    .copied()
                    .collect();
                state.emit(GameEvent::Surveil {
                    player: *player, count: actual as u32,
                });
                let stack_entry = state.currently_resolving
                    .expect("Effect::Surveil: no currently_resolving stack \
                             entry — Surveil must execute inside a stack \
                             resolution");
                state.push_pending_choice(
                    *player,
                    crate::actions::ChoiceContext::ResolvingStack(stack_entry),
                    crate::actions::ChoiceKind::OrderCards {
                        cards,
                        allowed: vec![
                            crate::actions::CardDestination::TopOfLibrary,
                            crate::actions::CardDestination::Graveyard,
                        ],
                    },
                );
            }

            // --- tokens / copies -----------------------------------------
            Effect::CreateToken { controller, token } => {
                create_token(state, *controller, token);
            }
            Effect::CopySpell { target } => {
                copy_spell_on_stack(state, *target);
            }
            Effect::CopyPermanent { target } => {
                // TODO(decision): copies may choose to enter tapped,
                // token/nontoken, etc. For Phase 1 we produce a token
                // copy with the same characteristics.
                copy_permanent(state, *target);
            }
            Effect::Cascade { source, controller } => {
                cascade_resolve(state, *source, *controller);
            }

            // --- counters ------------------------------------------------
            Effect::AddCounters { target, kind, count } => {
                if state.objects.get(*target).is_none() { return; }
                state.place_counters(
                    crate::replacement::CounterTarget::Object(*target),
                    *kind, *count);
            }
            Effect::RemoveCounters { target, kind, count } => {
                let Some(obj) = state.objects.get_mut(*target) else { return; };
                let removed = obj.remove_counters(*kind, *count);
                if removed > 0 {
                    state.emit(GameEvent::CounterRemoved {
                        object_id: *target, kind: *kind, count: removed,
                    });
                }
            }
            Effect::Proliferate => {
                proliferate(state);
            }
            Effect::MoveCounter { from, to, kind, count } => {
                if *count == 0 || from == to { return; }
                if state.objects.get(*from).is_none()
                    || state.objects.get(*to).is_none()
                { return; }
                let removed = state.objects.get_mut(*from).unwrap()
                    .remove_counters(*kind, *count);
                if removed == 0 { return; }
                state.emit(GameEvent::CounterRemoved {
                    object_id: *from, kind: *kind, count: removed,
                });
                // Destination placement routes through the pipeline so
                // Hardened Scales et al. see it (CR 614).
                state.place_counters(
                    crate::replacement::CounterTarget::Object(*to),
                    *kind, removed);
            }

            // --- pump ----------------------------------------------------
            Effect::Pump { target, power, toughness, duration, keywords } => {
                // P/T modification is a layer-7c effect; each keyword
                // grant is a layer-6 effect.
                if *power != 0 || *toughness != 0 {
                    state.add_continuous_effect(
                        crate::layers::ContinuousEffect::pump(
                            /*source=*/ *target, *target,
                            *power, *toughness, *duration));
                }
                for kw in keywords {
                    state.add_continuous_effect(
                        crate::layers::ContinuousEffect::grant_keyword(
                            /*source=*/ *target, *target, kw.clone(), *duration));
                }
            }
            Effect::Anthem { controller, power, toughness, duration } => {
                state.add_continuous_effect(
                    crate::layers::ContinuousEffect::anthem(
                        /*source=*/ crate::objects::NULL_OBJECT_ID,
                        *controller, *power, *toughness, *duration));
            }
            Effect::InstallContinuousEffect { effect } => {
                state.add_continuous_effect(effect.clone());
            }
            Effect::GrantFlashbackToInstantOrSorceryInGraveyard {
                source, controller, duration,
            } => {
                snapcaster_grant_flashback(state, *source, *controller, *duration);
            }
            Effect::GrantKeyword { target, keyword, duration } => {
                state.add_continuous_effect(
                    crate::layers::ContinuousEffect::grant_keyword(
                        /*source=*/ *target, *target, keyword.clone(), *duration));
            }
            Effect::SetBasePT { target, power, toughness, duration } => {
                state.add_continuous_effect(
                    crate::layers::ContinuousEffect::set_pt(
                        /*source=*/ *target, *target,
                        *power, *toughness, *duration));
            }

            // --- stack ---------------------------------------------------
            Effect::Counter { target } => {
                counter_stack_entry(state, *target);
            }
            Effect::CastFromHandFree { player, target } => {
                cast_from_zone_free(
                    state, *player, *target,
                    |z| matches!(z, Zone::Hand(_)));
            }
            Effect::CastFromGraveyard { player, target } => {
                cast_from_zone_free(
                    state, *player, *target,
                    |z| matches!(z, Zone::Graveyard(_)));
            }

            // --- state flips --------------------------------------------
            Effect::ChangeControl { target, new_controller } => {
                let Some(obj) = state.objects.get_mut(*target) else { return; };
                let old = obj.controller;
                if old == *new_controller { return; }
                obj.controller = *new_controller;
                state.emit(GameEvent::ControlChanged {
                    object_id: *target, old, new_ctrl: *new_controller,
                });
            }
            Effect::Transform { target } => {
                let Some(obj) = state.objects.get_mut(*target) else { return; };
                obj.status.transformed = !obj.status.transformed;
                state.emit(GameEvent::Transformed { object_id: *target });
            }
            Effect::Tap { target } => {
                let Some(obj) = state.objects.get_mut(*target) else { return; };
                if obj.tap() {
                    state.emit(GameEvent::Tapped { object_id: *target });
                }
            }
            Effect::Untap { target } => {
                let Some(obj) = state.objects.get_mut(*target) else { return; };
                if obj.untap() {
                    state.emit(GameEvent::Untapped { object_id: *target });
                }
            }
            Effect::WardPrompt { caster, cost, counter_target } => {
                // Resolution of a Ward trigger. Push a PayOrDecline on
                // the targeting spell's controller; a decline routes
                // to CounterStackEntry on the original source.
                let stack_entry = state.currently_resolving
                    .expect("Effect::WardPrompt: no currently_resolving stack \
                             entry — Ward must execute inside the Ward \
                             trigger's resolution");
                state.push_pending_choice(
                    *caster,
                    crate::actions::ChoiceContext::ResolvingStack(stack_entry),
                    crate::actions::ChoiceKind::PayOrDecline {
                        cost: cost.clone(),
                        on_decline: crate::actions::DeclineConsequence::CounterStackEntry(
                            *counter_target),
                    },
                );
            }
            Effect::Attach { equipment_or_aura, target } => {
                attach(state, *equipment_or_aura, *target);
            }

            // --- mana / phases ------------------------------------------
            Effect::AddMana { player, mana } => {
                if !valid_player(state, *player) { return; }
                for u in mana {
                    state.emit(GameEvent::ManaAdded {
                        player: *player, color: u.color, amount: 1,
                    });
                    state.player_mut(*player).mana_pool.add(u.clone());
                }
            }
            Effect::ExtraTurn { player } => {
                if !valid_player(state, *player) { return; }
                state.turn.queue_extra_turn(*player);
            }
            Effect::AdditionalCombatPhase => {
                state.turn.queue_extra_combat();
            }
            Effect::SkipNextPhase { .. } => {
                // TODO(phase-manipulation): we don't track per-player
                // skip queues yet; engine.rs will consult this at
                // advance_phase. No-op for now.
            }
            Effect::PlayExtraLand { player, amount } => {
                if !valid_player(state, *player) || *amount == 0 { return; }
                let pl = state.player_mut(*player);
                pl.land_plays_remaining = pl.land_plays_remaining.saturating_add(*amount);
                pl.land_plays_per_turn = pl.land_plays_per_turn.saturating_add(*amount);
            }
            Effect::EmptyManaPool { player } => {
                if !valid_player(state, *player) { return; }
                state.player_mut(*player).mana_pool.clear();
            }
            Effect::CreateEmblem { controller, emblem } => {
                if !valid_player(state, *controller) { return; }
                create_emblem(state, *controller, emblem);
            }

            // --- choice-requiring (push PickCards + follow-up) ----------
            Effect::Search { player, zone, filter, destination, reveal } => {
                push_search_choice(state, *player, *zone, filter,
                    *destination, *reveal);
            }
            Effect::Reanimate { player, filter, from_zone } => {
                push_reanimate_choice(state, *player, *from_zone, filter);
            }
            Effect::Sacrifice { player, filter, count } => {
                push_sacrifice_choice(state, *player, filter, *count);
            }
            Effect::TutorToHand { player, filter, reveal } => {
                push_search_choice(
                    state, *player, Zone::Library(*player), filter,
                    Zone::Hand(*player), *reveal);
            }
            Effect::TutorToBattlefield { player, filter, tapped } => {
                push_tutor_to_battlefield_choice(
                    state, *player, filter, *tapped);
            }

            // --- fight ---------------------------------------------------
            Effect::Fight { a, b } => {
                fight(state, *a, *b);
            }
            Effect::Goad { target, goader, duration } => {
                state.add_continuous_effect(
                    crate::layers::ContinuousEffect::goad(
                        /*source=*/ *target, *target, *goader, *duration));
            }
            Effect::ForbidAttacking { target, duration } => {
                state.add_continuous_effect(
                    crate::layers::ContinuousEffect::cant_attack(
                        /*source=*/ *target, *target, *duration));
            }

            // --- composites ---------------------------------------------
            Effect::ForEach { targets, effect } => {
                for _id in targets {
                    // The inner effect is already specialized for each id
                    // by the card's effect builder — we just iterate.
                    effect.execute(state);
                }
            }
            Effect::Conditional { condition, then, otherwise } => {
                if condition.evaluate(state) {
                    then.execute(state);
                } else if let Some(e) = otherwise {
                    e.execute(state);
                }
            }
            Effect::Sequence(steps) => {
                for step in steps {
                    step.execute(state);
                    if state.is_game_over() { break; }
                }
            }
        }
    }
}

// =============================================================================
// DiscardChoice
// =============================================================================

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DiscardChoice {
    /// Opponent chooses which card is discarded (e.g. Raven's Crime).
    OpponentChooses,
    /// Controller chooses (the default for most discard effects).
    ControllerChooses,
    /// Random discard (e.g. Hymn to Tourach).
    Random,
}

// =============================================================================
// Condition
// =============================================================================

// TODO(serialize): `Condition::Custom` carries a bare `fn` pointer.
// Migrate per Section 12 in Phase 3.
#[derive(Clone, Debug)]
pub enum Condition {
    ControlPermanentMatching(ObjectFilter),
    OpponentControlsPermanentMatching(ObjectFilter),
    LifeAtOrAbove(PlayerId, i32),
    LifeAtOrBelow(PlayerId, i32),
    /// Number of cards in the player's hand meets `condition`.
    CardsInHand(PlayerId, CmcCondition),
    GraveyardCount(PlayerId, CmcCondition),
    IsYourTurn(PlayerId),
    Custom(fn(&GameState) -> bool),
}

impl Condition {
    /// Evaluate against current state. The optional "source controller"
    /// needed by some filters' `You`/`Opponent` is read from the
    /// relevant condition's explicit `PlayerId` field.
    pub fn evaluate(&self, state: &GameState) -> bool {
        match self {
            Condition::ControlPermanentMatching(filter) => {
                // Evaluate for every player — "do I control a matching
                // permanent" interpreted as the canonical "player 0"
                // requires context we don't carry. We instead interpret
                // this variant as "does *any* player control a matching
                // permanent whose controller is the filter's `You`" —
                // which degenerates to "exists a permanent matching the
                // filter with that controller".
                //
                // For an explicit "player P controls X" test, use
                // `OpponentControlsPermanentMatching` with the right
                // filter. In Phase 1 we keep this simple: the filter's
                // own controller constraint decides which controllers
                // count. We pass an arbitrary `source_controller = 0`;
                // filters that don't use `You`/`Opponent` ignore it.
                state.objects.iter().any(|o|
                    o.is_permanent_on_battlefield()
                    && filter.matches(o, state, /*source=*/ 0))
            }
            Condition::OpponentControlsPermanentMatching(filter) => {
                state.objects.iter().any(|o|
                    o.is_permanent_on_battlefield()
                    && filter.matches(o, state, /*source=*/ 0))
            }
            Condition::LifeAtOrAbove(p, n) => {
                valid_player(state, *p) && state.player(*p).life >= *n
            }
            Condition::LifeAtOrBelow(p, n) => {
                valid_player(state, *p) && state.player(*p).life <= *n
            }
            Condition::CardsInHand(p, cond) => {
                if !valid_player(state, *p) { return false; }
                let n = state.zone_count(Zone::Hand(*p)) as u32;
                cond.matches(n)
            }
            Condition::GraveyardCount(p, cond) => {
                if !valid_player(state, *p) { return false; }
                let n = state.zone_count(Zone::Graveyard(*p)) as u32;
                cond.matches(n)
            }
            Condition::IsYourTurn(p) => state.active_player() == *p,
            Condition::Custom(f) => f(state),
        }
    }
}

// =============================================================================
// TokenDefinition
// =============================================================================

/// Token definition for [`Effect::CreateToken`].
#[derive(Clone, Debug)]
pub struct TokenDefinition {
    pub name: SmallString,
    pub colors: ColorSet,
    pub types: TypeLine,
    pub subtypes: SubtypeSet,
    pub power: Option<PtValue>,
    pub toughness: Option<PtValue>,
    pub keywords: Vec<KeywordAbility>,
    pub abilities: Vec<crate::triggers::TriggeredAbilityDef>,
}

/// Definition for [`Effect::CreateEmblem`]. Emblems are objects in
/// the command zone with no characteristics beyond a name — their
/// behavior is entirely their `abilities` vector.
#[derive(Clone, Debug)]
pub struct EmblemDefinition {
    pub name: SmallString,
    pub abilities: Vec<crate::triggers::TriggeredAbilityDef>,
}

impl TokenDefinition {
    /// Build the [`Characteristics`] block a token would have on the
    /// battlefield.
    pub fn to_characteristics(&self) -> Characteristics {
        Characteristics {
            name: self.name,
            mana_cost: None,
            colors: self.colors,
            types: self.types,
            subtypes: self.subtypes.clone(),
            supertypes: SupertypeSet::default(),
            power: self.power,
            toughness: self.toughness,
            loyalty: None,
            abilities_text: Vec::new(),
            keywords: self.keywords.clone(),
            is_aura: false,
            is_fortification: false,
        }
    }
}

// =============================================================================
// KeywordAbility
// =============================================================================

#[derive(Clone, Debug, Hash, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum KeywordAbility {
    // Evergreen
    Flying, FirstStrike, DoubleStrike, Deathtouch,
    Haste, Hexproof, Indestructible, Lifelink, Menace,
    Reach, Trample, Vigilance, Ward(crate::mana::ManaCost), Flash,
    Defender,
    /// CR 702.16 — Protection from a quality. DEBT: can't be dealt
    /// damage by, equipped/enchanted by, blocked by, or targeted by
    /// sources that match the quality.
    Protection(ProtectionQuality),
    // Returning / frequent
    Convoke, Delve, Improvise, Prowess, Affinity(SubtypeFilter),
    Equip(crate::mana::ManaCost), Enchant(EnchantFilter),
    Cycling(crate::mana::ManaCost), Flashback(crate::mana::ManaCost),
    Kicker(crate::mana::ManaCost), Madness(crate::mana::ManaCost),
    Morph(crate::mana::ManaCost), Manifest,
    // Set-specific (extensible)
    Surveil(u32), Explore, Adapt(u32),
    Foretell(crate::mana::ManaCost), Learn, Connive,
    Discover(u32), Bargain, Offspring(crate::mana::ManaCost),
    Impending { mana_cost: crate::mana::ManaCost, time_counters: u32 },
    /// CR 702.40 — Storm. "When you cast this spell, copy it for each
    /// other spell that was cast before it this turn." Wired as a
    /// [`crate::triggers::TriggeredAbilityDef`] via
    /// [`crate::keywords::storm_trigger_def`].
    Storm,
    /// CR 702.85 — Cascade. "When you cast this spell, exile cards
    /// from the top of your library until you exile a nonland card
    /// with mana value less than this spell's. You may cast that
    /// card without paying its mana cost. Put the exiled cards on
    /// the bottom of your library in a random order." Wired via
    /// [`crate::keywords::cascade_trigger_def`].
    Cascade,
    Custom { name: String, implementation: KeywordImpl },
}

/// Placeholder for keyword implementation details. Filled in once the
/// registry-dispatched keyword system (Tasks #15-16) lands.
#[derive(Clone, Debug, Hash, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct KeywordImpl;

/// CR 702.16 — Quality a Protection keyword grants immunity against.
#[derive(Clone, Debug, Hash, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum ProtectionQuality {
    /// "Protection from red" — matches sources that are that color.
    Color(Color),
    /// "Protection from all colors" — any source with any color.
    AnyColor,
    /// "Protection from Goblins" — matches by subtype name.
    CreatureType(SmallString),
    /// "Protection from everything" (Pristine Angel-style).
    Everything,
}

impl ProtectionQuality {
    /// Does a source matching these characteristics carry this quality?
    pub fn matches_source(&self, source: &Characteristics) -> bool {
        match self {
            Self::Everything => true,
            Self::Color(c) => source.colors.contains(*c),
            Self::AnyColor => !source.colors.is_colorless(),
            Self::CreatureType(name) => source.subtypes.contains(*name),
        }
    }
}

/// Filter for subtypes (used by Affinity, etc.).
#[derive(Clone, Debug, Hash, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct SubtypeFilter(pub SmallString);

/// Filter for what an aura can enchant.
#[derive(Clone, Debug, Hash, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct EnchantFilter {
    pub filter: String,
}

// =============================================================================
// Helpers — private to this module
// =============================================================================

fn valid_player(state: &GameState, p: PlayerId) -> bool {
    (p as usize) < state.players.len()
}

/// CR 701.25 — greedy proliferate. Choose every permanent and player
/// that has a counter and add one of each kind already present.
/// Object counters emit `CounterAdded`; player counters currently
/// don't have a dedicated event (silent mutation) — add one when the
/// trigger system grows a poison/energy listener.
fn proliferate(state: &mut GameState) {
    // --- Permanents on the battlefield ---
    // Each (permanent, kind) placement is an independent
    // would-place-counters event per CR 614 — route each through
    // `place_counters` individually, not batched.
    let targets: Vec<(ObjectId, Vec<CounterKind>)> = state.objects
        .objects_in_zone(Zone::Battlefield)
        .filter_map(|o| {
            if o.counters.is_empty() { return None; }
            let mut kinds: Vec<CounterKind> = o.counters.keys().copied().collect();
            // Deterministic order so triggers replay identically.
            kinds.sort_by_key(|k| format!("{:?}", k));
            Some((o.id, kinds))
        })
        .collect();
    for (id, kinds) in targets {
        for kind in kinds {
            state.place_counters(
                crate::replacement::CounterTarget::Object(id), kind, 1);
        }
    }

    // --- Players with counters ---
    // Player-side counter types Phase 1 tracks: poison, energy,
    // experience. Poison/Energy route through `place_counters` so
    // any future player-target replacements apply uniformly.
    // `experience` has no `CounterKind` variant yet — incremented
    // directly until one is added.
    let np = state.num_players();
    for p in 0..np {
        let (has_poison, has_energy, has_experience) = {
            let pl = state.player(p);
            (pl.poison_counters > 0, pl.energy > 0, pl.experience > 0)
        };
        if has_poison {
            state.place_counters(
                crate::replacement::CounterTarget::Player(p),
                CounterKind::Poison, 1);
        }
        if has_energy {
            state.place_counters(
                crate::replacement::CounterTarget::Player(p),
                CounterKind::Energy, 1);
        }
        if has_experience {
            state.player_mut(p).experience += 1;
        }
    }
}

fn owner_of(state: &GameState, id: ObjectId) -> Option<PlayerId> {
    state.objects.get(id).map(|o| o.owner)
}

fn lose_life(state: &mut GameState, p: PlayerId, amount: u32) {
    if !valid_player(state, p) || amount == 0 { return; }
    state.player_mut(p).life -= amount as i32;
    state.emit(GameEvent::LifeLost { player: p, amount });
    // CR 704.5a (life ≤ 0 loses) is applied by SBA (Task #14), not
    // here — we only surface the life-loss event.
}

fn mill_one_card(state: &mut GameState, p: PlayerId) {
    if !valid_player(state, p) { return; }
    let Some(&top) = state.player(p).library_top_to_bottom.first() else {
        return;
    };
    state.move_object_to_zone(top, Zone::Graveyard(p), MoveCause::SpellResolution);
    state.emit(GameEvent::Milled { player: p, object_id: top });
}

fn discard_cards(
    state: &mut GameState,
    p: PlayerId,
    count: u32,
    choice: &DiscardChoice,
) {
    if !valid_player(state, p) || count == 0 { return; }
    let hand_ids = state.objects.ids_in_zone_sorted(Zone::Hand(p));
    if hand_ids.is_empty() { return; }
    let take = (count as usize).min(hand_ids.len());

    match choice {
        DiscardChoice::Random => {
            // Engine-chosen randomness — no agent decision needed.
            use rand::seq::SliceRandom;
            use rand::SeedableRng;
            let mut rng = rand_chacha::ChaCha8Rng::seed_from_u64(
                state.rng_seed.wrapping_add(p as u64 + 0x01));
            state.rng_seed = state.rng_seed.wrapping_add(1);
            let mut shuffled = hand_ids.clone();
            shuffled.shuffle(&mut rng);
            for id in shuffled.into_iter().take(take) {
                // Madness-aware discard helper: the card routes to
                // exile with `madness_pending=true` when it has the
                // Madness keyword, else to graveyard as before.
                let _ = state.discard_object(p, id, MoveCause::SpellResolution);
            }
        }
        DiscardChoice::ControllerChooses | DiscardChoice::OpponentChooses => {
            let choosing_player = match choice {
                DiscardChoice::ControllerChooses => p,
                DiscardChoice::OpponentChooses => {
                    // Single-opponent simplification: pick the player
                    // clockwise from `p` (APNAP order). Multiplayer
                    // "one chosen opponent" staging is Phase 2-B work.
                    let n = state.num_players();
                    (p + 1) % n
                }
                _ => unreachable!(),
            };
            let stack_entry = state.currently_resolving
                .expect("discard_cards: no currently_resolving stack \
                         entry — Discard must execute inside a stack \
                         resolution");
            state.pending_choice_follow_up = Some(
                crate::actions::ChoiceFollowUp::Discard { player: p });
            state.push_pending_choice(
                choosing_player,
                crate::actions::ChoiceContext::ResolvingStack(stack_entry),
                crate::actions::ChoiceKind::PickCards {
                    candidates: hand_ids,
                    min: take as u32,
                    max: take as u32,
                },
            );
        }
    }
}

fn create_token(
    state: &mut GameState,
    controller: PlayerId,
    token: &TokenDefinition,
) {
    if !valid_player(state, controller) { return; }
    let id = state.allocate_object_id();
    // Token owner = controller (CR 110.5a — "the player who created
    // a token is that token's owner"). We're minting a new object so
    // there's no upstream from-zone; use Stack as the conventional
    // pretend-origin for the ETB event, matching the cast path.
    let mut obj = GameObject::new(
        id, controller, Zone::Battlefield, /*card_id=*/ 0,
        token.to_characteristics(),
    );
    obj.is_token = true;
    state.objects.insert(obj);
    state.emit(GameEvent::TokenCreated { object_id: id, controller });
    // Tokens are fair game for global ETB replacements (Hardened
    // Scales, enter-tapped fields, etc.); route through the same
    // hook the cast/SBA paths use.
    state.after_enter_battlefield(id);
    state.emit(GameEvent::EntersBattlefield {
        object_id: id, from_zone: Zone::Stack, was_cast: false,
    });
}

fn manifest_top_of_library(state: &mut GameState, p: PlayerId) {
    if !valid_player(state, p) { return; }
    let top = state.player(p).library_top_to_bottom.first().copied();
    let Some(id) = top else { return; };
    // Re-id happens on the move; address the resulting battlefield
    // object by the new id the mover returns.
    let Some(new_id) = state.move_object_to_zone(
        id, Zone::Battlefield, MoveCause::SpellResolution) else { return; };
    let Some(obj) = state.objects.get_mut(new_id) else { return; };
    obj.status.face_down = true;
    obj.status.summoning_sick = true;
    // Save the original characteristics for when the card flips back
    // face-up; expose the face-down 2/2 vanilla profile for now.
    obj.characteristics = Characteristics {
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        types: TypeLine::new().with(TypeLine::CREATURE),
        ..Default::default()
    };
}

fn install_regenerate_shield(state: &mut GameState, target: ObjectId) {
    use crate::replacement::{
        ReplacementCondition, ReplacementDuration,
        ReplacementEffect, ReplacementKind,
    };
    state.add_replacement_effect(ReplacementEffect {
        source: target,
        id: 0,
        condition: ReplacementCondition::WouldDieSpecific {
            object_id: target,
        },
        kind: ReplacementKind::RegenerateShield,
        is_self_replacement: true,
        duration: ReplacementDuration::EndOfTurn,
    });
}

fn attach(state: &mut GameState, attacher: ObjectId, target: ObjectId) {
    if state.objects.get(attacher).is_none()
        || state.objects.get(target).is_none()
    { return; }
    // CR 702.16e — Protection prevents being enchanted/equipped by
    // qualifying sources. Once Protection quality matching lands, it
    // will gate this step; the detector is wired in `Protection` task.
    if state.is_protected_from_attachment(target, attacher) {
        return;
    }
    // Detach from prior target, if any.
    if let Some(prev) = state.objects.get(attacher).and_then(|o| o.attached_to) {
        if let Some(holder) = state.objects.get_mut(prev) {
            holder.attachments.retain(|&id| id != attacher);
        }
        state.emit(GameEvent::Detached {
            equipment_or_aura: attacher, from: prev,
        });
    }
    // Wire the new relationship.
    state.objects.get_mut(attacher).unwrap().attached_to = Some(target);
    state.objects.get_mut(target).unwrap().attachments.push(attacher);
    state.emit(GameEvent::AttachedTo {
        equipment_or_aura: attacher, target,
    });
}

fn create_emblem(
    state: &mut GameState,
    controller: PlayerId,
    emblem: &EmblemDefinition,
) {
    let id = state.allocate_object_id();
    let chars = Characteristics {
        name: emblem.name,
        ..Default::default()
    };
    let obj = GameObject::new(
        id, controller, Zone::Command, /*card_id=*/ 0, chars);
    state.objects.insert(obj);
    // TODO(events): add a dedicated `EmblemCreated` event once
    // triggers that watch for emblem creation land. For Phase 1 the
    // emblem is silent — its abilities register their own triggers.
    let _ = &emblem.abilities; // placeholder; ability registration
                                // happens through the trigger system
                                // when it grows an emblem path.
}

/// Push a copy of `target` onto the stack. If the original has any
/// target requirements, push a [`crate::actions::ChoiceKind::ChooseTargets`]
/// so the controller picks new targets per CR 706.10. Otherwise the
/// copy resolves immediately with empty targets.
fn copy_spell_on_stack(state: &mut GameState, target: ObjectId) {
    let Some(entry) = state.find_stack_entry(target).cloned() else { return; };
    let new_id = state.allocate_object_id();
    // Mirror the original's GameObject in the arena under the new id so
    // resolution/counter paths (which look up via state.objects) find
    // the copy. Copies are tokens that cease to exist when leaving the
    // stack (CR 112.7), but Phase 2 parks them in the graveyard-path
    // just like any other spell — that path needs the arena entry.
    let Some(src_obj) = state.objects.get(target).cloned() else { return; };
    let mut copy_obj = GameObject::new(
        new_id, src_obj.controller, Zone::Stack, src_obj.card_id,
        src_obj.characteristics.clone());
    copy_obj.owner = src_obj.owner;
    state.objects.insert(copy_obj);
    let mut copy = entry;
    copy.id = new_id;
    copy.source = new_id;
    let copy_controller = copy.controller;
    let requirements = copy.target_requirements.clone();
    state.push_stack_entry(copy);
    state.emit(GameEvent::CopyCreated { object_id: new_id, copying: target });

    if !requirements.is_empty() {
        state.pending_target_requirements = Some(requirements);
        state.pending_choice_follow_up = Some(
            crate::actions::ChoiceFollowUp::ApplyTargetsToStackEntry {
                entry_id: new_id,
            });
        state.push_pending_choice(
            copy_controller,
            crate::actions::ChoiceContext::ResolvingStack(new_id),
            crate::actions::ChoiceKind::ChooseTargets { source: new_id },
        );
    }
}

/// Snapcaster's ETB handler. Enumerates every instant or sorcery
/// card in any graveyard and pushes a `PickCards` prompt so the
/// controller picks one. The `GrantFlashbackEqualToOwnManaCost`
/// follow-up, invoked after the agent answers, installs the layer-6
/// grant on the chosen object.
///
/// Snapcaster picks a graveyard card with a `PickCards` prompt
/// rather than through the general TargetedTrigger machinery
/// ([`crate::triggers::TriggeredAbilityDef::target_requirements`]):
/// that machinery targets battlefield / stack / player objects
/// via [`crate::targets::TargetFilter`], whereas "target card in a
/// graveyard" needs graveyard-zone iteration that the current
/// TargetFilter set doesn't express. The PickCards stopgap is
/// fine at seed-card scale; if a second "target-in-graveyard"
/// triggered ability shows up we'll promote it to TargetFilter
/// proper.
fn snapcaster_grant_flashback(
    state: &mut GameState,
    source: ObjectId,
    controller: PlayerId,
    duration: crate::layers::Duration,
) {
    use crate::types::TypeLine;
    // Enumerate instants/sorceries across *every* player's graveyard
    // — Snapcaster's printed text says "a graveyard" without a
    // controller restriction.
    let mut candidates: Vec<ObjectId> = Vec::new();
    for p in 0..state.num_players() {
        for obj in state.objects.objects_in_zone(Zone::Graveyard(p)) {
            if obj.characteristics.types.is_instant()
                || obj.characteristics.types.is_sorcery()
            {
                candidates.push(obj.id);
            }
        }
    }
    // Stable ordering — mirrors the arena iteration pattern used
    // elsewhere in `legal_actions`.
    candidates.sort_unstable();

    state.pending_choice_follow_up = Some(
        crate::actions::ChoiceFollowUp::GrantFlashbackEqualToOwnManaCost {
            source, duration,
        });
    state.push_pending_choice(
        controller,
        crate::actions::ChoiceContext::ResolvingStack(source),
        crate::actions::ChoiceKind::PickCards {
            candidates,
            min: 0,
            max: 1,
        },
    );
    // Silence unused warning on TypeLine import when built without
    // the type predicates it fuels — those are enabled today, but
    // making the dependency explicit helps future-me.
    let _ = TypeLine::INSTANT;
}

/// CR 702.85 — cascade resolution.
///
/// Exile cards off the top of `controller`'s library until a nonland
/// with mana value strictly less than `source`'s MV appears (or the
/// library runs out). If found, stash a [`crate::state::PendingCascade`]
/// and push a [`crate::actions::ChoiceKind::YesNo`] may-cast prompt.
/// The dispatcher's YesNo-in-cascade-context arm handles both
/// outcomes, including the seeded-random bottom shuffle of the
/// non-cast exiles.
///
/// If no valid hit exists (e.g. all remaining library cards are
/// lands, or library ran out), all exiled cards go directly to the
/// bottom in random order without a prompt.
fn cascade_resolve(
    state: &mut GameState,
    source: ObjectId,
    controller: PlayerId,
) {
    if !valid_player(state, controller) { return; }

    // Source mana value — read from the source stack entry's
    // characteristics (for spells) or from the source object.
    let source_mv = state.stack.iter()
        .find(|e| e.id == source)
        .and_then(|e| match &e.kind {
            crate::stack::StackEntryKind::Spell { characteristics, .. } =>
                characteristics.mana_cost.as_ref().map(|c| c.mana_value()),
            _ => None,
        })
        .or_else(|| state.objects.get(source).map(|o| o.characteristics.mana_value()))
        .unwrap_or(0);

    let mut other_exiled: Vec<ObjectId> = Vec::new();
    let mut hit: Option<ObjectId> = None;

    // Exile off the top until a valid hit or library empties.
    while !state.player(controller).library_top_to_bottom.is_empty() {
        let top_id = state.player_mut(controller)
            .library_top_to_bottom.remove(0);
        let (is_land, mv) = state.objects.get(top_id)
            .map(|o| (o.is_land(), o.characteristics.mana_value()))
            .unwrap_or((false, 0));
        // Exile the card (move to Zone::Exile via the standard path so
        // zone-change triggers fire and LKI is maintained).
        let new_id = state.move_object_to_zone(
            top_id, Zone::Exile, crate::events::MoveCause::SpellResolution,
        );
        let exiled_id = new_id.unwrap_or(top_id);
        if !is_land && mv < source_mv {
            hit = Some(exiled_id);
            break;
        }
        other_exiled.push(exiled_id);
    }

    match hit {
        Some(hit_id) => {
            state.pending_cascade = Some(crate::state::PendingCascade {
                controller,
                hit: hit_id,
                other_exiled,
            });
            // Emit the may-cast prompt. The dispatcher consumes
            // pending_cascade on the YesNo response.
            state.push_pending_choice(
                controller,
                crate::actions::ChoiceContext::ResolvingStack(source),
                crate::actions::ChoiceKind::YesNo {
                    prompt: /*interned text hook — 0 is "cascade may-cast"*/ 0,
                },
            );
        }
        None => {
            // No valid hit — all exiled go to the bottom in random order.
            cascade_shuffle_to_bottom(state, controller, other_exiled);
        }
    }
}

/// Seeded-random bottom shuffle for cascade's non-cast exiles.
/// Advances `state.rng_seed` like [`GameState::shuffle_library`] does
/// so two cascades in the same game produce independent orderings.
pub(crate) fn cascade_shuffle_to_bottom(
    state: &mut GameState,
    controller: PlayerId,
    mut exiled: Vec<ObjectId>,
) {
    if exiled.is_empty() { return; }
    use rand::seq::SliceRandom;
    use rand::SeedableRng;
    let mut rng = rand_chacha::ChaCha8Rng::seed_from_u64(
        state.rng_seed.wrapping_add(controller as u64 + 0x5A));
    state.rng_seed = state.rng_seed.wrapping_add(1);
    exiled.shuffle(&mut rng);
    // Move each to the bottom of the library in shuffled order.
    for id in exiled {
        // The card is in Zone::Exile; move to Library.
        let new_id = state.move_object_to_zone(
            id, Zone::Library(controller),
            crate::events::MoveCause::SpellResolution,
        );
        // move_object_to_zone pushes onto the back of the library
        // via swap_to_zone_reid's default placement — which for
        // Library appends. Since we're iterating in shuffle order,
        // the final order respects the shuffle.
        let _ = new_id;
    }
}

fn copy_permanent(state: &mut GameState, target: ObjectId) {
    let Some(src) = state.objects.get(target).cloned() else { return; };
    let id = state.allocate_object_id();
    let mut token = GameObject::new(
        id, src.controller, Zone::Battlefield, /*card_id=*/ 0,
        src.characteristics.clone(),
    );
    token.is_token = true;
    state.objects.insert(token);
    state.emit(GameEvent::TokenCreated { object_id: id, controller: src.controller });
    state.emit(GameEvent::CopyCreated { object_id: id, copying: target });
    state.after_enter_battlefield(id);
    state.emit(GameEvent::EntersBattlefield {
        object_id: id, from_zone: Zone::Stack, was_cast: false,
    });
}

fn counter_stack_entry(state: &mut GameState, target: ObjectId) {
    let Some(entry) = state.remove_stack_entry_by_id(target) else { return; };
    if entry.is_spell() {
        state.counter_resolved_spell(entry);
    } else {
        state.counter_resolved_ability(entry);
    }
}

/// Shared body for [`Effect::CastFromHandFree`] and
/// [`Effect::CastFromGraveyard`]. `zone_ok` is the origin-zone
/// predicate; the cast is aborted if `target` isn't currently in an
/// acceptable zone. No targets/modes are chosen (Phase 1 limitation,
/// TODO(decision)); the cast pays no mana.
fn cast_from_zone_free<F: Fn(Zone) -> bool>(
    state: &mut GameState,
    player: PlayerId,
    target: ObjectId,
    zone_ok: F,
) {
    if !valid_player(state, player) { return; }
    let Some(obj) = state.objects.get(target) else { return; };
    if !zone_ok(obj.zone) { return; }
    if obj.is_land() { return; } // Lands aren't cast (CR 305.1).
    let entry_id = state.announce_spell_on_stack(
        target, player, crate::targets::TargetSelection::new(),
        Vec::new(), None, vec![]);
    state.emit_spell_cast(entry_id);
}

/// Collect candidate ids from `source_zone` that match `filter` and
/// are owned by `p`. Used by the search/tutor/reanimate migration.
fn collect_matching_candidates(
    state: &GameState,
    p: PlayerId,
    source_zone: Zone,
    filter: &ObjectFilter,
) -> Vec<ObjectId> {
    let mut ids: Vec<ObjectId> = state.objects.iter()
        .filter(|o| o.zone.same_kind(source_zone)
            && o.owner == p
            && filter.matches(o, state, p))
        .map(|o| o.id)
        .collect();
    ids.sort();
    ids
}

/// Push a PickCards choice for a Search / TutorToHand effect. If no
/// candidates match, still emit SearchedLibrary and (for tutor-shaped
/// searches of the player's own library) shuffle. The follow-up sends
/// the picked card to `destination`.
fn push_search_choice(
    state: &mut GameState,
    player: PlayerId,
    source_zone: Zone,
    filter: &ObjectFilter,
    destination: Zone,
    reveal: bool,
) {
    if !valid_player(state, player) { return; }
    let candidates = collect_matching_candidates(
        state, player, source_zone, filter);
    let shuffle_owner = match source_zone {
        Zone::Library(o) => Some(o),
        _ => None,
    };
    state.emit(GameEvent::SearchedLibrary {
        searching_player: player, library_owner: player,
    });
    if candidates.is_empty() {
        // No candidates: still shuffle per CR 701.20b.
        if let Some(o) = shuffle_owner {
            state.shuffle_library(o);
        }
        return;
    }
    let stack_entry = state.currently_resolving
        .expect("push_search_choice: no currently_resolving stack entry");
    state.pending_choice_follow_up = Some(
        crate::actions::ChoiceFollowUp::MoveToZone {
            destination, reveal, shuffle_library_owner: shuffle_owner,
        });
    state.push_pending_choice(
        player,
        crate::actions::ChoiceContext::ResolvingStack(stack_entry),
        crate::actions::ChoiceKind::PickCards {
            candidates, min: 0, max: 1,
        },
    );
}

/// Push a PickCards choice for TutorToBattlefield. Follow-up transfers
/// control to the searching player and optionally taps the entrant.
fn push_tutor_to_battlefield_choice(
    state: &mut GameState,
    player: PlayerId,
    filter: &ObjectFilter,
    tapped: bool,
) {
    if !valid_player(state, player) { return; }
    let source_zone = Zone::Library(player);
    let candidates = collect_matching_candidates(
        state, player, source_zone, filter);
    state.emit(GameEvent::SearchedLibrary {
        searching_player: player, library_owner: player,
    });
    if candidates.is_empty() {
        state.shuffle_library(player);
        return;
    }
    let stack_entry = state.currently_resolving
        .expect("push_tutor_to_battlefield_choice: no currently_resolving \
                 stack entry");
    state.pending_choice_follow_up = Some(
        crate::actions::ChoiceFollowUp::MoveToBattlefield {
            controller: player, tapped,
            shuffle_library_owner: Some(player),
        });
    state.push_pending_choice(
        player,
        crate::actions::ChoiceContext::ResolvingStack(stack_entry),
        crate::actions::ChoiceKind::PickCards {
            candidates, min: 0, max: 1,
        },
    );
}

/// Push a PickCards choice for a Reanimate-style effect. Follow-up
/// puts the card onto the battlefield under `player`'s control.
fn push_reanimate_choice(
    state: &mut GameState,
    player: PlayerId,
    from_zone: Zone,
    filter: &ObjectFilter,
) {
    if !valid_player(state, player) { return; }
    // Reanimate doesn't restrict by owner — any graveyard card matching
    // the filter is fair game (cf. Animate Dead). Collect broadly.
    let mut candidates: Vec<ObjectId> = state.objects.iter()
        .filter(|o| o.zone.same_kind(from_zone)
            && filter.matches(o, state, player))
        .map(|o| o.id)
        .collect();
    candidates.sort();
    if candidates.is_empty() { return; }
    let stack_entry = state.currently_resolving
        .expect("push_reanimate_choice: no currently_resolving stack entry");
    state.pending_choice_follow_up = Some(
        crate::actions::ChoiceFollowUp::MoveToBattlefield {
            controller: player, tapped: false,
            shuffle_library_owner: None,
        });
    state.push_pending_choice(
        player,
        crate::actions::ChoiceContext::ResolvingStack(stack_entry),
        crate::actions::ChoiceKind::PickCards {
            candidates, min: 0, max: 1,
        },
    );
}

/// Push a PickCards choice for a Sacrifice effect. `count` is the
/// minimum number the player must sacrifice, clamped to however many
/// permanents match (CR 701.16 — can't sacrifice more than you have).
fn push_sacrifice_choice(
    state: &mut GameState,
    player: PlayerId,
    filter: &ObjectFilter,
    count: u32,
) {
    if !valid_player(state, player) || count == 0 { return; }
    let candidates: Vec<ObjectId> = state.objects.iter()
        .filter(|o| o.is_permanent_on_battlefield()
            && o.controller == player
            && filter.matches(o, state, player))
        .map(|o| o.id)
        .collect();
    let required = (count as usize).min(candidates.len()) as u32;
    if required == 0 { return; }
    let stack_entry = state.currently_resolving
        .expect("push_sacrifice_choice: no currently_resolving stack entry");
    state.pending_choice_follow_up = Some(
        crate::actions::ChoiceFollowUp::Sacrifice { player });
    state.push_pending_choice(
        player,
        crate::actions::ChoiceContext::ResolvingStack(stack_entry),
        crate::actions::ChoiceKind::PickCards {
            candidates, min: required, max: required,
        },
    );
}

fn fight(state: &mut GameState, a: ObjectId, b: ObjectId) {
    let Some(a_power) = state.computed_power(a) else { return; };
    let Some(b_power) = state.computed_power(b) else { return; };
    // Each deals damage equal to its power to the other (CR 701.12).
    if a_power > 0 {
        state.deal_damage(a, DamageTarget::Object(b), a_power as u32, false);
    }
    if b_power > 0 {
        state.deal_damage(b, DamageTarget::Object(a), b_power as u32, false);
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mana::{ManaCost, ManaUnit};
    use crate::stack::{ModeChoice, StackEntry};
    use crate::targets::TargetSelection;

    // --- helpers -------------------------------------------------------------

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

    fn put_creature(state: &mut GameState, owner: PlayerId, zone: Zone, p: i32, t: i32)
        -> ObjectId
    {
        let id = state.allocate_object_id();
        let mut obj = GameObject::new(id, owner, zone, 1, creature_chars(p, t));
        obj.controller = owner;
        state.objects.insert(obj);
        if let Zone::Library(lib_owner) = zone {
            state.player_mut(lib_owner).library_top_to_bottom.push(id);
        }
        id
    }

    fn put_instant(state: &mut GameState, owner: PlayerId, zone: Zone) -> ObjectId {
        let id = state.allocate_object_id();
        let chars = Characteristics {
            mana_cost: Some(ManaCost::parse("{R}").unwrap()),
            types: TypeLine::INSTANT.into(),
            ..Default::default()
        };
        state.objects.insert(GameObject::new(id, owner, zone, 2, chars));
        if let Zone::Library(lib_owner) = zone {
            state.player_mut(lib_owner).library_top_to_bottom.push(id);
        }
        id
    }

    // --- damage / life ------------------------------------------------------

    #[test]
    fn deal_damage_to_creature_marks_damage() {
        let mut s = GameState::new(2, 0);
        let c = put_creature(&mut s, 0, Zone::Battlefield, 2, 2);
        Effect::DealDamage {
            source: 999,
            target: DamageTarget::Object(c),
            amount: 3,
        }.execute(&mut s);
        assert_eq!(s.objects.get(c).unwrap().damage_marked, 3);
        assert!(s.event_log.iter().any(|e| matches!(e,
            GameEvent::DamageDealt { target: DamageTarget::Object(id), amount: 3, .. }
                if *id == c)));
    }

    #[test]
    fn deal_damage_to_player_reduces_life_and_emits_life_lost() {
        let mut s = GameState::new(2, 0);
        Effect::DealDamage {
            source: 1, target: DamageTarget::Player(1), amount: 4,
        }.execute(&mut s);
        assert_eq!(s.player(1).life, 16);
        assert!(s.event_log.iter().any(|e| matches!(e,
            GameEvent::LifeLost { player: 1, amount: 4 })));
        assert!(s.event_log.iter().any(|e| matches!(e,
            GameEvent::DamageDealt { target: DamageTarget::Player(1), .. })));
    }

    #[test]
    fn gain_life_and_lose_life() {
        let mut s = GameState::new(2, 0);
        Effect::GainLife { player: 0, amount: 5 }.execute(&mut s);
        assert_eq!(s.player(0).life, 25);
        Effect::LoseLife { player: 0, amount: 10 }.execute(&mut s);
        assert_eq!(s.player(0).life, 15);
    }

    #[test]
    fn set_life_total_emits_life_set() {
        let mut s = GameState::new(2, 0);
        Effect::SetLifeTotal { player: 0, amount: 7 }.execute(&mut s);
        assert_eq!(s.player(0).life, 7);
        assert!(s.event_log.iter().any(|e| matches!(e,
            GameEvent::LifeSet { player: 0, old: 20, new_total: 7 })));
    }

    // --- anthem / grant-keyword / set-base-pt -----------------------------

    #[test]
    fn anthem_buffs_creatures_of_named_controller() {
        use crate::layers::Duration;
        let mut s = GameState::new(2, 0);
        let mine = put_creature(&mut s, 0, Zone::Battlefield, 2, 2);
        let other_mine = put_creature(&mut s, 0, Zone::Battlefield, 1, 1);
        let theirs = put_creature(&mut s, 1, Zone::Battlefield, 3, 3);

        Effect::Anthem {
            controller: 0, power: 1, toughness: 1,
            duration: Duration::EndOfTurn,
        }.execute(&mut s);

        assert_eq!(s.computed_power(mine), Some(3));
        assert_eq!(s.computed_toughness(mine), Some(3));
        assert_eq!(s.computed_power(other_mine), Some(2));
        assert_eq!(s.computed_power(theirs), Some(3)); // untouched
    }

    #[test]
    fn grant_keyword_adds_keyword_to_target() {
        use crate::layers::Duration;
        let mut s = GameState::new(2, 0);
        let c = put_creature(&mut s, 0, Zone::Battlefield, 2, 2);
        Effect::GrantKeyword {
            target: c,
            keyword: KeywordAbility::Flying,
            duration: Duration::EndOfTurn,
        }.execute(&mut s);
        assert!(s.has_keyword(c, &KeywordAbility::Flying));
    }

    #[test]
    fn set_base_pt_overrides_both_power_and_toughness() {
        use crate::layers::Duration;
        let mut s = GameState::new(2, 0);
        let c = put_creature(&mut s, 0, Zone::Battlefield, 5, 5);
        Effect::SetBasePT {
            target: c, power: 1, toughness: 1,
            duration: Duration::EndOfTurn,
        }.execute(&mut s);
        assert_eq!(s.computed_power(c), Some(1));
        assert_eq!(s.computed_toughness(c), Some(1));
    }

    #[test]
    fn set_base_pt_then_pump_stacks_correctly() {
        // Layer 7b runs before 7c: SetBasePT to 1/1, then pump +2/+2
        // gives 3/3.
        use crate::layers::Duration;
        let mut s = GameState::new(2, 0);
        let c = put_creature(&mut s, 0, Zone::Battlefield, 5, 5);
        Effect::SetBasePT {
            target: c, power: 1, toughness: 1,
            duration: Duration::EndOfTurn,
        }.execute(&mut s);
        Effect::Pump {
            target: c, power: 2, toughness: 2,
            duration: Duration::EndOfTurn, keywords: vec![],
        }.execute(&mut s);
        assert_eq!(s.computed_power(c), Some(3));
        assert_eq!(s.computed_toughness(c), Some(3));
    }

    // --- goad / forbid-attacking ------------------------------------------

    #[test]
    fn forbid_attacking_installs_cant_attack_restriction() {
        use crate::layers::Duration;
        let mut s = GameState::new(2, 0);
        let c = put_creature(&mut s, 0, Zone::Battlefield, 2, 2);
        Effect::ForbidAttacking {
            target: c,
            duration: Duration::EndOfTurn,
        }.execute(&mut s);
        assert!(s.cant_attack(c));
    }

    #[test]
    fn goad_records_goader_in_state() {
        use crate::layers::Duration;
        let mut s = GameState::new(3, 0);
        let c = put_creature(&mut s, 1, Zone::Battlefield, 2, 2);
        Effect::Goad {
            target: c,
            goader: 0,
            duration: Duration::UntilYourNextTurn(0),
        }.execute(&mut s);
        assert_eq!(s.goaders_of(c), vec![0]);
    }

    #[test]
    fn goad_accumulates_multiple_goaders() {
        use crate::layers::Duration;
        let mut s = GameState::new(3, 0);
        let c = put_creature(&mut s, 2, Zone::Battlefield, 2, 2);
        Effect::Goad { target: c, goader: 0, duration: Duration::EndOfTurn }
            .execute(&mut s);
        Effect::Goad { target: c, goader: 1, duration: Duration::EndOfTurn }
            .execute(&mut s);
        let mut gs = s.goaders_of(c);
        gs.sort();
        assert_eq!(gs, vec![0, 1]);
    }

    // --- damage prevention / redirection ----------------------------------

    #[test]
    fn prevent_damage_up_to_absorbs_then_lets_excess_through() {
        use crate::replacement::ReplacementDuration;
        let mut s = GameState::new(2, 0);
        // "Prevent the next 3 damage that would be dealt to player 1."
        Effect::PreventDamage {
            target: DamageTarget::Player(1),
            amount: Some(3),
            duration: ReplacementDuration::EndOfTurn,
        }.execute(&mut s);

        // 5 damage dealt; 3 prevented, 2 gets through.
        Effect::DealDamage {
            source: 99,
            target: DamageTarget::Player(1),
            amount: 5,
        }.execute(&mut s);
        assert_eq!(s.player(1).life, 20 - 2);
    }

    #[test]
    fn prevent_damage_all_blocks_entirely() {
        use crate::replacement::ReplacementDuration;
        let mut s = GameState::new(2, 0);
        Effect::PreventDamage {
            target: DamageTarget::Player(1),
            amount: None,  // prevent all
            duration: ReplacementDuration::EndOfTurn,
        }.execute(&mut s);

        Effect::DealDamage {
            source: 99,
            target: DamageTarget::Player(1),
            amount: 5,
        }.execute(&mut s);
        assert_eq!(s.player(1).life, 20);
    }

    #[test]
    fn prevent_damage_only_shields_named_target() {
        use crate::replacement::ReplacementDuration;
        let mut s = GameState::new(2, 0);
        Effect::PreventDamage {
            target: DamageTarget::Player(1),
            amount: None,
            duration: ReplacementDuration::EndOfTurn,
        }.execute(&mut s);

        // Damage to a different player goes through.
        Effect::DealDamage {
            source: 99, target: DamageTarget::Player(0), amount: 3,
        }.execute(&mut s);
        assert_eq!(s.player(0).life, 17);
    }

    #[test]
    fn manifest_puts_top_of_library_onto_battlefield_face_down() {
        let mut s = GameState::new(2, 0);
        // Seed a single card on top of library — originally an instant,
        // to confirm Manifest overrides its characteristics.
        let top = put_instant(&mut s, 0, Zone::Library(0));
        s.player_mut(0).library_top_to_bottom.push(top);

        Effect::Manifest { player: 0 }.execute(&mut s);

        let obj = s.objects.objects_in_zone(Zone::Battlefield).next().unwrap();
        assert!(obj.status.face_down);
        assert!(obj.is_creature());
        assert_eq!(obj.characteristics.power, Some(PtValue::Fixed(2)));
        assert_eq!(obj.characteristics.toughness, Some(PtValue::Fixed(2)));
        // Library empty; the old id is gone from the arena (re-id'd).
        assert!(s.player(0).library_top_to_bottom.is_empty());
        assert!(s.objects.get(top).is_none());
    }

    #[test]
    fn manifest_empty_library_is_noop() {
        let mut s = GameState::new(2, 0);
        Effect::Manifest { player: 0 }.execute(&mut s);
        // No panic, no state change.
        assert_eq!(s.objects.count_in_zone(Zone::Battlefield), 0);
    }

    #[test]
    fn regenerate_saves_creature_from_lethal_damage() {
        let mut s = GameState::new(2, 0);
        let c = put_creature(&mut s, 0, Zone::Battlefield, 2, 2);
        Effect::Regenerate { target: c }.execute(&mut s);

        // Deal lethal damage.
        s.objects.get_mut(c).unwrap().mark_damage(5);
        crate::sba::apply_state_based_actions(&mut s);

        // Creature is still on the battlefield: regenerated.
        assert_eq!(s.objects.get(c).unwrap().zone, Zone::Battlefield);
        // Damage cleared and tapped.
        assert_eq!(s.objects.get(c).unwrap().damage_marked, 0);
        assert!(s.objects.get(c).unwrap().is_tapped());
    }

    #[test]
    fn regenerate_shield_is_consumed_after_firing() {
        let mut s = GameState::new(2, 0);
        let c = put_creature(&mut s, 0, Zone::Battlefield, 2, 2);
        Effect::Regenerate { target: c }.execute(&mut s);

        s.objects.get_mut(c).unwrap().mark_damage(5);
        crate::sba::apply_state_based_actions(&mut s);
        // First lethal: regenerated (no zone change → id stable).
        assert_eq!(s.objects.get(c).unwrap().zone, Zone::Battlefield);
        // Re-damage lethally.
        s.objects.get_mut(c).unwrap().mark_damage(5);
        crate::sba::apply_state_based_actions(&mut s);
        // Shield was consumed → creature now dies and is re-id'd.
        assert_eq!(s.zone_count(Zone::Graveyard(0)), 1);
        assert_eq!(s.zone_count(Zone::Battlefield), 0);
    }

    #[test]
    fn redirect_damage_moves_damage_to_new_target() {
        use crate::replacement::ReplacementDuration;
        let mut s = GameState::new(2, 0);
        Effect::RedirectDamage {
            from: DamageTarget::Player(0),
            to: DamageTarget::Player(1),
            duration: ReplacementDuration::EndOfTurn,
        }.execute(&mut s);

        Effect::DealDamage {
            source: 99, target: DamageTarget::Player(0), amount: 4,
        }.execute(&mut s);
        // Player 0 untouched; player 1 ate the redirected damage.
        assert_eq!(s.player(0).life, 20);
        assert_eq!(s.player(1).life, 16);
    }

    // --- zone changes -------------------------------------------------------

    #[test]
    fn destroy_creature_goes_to_graveyard_and_emits_dies() {
        let mut s = GameState::new(2, 0);
        let c = put_creature(&mut s, 0, Zone::Battlefield, 2, 2);
        Effect::DestroyPermanent { target: c }.execute(&mut s);
        assert_eq!(s.zone_count(Zone::Graveyard(0)), 1);
        assert_eq!(s.zone_count(Zone::Battlefield), 0);
        assert!(s.event_log.iter().any(|e| matches!(e,
            GameEvent::Dies { object_id } if *object_id == c)));
        assert!(s.event_log.iter().any(|e| matches!(e,
            GameEvent::LeavesBattlefield { .. })));
    }

    #[test]
    fn exile_permanent_goes_to_exile_and_emits_exiled() {
        let mut s = GameState::new(2, 0);
        let c = put_creature(&mut s, 0, Zone::Battlefield, 2, 2);
        Effect::ExilePermanent { target: c }.execute(&mut s);
        assert_eq!(s.zone_count(Zone::Exile), 1);
        assert_eq!(s.zone_count(Zone::Battlefield), 0);
        assert!(s.event_log.iter().any(|e| matches!(e,
            GameEvent::Exiled { from: Zone::Battlefield, .. })));
    }

    #[test]
    fn return_to_hand_respects_owner() {
        let mut s = GameState::new(2, 0);
        let c = put_creature(&mut s, 1, Zone::Battlefield, 2, 2);
        Effect::ReturnToHand { target: c }.execute(&mut s);
        assert_eq!(s.zone_count(Zone::Hand(1)), 1);
        assert_eq!(s.zone_count(Zone::Battlefield), 0);
    }

    #[test]
    fn destroy_noncreature_does_not_emit_dies() {
        let mut s = GameState::new(2, 0);
        let id = s.allocate_object_id();
        let chars = Characteristics {
            types: TypeLine::ARTIFACT.into(), ..Default::default()
        };
        s.objects.insert(GameObject::new(id, 0, Zone::Battlefield, 1, chars));
        Effect::DestroyPermanent { target: id }.execute(&mut s);
        assert!(!s.event_log.iter().any(|e| matches!(e, GameEvent::Dies { .. })));
    }

    #[test]
    fn destroy_missing_target_is_noop() {
        let mut s = GameState::new(2, 0);
        Effect::DestroyPermanent { target: 999 }.execute(&mut s);
        assert!(s.event_log.is_empty());
    }

    #[test]
    fn destroy_does_not_remove_indestructible_creature() {
        let mut s = GameState::new(2, 0);
        let c = put_creature(&mut s, 0, Zone::Battlefield, 2, 2);
        s.objects.get_mut(c).unwrap().characteristics.keywords
            .push(KeywordAbility::Indestructible);
        Effect::DestroyPermanent { target: c }.execute(&mut s);
        assert_eq!(s.objects.get(c).unwrap().zone, Zone::Battlefield);
    }

    // --- library manipulation ----------------------------------------------

    /// Seed `p`'s library with `n` fresh objects, return ids top→bottom.
    fn seed_library(s: &mut GameState, p: PlayerId, n: u32) -> Vec<ObjectId> {
        let mut ids = Vec::with_capacity(n as usize);
        for _ in 0..n {
            let id = s.allocate_object_id();
            let obj = GameObject::new(
                id, p, Zone::Library(p), /*card_id=*/ 0,
                Characteristics::default());
            s.objects.insert(obj);
            s.player_mut(p).library_top_to_bottom.push(id);
            ids.push(id);
        }
        ids
    }

    #[test]
    fn put_on_bottom_of_library_moves_target_to_library_bottom() {
        let mut s = GameState::new(2, 0);
        let existing = seed_library(&mut s, 0, 3);
        let c = put_creature(&mut s, 0, Zone::Battlefield, 2, 2);
        Effect::PutOnBottomOfLibrary { target: c }.execute(&mut s);
        let lib = &s.player(0).library_top_to_bottom;
        // Re-id means the bottom card has a new id; the card at that
        // slot is the only library object not from the seeded set.
        assert_eq!(lib.len(), 4);
        assert_eq!(&lib[..3], &existing[..]);
        let new_bottom = *lib.last().unwrap();
        assert!(!existing.contains(&new_bottom));
        assert_eq!(s.objects.get(new_bottom).unwrap().zone, Zone::Library(0));
        // Old id is gone from arena.
        assert!(s.objects.get(c).is_none());
    }

    #[test]
    fn shuffle_emits_event_and_permutes_library() {
        let mut s = GameState::new(2, 0);
        let ids = seed_library(&mut s, 0, 12);
        Effect::Shuffle { player: 0 }.execute(&mut s);
        let after: Vec<ObjectId> = s.player(0).library_top_to_bottom.clone();
        assert_eq!(after.len(), ids.len());
        assert!(s.event_log.iter().any(|e|
            matches!(e, GameEvent::LibraryShuffled { player: 0 })));
    }

    #[test]
    fn shuffle_is_deterministic_per_seed() {
        let mut a = GameState::new(2, 42);
        let mut b = GameState::new(2, 42);
        seed_library(&mut a, 0, 10);
        seed_library(&mut b, 0, 10);
        Effect::Shuffle { player: 0 }.execute(&mut a);
        Effect::Shuffle { player: 0 }.execute(&mut b);
        assert_eq!(
            a.player(0).library_top_to_bottom,
            b.player(0).library_top_to_bottom,
        );
    }

    /// Scry now pushes an `OrderCards` pending choice and does NOT
    /// mutate the library until the agent answers. These tests cover
    /// the effect-side behavior; end-to-end yield/submit cycles are in
    /// `engine::resolution_choice_framework_tests::scry_*`.
    #[test]
    fn scry_pushes_order_cards_choice_and_leaves_library_untouched() {
        use crate::actions::{CardDestination, ChoiceKind};
        let mut s = GameState::new(2, 0);
        let ids = seed_library(&mut s, 0, 5);
        s.currently_resolving = Some(999);
        Effect::Scry { player: 0, count: 2 }.execute(&mut s);

        // Library unchanged until the choice resolves.
        assert_eq!(s.player(0).library_top_to_bottom, ids);
        assert!(s.event_log.iter().any(|e|
            matches!(e, GameEvent::Scry { player: 0, count: 2 })));

        let pc = s.pending_choice.as_ref()
            .expect("Scry should push a pending choice");
        assert_eq!(pc.choosing_player, 0);
        match &pc.kind {
            ChoiceKind::OrderCards { cards, allowed } => {
                assert_eq!(cards, &ids[..2]);
                assert!(allowed.contains(&CardDestination::TopOfLibrary));
                assert!(allowed.contains(&CardDestination::BottomOfLibrary));
            }
            other => panic!("expected OrderCards, got {other:?}"),
        }
    }

    #[test]
    fn scry_clamps_count_to_library_size() {
        use crate::actions::ChoiceKind;
        let mut s = GameState::new(2, 0);
        seed_library(&mut s, 0, 2);
        s.currently_resolving = Some(999);
        Effect::Scry { player: 0, count: 5 }.execute(&mut s);
        assert!(s.event_log.iter().any(|e|
            matches!(e, GameEvent::Scry { player: 0, count: 2 })));
        let pc = s.pending_choice.as_ref().unwrap();
        if let ChoiceKind::OrderCards { cards, .. } = &pc.kind {
            assert_eq!(cards.len(), 2);
        } else {
            panic!("expected OrderCards");
        }
    }

    #[test]
    fn scry_on_empty_library_emits_zero_and_skips_choice() {
        let mut s = GameState::new(2, 0);
        s.currently_resolving = Some(999);
        Effect::Scry { player: 0, count: 2 }.execute(&mut s);
        assert!(s.pending_choice.is_none(),
            "empty library should not push a choice");
        assert!(s.event_log.iter().any(|e|
            matches!(e, GameEvent::Scry { player: 0, count: 0 })));
    }

    // --- graveyard returns ------------------------------------------------

    fn put_creature_in_graveyard(
        s: &mut GameState, owner: PlayerId, p: i32, t: i32,
    ) -> ObjectId {
        put_creature(s, owner, Zone::Graveyard(owner), p, t)
    }

    #[test]
    fn return_from_graveyard_to_battlefield_moves_to_bf_and_emits_etb() {
        let mut s = GameState::new(2, 0);
        let c = put_creature_in_graveyard(&mut s, 0, 2, 2);
        Effect::ReturnFromGraveyardToBattlefield { target: c }.execute(&mut s);
        assert_eq!(s.zone_count(Zone::Battlefield), 1);
        assert_eq!(s.zone_count(Zone::Graveyard(0)), 0);
        assert!(s.event_log.iter().any(|e| matches!(e,
            GameEvent::EntersBattlefield { was_cast: false, .. })));
    }

    #[test]
    fn return_from_graveyard_to_battlefield_noop_when_not_in_graveyard() {
        let mut s = GameState::new(2, 0);
        let c = put_creature(&mut s, 0, Zone::Battlefield, 2, 2);
        Effect::ReturnFromGraveyardToBattlefield { target: c }.execute(&mut s);
        // Already on battlefield — still there, no spurious events.
        assert_eq!(s.objects.get(c).unwrap().zone, Zone::Battlefield);
    }

    #[test]
    fn return_from_graveyard_to_hand_sends_to_owner_hand() {
        let mut s = GameState::new(2, 0);
        let _c = put_creature_in_graveyard(&mut s, 1, 2, 2);
        Effect::ReturnFromGraveyardToHand { target: _c }.execute(&mut s);
        assert_eq!(s.zone_count(Zone::Hand(1)), 1);
        assert_eq!(s.zone_count(Zone::Graveyard(1)), 0);
    }

    #[test]
    fn return_from_graveyard_to_hand_noop_when_not_in_graveyard() {
        let mut s = GameState::new(2, 0);
        let c = put_creature(&mut s, 0, Zone::Battlefield, 2, 2);
        Effect::ReturnFromGraveyardToHand { target: c }.execute(&mut s);
        assert_eq!(s.objects.get(c).unwrap().zone, Zone::Battlefield);
    }

    #[test]
    fn exile_from_graveyard_moves_to_exile() {
        let mut s = GameState::new(2, 0);
        let _c = put_creature_in_graveyard(&mut s, 0, 2, 2);
        Effect::ExileFromGraveyard { target: _c }.execute(&mut s);
        assert_eq!(s.zone_count(Zone::Exile), 1);
        assert_eq!(s.zone_count(Zone::Graveyard(0)), 0);
        assert!(s.event_log.iter().any(|e| matches!(e,
            GameEvent::Exiled { from: Zone::Graveyard(0), .. })));
    }

    #[test]
    fn exile_from_graveyard_noop_when_not_in_graveyard() {
        let mut s = GameState::new(2, 0);
        let c = put_creature(&mut s, 0, Zone::Battlefield, 2, 2);
        Effect::ExileFromGraveyard { target: c }.execute(&mut s);
        assert_eq!(s.objects.get(c).unwrap().zone, Zone::Battlefield);
    }

    /// Surveil now pushes an `OrderCards { Top | Graveyard }` choice
    /// and does NOT move cards until the agent answers.
    #[test]
    fn surveil_pushes_order_cards_choice_with_top_and_graveyard() {
        use crate::actions::{CardDestination, ChoiceKind};
        let mut s = GameState::new(2, 0);
        let ids = seed_library(&mut s, 0, 5);
        s.currently_resolving = Some(999);
        Effect::Surveil { player: 0, count: 3 }.execute(&mut s);

        // Library unchanged until the choice resolves.
        assert_eq!(s.player(0).library_top_to_bottom, ids);
        assert!(s.event_log.iter().any(|e|
            matches!(e, GameEvent::Surveil { player: 0, count: 3 })));

        let pc = s.pending_choice.as_ref()
            .expect("Surveil should push a pending choice");
        assert_eq!(pc.choosing_player, 0);
        match &pc.kind {
            ChoiceKind::OrderCards { cards, allowed } => {
                assert_eq!(cards, &ids[..3]);
                assert!(allowed.contains(&CardDestination::TopOfLibrary));
                assert!(allowed.contains(&CardDestination::Graveyard));
                assert!(!allowed.contains(&CardDestination::BottomOfLibrary),
                    "Surveil does not offer Bottom");
            }
            other => panic!("expected OrderCards, got {other:?}"),
        }
    }

    #[test]
    fn surveil_on_empty_library_emits_zero_and_skips_choice() {
        let mut s = GameState::new(2, 0);
        s.currently_resolving = Some(999);
        Effect::Surveil { player: 0, count: 2 }.execute(&mut s);
        assert!(s.pending_choice.is_none());
        assert!(s.event_log.iter().any(|e|
            matches!(e, GameEvent::Surveil { player: 0, count: 0 })));
    }

    // --- counters -----------------------------------------------------------

    #[test]
    fn add_and_remove_counters() {
        let mut s = GameState::new(2, 0);
        let c = put_creature(&mut s, 0, Zone::Battlefield, 2, 2);
        Effect::AddCounters {
            target: c, kind: CounterKind::PlusOnePlusOne, count: 3,
        }.execute(&mut s);
        assert_eq!(s.objects.get(c).unwrap()
            .count_counters(CounterKind::PlusOnePlusOne), 3);
        Effect::RemoveCounters {
            target: c, kind: CounterKind::PlusOnePlusOne, count: 2,
        }.execute(&mut s);
        assert_eq!(s.objects.get(c).unwrap()
            .count_counters(CounterKind::PlusOnePlusOne), 1);
    }

    #[test]
    fn add_counters_zero_emits_no_event() {
        let mut s = GameState::new(2, 0);
        let c = put_creature(&mut s, 0, Zone::Battlefield, 2, 2);
        Effect::AddCounters {
            target: c, kind: CounterKind::PlusOnePlusOne, count: 0,
        }.execute(&mut s);
        assert!(!s.event_log.iter().any(|e|
            matches!(e, GameEvent::CounterAdded { .. })));
    }

    // --- proliferate / MoveCounter ----------------------------------------

    #[test]
    fn proliferate_adds_one_of_each_existing_kind_to_permanents() {
        let mut s = GameState::new(2, 0);
        let c = put_creature(&mut s, 0, Zone::Battlefield, 2, 2);
        s.objects.get_mut(c).unwrap()
            .add_counters(CounterKind::PlusOnePlusOne, 2);
        s.objects.get_mut(c).unwrap()
            .add_counters(CounterKind::Loyalty, 1);
        // Another permanent with no counters — left alone.
        let untouched = put_creature(&mut s, 0, Zone::Battlefield, 1, 1);

        Effect::Proliferate.execute(&mut s);

        assert_eq!(s.objects.get(c).unwrap()
            .count_counters(CounterKind::PlusOnePlusOne), 3);
        assert_eq!(s.objects.get(c).unwrap()
            .count_counters(CounterKind::Loyalty), 2);
        assert!(s.objects.get(untouched).unwrap().counters.is_empty());
    }

    #[test]
    fn proliferate_bumps_player_poison_energy_experience() {
        let mut s = GameState::new(2, 0);
        s.player_mut(0).poison_counters = 3;
        s.player_mut(0).energy = 2;
        s.player_mut(1).experience = 1;

        Effect::Proliferate.execute(&mut s);

        assert_eq!(s.player(0).poison_counters, 4);
        assert_eq!(s.player(0).energy, 3);
        assert_eq!(s.player(1).experience, 2);
    }

    #[test]
    fn proliferate_ignores_players_with_no_counters() {
        let mut s = GameState::new(2, 0);
        // Both players start at 0 poison/energy/experience.
        Effect::Proliferate.execute(&mut s);
        assert_eq!(s.player(0).poison_counters, 0);
        assert_eq!(s.player(0).energy, 0);
    }

    #[test]
    fn move_counter_transfers_counters() {
        let mut s = GameState::new(2, 0);
        let from = put_creature(&mut s, 0, Zone::Battlefield, 2, 2);
        let to = put_creature(&mut s, 0, Zone::Battlefield, 1, 1);
        s.objects.get_mut(from).unwrap()
            .add_counters(CounterKind::PlusOnePlusOne, 3);

        Effect::MoveCounter {
            from, to, kind: CounterKind::PlusOnePlusOne, count: 2,
        }.execute(&mut s);

        assert_eq!(s.objects.get(from).unwrap()
            .count_counters(CounterKind::PlusOnePlusOne), 1);
        assert_eq!(s.objects.get(to).unwrap()
            .count_counters(CounterKind::PlusOnePlusOne), 2);
    }

    #[test]
    fn move_counter_clamps_to_available() {
        let mut s = GameState::new(2, 0);
        let from = put_creature(&mut s, 0, Zone::Battlefield, 2, 2);
        let to = put_creature(&mut s, 0, Zone::Battlefield, 1, 1);
        s.objects.get_mut(from).unwrap()
            .add_counters(CounterKind::Charge, 1);
        // Asked for 5, only 1 available.
        Effect::MoveCounter {
            from, to, kind: CounterKind::Charge, count: 5,
        }.execute(&mut s);
        assert_eq!(s.objects.get(from).unwrap()
            .count_counters(CounterKind::Charge), 0);
        assert_eq!(s.objects.get(to).unwrap()
            .count_counters(CounterKind::Charge), 1);
    }

    #[test]
    fn move_counter_no_event_when_none_to_move() {
        let mut s = GameState::new(2, 0);
        let from = put_creature(&mut s, 0, Zone::Battlefield, 2, 2);
        let to = put_creature(&mut s, 0, Zone::Battlefield, 1, 1);
        // No counters on `from`.
        Effect::MoveCounter {
            from, to, kind: CounterKind::Charge, count: 2,
        }.execute(&mut s);
        assert!(!s.event_log.iter().any(|e|
            matches!(e, GameEvent::CounterAdded { .. })));
    }

    // --- tokens -------------------------------------------------------------

    #[test]
    fn create_token_populates_battlefield() {
        let mut s = GameState::new(2, 0);
        let def = TokenDefinition {
            name: 0,
            colors: ColorSet::white(),
            types: TypeLine::CREATURE.into(),
            subtypes: SubtypeSet::new(),
            power: Some(PtValue::Fixed(1)),
            toughness: Some(PtValue::Fixed(1)),
            keywords: vec![],
            abilities: vec![],
        };
        Effect::CreateToken { controller: 0, token: def }.execute(&mut s);
        let tokens: Vec<_> = s.objects.iter()
            .filter(|o| o.zone.is_battlefield()).collect();
        assert_eq!(tokens.len(), 1);
        assert!(tokens[0].status.summoning_sick);
    }

    // --- tap / untap / transform / control ---------------------------------

    #[test]
    fn tap_and_untap_emit_events() {
        let mut s = GameState::new(2, 0);
        let c = put_creature(&mut s, 0, Zone::Battlefield, 2, 2);
        Effect::Tap { target: c }.execute(&mut s);
        assert!(s.objects.get(c).unwrap().is_tapped());
        assert!(s.event_log.iter().any(|e|
            matches!(e, GameEvent::Tapped { object_id } if *object_id == c)));

        Effect::Untap { target: c }.execute(&mut s);
        assert!(!s.objects.get(c).unwrap().is_tapped());
        assert!(s.event_log.iter().any(|e|
            matches!(e, GameEvent::Untapped { object_id } if *object_id == c)));
    }

    #[test]
    fn tap_already_tapped_emits_nothing() {
        let mut s = GameState::new(2, 0);
        let c = put_creature(&mut s, 0, Zone::Battlefield, 2, 2);
        s.objects.get_mut(c).unwrap().tap();
        let before = s.event_log.len();
        Effect::Tap { target: c }.execute(&mut s);
        assert_eq!(s.event_log.len(), before);
    }

    #[test]
    fn change_control_flips_controller() {
        let mut s = GameState::new(2, 0);
        let c = put_creature(&mut s, 0, Zone::Battlefield, 2, 2);
        Effect::ChangeControl {
            target: c, new_controller: 1,
        }.execute(&mut s);
        assert_eq!(s.objects.get(c).unwrap().controller, 1);
        assert!(s.event_log.iter().any(|e| matches!(e,
            GameEvent::ControlChanged { old: 0, new_ctrl: 1, .. })));
    }

    #[test]
    fn transform_toggles_status() {
        let mut s = GameState::new(2, 0);
        let c = put_creature(&mut s, 0, Zone::Battlefield, 2, 2);
        assert!(!s.objects.get(c).unwrap().status.transformed);
        Effect::Transform { target: c }.execute(&mut s);
        assert!(s.objects.get(c).unwrap().status.transformed);
        Effect::Transform { target: c }.execute(&mut s);
        assert!(!s.objects.get(c).unwrap().status.transformed);
    }

    // --- mana / phases ------------------------------------------------------

    #[test]
    fn add_mana_puts_units_in_pool() {
        let mut s = GameState::new(2, 0);
        let mana = vec![
            ManaUnit::plain(ManaColor::Red, 0),
            ManaUnit::plain(ManaColor::Red, 0),
        ];
        Effect::AddMana { player: 0, mana }.execute(&mut s);
        assert_eq!(s.player(0).mana_pool.len(), 2);
    }

    #[test]
    fn extra_turn_queues() {
        let mut s = GameState::new(2, 0);
        Effect::ExtraTurn { player: 1 }.execute(&mut s);
        assert_eq!(s.turn.extra_turns.len(), 1);
        assert_eq!(s.turn.extra_turns.front(), Some(&1));
    }

    #[test]
    fn additional_combat_phase_bumps_counter() {
        let mut s = GameState::new(2, 0);
        assert_eq!(s.turn.extra_combats, 0);
        Effect::AdditionalCombatPhase.execute(&mut s);
        assert_eq!(s.turn.extra_combats, 1);
    }

    // --- fight --------------------------------------------------------------

    #[test]
    fn fight_has_both_creatures_mark_damage() {
        let mut s = GameState::new(2, 0);
        let a = put_creature(&mut s, 0, Zone::Battlefield, 3, 3);
        let b = put_creature(&mut s, 1, Zone::Battlefield, 2, 4);
        Effect::Fight { a, b }.execute(&mut s);
        assert_eq!(s.objects.get(a).unwrap().damage_marked, 2);
        assert_eq!(s.objects.get(b).unwrap().damage_marked, 3);
    }

    // --- composites ---------------------------------------------------------

    #[test]
    fn sequence_runs_in_order() {
        let mut s = GameState::new(2, 0);
        let c = put_creature(&mut s, 0, Zone::Battlefield, 2, 2);
        let seq = Effect::Sequence(vec![
            Effect::AddCounters { target: c, kind: CounterKind::PlusOnePlusOne, count: 2 },
            Effect::Tap { target: c },
            Effect::GainLife { player: 0, amount: 3 },
        ]);
        seq.execute(&mut s);
        let obj = s.objects.get(c).unwrap();
        assert_eq!(obj.count_counters(CounterKind::PlusOnePlusOne), 2);
        assert!(obj.is_tapped());
        assert_eq!(s.player(0).life, 23);
    }

    #[test]
    fn conditional_executes_then_when_true() {
        let mut s = GameState::new(2, 0);
        let cond = Condition::LifeAtOrAbove(0, 20);
        let effect = Effect::Conditional {
            condition: cond,
            then: Box::new(Effect::GainLife { player: 0, amount: 5 }),
            otherwise: None,
        };
        effect.execute(&mut s);
        assert_eq!(s.player(0).life, 25);
    }

    #[test]
    fn conditional_executes_otherwise_when_false() {
        let mut s = GameState::new(2, 0);
        let cond = Condition::LifeAtOrBelow(0, 5);
        let effect = Effect::Conditional {
            condition: cond,
            then: Box::new(Effect::GainLife { player: 0, amount: 5 }),
            otherwise: Some(Box::new(Effect::LoseLife { player: 0, amount: 3 })),
        };
        effect.execute(&mut s);
        assert_eq!(s.player(0).life, 17);
    }

    #[test]
    fn for_each_iterates() {
        // ForEach executes the boxed effect once per target id; the
        // inner effect is assumed pre-specialized. Verify iteration
        // count via side-effect (stacking life gains).
        let mut s = GameState::new(2, 0);
        let effect = Effect::ForEach {
            targets: vec![0, 1, 2], // any ids — iteration is count-driven
            effect: Box::new(Effect::GainLife { player: 0, amount: 1 }),
        };
        effect.execute(&mut s);
        assert_eq!(s.player(0).life, 23);
    }

    // --- Condition evaluation ----------------------------------------------

    #[test]
    fn condition_life_checks() {
        let mut s = GameState::new(2, 0);
        assert!(Condition::LifeAtOrAbove(0, 20).evaluate(&s));
        assert!(!Condition::LifeAtOrAbove(0, 21).evaluate(&s));
        s.player_mut(0).life = 5;
        assert!(Condition::LifeAtOrBelow(0, 5).evaluate(&s));
        assert!(!Condition::LifeAtOrBelow(0, 4).evaluate(&s));
    }

    #[test]
    fn condition_is_your_turn() {
        let s = GameState::new(2, 0);
        assert!(Condition::IsYourTurn(0).evaluate(&s));
        assert!(!Condition::IsYourTurn(1).evaluate(&s));
    }

    #[test]
    fn condition_custom_fn_pointer_evaluates() {
        fn has_many_players(s: &GameState) -> bool { s.num_players() > 1 }
        let s = GameState::new(3, 0);
        assert!(Condition::Custom(has_many_players).evaluate(&s));
    }

    #[test]
    fn condition_control_permanent_matching_finds_matches() {
        let mut s = GameState::new(2, 0);
        put_creature(&mut s, 0, Zone::Battlefield, 2, 2);
        let cond = Condition::ControlPermanentMatching(ObjectFilter::creature());
        assert!(cond.evaluate(&s));
        let cond = Condition::ControlPermanentMatching(
            ObjectFilter::new().with_types(TypeLine::PLANESWALKER.into()));
        assert!(!cond.evaluate(&s));
    }

    #[test]
    fn condition_cards_in_hand() {
        let mut s = GameState::new(2, 0);
        let cond = Condition::CardsInHand(0, CmcCondition::Eq(0));
        assert!(cond.evaluate(&s));
        put_instant(&mut s, 0, Zone::Hand(0));
        let cond = Condition::CardsInHand(0, CmcCondition::Ge(1));
        assert!(cond.evaluate(&s));
    }

    // --- Counter (stack interaction) ---------------------------------------

    // --- Protection / Attach ----------------------------------------------

    #[test]
    fn protection_from_red_prevents_damage_from_red_source() {
        let mut s = GameState::new(2, 0);
        let defender = put_creature(&mut s, 0, Zone::Battlefield, 3, 3);
        s.objects.get_mut(defender).unwrap().characteristics.keywords
            .push(KeywordAbility::Protection(ProtectionQuality::Color(Color::Red)));
        // Red source in the arena — its color matters.
        let src = s.allocate_object_id();
        let red_chars = Characteristics {
            colors: ColorSet::red(),
            types: TypeLine::INSTANT.into(),
            ..Default::default()
        };
        s.objects.insert(GameObject::new(src, 1, Zone::Stack, 1, red_chars));

        s.deal_damage(src, DamageTarget::Object(defender), 3, false);
        assert_eq!(s.objects.get(defender).unwrap().damage_marked, 0);
    }

    #[test]
    fn protection_from_red_allows_damage_from_non_red_source() {
        let mut s = GameState::new(2, 0);
        let defender = put_creature(&mut s, 0, Zone::Battlefield, 3, 3);
        s.objects.get_mut(defender).unwrap().characteristics.keywords
            .push(KeywordAbility::Protection(ProtectionQuality::Color(Color::Red)));
        let src = s.allocate_object_id();
        let blue_chars = Characteristics {
            colors: ColorSet::blue(),
            types: TypeLine::INSTANT.into(),
            ..Default::default()
        };
        s.objects.insert(GameObject::new(src, 1, Zone::Stack, 1, blue_chars));

        s.deal_damage(src, DamageTarget::Object(defender), 2, false);
        assert_eq!(s.objects.get(defender).unwrap().damage_marked, 2);
    }

    #[test]
    fn attach_wires_attached_to_and_attachments() {
        let mut s = GameState::new(2, 0);
        let creature = put_creature(&mut s, 0, Zone::Battlefield, 2, 2);
        let aura = s.allocate_object_id();
        let aura_chars = Characteristics {
            types: TypeLine::new().with(TypeLine::ENCHANTMENT),
            ..Default::default()
        };
        s.objects.insert(GameObject::new(
            aura, 0, Zone::Battlefield, 1, aura_chars));

        Effect::Attach {
            equipment_or_aura: aura, target: creature,
        }.execute(&mut s);

        assert_eq!(s.objects.get(aura).unwrap().attached_to, Some(creature));
        assert_eq!(s.objects.get(creature).unwrap().attachments, vec![aura]);
        assert!(s.event_log.iter().any(|e|
            matches!(e, GameEvent::AttachedTo { equipment_or_aura, target }
                if *equipment_or_aura == aura && *target == creature)));
    }

    #[test]
    fn attach_moves_from_prior_target() {
        let mut s = GameState::new(2, 0);
        let first = put_creature(&mut s, 0, Zone::Battlefield, 2, 2);
        let second = put_creature(&mut s, 0, Zone::Battlefield, 3, 3);
        let aura = s.allocate_object_id();
        s.objects.insert(GameObject::new(
            aura, 0, Zone::Battlefield, 1, Characteristics {
                types: TypeLine::new().with(TypeLine::ENCHANTMENT),
                ..Default::default()
            }));

        Effect::Attach { equipment_or_aura: aura, target: first }.execute(&mut s);
        Effect::Attach { equipment_or_aura: aura, target: second }.execute(&mut s);

        assert_eq!(s.objects.get(aura).unwrap().attached_to, Some(second));
        assert!(s.objects.get(first).unwrap().attachments.is_empty());
        assert_eq!(s.objects.get(second).unwrap().attachments, vec![aura]);
    }

    #[test]
    fn attach_blocked_when_target_has_protection_from_source() {
        let mut s = GameState::new(2, 0);
        let creature = put_creature(&mut s, 0, Zone::Battlefield, 2, 2);
        s.objects.get_mut(creature).unwrap().characteristics.keywords
            .push(KeywordAbility::Protection(ProtectionQuality::Color(Color::Red)));
        let red_aura = s.allocate_object_id();
        s.objects.insert(GameObject::new(
            red_aura, 0, Zone::Battlefield, 1, Characteristics {
                colors: ColorSet::red(),
                types: TypeLine::new().with(TypeLine::ENCHANTMENT),
                ..Default::default()
            }));

        Effect::Attach { equipment_or_aura: red_aura, target: creature }
            .execute(&mut s);
        assert!(s.objects.get(red_aura).unwrap().attached_to.is_none());
    }

    #[test]
    fn play_extra_land_bumps_land_plays() {
        let mut s = GameState::new(2, 0);
        assert_eq!(s.player(0).land_plays_remaining, 1);
        Effect::PlayExtraLand { player: 0, amount: 2 }.execute(&mut s);
        assert_eq!(s.player(0).land_plays_remaining, 3);
        assert_eq!(s.player(0).land_plays_per_turn, 3);
    }

    #[test]
    fn empty_mana_pool_clears_all_colors() {
        let mut s = GameState::new(2, 0);
        s.player_mut(0).mana_pool.add_mana(ManaColor::Red, 3, 0);
        s.player_mut(0).mana_pool.add_mana(ManaColor::Green, 2, 0);
        assert!(!s.player(0).mana_pool.is_empty());
        Effect::EmptyManaPool { player: 0 }.execute(&mut s);
        assert!(s.player(0).mana_pool.is_empty());
    }

    #[test]
    fn create_emblem_spawns_command_zone_object() {
        let mut s = GameState::new(2, 0);
        let before = s.objects.count_in_zone(Zone::Command);
        Effect::CreateEmblem {
            controller: 0,
            emblem: EmblemDefinition {
                name: 0,
                abilities: Vec::new(),
            },
        }.execute(&mut s);
        assert_eq!(s.objects.count_in_zone(Zone::Command), before + 1);
        let emblem_id = s.objects.objects_in_zone(Zone::Command)
            .next().unwrap().id;
        assert_eq!(s.objects.get(emblem_id).unwrap().controller, 0);
    }

    #[test]
    fn cast_from_hand_free_places_spell_on_stack_without_mana() {
        let mut s = GameState::new(2, 0);
        let card = put_instant(&mut s, 0, Zone::Hand(0));
        let before_mana = s.player(0).mana_pool.clone();

        Effect::CastFromHandFree { player: 0, target: card }.execute(&mut s);

        // Hand → stack re-ids the card; the stack entry carries the new id.
        assert_eq!(s.stack_size(), 1);
        let stack_id = s.top_of_stack().unwrap().id;
        assert_eq!(s.objects.get(stack_id).unwrap().zone, Zone::Stack);
        assert!(s.objects.get(card).is_none(), "old id must be gone from arena");
        // Mana pool untouched (cast was free).
        assert_eq!(s.player(0).mana_pool, before_mana);
        assert!(s.event_log.iter().any(|e|
            matches!(e, GameEvent::SpellCast { object_id, .. } if *object_id == stack_id)));
    }

    #[test]
    fn cast_from_hand_free_noop_when_target_not_in_hand() {
        let mut s = GameState::new(2, 0);
        let card = put_instant(&mut s, 0, Zone::Graveyard(0));
        Effect::CastFromHandFree { player: 0, target: card }.execute(&mut s);
        // Still in graveyard.
        assert_eq!(s.objects.get(card).unwrap().zone, Zone::Graveyard(0));
        assert_eq!(s.stack_size(), 0);
    }

    #[test]
    fn cast_from_graveyard_places_spell_on_stack() {
        let mut s = GameState::new(2, 0);
        let card = put_instant(&mut s, 0, Zone::Graveyard(0));
        Effect::CastFromGraveyard { player: 0, target: card }.execute(&mut s);
        assert_eq!(s.stack_size(), 1);
        let stack_id = s.top_of_stack().unwrap().id;
        assert_eq!(s.objects.get(stack_id).unwrap().zone, Zone::Stack);
        assert!(s.objects.get(card).is_none());
        assert!(s.event_log.iter().any(|e|
            matches!(e, GameEvent::SpellCast { object_id, .. } if *object_id == stack_id)));
    }

    #[test]
    fn cast_from_graveyard_noop_when_target_not_in_graveyard() {
        let mut s = GameState::new(2, 0);
        let card = put_instant(&mut s, 0, Zone::Hand(0));
        Effect::CastFromGraveyard { player: 0, target: card }.execute(&mut s);
        assert_eq!(s.objects.get(card).unwrap().zone, Zone::Hand(0));
        assert_eq!(s.stack_size(), 0);
    }

    #[test]
    fn cast_from_hand_free_rejects_lands() {
        let mut s = GameState::new(2, 0);
        let id = s.allocate_object_id();
        let chars = Characteristics {
            mana_cost: None,
            types: TypeLine::LAND.into(),
            ..Default::default()
        };
        s.objects.insert(GameObject::new(id, 0, Zone::Hand(0), 1, chars));
        Effect::CastFromHandFree { player: 0, target: id }.execute(&mut s);
        // Still in hand.
        assert_eq!(s.objects.get(id).unwrap().zone, Zone::Hand(0));
        assert_eq!(s.stack_size(), 0);
    }

    #[test]
    fn counter_removes_stack_spell_and_emits_countered() {
        let mut s = GameState::new(2, 0);
        let card = put_instant(&mut s, 0, Zone::Hand(0));
        let stack_id = s.announce_spell_on_stack(
            card, 0, TargetSelection::new(), vec![], None, vec![]);
        assert_eq!(s.stack_size(), 1);

        Effect::Counter { target: stack_id }.execute(&mut s);
        assert!(s.stack_is_empty());
        assert_eq!(s.zone_count(Zone::Graveyard(0)), 1);
        assert!(s.event_log.iter().any(|e|
            matches!(e, GameEvent::SpellCountered { object_id } if *object_id == stack_id)));
    }

    #[test]
    fn counter_on_ability_stack_entry_only_emits_event() {
        let mut s = GameState::new(2, 0);
        s.push_stack_entry(StackEntry::new_activated_ability(
            42, 10, 0, /*card=*/ 1, /*ability=*/ 1, "T: tap".into(),
            TargetSelection::new(), vec![ModeChoice::empty()], None));
        Effect::Counter { target: 42 }.execute(&mut s);
        assert!(s.stack_is_empty());
        assert!(s.event_log.iter().any(|e|
            matches!(e, GameEvent::SpellCountered { object_id: 42 })));
    }

    // --- Mill --------------------------------------------------------------

    #[test]
    fn mill_moves_top_of_library_to_graveyard() {
        let mut s = GameState::new(2, 0);
        let _a = put_instant(&mut s, 0, Zone::Library(0));
        let _b = put_instant(&mut s, 0, Zone::Library(0));
        Effect::Mill { player: 0, count: 1 }.execute(&mut s);
        assert_eq!(s.zone_count(Zone::Graveyard(0)), 1);
        assert_eq!(s.zone_count(Zone::Library(0)), 1);
    }

    // --- Search / Tutor / Sacrifice --------------------------------------
    //
    // These now push a PickCards choice + fill `pending_choice_follow_up`.
    // Module-level tests verify the push shape and follow-up; the
    // end-to-end yield/submit tests live in
    // `engine::resolution_choice_framework_tests`.

    #[test]
    fn search_pushes_pick_cards_choice_with_candidates() {
        use crate::actions::{ChoiceFollowUp, ChoiceKind};
        let mut s = GameState::new(2, 0);
        let _other = put_instant(&mut s, 0, Zone::Library(0));
        let cat = put_creature(&mut s, 0, Zone::Library(0), 1, 1);
        s.currently_resolving = Some(999);
        Effect::Search {
            player: 0,
            zone: Zone::Library(0),
            filter: ObjectFilter::creature(),
            destination: Zone::Hand(0),
            reveal: false,
        }.execute(&mut s);
        let pc = s.pending_choice.as_ref().unwrap();
        match &pc.kind {
            ChoiceKind::PickCards { candidates, min, max } => {
                assert_eq!(*min, 0);
                assert_eq!(*max, 1);
                assert!(candidates.contains(&cat));
            }
            other => panic!("expected PickCards, got {other:?}"),
        }
        match s.pending_choice_follow_up.as_ref().unwrap() {
            ChoiceFollowUp::MoveToZone {
                destination, shuffle_library_owner, ..
            } => {
                assert_eq!(*destination, Zone::Hand(0));
                assert_eq!(*shuffle_library_owner, Some(0));
            }
            other => panic!("expected MoveToZone, got {other:?}"),
        }
    }

    #[test]
    fn tutor_to_hand_no_candidates_shuffles_without_pushing_choice() {
        let mut s = GameState::new(2, 0);
        let inst = put_instant(&mut s, 0, Zone::Library(0));
        s.player_mut(0).library_top_to_bottom.push(inst);
        s.currently_resolving = Some(999);

        Effect::TutorToHand {
            player: 0,
            filter: ObjectFilter::creature(),
            reveal: false,
        }.execute(&mut s);

        assert!(s.pending_choice.is_none(),
            "no candidates → no choice pushed");
        assert_eq!(s.objects.get(inst).unwrap().zone, Zone::Library(0));
        assert!(s.event_log.iter().any(|e|
            matches!(e, GameEvent::LibraryShuffled { player: 0 })));
    }

    #[test]
    fn discard_controller_chooses_pushes_pick_cards_to_self() {
        use crate::actions::{ChoiceFollowUp, ChoiceKind};
        use crate::effects::DiscardChoice;
        let mut s = GameState::new(2, 0);
        let _h1 = put_instant(&mut s, 0, Zone::Hand(0));
        let _h2 = put_instant(&mut s, 0, Zone::Hand(0));
        s.currently_resolving = Some(999);
        Effect::Discard {
            player: 0, count: 1, choice: DiscardChoice::ControllerChooses,
        }.execute(&mut s);
        let pc = s.pending_choice.as_ref().unwrap();
        assert_eq!(pc.choosing_player, 0);
        match &pc.kind {
            ChoiceKind::PickCards { candidates, min, max } => {
                assert_eq!(candidates.len(), 2);
                assert_eq!(*min, 1);
                assert_eq!(*max, 1);
            }
            other => panic!("expected PickCards, got {other:?}"),
        }
        assert!(matches!(s.pending_choice_follow_up.as_ref().unwrap(),
            ChoiceFollowUp::Discard { player: 0 }));
    }

    #[test]
    fn discard_opponent_chooses_pushes_pick_cards_to_opponent() {
        use crate::effects::DiscardChoice;
        let mut s = GameState::new(2, 0);
        let _h1 = put_instant(&mut s, 0, Zone::Hand(0));
        s.currently_resolving = Some(999);
        Effect::Discard {
            player: 0, count: 1, choice: DiscardChoice::OpponentChooses,
        }.execute(&mut s);
        let pc = s.pending_choice.as_ref().unwrap();
        assert_eq!(pc.choosing_player, 1,
            "opponent (player 1) picks which of player 0's cards gets discarded");
    }

    #[test]
    fn discard_random_picks_without_yielding() {
        use crate::effects::DiscardChoice;
        let mut s = GameState::new(2, 0);
        let _h1 = put_instant(&mut s, 0, Zone::Hand(0));
        let _h2 = put_instant(&mut s, 0, Zone::Hand(0));
        Effect::Discard {
            player: 0, count: 1, choice: DiscardChoice::Random,
        }.execute(&mut s);
        assert!(s.pending_choice.is_none(),
            "random discard uses engine RNG, no agent decision");
        assert_eq!(s.zone_count(Zone::Graveyard(0)), 1);
        assert_eq!(s.zone_count(Zone::Hand(0)), 1);
    }

    #[test]
    fn sacrifice_pushes_pick_cards_with_exact_count() {
        use crate::actions::{ChoiceFollowUp, ChoiceKind};
        let mut s = GameState::new(2, 0);
        let _a = put_creature(&mut s, 0, Zone::Battlefield, 1, 1);
        let _b = put_creature(&mut s, 0, Zone::Battlefield, 2, 2);
        s.currently_resolving = Some(999);
        Effect::Sacrifice {
            player: 0,
            filter: ObjectFilter::creature(),
            count: 1,
        }.execute(&mut s);
        let pc = s.pending_choice.as_ref().unwrap();
        match &pc.kind {
            ChoiceKind::PickCards { candidates, min, max } => {
                assert_eq!(*min, 1);
                assert_eq!(*max, 1);
                assert_eq!(candidates.len(), 2);
            }
            other => panic!("expected PickCards, got {other:?}"),
        }
        assert!(matches!(
            s.pending_choice_follow_up.as_ref().unwrap(),
            ChoiceFollowUp::Sacrifice { player: 0 },
        ));
    }

    // --- Defensive: missing targets ----------------------------------------

    #[test]
    fn operations_on_missing_target_are_noops() {
        let mut s = GameState::new(2, 0);
        for e in [
            Effect::Tap { target: 999 },
            Effect::Untap { target: 999 },
            Effect::DestroyPermanent { target: 999 },
            Effect::ExilePermanent { target: 999 },
            Effect::ReturnToHand { target: 999 },
            Effect::AddCounters {
                target: 999, kind: CounterKind::PlusOnePlusOne, count: 1,
            },
        ] {
            e.execute(&mut s);
        }
        assert!(s.event_log.is_empty());
    }

    // --- Copy ---------------------------------------------------------------

    #[test]
    fn copy_spell_adds_new_stack_entry() {
        let mut s = GameState::new(2, 0);
        let card = put_instant(&mut s, 0, Zone::Hand(0));
        let stack_id = s.announce_spell_on_stack(
            card, 0, TargetSelection::new(), vec![], None, vec![]);
        Effect::CopySpell { target: stack_id }.execute(&mut s);
        assert_eq!(s.stack_size(), 2);
        let ids: Vec<_> = s.stack_entries().iter().map(|e| e.id).collect();
        // Two distinct ids on the stack, and neither is the pre-cast
        // hand id (that one got re-id'd away on announce).
        assert!(ids.iter().all(|id| *id != card));
        assert_ne!(ids[0], ids[1]);
    }

    #[test]
    fn copy_permanent_creates_token_copy() {
        let mut s = GameState::new(2, 0);
        let c = put_creature(&mut s, 0, Zone::Battlefield, 2, 3);
        Effect::CopyPermanent { target: c }.execute(&mut s);
        let creatures: Vec<_> = s.objects.iter()
            .filter(|o| o.zone.is_battlefield() && o.is_creature())
            .collect();
        assert_eq!(creatures.len(), 2);
    }
}
