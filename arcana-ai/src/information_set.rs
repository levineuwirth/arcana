//! Information-set projection (spec §13).
//!
//! [`project`] takes a full [`GameState`] and a perspective player,
//! and returns an [`ObservableState`] that represents only what
//! that player can see. Hidden objects (opponent hand contents, all
//! libraries, modulo `known_cards`) are **anonymized in place**:
//! their characteristics and per-object metadata are blanked, but
//! they retain `id`, `owner`, `controller`, and `zone` so zone
//! counts and ownership remain visible.
//!
//! # Visibility rules (v0)
//!
//! For each [`GameObject`] in the projected state:
//!
//! | Location                                 | Visible to perspective?                     |
//! | ---------------------------------------- | ------------------------------------------- |
//! | Perspective's own hand                   | yes                                         |
//! | Opponent's hand                          | no — unless id ∈ `perspective.known_cards`  |
//! | Any library (incl. own)                  | no — unless id ∈ `perspective.known_cards`  |
//! | Battlefield                              | yes                                         |
//! | Graveyard / Stack / Exile / Command      | yes                                         |
//! | Face-down on battlefield (Morph/Manifest) | **deferred to v1**                         |
//! | Face-down in exile                       | **deferred to v1**                          |
//!
//! `PlayerState` is left untouched: every field there is public
//! information (life, mana pool, poison, energy, experience,
//! `has_lost`, `has_conceded`, `commander_damage`, …) or is
//! deducible from the public game history (`known_cards` itself —
//! an attentive opponent tracks scry/peek effects, so masking it
//! adds complexity without strategic value for RL purposes). If
//! that conclusion changes — e.g., we want to mask the opponent's
//! `known_cards` to model an imperfect-recall agent — the
//! projection extends here, not in any consumer.
//!
//! # The footgun and the mitigation
//!
//! After projection, [`ObservableState::state`] is a `GameState`
//! that the encoder, the legality checker, and any other consumer
//! can read with the same API as a real game state. Anonymous
//! objects show up in `state.objects.iter()` looking like ordinary
//! objects with all-default characteristics — empty name, no
//! types, zero PT, no colors. **Card-specific consumers must
//! consult [`ObservableState::is_anonymous`] before reading
//! characteristics.** The [`ObservableState::anonymous_ids`] set
//! is the authoritative source; default-shaped characteristics is
//! just the rendering of that fact.
//!
//! The v0 [`crate::observation::BasicE2Encoder`] reads only public-
//! zone aggregates and zone counts, so projection is a no-op for
//! it today. Future card-specific encoder features (per-color hand
//! cmc distribution, etc.) MUST hook into `is_anonymous` or the
//! encoder will silently leak information through anonymized
//! objects.
//!
//! # Determinization
//!
//! [`determinize`] is stubbed. Determinization — sampling a
//! concrete `GameState` consistent with an `ObservableState` — is
//! IS-MCTS's table-stakes operation but it needs a deck multiset
//! to draw from, and Phase 2's deck-list shape isn't settled yet.
//! See the function's doc comment for the (provisional) signature
//! caveats.

use std::collections::HashSet;

use arcana_core::objects::{Characteristics, GameObject, ObjectId};
use arcana_core::state::GameState;
use arcana_core::types::{PermanentStatus, PlayerId};
use arcana_core::zones::Zone;

// =============================================================================
// ObservableState
// =============================================================================

/// One player's view of a [`GameState`]. Constructed by [`project`];
/// see module docs for visibility semantics.
#[derive(Debug, Clone)]
pub struct ObservableState {
    /// Projected `GameState`. Hidden objects have been anonymized
    /// in place — their `characteristics`, `counters`, `attachments`,
    /// `damage_marked`, and per-object status flags are blanked,
    /// but `id`, `owner`, `controller`, and `zone` are preserved so
    /// the zone-count topology survives.
    ///
    /// **Card-specific consumers must consult [`Self::is_anonymous`]
    /// before reading object characteristics.** Default-shaped
    /// characteristics on anonymous objects can theoretically
    /// collide with default-shaped characteristics on real objects;
    /// the anonymous_ids set is the source of truth.
    pub state: GameState,

    /// IDs of objects that were anonymized during projection.
    /// Includes opponent hand cards (in zones not visible to
    /// `perspective`) and library cards not in `known_cards`.
    pub anonymous_ids: HashSet<ObjectId>,

    /// The player whose perspective this projection takes.
    pub perspective: PlayerId,
}

impl ObservableState {
    /// True if `id` was anonymized during projection. Card-specific
    /// consumers MUST call this before reading
    /// `state.objects.get(id)?.characteristics` — see module docs.
    pub fn is_anonymous(&self, id: ObjectId) -> bool {
        self.anonymous_ids.contains(&id)
    }
}

// =============================================================================
// project
// =============================================================================

/// Project `state` to the [`ObservableState`] visible to
/// `perspective`. Non-destructive: clones `state` and mutates the
/// clone.
///
/// # Panics
/// Panics if `perspective` is out of range for `state.players`.
pub fn project(state: &GameState, perspective: PlayerId) -> ObservableState {
    assert!(
        (perspective as usize) < state.players.len(),
        "perspective {perspective} out of range for {} players",
        state.players.len()
    );

    let mut projected = state.clone();
    let mut anonymous_ids = HashSet::new();

    // Snapshot perspective's known_cards. We hold it by-value so the
    // borrow-checker doesn't flag the iter_mut on objects below.
    let known: HashSet<ObjectId> = projected
        .player(perspective)
        .known_cards
        .iter()
        .copied()
        .collect();

    for obj in projected.objects.iter_mut() {
        if !is_visible(perspective, obj, &known) {
            anonymize_object_in_place(obj);
            anonymous_ids.insert(obj.id);
        }
    }

    ObservableState { state: projected, anonymous_ids, perspective }
}

/// Whether a given object is visible to `perspective`. See module
/// docs for the visibility table.
fn is_visible(
    perspective: PlayerId,
    obj: &GameObject,
    known: &HashSet<ObjectId>,
) -> bool {
    if known.contains(&obj.id) {
        return true;
    }
    match obj.zone {
        Zone::Library(_) => false,
        Zone::Hand(p) => p == perspective,
        // Battlefield, Stack, Graveyard, Exile, Command — all public
        // in v0. Face-down on battlefield / face-down in exile is
        // out of scope; defer to v1.
        _ => true,
    }
}

/// Centralized anonymization. Every field that could leak hidden
/// information is reset to its default value here. **All anonymous-
/// object construction routes through this function** so a future
/// `Characteristics` field addition needs to update one place.
fn anonymize_object_in_place(obj: &mut GameObject) {
    obj.characteristics = Characteristics::default();
    obj.counters.clear();
    obj.attachments.clear();
    obj.attached_to = None;
    obj.damage_marked = 0;
    obj.has_deathtouch_damage = false;
    obj.status = PermanentStatus::default();
    obj.madness_pending = false;
    obj.adventure_exile_pending = false;
    obj.is_token = false;
    obj.visible_face = 0;
    obj.default_face_characteristics = None;
    // abilities is a Vec<AbilityId>; cleared so the projected object
    // doesn't expose which printed abilities the hidden card had.
    obj.abilities.clear();
}

// =============================================================================
// determinize (provisional stub)
// =============================================================================

/// Sample a concrete [`GameState`] consistent with `observable`.
/// Used by IS-MCTS rollouts to materialize a "world" that respects
/// the perspective player's information set.
///
/// # ⚠ PROVISIONAL SIGNATURE
///
/// The real implementation will likely need parameters not present
/// here:
///
/// * **A deck multiset** — the cards that started in each library,
///   so the sampler knows the universe of possibilities to draw
///   from. Phase 2's deck-list shape isn't settled yet.
/// * **Possibly `Option<GameState>` return** — some observed states
///   are inconsistent with any concrete world (the engine should
///   never produce one, but defensive coding is cheap).
/// * **Possibly batched `Vec<GameState>` return** — IS-MCTS
///   typically wants N independent samples per node; pulling them
///   one at a time wastes a bunch of setup.
///
/// Determinization also needs to respect cross-zone correlation:
/// the same N "unknown" cards are split between opponent's hand and
/// their library, so a sampler that draws each zone independently
/// would over- or under-count. The real implementation samples a
/// coherent assignment of the unknown multiset to the unknown
/// zones.
///
/// This signature **will change** when the implementation lands.
/// Don't write call sites against it yet.
pub fn determinize(_observable: &ObservableState, _seed: u64) -> GameState {
    unimplemented!(
        "determinization needs a deck multiset and a consistency-respecting \
         sampler. Phase 2 hasn't shaped the deck-list type yet; revisit when it does."
    )
}

// =============================================================================
// tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use arcana_core::objects::GameObject;

    /// Build a state with `n` players, each with `library`/`hand`/
    /// `graveyard`/`battlefield` synthetic objects in the given
    /// zones. Returns the state plus a vector of all-allocated ids
    /// in insertion order so tests can refer to specific objects.
    fn fixture_state() -> (GameState, FixtureIds) {
        let mut state = GameState::new(2, 0);

        // Allocate ids by hand (without going through engine
        // machinery, which has its own setup costs we don't need).
        let p0_hand = state.allocate_object_id();
        let p1_hand = state.allocate_object_id();
        let p0_lib = state.allocate_object_id();
        let p1_lib = state.allocate_object_id();
        let bf_obj = state.allocate_object_id();
        let gy_obj = state.allocate_object_id();
        let stk_obj = state.allocate_object_id();
        let ex_obj = state.allocate_object_id();

        state
            .objects
            .insert(make_object(p0_hand, 0, 0, Zone::Hand(0), "p0 hand card"));
        state
            .objects
            .insert(make_object(p1_hand, 1, 1, Zone::Hand(1), "p1 hand card"));
        state
            .objects
            .insert(make_object(p0_lib, 0, 0, Zone::Library(0), "p0 lib card"));
        state
            .objects
            .insert(make_object(p1_lib, 1, 1, Zone::Library(1), "p1 lib card"));
        state
            .objects
            .insert(make_object(bf_obj, 0, 0, Zone::Battlefield, "bf"));
        state
            .objects
            .insert(make_object(gy_obj, 1, 1, Zone::Graveyard(1), "gy"));
        state
            .objects
            .insert(make_object(stk_obj, 0, 0, Zone::Stack, "stk"));
        state
            .objects
            .insert(make_object(ex_obj, 1, 1, Zone::Exile, "ex"));

        (
            state,
            FixtureIds {
                p0_hand,
                p1_hand,
                p0_lib,
                p1_lib,
                bf_obj,
                gy_obj,
                stk_obj,
                ex_obj,
            },
        )
    }

    struct FixtureIds {
        p0_hand: ObjectId,
        p1_hand: ObjectId,
        p0_lib: ObjectId,
        p1_lib: ObjectId,
        bf_obj: ObjectId,
        gy_obj: ObjectId,
        stk_obj: ObjectId,
        ex_obj: ObjectId,
    }

    /// Build a GameObject with non-default characteristics so we
    /// can detect anonymization (the default characteristics will
    /// differ from the marked ones).
    fn make_object(
        id: ObjectId,
        owner: PlayerId,
        controller: PlayerId,
        zone: Zone,
        _label: &str,
    ) -> GameObject {
        // Marker characteristics so anonymization is detectable on
        // the object level. is_aura sits inside Characteristics;
        // damage_marked sits outside. Both should be reset by
        // anonymize_object_in_place.
        let mut chars = Characteristics::default();
        chars.is_aura = true;
        let mut obj = GameObject::new(id, owner, zone, 0, chars);
        obj.controller = controller;
        obj.damage_marked = 7;
        obj
    }

    fn obj<'a>(state: &'a GameState, id: ObjectId) -> &'a GameObject {
        state.objects.get(id).expect("object exists in fixture")
    }

    // -- visibility --------------------------------------------------

    #[test]
    fn own_hand_visible_opponent_hand_anonymized() {
        let (state, ids) = fixture_state();
        let view = project(&state, 0);

        // Own hand visible.
        assert!(!view.is_anonymous(ids.p0_hand));
        assert!(obj(&view.state, ids.p0_hand).characteristics.is_aura);

        // Opponent hand anonymized.
        assert!(view.is_anonymous(ids.p1_hand));
        assert!(!obj(&view.state, ids.p1_hand).characteristics.is_aura);
        assert_eq!(obj(&view.state, ids.p1_hand).damage_marked, 0);

        // Owner / controller / zone preserved on the anonymous
        // object — count topology must survive.
        let p1h = obj(&view.state, ids.p1_hand);
        assert_eq!(p1h.owner, 1);
        assert_eq!(p1h.controller, 1);
        assert!(matches!(p1h.zone, Zone::Hand(1)));
    }

    #[test]
    fn libraries_anonymized_for_both_players() {
        let (state, ids) = fixture_state();
        let view_p0 = project(&state, 0);
        // Own library hidden from owner: face-down deck, no scry.
        assert!(view_p0.is_anonymous(ids.p0_lib));
        assert!(view_p0.is_anonymous(ids.p1_lib));

        let view_p1 = project(&state, 1);
        assert!(view_p1.is_anonymous(ids.p0_lib));
        assert!(view_p1.is_anonymous(ids.p1_lib));
    }

    #[test]
    fn known_cards_overrides_hidden_zones() {
        let (mut state, ids) = fixture_state();
        // Mark p0 as having seen p1's hand card (e.g., from a
        // peek-effect). Project from p0's perspective; the card
        // should be visible despite being in opponent's hand.
        state.player_mut(0).known_cards.insert(ids.p1_hand);
        let view = project(&state, 0);
        assert!(!view.is_anonymous(ids.p1_hand));
        assert!(obj(&view.state, ids.p1_hand).characteristics.is_aura);
    }

    #[test]
    fn known_cards_with_stale_id_does_not_corrupt_projection() {
        // Engine-side known_cards lifecycle bug or a card that
        // already moved to a public zone shouldn't break
        // projection. The override is a strict "if id ∈ set, show";
        // a stale id pointing at a battlefield object is a no-op
        // (the object would be visible anyway); a stale id pointing
        // at nothing is harmless.
        let (mut state, _ids) = fixture_state();
        // Stale id that doesn't correspond to any object.
        state.player_mut(0).known_cards.insert(99_999);
        // Stale id that corresponds to a public-zone object.
        // (Shouldn't matter since battlefield is visible regardless.)
        let bf_id = state
            .objects
            .objects_in_zone(Zone::Battlefield)
            .next()
            .unwrap()
            .id;
        state.player_mut(0).known_cards.insert(bf_id);

        let view = project(&state, 0);
        // Battlefield object still visible.
        assert!(!view.is_anonymous(bf_id));
        // Projection didn't crash on the dangling id.
        assert!(!view.is_anonymous(99_999));
    }

    #[test]
    fn public_zones_unchanged() {
        let (state, ids) = fixture_state();
        let view = project(&state, 0);
        for id in [ids.bf_obj, ids.gy_obj, ids.stk_obj, ids.ex_obj] {
            assert!(
                !view.is_anonymous(id),
                "public-zone object {id} must not be anonymized"
            );
            // Sanity: the marker characteristics survived.
            assert!(obj(&view.state, id).characteristics.is_aura);
            assert_eq!(obj(&view.state, id).damage_marked, 7);
        }
    }

    // -- non-destructiveness -----------------------------------------

    #[test]
    fn project_does_not_mutate_input_state() {
        let (state, ids) = fixture_state();
        // Deep-snapshot the original via Clone.
        let snapshot = state.clone();
        let _ = project(&state, 0);

        // The original opponent-hand card should still carry its
        // marker characteristics. If projection accidentally
        // mutated through shared structure, this would fail.
        assert!(obj(&state, ids.p1_hand).characteristics.is_aura);
        assert_eq!(obj(&state, ids.p1_hand).damage_marked, 7);

        // Stronger check: every object byte-for-byte unchanged.
        // ObjectArena doesn't impl PartialEq, so compare via
        // pairwise object equality on the IDs we know about.
        for id in [
            ids.p0_hand,
            ids.p1_hand,
            ids.p0_lib,
            ids.p1_lib,
            ids.bf_obj,
            ids.gy_obj,
            ids.stk_obj,
            ids.ex_obj,
        ] {
            let before = obj(&snapshot, id);
            let after = obj(&state, id);
            assert_eq!(before.zone, after.zone);
            assert_eq!(before.owner, after.owner);
            assert_eq!(before.damage_marked, after.damage_marked);
            assert_eq!(before.characteristics.is_aura, after.characteristics.is_aura);
        }
    }

    // -- anonymous_ids accuracy ---------------------------------------

    #[test]
    fn anonymous_ids_set_matches_anonymized_objects() {
        let (state, ids) = fixture_state();
        let view = project(&state, 0);
        // Expected: opponent's hand + both libraries → 3 ids.
        let expected: HashSet<ObjectId> =
            [ids.p1_hand, ids.p0_lib, ids.p1_lib].into_iter().collect();
        assert_eq!(view.anonymous_ids, expected);

        // is_anonymous mirrors the set membership exactly.
        for id in [ids.p0_hand, ids.bf_obj, ids.gy_obj, ids.stk_obj, ids.ex_obj] {
            assert!(!view.is_anonymous(id));
        }
        for id in expected {
            assert!(view.is_anonymous(id));
        }
    }

    // -- determinism --------------------------------------------------

    #[test]
    fn project_is_deterministic() {
        let (state, ids) = fixture_state();
        let a = project(&state, 0);
        let b = project(&state, 0);
        assert_eq!(a.anonymous_ids, b.anonymous_ids);
        assert_eq!(a.perspective, b.perspective);
        // Spot-check object identity.
        assert_eq!(
            obj(&a.state, ids.p1_hand).characteristics.is_aura,
            obj(&b.state, ids.p1_hand).characteristics.is_aura,
        );
    }

    // -- topology preservation ---------------------------------------

    #[test]
    fn zone_counts_are_preserved_across_projection() {
        let (state, _ids) = fixture_state();
        let view = project(&state, 0);
        for zone in [
            Zone::Hand(0),
            Zone::Hand(1),
            Zone::Library(0),
            Zone::Library(1),
            Zone::Battlefield,
            Zone::Graveyard(1),
            Zone::Stack,
            Zone::Exile,
        ] {
            assert_eq!(
                state.zone_count(zone),
                view.state.zone_count(zone),
                "zone count drift at {zone:?}"
            );
        }
    }

    // -- PlayerState policy ------------------------------------------

    #[test]
    fn player_state_is_unchanged_across_projection() {
        // The visibility table declares all PlayerState fields
        // public. This test pins that decision: if a future
        // projection masks a field there, this test fails loudly so
        // the change is deliberate.
        let (mut state, _ids) = fixture_state();
        state.player_mut(0).life = 17;
        state.player_mut(1).life = 13;
        state.player_mut(0).poison_counters = 4;
        state.player_mut(1).energy = 9;

        let view = project(&state, 0);

        for p in 0..2 {
            assert_eq!(state.player(p).life, view.state.player(p).life);
            assert_eq!(
                state.player(p).poison_counters,
                view.state.player(p).poison_counters
            );
            assert_eq!(state.player(p).energy, view.state.player(p).energy);
            assert_eq!(
                state.player(p).has_lost,
                view.state.player(p).has_lost
            );
            assert_eq!(
                state.player(p).land_plays_remaining,
                view.state.player(p).land_plays_remaining
            );
        }
    }

    // -- determinize stub --------------------------------------------

    #[test]
    #[should_panic(expected = "determinization")]
    fn determinize_panics_until_implemented() {
        // Smoke test that the stub fires its informative message.
        // When the real implementation lands and the stub goes
        // away, this test goes with it.
        let (state, _ids) = fixture_state();
        let view = project(&state, 0);
        let _ = determinize(&view, 0);
    }

    // -- API ergonomics ----------------------------------------------

    #[test]
    fn perspective_field_is_carried_on_observable_state() {
        let (state, _ids) = fixture_state();
        let view = project(&state, 1);
        assert_eq!(view.perspective, 1);
    }

    #[test]
    #[should_panic(expected = "perspective")]
    fn out_of_range_perspective_panics() {
        let (state, _ids) = fixture_state();
        let _ = project(&state, 5);
    }
}
