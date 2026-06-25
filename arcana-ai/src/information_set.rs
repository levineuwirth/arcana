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
//! [`determinize`] samples a concrete `GameState` consistent with an
//! `ObservableState` — IS-MCTS's table-stakes operation. It takes the
//! per-player starting [`DeckList`]s as the universe of hidden cards,
//! shuffles each player's unseen residual multiset across their anonymous
//! slots, and re-instantiates real card identities. See its doc comment.

use std::collections::HashSet;

use arcana_core::engine::instantiate_card_object;
use arcana_core::objects::{Characteristics, GameObject, ObjectId};
use arcana_core::registry::CardRegistry;
use arcana_core::state::GameState;
use arcana_core::types::{CardId, PermanentStatus, PlayerId};
use arcana_core::zones::Zone;
use rand::seq::SliceRandom;
use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;

/// A starting deck: the multiset of [`CardId`]s a player began with (order
/// irrelevant). Same shape `new_game` / [`crate::search`] / `GameRecord` use.
/// Determinization needs it to know the universe of hidden cards to sample.
pub type DeckList = Vec<CardId>;

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

/// Sample a concrete [`GameState`] consistent with `observable`: every
/// anonymized (hidden) object is filled in with a real card sampled from the
/// universe of cards that could be there. Used by IS-MCTS to materialize a
/// "world" that respects the perspective player's information set so rollouts
/// can run on the true engine.
///
/// `decks[p]` is player `p`'s starting deck multiset — the universe of cards
/// that could be in `p`'s hidden zones.
///
/// # Model
/// For each player `p`, the **hidden** (anonymous) objects owned by `p` are
/// exactly their cards whose identity the perspective can't see:
/// * the opponent's hand AND library, and
/// * the perspective player's own library (face-down deck — the multiset is
///   known, the order isn't).
///
/// The set of cards that fill them = `decks[p]` minus the card identities the
/// perspective can already see among `p`'s objects (hand-if-own, battlefield,
/// graveyard, exile, stack, and any `known_cards`). That residual multiset is
/// shuffled and dealt across `p`'s anonymous slots — which automatically
/// respects cross-zone correlation (a card dealt to the opponent's hand can't
/// also be in their library) and the public hand SIZE (the number of anonymous
/// hand slots is fixed). Cards visible via `known_cards` keep their real
/// identity and library position; only the genuinely-unknown remainder moves.
///
/// Tokens and copies (card ids not present in the deck) are skipped by the
/// subtraction, so they don't perturb the count. The sampler is total and
/// panic-free: if the residual ever mismatches the slot count (e.g. a card
/// changed its `card_id` via transform), leftover slots are filled cyclically
/// from the deck rather than left blank or panicking mid-search.
///
/// The result has NO anonymous objects — it is a full state the engine can
/// `step`. `seed` makes the sample reproducible.
pub fn determinize(
    observable: &ObservableState,
    decks: &[DeckList],
    registry: &CardRegistry,
    seed: u64,
) -> GameState {
    let mut state = observable.state.clone();
    let mut rng = ChaCha8Rng::seed_from_u64(seed);

    for p in 0..state.num_players() {
        // Hidden slots owned by p (sorted → deterministic given the seed).
        let mut anon: Vec<ObjectId> = state
            .objects
            .iter()
            .filter(|o| o.owner == p && observable.anonymous_ids.contains(&o.id))
            .map(|o| o.id)
            .collect();
        if anon.is_empty() {
            continue;
        }
        anon.sort_unstable();

        // Residual = deck minus the card identities already visible for p.
        let deck: &[CardId] = decks.get(p as usize).map(Vec::as_slice).unwrap_or(&[]);
        let mut residual: Vec<CardId> = deck.to_vec();
        for o in state.objects.iter() {
            if o.owner == p && !observable.anonymous_ids.contains(&o.id) {
                if let Some(pos) = residual.iter().position(|&c| c == o.card_id) {
                    residual.swap_remove(pos);
                }
            }
        }
        residual.shuffle(&mut rng);
        debug_assert_eq!(
            anon.len(), residual.len(),
            "determinize: player {p} has {} hidden slots but {} residual cards",
            anon.len(), residual.len()
        );

        // Deal residual cards into the hidden slots, re-instantiating each
        // object's identity exactly as game setup does.
        for (i, &id) in anon.iter().enumerate() {
            let card_id = match residual.get(i) {
                Some(&c) => c,
                // Defensive fallback (should not happen for vanilla decks):
                // cycle through the deck so no slot is left blank.
                None if !deck.is_empty() => deck[i % deck.len()],
                None => continue,
            };
            let (owner, zone) = {
                let o = state.objects.get(id).expect("anon id exists");
                (o.owner, o.zone)
            };
            state.objects.remove(id);
            state
                .objects
                .insert(instantiate_card_object(registry, id, owner, zone, card_id));
        }
    }

    state
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

    // -- determinize -------------------------------------------------

    /// A real 2-player game projected to a perspective and then determinized
    /// should: (a) leave NO object with default/blank identity in the hidden
    /// zones, (b) preserve every player's per-zone object COUNT, and (c) keep
    /// the perspective's own visible cards (hand) exactly as-is.
    #[test]
    fn determinize_fills_hidden_zones_consistently() {
        use arcana_core::engine::new_game;
        let reg = arcana_cards::build_catalog();
        let decks = vec![
            arcana_cards::sample_deck(&reg, 4),
            arcana_cards::sample_deck(&reg, 9),
        ];
        // Advance past mulligans so hands/libraries are populated.
        let (mut state, _y) = new_game(decks.clone(), &reg, 21);
        for a in [arcana_core::actions::Action::MulliganKeep,
                  arcana_core::actions::Action::MulliganKeep] {
            let (s, _y) = arcana_core::engine::step(state, a, &reg);
            state = s;
        }

        let view = project(&state, 0);
        let world = determinize(&view, &decks, &reg, 1234);

        // (b) Per-player per-zone counts are unchanged by determinization.
        for p in 0..state.num_players() {
            for zone in [Zone::Hand(p), Zone::Library(p), Zone::Graveyard(p)] {
                let before = state.objects.objects_in_zone(zone).count();
                let after = world.objects.objects_in_zone(zone).count();
                assert_eq!(before, after, "zone {zone:?} count changed");
            }
        }

        // (a) No anonymous object survives in the determinized world: each
        // formerly-hidden object now has a real card_id present in its owner's
        // deck.
        for id in &view.anonymous_ids {
            let o = world.objects.get(*id).expect("filled object exists");
            assert!(
                decks[o.owner as usize].contains(&o.card_id),
                "hidden object {id} got card {} not in owner {}'s deck",
                o.card_id, o.owner
            );
        }

        // (c) The perspective's own hand cards are untouched (still their real
        // identities — they were never anonymous).
        for o in state.objects.objects_in_zone(Zone::Hand(0)) {
            let w = world.objects.get(o.id).unwrap();
            assert_eq!(w.card_id, o.card_id, "own hand card {} changed", o.id);
        }
    }

    /// The perspective player's own library is hidden (face-down deck), so
    /// determinization must reshuffle it from the KNOWN multiset: the set of
    /// card ids must match the real library exactly (as a multiset), even
    /// though the order may differ.
    #[test]
    fn determinize_preserves_own_library_multiset() {
        use arcana_core::engine::new_game;
        use std::collections::BTreeMap;
        let reg = arcana_cards::build_catalog();
        let decks = vec![
            arcana_cards::sample_deck(&reg, 4),
            arcana_cards::sample_deck(&reg, 9),
        ];
        let (mut state, _y) = new_game(decks.clone(), &reg, 21);
        for a in [arcana_core::actions::Action::MulliganKeep,
                  arcana_core::actions::Action::MulliganKeep] {
            let (s, _y) = arcana_core::engine::step(state, a, &reg);
            state = s;
        }

        let view = project(&state, 0);
        let world = determinize(&view, &decks, &reg, 77);

        let multiset = |st: &GameState, zone: Zone| -> BTreeMap<CardId, u32> {
            let mut m = BTreeMap::new();
            for o in st.objects.objects_in_zone(zone) {
                *m.entry(o.card_id).or_default() += 1;
            }
            m
        };
        assert_eq!(
            multiset(&state, Zone::Library(0)),
            multiset(&world, Zone::Library(0)),
            "own library multiset must be preserved by determinization"
        );
    }

    /// A determinized world is a fully-valid `GameState`: the engine can `step`
    /// it for many turns without panicking (no leftover anonymous/blank cards,
    /// zone indices intact). This is the property IS-MCTS rollouts rely on.
    #[test]
    fn determinized_world_is_playable() {
        use arcana_core::engine::{new_game, step};
        use arcana_core::legal_actions::legal_actions;
        let reg = arcana_cards::build_catalog();
        let decks = vec![
            arcana_cards::sample_deck(&reg, 4),
            arcana_cards::sample_deck(&reg, 9),
        ];
        let (state, _y) = new_game(decks.clone(), &reg, 21);
        let view = project(&state, 0);
        let mut s = determinize(&view, &decks, &reg, 2024);

        let mut steps = 0;
        while steps < 200 && !s.is_game_over() {
            let legal = legal_actions(&s, &reg);
            // Prefer a non-concede action so the smoke test actually advances.
            let act = legal.iter()
                .find(|a| !matches!(a, arcana_core::actions::Action::Concede))
                .or_else(|| legal.first());
            let Some(act) = act.cloned() else { break };
            let (ns, _y) = step(s, act, &reg);
            s = ns;
            steps += 1;
        }
        assert!(steps > 0, "determinized world should be steppable");
    }

    /// Determinization is reproducible: same seed → identical world.
    #[test]
    fn determinize_is_deterministic_given_seed() {
        use arcana_core::engine::new_game;
        let reg = arcana_cards::build_catalog();
        let decks = vec![
            arcana_cards::sample_deck(&reg, 4),
            arcana_cards::sample_deck(&reg, 9),
        ];
        let (state, _y) = new_game(decks.clone(), &reg, 21);
        let view = project(&state, 1);
        let w1 = determinize(&view, &decks, &reg, 555);
        let w2 = determinize(&view, &decks, &reg, 555);
        for id in &view.anonymous_ids {
            assert_eq!(
                w1.objects.get(*id).unwrap().card_id,
                w2.objects.get(*id).unwrap().card_id,
                "same seed must produce the same determinization"
            );
        }
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
