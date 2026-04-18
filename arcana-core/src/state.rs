//! Top-level game state and player state.
//!
//! Addendum Section 4.1 / Listing 1, Phase 1 Task #6. Depends on tasks 1–5.
//!
//! `GameState` is the root of every game. It contains all player state, all
//! in-game objects, the stack, turn/priority machinery, and the event log.
//! The spec's first design principle is "state is a value": every field must
//! be `Clone`-friendly so tree search can fork state cheaply.
//!
//! **TODO(serialize)**: `GameState` can't derive `Serialize`/`Deserialize`
//! yet. The blockers are `ContinuousEffect` (layers.rs) and
//! `ReplacementEffect` (replacement.rs) which carry bare `fn` pointers —
//! serde has no way to round-trip those. The fix is the `ConditionFnId`
//! registry pattern from addendum Section 12: replace `fn` fields with
//! `u32` IDs into a `CardRegistry` function table. Planned for Phase 3,
//! before Python bindings land.

use serde::{Serialize, Deserialize};
use std::collections::{HashMap, HashSet};

use crate::combat::CombatState;
use crate::events::GameEvent;
use crate::format::FormatConfig;
use crate::layers::ContinuousEffect;
use crate::mana::ManaPool;
use crate::objects::{ObjectArena, ObjectId, GameObject};
use crate::priority::PriorityState;
use crate::stack::StackEntry;
use crate::triggers::{DelayedTrigger, TriggerLedger};
use crate::turn::TurnState;
use crate::types::PlayerId;
use crate::zones::{Zone, ZoneKind};

/// Default starting life for constructed formats (CR 103.4).
pub const DEFAULT_STARTING_LIFE: i32 = 20;

/// `next_object_id` starts at 1 so 0 can serve as [`NULL_OBJECT_ID`].
///
/// [`NULL_OBJECT_ID`]: crate::objects::NULL_OBJECT_ID
pub const FIRST_OBJECT_ID: ObjectId = 1;

// =============================================================================
// GameState
// =============================================================================

// TODO(serialize): `GameState` transitively contains `ContinuousEffect`
// (layers.rs) and `ReplacementEffect` (replacement.rs) which carry bare
// `fn` pointers. Cannot derive Serialize/Deserialize until those migrate
// to `ConditionFnId` (addendum Section 12) in Phase 3. PlayerState,
// ObjectArena, and the event log *do* serialize individually.
#[derive(Clone, Debug)]
pub struct GameState {
    /// Per-player state (typically 2; extensible to multiplayer).
    pub players: Vec<PlayerState>,
    /// The stack (LIFO of spells and abilities).
    pub stack: Vec<StackEntry>,
    /// Every game object keyed by [`ObjectId`].
    pub objects: ObjectArena,
    /// Turn structure.
    pub turn: TurnState,
    /// Priority state.
    pub priority: PriorityState,
    /// Combat state — `Some` during combat phases.
    pub combat: Option<CombatState>,
    /// Continuous effects currently active (CR 613).
    pub continuous_effects: Vec<ContinuousEffect>,
    /// Replacement effects waiting to intercept would-be events (CR 614).
    pub replacement_effects: Vec<crate::replacement::ReplacementEffect>,
    /// Delayed triggers waiting to fire.
    pub delayed_triggers: Vec<DelayedTrigger>,
    /// Fires-per-turn ledger for `TriggerFrequency::OncePerTurn`
    /// triggers (and the tally side of `OncePerGame`). Keyed by the
    /// `(source_id, trigger_id)` pair.
    pub triggers_fired_this_turn: TriggerLedger,
    /// Fires-per-game ledger for `TriggerFrequency::OncePerGame`.
    /// Cumulative across turns.
    pub triggers_fired_this_game: TriggerLedger,
    /// Monotonic counter for continuous-effect timestamps (CR 613.7).
    pub timestamp_counter: u64,
    /// Index into [`event_log`] marking how far the engine's
    /// triggered-ability scanner has processed. Events before this
    /// cursor have already been considered for trigger firing;
    /// events at or after it are pending. Advanced by
    /// [`crate::engine::step`]'s settle loop (Gap #2).
    ///
    /// [`event_log`]: Self::event_log
    pub trigger_event_cursor: usize,
    /// Format configuration — starting life, hand sizes, mulligan
    /// rule, deck constraints. Defaults to
    /// [`FormatConfig::standard_2026`]; override via
    /// [`crate::engine::new_game_with_format`].
    pub format: FormatConfig,
    /// Seed for the deterministic RNG. Task #20 will replace this with a
    /// full `ChaCha8Rng` once we need to draw / shuffle.
    pub rng_seed: u64,
    /// Number of spells cast this turn. Incremented in
    /// [`Self::announce_spell_on_stack`] (cast path); copies do NOT
    /// increment per CR 707.10. Reset to 0 in
    /// [`crate::turn::TurnState::start_next_turn`]. Storm reads the
    /// snapshot taken on the cast spell's
    /// [`crate::stack::StackEntry::storm_count_at_cast`] — the value
    /// AT cast time (not at resolution).
    pub storm_count: u32,
    /// Monotonic counter for unique object ids. Use
    /// [`allocate_object_id`](Self::allocate_object_id) to draw.
    pub next_object_id: ObjectId,
    /// Game result — `None` while in progress.
    pub result: Option<GameResult>,
    /// Event log for trigger matching and replay.
    pub event_log: Vec<GameEvent>,
    /// A mid-resolution choice waiting for the agent's response. When
    /// `Some`, the engine yields
    /// [`crate::engine::EngineYield::PendingDecision`] with
    /// [`crate::actions::DecisionContext::ResolutionChoice`] and no
    /// other actions are legal except
    /// [`crate::actions::Action::SubmitResolutionChoice`] (and
    /// [`crate::actions::Action::Concede`]). See the
    /// [resolution-choice framework][arch] design for semantics.
    ///
    /// Single-slot (Phase 2-A): effects that need multiple choices
    /// decompose into sequential single-choice submissions.
    ///
    /// [arch]: crate::actions::PendingChoice
    pub pending_choice: Option<crate::actions::PendingChoice>,
    /// Monotonic id allocator for [`Self::pending_choice`]. Each new
    /// choice gets a fresh id; submitting a response with a stale id
    /// is a hard error.
    pub next_choice_id: u64,
    /// A spell/ability resolution that parked itself mid-way when one
    /// of its effects pushed a [`Self::pending_choice`]. Resumed by the
    /// engine once the choice is answered.
    pub pending_resolution: Option<crate::actions::PendingResolution>,
    /// Scratch slot set by the engine around effect execution so
    /// stack-resolution effects (Scry, Surveil, Tutor, Ward, …) can
    /// reach the resolving stack entry's id for [`crate::actions::ChoiceContext::ResolvingStack`].
    /// Only non-`None` while inside an effect call dispatched from
    /// [`crate::engine::resolve_top_of_stack`] (or its resumption).
    pub currently_resolving: Option<ObjectId>,
    /// Follow-up semantics for the current [`Self::pending_choice`]
    /// when it's a [`crate::actions::ChoiceKind::PickCards`]. The
    /// effect that pushed the choice fills this slot; the dispatcher
    /// takes and applies it to the answered ids. `None` is valid when
    /// the PickCards handler is hardcoded (e.g. Legend-rule SBA).
    pub pending_choice_follow_up: Option<crate::actions::ChoiceFollowUp>,
    /// Companion slot to a pending
    /// [`crate::actions::ChoiceKind::ChooseTargets`]: the actual
    /// requirements the agent's submitted [`crate::targets::TargetSelection`]
    /// must satisfy. Lives outside `ChoiceKind` because
    /// [`crate::targets::TargetRequirement`] carries fn-pointer
    /// filters (so it can't be Hash/Eq/Serialize). Set when the
    /// pipeline pushes the choice; cleared when the response is
    /// applied (or on Concede).
    pub pending_target_requirements: Option<Vec<crate::targets::TargetRequirement>>,
    /// Companion slot to a pending [`crate::actions::ChoiceKind::YesNo`]
    /// emitted during cascade (CR 702.85). Populated by
    /// [`crate::effects::Effect::Cascade`] just before pushing the
    /// may-cast prompt. The dispatcher consumes it on response: yes →
    /// cast `hit` for free + bottom-shuffle `other_exiled`; no →
    /// bottom-shuffle the full exiled list (incl. `hit`) via the
    /// engine's seeded RNG.
    pub pending_cascade: Option<PendingCascade>,
    /// Last-known-information table (CR 603.10 / 400.7). When an
    /// object changes zones it becomes a new object with a new
    /// [`ObjectId`]; any ability that must "look back" at the
    /// pre-transition state (dies triggers, leaves-the-battlefield
    /// triggers, "exile the attacking creature" delayed triggers,
    /// replacement-effect classification) reads from this map keyed
    /// by the old id.
    ///
    /// Populated by [`Self::move_object_to_zone`] just before the
    /// arena swaps in the re-id'd object. Cleared at the top of each
    /// trigger-sweep iteration in
    /// [`crate::engine::run_sba_and_triggers`] — one full SBA+trigger
    /// pass is the CR-mandated retention window for LKI used by
    /// triggers that watch zone changes.
    pub lki: HashMap<ObjectId, GameObject>,
}

/// Parked cascade state between exile-and-prompt and the YesNo
/// response. See [`GameState::pending_cascade`] for semantics.
#[derive(Clone, Debug)]
pub struct PendingCascade {
    pub controller: PlayerId,
    /// The card currently offered to cast for free. On yes-response,
    /// cast via the free-cast path; on no-response, joins
    /// `other_exiled` in the bottom shuffle.
    pub hit: ObjectId,
    /// The lands (and any other cards) exiled before the hit. These
    /// always go to the bottom in seeded-random order regardless of
    /// the may-cast answer.
    pub other_exiled: Vec<ObjectId>,
}

impl GameState {
    /// Build an empty game skeleton with `num_players` players, each at the
    /// default starting life. No decks, no hands, no objects — those are
    /// populated by `engine::new_game` (Task #20). Use this for unit tests
    /// and as the base for more specialized setups.
    ///
    /// Uses [`FormatConfig::standard_2026`] for starting life, hand sizes,
    /// and mulligan rule. Callers wanting a different format should call
    /// [`Self::with_format`] or [`crate::engine::new_game_with_format`].
    pub fn new(num_players: u8, rng_seed: u64) -> Self {
        Self::with_format(num_players, rng_seed, FormatConfig::standard_2026())
    }

    /// Like [`Self::new`] but with an explicit format. Honors
    /// `format.starting_life` when constructing each player.
    pub fn with_format(num_players: u8, rng_seed: u64, format: FormatConfig) -> Self {
        assert!(num_players >= 1, "a game needs at least one player");
        let starting_life = format.starting_life;
        let players = (0..num_players)
            .map(|id| PlayerState::new(id, starting_life))
            .collect();
        Self {
            players,
            stack: Vec::new(),
            objects: ObjectArena::new(),
            turn: TurnState::new_initial(/*active=*/ 0),
            priority: PriorityState::new(/*player=*/ 0),
            combat: None,
            continuous_effects: Vec::new(),
            replacement_effects: Vec::new(),
            delayed_triggers: Vec::new(),
            triggers_fired_this_turn: TriggerLedger::new(),
            triggers_fired_this_game: TriggerLedger::new(),
            timestamp_counter: 0,
            trigger_event_cursor: 0,
            format,
            rng_seed,
            storm_count: 0,
            pending_target_requirements: None,
            pending_cascade: None,
            next_object_id: FIRST_OBJECT_ID,
            result: None,
            event_log: Vec::new(),
            pending_choice: None,
            next_choice_id: 1,
            pending_resolution: None,
            currently_resolving: None,
            pending_choice_follow_up: None,
            lki: HashMap::new(),
        }
    }

    // --- spec-match game constructors --------------------------------------
    //
    // The spec (Listing 40) expects `GameState::new_game(decks, registry,
    // seed) → Self` and `new_game_with_format(decks, format, registry,
    // seed) → Self` as the canonical game-starting constructors. These
    // delegate to the engine's game builders, discarding the initial
    // `EngineYield` since the spec shape returns state only. Callers
    // that want the yield (the normal path) should use
    // [`crate::engine::new_game`] / [`crate::engine::new_game_with_format`].

    /// Build and populate a full game from per-player decks of
    /// [`crate::types::CardId`]s, using [`crate::format::FormatConfig::standard_2026`].
    /// Opens with a mulligan decision for the active player.
    pub fn new_game(
        decks: Vec<Vec<crate::types::CardId>>,
        registry: &crate::registry::CardRegistry,
        seed: u64,
    ) -> Self {
        crate::engine::new_game(decks, registry, seed).0
    }

    /// Like [`Self::new_game`] but with an explicit
    /// [`crate::format::FormatConfig`].
    pub fn new_game_with_format(
        decks: Vec<Vec<crate::types::CardId>>,
        format: crate::format::FormatConfig,
        registry: &crate::registry::CardRegistry,
        seed: u64,
    ) -> Self {
        crate::engine::new_game_with_format(decks, format, registry, seed).0
    }

    // --- player access ------------------------------------------------------

    pub fn num_players(&self) -> u8 { self.players.len() as u8 }

    /// Bounds-checked read access to a player. Panics on an invalid id since
    /// that's a programming bug, not recoverable runtime state.
    pub fn player(&self, id: PlayerId) -> &PlayerState {
        self.players.get(id as usize)
            .unwrap_or_else(|| panic!("GameState::player({id}): out of range (num_players={})", self.num_players()))
    }

    pub fn player_mut(&mut self, id: PlayerId) -> &mut PlayerState {
        let n = self.num_players();
        self.players.get_mut(id as usize)
            .unwrap_or_else(|| panic!("GameState::player_mut({id}): out of range (num_players={n})"))
    }

    /// Currently-active player (whose turn it is).
    pub fn active_player(&self) -> PlayerId { self.turn.active_player }

    /// Player who currently has priority.
    pub fn priority_player(&self) -> PlayerId { self.priority.player }

    /// Iterator over every player id that is not `player`. In a 2-player
    /// game this yields exactly one opponent.
    pub fn opponents_of(&self, player: PlayerId) -> impl Iterator<Item = PlayerId> + '_ {
        (0..self.num_players()).filter(move |id| *id != player)
    }

    /// Convenience: in a 2-player game, the single opponent. Panics in
    /// games with a different player count.
    pub fn sole_opponent(&self, player: PlayerId) -> PlayerId {
        assert_eq!(self.num_players(), 2,
            "sole_opponent() only valid for 2-player games");
        if player == 0 { 1 } else { 0 }
    }

    // --- object id allocation ----------------------------------------------

    /// Return the next fresh [`ObjectId`] and advance the counter. Every new
    /// object (cards entering the game, tokens created, stack-entry copies)
    /// must be assigned an id this way so the arena stays consistent.
    pub fn allocate_object_id(&mut self) -> ObjectId {
        let id = self.next_object_id;
        self.next_object_id = self.next_object_id
            .checked_add(1)
            .expect("ObjectId counter overflow — 4 billion objects in one game?!");
        id
    }

    // --- object queries (delegate to the arena) ----------------------------

    pub fn objects_in_zone(&self, zone: Zone) -> impl Iterator<Item = &GameObject> + '_ {
        self.objects.objects_in_zone(zone)
    }

    pub fn objects_in_zone_kind(&self, kind: ZoneKind) -> impl Iterator<Item = &GameObject> + '_ {
        self.objects.objects_in_zone_kind(kind)
    }

    pub fn zone_count(&self, zone: Zone) -> usize {
        self.objects.count_in_zone(zone)
    }

    /// All objects in any graveyard. Useful for effects like "target card in
    /// any graveyard" / "until end of turn, each player casts spells from
    /// their graveyard."
    pub fn all_graveyards(&self) -> impl Iterator<Item = &GameObject> + '_ {
        self.objects.objects_in_zone_kind(ZoneKind::Graveyard)
    }

    /// Per-player life totals, in player-id order. Useful for the AI
    /// observation encoder.
    pub fn life_totals(&self) -> Vec<i32> {
        self.players.iter().map(|p| p.life).collect()
    }

    // --- last-known information (CR 603.10 / 400.7) ------------------------

    /// Snapshot `obj` into [`Self::lki`] under its current id. Called
    /// by [`Self::move_object_to_zone`] right before the object is
    /// removed from the arena and re-inserted under a fresh id.
    pub fn store_lki(&mut self, obj: GameObject) {
        self.lki.insert(obj.id, obj);
    }

    /// Fetch the LKI snapshot for `id`, if any. Used by trigger
    /// matching, replacement-effect classification, and any other code
    /// path that needs the pre-transition characteristics of an object
    /// that has already moved zones.
    pub fn lki(&self, id: ObjectId) -> Option<&GameObject> {
        self.lki.get(&id)
    }

    /// Live arena entry first; LKI snapshot as fallback. Preferred
    /// lookup for code that doesn't care whether the object is still
    /// in its original zone or has moved on (e.g. dies-trigger scan).
    pub fn object_or_lki(&self, id: ObjectId) -> Option<&GameObject> {
        self.objects.get(id).or_else(|| self.lki(id))
    }

    /// Clear the LKI table. Called at the top of each settle pass in
    /// [`crate::engine::run_sba_and_triggers`] — one SBA-plus-trigger
    /// sweep is the retention window CR requires for leaves-the-
    /// battlefield triggers.
    pub fn clear_lki(&mut self) {
        self.lki.clear();
    }

    // --- ETB lifecycle hook -------------------------------------------------

    /// Unified ETB lifecycle hook. Every code path that puts a fresh
    /// object onto the battlefield under a newly allocated id must
    /// call this just after the arena insert and just before the
    /// [`GameEvent::EntersBattlefield`] (or analogous) is emitted.
    ///
    /// Responsibilities, in this order:
    /// 1. Fold in ETB-event replacements
    ///    ([`crate::replacement::ReplacementKind::EtbTapped`],
    ///    [`crate::replacement::ReplacementKind::EtbWithCounters`])
    ///    gathered from [`Self::collect_etb_replacements`]. These
    ///    rewrite the entry itself — counters added, tap flag set.
    /// 2. Set summoning sickness for creatures and planeswalkers
    ///    (CR 302.1 / 306.5b).
    ///
    /// # Scope
    ///
    /// *Only* handles ETB-event replacements. Downstream replacements
    /// that fire on events produced BY the ETB (notably the counter-
    /// placement pipeline — Hardened Scales, Doubling Season, Winding
    /// Constrictor) belong to a separate hook tied to counter
    /// placement. See the `modular_plus_hardened_scales_yields_four_counters`
    /// test marker for the canonical trap.
    ///
    /// Card-inherent self-replacements (Modular's "enters with N
    /// counters", Clone's "enters as a copy", Battles' defense
    /// counters, Planeswalkers' starting loyalty) require per-card
    /// replacement effects to live on the `CardDefinition` — that's
    /// Phase 3 work and out of scope here.
    pub fn after_enter_battlefield(&mut self, id: ObjectId) {
        let replacements = self.collect_etb_replacements(id);
        // Route ETB-event counter placement through the counter-placement
        // pipeline so downstream replacements (Hardened Scales et al.) can
        // intercept. This is the Modular + Hardened Scales handoff.
        for (kind, count) in replacements.additional_counters {
            self.place_counters(
                crate::replacement::CounterTarget::Object(id), kind, count);
        }
        if let Some(obj) = self.objects.get_mut(id) {
            if replacements.enter_tapped {
                obj.tap();
            }
            if obj.is_creature() || obj.is_planeswalker() {
                obj.status.summoning_sick = true;
            }
        }
    }

    // --- zone changes --------------------------------------------------------

    /// Move `id` from its current zone to `destination`. Handles the
    /// full event cascade per CR 400.7:
    /// - emits [`GameEvent::LeavesBattlefield`] if coming off the field
    /// - emits [`GameEvent::ZoneChange`] carrying both the old id and
    ///   the freshly allocated `new_id`
    /// - emits one of [`GameEvent::Dies`] /
    ///   [`GameEvent::PutIntoGraveyard`] / [`GameEvent::Exiled`] /
    ///   [`GameEvent::EntersBattlefield`] as appropriate
    /// - snapshots the pre-move object into [`Self::lki`] under the old
    ///   id so dies-triggers and leaves-battlefield triggers can look
    ///   back at its characteristics
    /// - resets transient per-zone fields (counters, damage, tapped,
    ///   attachments) via [`crate::objects::GameObject::reset_on_zone_change`]
    ///
    /// Per CR 400.7 the object becomes a new object with a fresh
    /// [`ObjectId`] when it changes zones. This function performs that
    /// re-id and returns the new id. Events about something *leaving*
    /// (`LeavesBattlefield`, `Dies`, `ZoneChange::object_id`) carry
    /// the OLD id; events about something *arriving*
    /// (`EntersBattlefield`, `PutIntoGraveyard`, `Exiled`,
    /// `ZoneChange::new_id`) carry the NEW id.
    ///
    /// Returns `None` if `id` doesn't exist or is already in
    /// `destination` (no move happened). Otherwise returns `Some(new_id)`.
    /// This is the canonical zone-mover — combat, effects, and SBAs
    /// all route through it.
    pub fn move_object_to_zone(
        &mut self,
        id: ObjectId,
        destination: Zone,
        cause: crate::events::MoveCause,
    ) -> Option<ObjectId> {
        let from = self.objects.get(id)?.zone;
        if from == destination { return None; }

        let was_creature_or_pw = self.objects.get(id).map_or(false,
            |o| o.is_creature() || o.is_planeswalker());
        let from_battlefield = from.is_battlefield();
        let to_graveyard = matches!(destination, Zone::Graveyard(_));
        let to_exile = destination.is_exile();
        let to_battlefield = destination.is_battlefield();

        if from_battlefield {
            self.emit(GameEvent::LeavesBattlefield {
                object_id: id, destination,
            });
            // Expire continuous effects sourced from this object with
            // Duration::WhileSourceOnBattlefield. CR 611.2e: effects
            // that reference a permanent's presence end when that
            // permanent leaves the battlefield. Without this hook,
            // anthems, Pacifism-style "can't attack" effects, and
            // other source-bound effects persist past their source's
            // death — surfaced by the Glorious Anthem integration
            // test when the enchantment was sent to the graveyard
            // but its +1/+1 stayed on the board.
            self.expire_effects_from_source(id);
        }

        let (new_id, from) = self.swap_to_zone_reid(id, destination)?;

        self.emit(GameEvent::ZoneChange {
            object_id: id, from, to: destination, new_id, cause,
        });
        if from_battlefield && to_graveyard && was_creature_or_pw {
            self.emit(GameEvent::Dies { object_id: id });
        }
        if to_graveyard {
            self.emit(GameEvent::PutIntoGraveyard { object_id: new_id, from });
        } else if to_exile {
            self.emit(GameEvent::Exiled { object_id: new_id, from });
        } else if to_battlefield {
            // Fold ETB-event replacements and summoning sickness in
            // BEFORE the event fires so trigger matchers see the
            // final state of the entering permanent.
            self.after_enter_battlefield(new_id);
            self.emit(GameEvent::EntersBattlefield {
                object_id: new_id, from_zone: from, was_cast: false,
            });
        }

        Some(new_id)
    }

    /// Arena re-id core shared by [`Self::move_object_to_zone`] and
    /// [`crate::stack::GameState::finalize_resolved_spell`] (and any
    /// other future path that performs a CR 400.7 zone transition
    /// needing a fresh [`ObjectId`]).
    ///
    /// Responsibilities:
    /// - pull the object out of the arena under the old id
    /// - scrub bidirectional attachment links that referenced the old id
    /// - drop the old id from the owning library's ordering (if leaving
    ///   a library)
    /// - snapshot the pre-move object into [`Self::lki`]
    /// - allocate a fresh id, reset per-zone fields, and re-insert
    /// - append the new id to the destination library's ordering (if
    ///   entering a library)
    /// - reset controller to owner when the destination is NOT the
    ///   battlefield; for battlefield destinations the caller is
    ///   responsible for setting the post-move controller before the
    ///   [`Self::after_enter_battlefield`] hook runs
    ///
    /// Does NOT emit events and does NOT call
    /// [`Self::after_enter_battlefield`] — event emission and the ETB
    /// hook are caller-specific (spell-cast was_cast, token creation,
    /// etc.). Returns `(new_id, from_zone)` or `None` if the object
    /// doesn't exist or already is in `destination`.
    pub(crate) fn swap_to_zone_reid(
        &mut self,
        id: ObjectId,
        destination: Zone,
    ) -> Option<(ObjectId, Zone)> {
        let from = self.objects.get(id)?.zone;
        if from == destination { return None; }

        if let Zone::Library(p) = from {
            let lib = &mut self.player_mut(p).library_top_to_bottom;
            lib.retain(|&o| o != id);
        }

        let mut obj = self.objects.remove(id)
            .expect("swap_to_zone_reid: object vanished between lookup and remove");

        // Attachments are maintained bidirectionally; if this object
        // had an `attached_to`, `reset_on_zone_change` clears it but
        // the target's `attachments` vec still references the old id.
        // Clean that up before we lose the old id.
        if let Some(host) = obj.attached_to {
            if let Some(host_obj) = self.objects.get_mut(host) {
                host_obj.attachments.retain(|&a| a != id);
            }
        }
        for &child in obj.attachments.clone().iter() {
            if let Some(child_obj) = self.objects.get_mut(child) {
                child_obj.attached_to = None;
            }
        }
        self.store_lki(obj.clone());

        let new_id = self.allocate_object_id();
        obj.id = new_id;
        obj.zone = destination;
        obj.reset_on_zone_change();
        // Controller reverts to owner everywhere except the
        // battlefield (CR 110.2a). Callers that want a specific
        // battlefield controller set it on the returned new_id
        // before calling [`Self::after_enter_battlefield`].
        if !destination.is_battlefield() {
            obj.controller = obj.owner;
        }
        self.objects.insert(obj);

        if let Zone::Library(p) = destination {
            self.player_mut(p).library_top_to_bottom.push(new_id);
        }

        Some((new_id, from))
    }

    // --- library ordering / drawing ----------------------------------------

    /// Top card of `player`'s library, if any.
    pub fn top_of_library(&self, player: PlayerId) -> Option<ObjectId> {
        self.player(player).library_top_to_bottom.first().copied()
    }

    /// Draw the top card of `player`'s library into their hand, emit
    /// [`GameEvent::DrawCard`], and return the id drawn. If the library
    /// is empty, sets the player's [`has_drawn_from_empty_library`]
    /// flag (CR 704.5b) and returns `None`.
    ///
    /// [`has_drawn_from_empty_library`]: PlayerState::has_drawn_from_empty_library
    pub fn draw_one_card(&mut self, player: PlayerId) -> Option<ObjectId> {
        let top = self.player_mut(player).library_top_to_bottom.first().copied();
        match top {
            Some(id) => {
                self.move_object_to_zone(
                    id, Zone::Hand(player), crate::events::MoveCause::Draw);
                self.emit(GameEvent::DrawCard { player, object_id: id });
                Some(id)
            }
            None => {
                self.player_mut(player).has_drawn_from_empty_library = true;
                None
            }
        }
    }

    /// Put `id` on top of `player`'s library. The object must already be
    /// in `Zone::Library(player)` — call [`move_object_to_zone`] first
    /// if coming from elsewhere, then reorder with this helper.
    ///
    /// [`move_object_to_zone`]: Self::move_object_to_zone
    pub fn put_on_top_of_library(&mut self, id: ObjectId, player: PlayerId) {
        let lib = &mut self.player_mut(player).library_top_to_bottom;
        lib.retain(|&o| o != id);
        lib.insert(0, id);
    }

    /// Put `id` on the bottom of `player`'s library.
    pub fn put_on_bottom_of_library(&mut self, id: ObjectId, player: PlayerId) {
        let lib = &mut self.player_mut(player).library_top_to_bottom;
        lib.retain(|&o| o != id);
        lib.push(id);
    }

    /// Allocate a fresh id for a [`crate::actions::PendingChoice`]. Every
    /// pushed pending choice gets a monotonically increasing id so
    /// stale agent replies are rejected by the engine.
    pub fn allocate_choice_id(&mut self) -> u64 {
        let id = self.next_choice_id;
        self.next_choice_id = self.next_choice_id.checked_add(1)
            .expect("choice id counter overflowed u64");
        id
    }

    /// Push a new pending choice. Panics if one is already pending —
    /// the single-slot invariant must be upheld by the effect code.
    pub fn push_pending_choice(
        &mut self,
        choosing_player: PlayerId,
        context: crate::actions::ChoiceContext,
        kind: crate::actions::ChoiceKind,
    ) -> u64 {
        assert!(self.pending_choice.is_none(),
            "push_pending_choice: a choice is already pending (single-slot)");
        let id = self.allocate_choice_id();
        self.pending_choice = Some(crate::actions::PendingChoice {
            id, choosing_player, context, kind,
        });
        id
    }

    /// Deterministically shuffle `player`'s library using the engine's
    /// seeded RNG, advancing the seed so subsequent shuffles differ.
    /// Emits [`GameEvent::LibraryShuffled`].
    pub fn shuffle_library(&mut self, player: PlayerId) {
        use rand::seq::SliceRandom;
        use rand::SeedableRng;
        let mut rng = rand_chacha::ChaCha8Rng::seed_from_u64(
            self.rng_seed.wrapping_add(player as u64));
        self.rng_seed = self.rng_seed
            .wrapping_add(1)
            .wrapping_mul(self.turn.turn_number.max(1) as u64);
        let lib = &mut self.player_mut(player).library_top_to_bottom;
        lib.shuffle(&mut rng);
        self.emit(GameEvent::LibraryShuffled { player });
    }

    // --- events -------------------------------------------------------------

    /// Append an event to the log. The trigger matcher picks events up from
    /// here; replay reconstructs the game from the same sequence.
    pub fn emit(&mut self, event: GameEvent) { self.event_log.push(event); }

    /// Append several events atomically.
    pub fn emit_all<I: IntoIterator<Item = GameEvent>>(&mut self, events: I) {
        self.event_log.extend(events);
    }

    // --- game-over ---------------------------------------------------------

    pub fn is_game_over(&self) -> bool { self.result.is_some() }
}

// =============================================================================
// PlayerState
// =============================================================================

/// Per-player game state. Cloning a `PlayerState` is a deep copy of all
/// fields; `GameState` clones in turn clone each `PlayerState`.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct PlayerState {
    pub id: PlayerId,
    pub life: i32,
    pub mana_pool: ManaPool,
    /// Land plays remaining this turn.
    pub land_plays_remaining: u32,
    /// Base land plays per turn (raised by Exploration, Azusa, etc.).
    pub land_plays_per_turn: u32,
    /// Sticky flag for CR 704.5b — loses on next SBA check.
    pub has_drawn_from_empty_library: bool,
    pub poison_counters: u32,
    pub commander_damage: HashMap<ObjectId, u32>,
    pub has_lost: bool,
    pub has_conceded: bool,
    /// Objects whose identity this player is known to have learned about
    /// (e.g. revealed from hand, scried to top). Drives the `View`
    /// projection used for information-set reasoning.
    pub known_cards: HashSet<ObjectId>,
    /// Energy counters (Kaladesh+ mechanic) — tracked per CR 122.2b.
    pub energy: u32,
    /// Experience counters (Commander mechanic).
    pub experience: u32,
    /// Library ordering. `library_top_to_bottom[0]` is the top of the
    /// library (next card drawn); the last element is the bottom.
    /// Maintained by [`GameState::move_object_to_zone`] and the dedicated
    /// draw / put-on-top / put-on-bottom helpers.
    pub library_top_to_bottom: Vec<ObjectId>,
    /// London-mulligan tally (CR 103.4a): number of mulligans this
    /// player has taken this game. Equals the number of cards they
    /// owe to the bottom of their library when they finally keep.
    pub mulligans_taken: u32,
    /// Has this player locked in their opening-hand decision?
    /// Flipped by `MulliganKeep` / `BottomCards`; used by the engine
    /// to round-robin the mulligan phase.
    pub mulligan_decided: bool,
}

impl PlayerState {
    pub fn new(id: PlayerId, starting_life: i32) -> Self {
        Self {
            id,
            life: starting_life,
            mana_pool: ManaPool::new(),
            land_plays_remaining: 1,
            land_plays_per_turn: 1,
            has_drawn_from_empty_library: false,
            poison_counters: 0,
            commander_damage: HashMap::new(),
            has_lost: false,
            has_conceded: false,
            known_cards: HashSet::new(),
            energy: 0,
            experience: 0,
            library_top_to_bottom: Vec::new(),
            mulligans_taken: 0,
            mulligan_decided: false,
        }
    }

    /// True if this player can still play a land this turn.
    pub fn can_play_land(&self) -> bool {
        !self.has_lost && self.land_plays_remaining > 0
    }

    /// Reset `land_plays_remaining = land_plays_per_turn`. Called at the
    /// start of each turn by the engine.
    pub fn reset_land_plays(&mut self) {
        self.land_plays_remaining = self.land_plays_per_turn;
    }

    /// True if this player has not yet lost the game.
    pub fn is_alive(&self) -> bool { !self.has_lost }
}

// =============================================================================
// Turn / priority helpers — convenience constructors
// =============================================================================
//
// These live here rather than in turn.rs / priority.rs so that state.rs has
// what it needs to build an empty GameState; turn.rs and priority.rs can
// also add richer APIs later.

impl TurnState {
    /// State at the very start of game — turn 1, untap step, given active
    /// player.
    pub fn new_initial(active_player: PlayerId) -> Self {
        Self {
            active_player,
            turn_number: 1,
            phase: crate::turn::Phase::Beginning,
            step: crate::turn::Step::Untap,
            extra_turns: std::collections::VecDeque::new(),
            extra_combats: 0,
        }
    }
}

impl PriorityState {
    /// State at the very start of game — the named player has priority,
    /// zero passes.
    pub fn new(player: PlayerId) -> Self {
        Self {
            player,
            consecutive_passes: 0,
            special_action: None,
        }
    }
}

// =============================================================================
// GameResult
// =============================================================================

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum GameResult {
    Win(PlayerId),
    Draw,
    /// Multiplayer: one player eliminated but the game continues.
    Eliminated(PlayerId),
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::events::GameEvent;

    // --- construction --------------------------------------------------------

    #[test]
    fn new_creates_requested_player_count() {
        let s = GameState::new(2, 42);
        assert_eq!(s.num_players(), 2);
        assert_eq!(s.players.len(), 2);
    }

    #[test]
    fn new_starts_each_player_at_20_life() {
        let s = GameState::new(4, 0);
        for p in 0..4 {
            assert_eq!(s.player(p).life, DEFAULT_STARTING_LIFE);
        }
    }

    #[test]
    fn new_initial_ids_and_flags() {
        let s = GameState::new(2, 0);
        assert_eq!(s.next_object_id, FIRST_OBJECT_ID);
        assert!(s.event_log.is_empty());
        assert!(s.stack.is_empty());
        assert!(s.objects.is_empty());
        assert!(s.combat.is_none());
        assert!(s.result.is_none());
        assert!(!s.is_game_over());
    }

    #[test]
    fn new_seed_is_preserved() {
        let s = GameState::new(2, 12345);
        assert_eq!(s.rng_seed, 12345);
    }

    #[test]
    #[should_panic(expected = "at least one player")]
    fn new_with_zero_players_panics() {
        let _ = GameState::new(0, 0);
    }

    // --- player access -------------------------------------------------------

    #[test]
    fn player_mut_allows_mutation() {
        let mut s = GameState::new(2, 0);
        s.player_mut(0).life = 15;
        assert_eq!(s.player(0).life, 15);
        assert_eq!(s.player(1).life, DEFAULT_STARTING_LIFE);
    }

    #[test]
    #[should_panic(expected = "out of range")]
    fn player_out_of_range_panics() {
        let s = GameState::new(2, 0);
        let _ = s.player(5);
    }

    // --- active / priority / opponents --------------------------------------

    #[test]
    fn active_and_priority_player_default_to_zero() {
        let s = GameState::new(2, 0);
        assert_eq!(s.active_player(), 0);
        assert_eq!(s.priority_player(), 0);
    }

    #[test]
    fn opponents_of_excludes_self() {
        let s = GameState::new(4, 0);
        let opps: Vec<_> = s.opponents_of(1).collect();
        assert_eq!(opps, vec![0, 2, 3]);
    }

    #[test]
    fn sole_opponent_in_two_player_game() {
        let s = GameState::new(2, 0);
        assert_eq!(s.sole_opponent(0), 1);
        assert_eq!(s.sole_opponent(1), 0);
    }

    #[test]
    #[should_panic(expected = "only valid for 2-player")]
    fn sole_opponent_panics_outside_two_player() {
        let s = GameState::new(4, 0);
        let _ = s.sole_opponent(0);
    }

    // --- allocate_object_id --------------------------------------------------

    #[test]
    fn allocate_object_id_is_monotonic_and_unique() {
        let mut s = GameState::new(2, 0);
        let a = s.allocate_object_id();
        let b = s.allocate_object_id();
        let c = s.allocate_object_id();
        assert_eq!(a, FIRST_OBJECT_ID);
        assert_eq!(b, a + 1);
        assert_eq!(c, b + 1);
        assert_eq!(s.next_object_id, c + 1);
    }

    // --- emit ---------------------------------------------------------------

    #[test]
    fn emit_appends_events_in_order() {
        let mut s = GameState::new(2, 0);
        s.emit(GameEvent::TurnBegins { player: 0, turn_number: 1 });
        s.emit(GameEvent::TurnEnds { player: 0 });
        assert_eq!(s.event_log.len(), 2);
        assert!(matches!(s.event_log[0], GameEvent::TurnBegins { .. }));
        assert!(matches!(s.event_log[1], GameEvent::TurnEnds { .. }));
    }

    #[test]
    fn emit_all_extends_log() {
        let mut s = GameState::new(2, 0);
        s.emit_all([
            GameEvent::TurnBegins { player: 0, turn_number: 1 },
            GameEvent::PhaseBegins { phase: crate::turn::Phase::PreCombatMain },
        ]);
        assert_eq!(s.event_log.len(), 2);
    }

    // --- zone queries --------------------------------------------------------

    #[test]
    fn zone_queries_delegate_to_arena() {
        use crate::objects::{GameObject, Characteristics};
        use crate::types::{TypeLine, PtValue};

        let mut s = GameState::new(2, 0);
        let creature_char = Characteristics {
            types: TypeLine::CREATURE.into(),
            power: Some(PtValue::Fixed(2)),
            toughness: Some(PtValue::Fixed(2)),
            ..Default::default()
        };
        let id = s.allocate_object_id();
        s.objects.insert(GameObject::new(id, 0, Zone::Battlefield, 1, creature_char));

        assert_eq!(s.zone_count(Zone::Battlefield), 1);
        assert_eq!(s.zone_count(Zone::Hand(0)), 0);
        assert_eq!(s.objects_in_zone(Zone::Battlefield).count(), 1);
    }

    // --- clone independence --------------------------------------------------

    #[test]
    fn clone_of_state_is_independent() {
        let mut a = GameState::new(2, 0);
        let mut b = a.clone();

        // Mutate clone.
        b.player_mut(0).life = 1;
        b.emit(GameEvent::TurnEnds { player: 0 });
        let _ = b.allocate_object_id();

        // Original is untouched.
        assert_eq!(a.player(0).life, DEFAULT_STARTING_LIFE);
        assert!(a.event_log.is_empty());
        assert_eq!(a.next_object_id, FIRST_OBJECT_ID);

        // Clone has the mutations.
        assert_eq!(b.player(0).life, 1);
        assert_eq!(b.event_log.len(), 1);
        assert_eq!(b.next_object_id, FIRST_OBJECT_ID + 1);

        // Original modification doesn't leak to the clone either.
        a.player_mut(1).life = 5;
        assert_eq!(b.player(1).life, DEFAULT_STARTING_LIFE);
    }

    // --- game over -----------------------------------------------------------

    #[test]
    fn is_game_over_reflects_result() {
        let mut s = GameState::new(2, 0);
        assert!(!s.is_game_over());
        s.result = Some(GameResult::Win(0));
        assert!(s.is_game_over());
    }

    // --- PlayerState helpers -------------------------------------------------

    #[test]
    fn can_play_land_starts_true() {
        let p = PlayerState::new(0, 20);
        assert!(p.can_play_land());
        assert_eq!(p.land_plays_remaining, 1);
    }

    #[test]
    fn can_play_land_is_false_after_lost() {
        let mut p = PlayerState::new(0, 20);
        p.has_lost = true;
        assert!(!p.can_play_land());
    }

    #[test]
    fn reset_land_plays_to_per_turn() {
        let mut p = PlayerState::new(0, 20);
        p.land_plays_per_turn = 2;
        p.land_plays_remaining = 0;
        p.reset_land_plays();
        assert_eq!(p.land_plays_remaining, 2);
    }

    #[test]
    fn is_alive_tracks_has_lost() {
        let mut p = PlayerState::new(0, 20);
        assert!(p.is_alive());
        p.has_lost = true;
        assert!(!p.is_alive());
    }

    #[test]
    fn life_totals_reads_in_player_order() {
        let mut s = GameState::new(3, 0);
        s.player_mut(0).life = 18;
        s.player_mut(1).life = 7;
        s.player_mut(2).life = 30;
        assert_eq!(s.life_totals(), vec![18, 7, 30]);
    }

    // --- spec-match new_game / new_game_with_format ------------------------

    fn register_stub_land(
        registry: &mut crate::registry::CardRegistry,
    ) -> crate::types::CardId {
        let name = registry.interner_mut().intern("Mountain");
        let chars = crate::objects::Characteristics {
            name,
            types: crate::types::TypeLine::LAND.into(),
            ..Default::default()
        };
        registry.register(
            crate::registry::CardDefinition::new(name, chars))
    }

    #[test]
    fn new_game_associated_fn_matches_engine_new_game() {
        let mut registry = crate::registry::CardRegistry::new();
        let card = register_stub_land(&mut registry);
        let deck = vec![card; 60];
        let from_state = GameState::new_game(
            vec![deck.clone(), deck.clone()], &registry, 7);
        let (from_engine, _) = crate::engine::new_game(
            vec![deck.clone(), deck], &registry, 7);
        assert_eq!(from_state.format, from_engine.format);
        assert_eq!(from_state.player(0).life, from_engine.player(0).life);
        assert_eq!(
            from_state.objects.count_in_zone(crate::zones::Zone::Hand(0)),
            from_engine.objects.count_in_zone(crate::zones::Zone::Hand(0)),
        );
    }

    #[test]
    fn new_game_with_format_associated_fn_uses_custom_format() {
        let mut registry = crate::registry::CardRegistry::new();
        let card = register_stub_land(&mut registry);
        let deck = vec![card; 100];
        let s = GameState::new_game_with_format(
            vec![deck.clone(), deck],
            crate::format::FormatConfig::commander(),
            &registry, 42,
        );
        assert_eq!(s.player(0).life, 40);
        assert!(s.format.use_command_zone);
    }
}
