//! Game objects: `ObjectId`, `ObjectArena`, `GameObject`, `Characteristics`.
//!
//! Addendum Section 4.2 / Listing 2, Phase 1 Task #4. Depends on tasks 1–3
//! (types, mana, zones).
//!
//! Every in-game entity — cards, tokens, copies on the stack, emblems — is
//! a [`GameObject`] living in an [`ObjectArena`] and keyed by a [`ObjectId`].
//! This design gives O(1) lookup/removal without the pointer indirection a
//! tree of `Rc`s would require, and it keeps `GameState` trivially cloneable
//! (the spec's first design principle).
//!
//! The arena currently uses `crate::collections::HashMap`. The spec (Section
//! 4.2) notes it can be swapped to `im::HashMap` (HAMT) later to reduce
//! clone cost from O(n) to O(log n). That migration is behind a feature
//! flag we'll add when profiling shows clone is the bottleneck.

use serde::{Serialize, Deserialize};
use crate::collections::HashMap;

use crate::mana::ManaCost;
use crate::types::*;
use crate::zones::{Zone, ZoneKind};

pub type ObjectId = u32;

/// Sentinel for "no object" — never assigned by the arena. Used by tests
/// and as a placeholder for the pre-game state. Real object ids start at 1.
pub const NULL_OBJECT_ID: ObjectId = 0;

// =============================================================================
// ObjectArena
// =============================================================================

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct ObjectArena {
    objects: HashMap<ObjectId, GameObject>,
}

impl ObjectArena {
    pub fn new() -> Self { Self::default() }

    pub fn len(&self) -> usize { self.objects.len() }
    pub fn is_empty(&self) -> bool { self.objects.is_empty() }
    pub fn contains(&self, id: ObjectId) -> bool { self.objects.contains_key(&id) }

    pub fn get(&self, id: ObjectId) -> Option<&GameObject> {
        self.objects.get(&id)
    }
    pub fn get_mut(&mut self, id: ObjectId) -> Option<&mut GameObject> {
        self.objects.get_mut(&id)
    }

    /// Insert an object. Returns its id for convenience; panics if an object
    /// with the same id already exists (a bug — ids should be monotonic).
    pub fn insert(&mut self, obj: GameObject) -> ObjectId {
        let id = obj.id;
        if self.objects.insert(id, obj).is_some() {
            panic!("ObjectArena: duplicate ObjectId {id} inserted");
        }
        id
    }

    pub fn remove(&mut self, id: ObjectId) -> Option<GameObject> {
        self.objects.remove(&id)
    }

    pub fn iter(&self) -> impl Iterator<Item = &GameObject> + '_ {
        self.objects.values()
    }
    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut GameObject> + '_ {
        self.objects.values_mut()
    }
    pub fn ids(&self) -> impl Iterator<Item = ObjectId> + '_ {
        self.objects.keys().copied()
    }

    /// All objects currently in `zone`.
    pub fn objects_in_zone(&self, zone: Zone) -> impl Iterator<Item = &GameObject> + '_ {
        self.iter().filter(move |o| o.zone == zone)
    }

    /// All objects in any zone of kind `kind` (e.g. "any graveyard").
    pub fn objects_in_zone_kind(&self, kind: ZoneKind) -> impl Iterator<Item = &GameObject> + '_ {
        self.iter().filter(move |o| o.zone.kind() == kind)
    }

    /// All objects controlled by `player`.
    pub fn objects_controlled_by(&self, player: PlayerId) -> impl Iterator<Item = &GameObject> + '_ {
        self.iter().filter(move |o| o.controller == player)
    }

    pub fn count_in_zone(&self, zone: Zone) -> usize {
        self.objects_in_zone(zone).count()
    }

    /// Ids in a zone, sorted ascending. Useful for deterministic iteration
    /// (replay, debug dumps) since the underlying HashMap is unordered.
    pub fn ids_in_zone_sorted(&self, zone: Zone) -> Vec<ObjectId> {
        let mut ids: Vec<ObjectId> = self.objects_in_zone(zone).map(|o| o.id).collect();
        ids.sort_unstable();
        ids
    }
}

// =============================================================================
// GameObject
// =============================================================================

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GameObject {
    pub id: ObjectId,
    pub owner: PlayerId,
    pub controller: PlayerId,
    pub zone: Zone,
    /// Key into [`crate::registry::CardRegistry`]. The registry holds
    /// the printed card data (spell abilities, triggers, etc.). Real
    /// card definitions live in the `arcana-cards` catalog crate,
    /// which registers them against this same `CardRegistry` type.
    pub card_id: CardId,
    /// Current computed characteristics (after the layer system applies, in
    /// the fully-implemented engine). For now, this is the base
    /// characteristics from the card registry, possibly modified.
    pub characteristics: Characteristics,
    pub counters: CounterMap,
    pub attachments: Vec<ObjectId>,
    pub attached_to: Option<ObjectId>,
    pub damage_marked: u32,
    /// CR 702.2b — set to `true` when a creature has been dealt any
    /// nonzero amount of damage by a source with Deathtouch since its
    /// damage was last cleared. Treated as lethal by the SBA regardless
    /// of total damage versus toughness. Cleared at cleanup alongside
    /// [`Self::damage_marked`].
    pub has_deathtouch_damage: bool,
    pub abilities: Vec<AbilityId>,
    pub status: PermanentStatus,
    /// CR 702.34a — Madness replacement marker. Set when the
    /// discard-to-madness-exile replacement routes the card to
    /// [`Zone::Exile`] instead of graveyard; cleared when the card
    /// leaves exile (madness cast succeeds, or some other effect
    /// moves it). While the flag is on, [`crate::legal_actions`]
    /// emits a [`CastModifier::Madness`] cast path for the object
    /// using its [`crate::effects::KeywordAbility::Madness`] cost.
    /// The flag does not migrate across zone-change re-ids — it
    /// only lives during the brief exile window between discard and
    /// cast/cleanup.
    pub madness_pending: bool,
    /// CR 712 — which printed face of a multi-face card this object
    /// is currently showing. `0` = front face (default, and the only
    /// state for single-face cards). `1` = back face — set during
    /// cast/play of an MDFC back face or a Transform back (CR 712.4 /
    /// CR 712.5) so activated-ability enumeration and face-gated
    /// characteristics agree about which face is live.
    ///
    /// Preserved across re-ids (zone changes) because `reset_on_zone_change`
    /// does not clear it; that's a Phase-2 simplification — CR 712.2b
    /// says the card reverts to front face in zones other than stack
    /// and battlefield, but no current seed exercises the revert. See
    /// `CardDefinition::with_mdfc_back` for the full note.
    pub visible_face: u8,
    /// CR 715 — Adventure exile marker. Set when an Adventure spell
    /// (cast via [`crate::actions::CastModifier::Adventure`]) leaves
    /// the stack via resolution, counter, or fizzle — the card
    /// routes to [`Zone::Exile`] instead of graveyard and this flag
    /// opens the creature-cast window. While flagged,
    /// [`crate::legal_actions`] emits a
    /// [`crate::actions::CastModifier::AdventureCreature`] cast path
    /// for the object using its printed creature-face mana cost.
    /// The flag does not survive the re-id on the exile→stack (and
    /// subsequent stack→battlefield) moves, so the resulting
    /// battlefield creature is an ordinary object with no adventure
    /// residue.
    pub adventure_exile_pending: bool,
}

impl GameObject {
    /// Construct a new object. `controller` defaults to `owner`; change it
    /// explicitly for effects that move control (Threaten, Control Magic).
    pub fn new(
        id: ObjectId,
        owner: PlayerId,
        zone: Zone,
        card_id: CardId,
        characteristics: Characteristics,
    ) -> Self {
        Self {
            id,
            owner,
            controller: owner,
            zone,
            card_id,
            characteristics,
            counters: CounterMap::new(),
            attachments: Vec::new(),
            attached_to: None,
            damage_marked: 0,
            has_deathtouch_damage: false,
            abilities: Vec::new(),
            status: PermanentStatus::default(),
            madness_pending: false,
            adventure_exile_pending: false,
            visible_face: 0,
        }
    }

    // --- Type-line queries (forwarded to characteristics for ergonomics) ---

    pub fn is_creature(&self)       -> bool { self.characteristics.types.is_creature() }
    pub fn is_land(&self)           -> bool { self.characteristics.types.is_land() }
    pub fn is_artifact(&self)       -> bool { self.characteristics.types.is_artifact() }
    pub fn is_enchantment(&self)    -> bool { self.characteristics.types.is_enchantment() }
    pub fn is_planeswalker(&self)   -> bool { self.characteristics.types.is_planeswalker() }
    pub fn is_instant(&self)        -> bool { self.characteristics.types.is_instant() }
    pub fn is_sorcery(&self)        -> bool { self.characteristics.types.is_sorcery() }
    pub fn is_spell(&self)          -> bool { self.characteristics.types.is_spell() }

    /// On the battlefield and of a permanent type. Per CR 110.1, "permanent"
    /// refers to cards/tokens on the battlefield specifically.
    pub fn is_permanent_on_battlefield(&self) -> bool {
        self.zone.is_battlefield() && self.characteristics.types.is_permanent()
    }

    // --- Tapped status ------------------------------------------------------

    pub fn is_tapped(&self) -> bool { self.status.tapped }

    /// Tap this object. Returns `true` if the state changed, `false` if it
    /// was already tapped.
    pub fn tap(&mut self) -> bool {
        if self.status.tapped { false } else { self.status.tapped = true; true }
    }

    /// Untap this object. Returns `true` if the state changed.
    pub fn untap(&mut self) -> bool {
        if !self.status.tapped { false } else { self.status.tapped = false; true }
    }

    // --- Counters -----------------------------------------------------------

    pub fn count_counters(&self, kind: CounterKind) -> u32 {
        self.counters.get(&kind).copied().unwrap_or(0)
    }

    pub fn has_counter(&self, kind: CounterKind) -> bool {
        self.count_counters(kind) > 0
    }

    /// Add `count` counters of `kind`. No-op if `count == 0`.
    pub fn add_counters(&mut self, kind: CounterKind, count: u32) {
        if count == 0 { return; }
        *self.counters.entry(kind).or_insert(0) += count;
    }

    /// Remove up to `count` counters of `kind`. Returns the number actually
    /// removed, capped at the current count. The entry is evicted when its
    /// count drops to zero.
    pub fn remove_counters(&mut self, kind: CounterKind, count: u32) -> u32 {
        if count == 0 { return 0; }
        match self.counters.get_mut(&kind) {
            Some(n) => {
                let removed = (*n).min(count);
                *n -= removed;
                if *n == 0 { self.counters.remove(&kind); }
                removed
            }
            None => 0,
        }
    }

    /// Annihilate matching +1/+1 and -1/-1 counters (CR 704.5p). Returns the
    /// number annihilated of each. Called by the SBA check.
    pub fn annihilate_pt_counters(&mut self) -> u32 {
        let plus  = self.count_counters(CounterKind::PlusOnePlusOne);
        let minus = self.count_counters(CounterKind::MinusOneMinusOne);
        let pairs = plus.min(minus);
        if pairs > 0 {
            self.remove_counters(CounterKind::PlusOnePlusOne, pairs);
            self.remove_counters(CounterKind::MinusOneMinusOne, pairs);
        }
        pairs
    }

    // --- Damage -------------------------------------------------------------

    pub fn mark_damage(&mut self, amount: u32) {
        self.damage_marked = self.damage_marked.saturating_add(amount);
    }

    pub fn clear_damage(&mut self) {
        self.damage_marked = 0;
        self.has_deathtouch_damage = false;
    }

    // --- Raw P/T (counter arithmetic only; NOT the layer-system answer) -----
    //
    // These helpers are deliberately narrow and ugly-named. They apply
    // layer 7d (P/T counters) to the object's stored base characteristics
    // and nothing else. The full answer to "what is this creature's power
    // right now" requires the layer system (CR 613) and access to other
    // objects (Glorious Anthem, Humility, opponent auras, etc.), which is
    // the responsibility of `GameState::computed_power` in `layers.rs`.
    //
    // Use `raw_*_with_counters` only when you explicitly want counter math
    // on an object in isolation — typically in tests or in the SBA when
    // the layer system has already baked its result into `characteristics`.

    /// Base power plus the net of +1/+1 and -1/-1 counters. Returns `None`
    /// if the object has no base power (i.e. isn't a creature) or its base
    /// P uses `*` without a provided `star_value`.
    pub fn raw_power_with_counters(&self, star_value: Option<i32>) -> Option<i32> {
        let base = self.characteristics.power?.resolve(star_value)?;
        Some(base + self.pt_counter_adjustment())
    }

    /// Base toughness plus the net of +1/+1 and -1/-1 counters. See
    /// [`raw_power_with_counters`](Self::raw_power_with_counters) for the
    /// caveats — this is NOT the full layer-system answer.
    pub fn raw_toughness_with_counters(&self, star_value: Option<i32>) -> Option<i32> {
        let base = self.characteristics.toughness?.resolve(star_value)?;
        Some(base + self.pt_counter_adjustment())
    }

    /// Net P/T swing from +1/+1 and -1/-1 counters. Equivalent under
    /// annihilation (either order of arithmetic gives the same sum).
    fn pt_counter_adjustment(&self) -> i32 {
        let plus  = self.count_counters(CounterKind::PlusOnePlusOne) as i32;
        let minus = self.count_counters(CounterKind::MinusOneMinusOne) as i32;
        plus - minus
    }

    /// CR 704.5g predicate using raw (counter-only) toughness. The SBA in
    /// Phase 1 can use this; once the layer system lands, the SBA will
    /// instead call a state-level helper that knows about Humility etc.
    ///
    /// Returns `false` when toughness is 0 or negative — that's CR 704.5f
    /// territory (separate SBA).
    pub fn has_raw_lethal_damage(&self) -> bool {
        match self.raw_toughness_with_counters(None) {
            Some(t) if t > 0 => self.damage_marked as i32 >= t,
            _ => false,
        }
    }

    // --- Attachment ---------------------------------------------------------

    pub fn is_attached(&self) -> bool { self.attached_to.is_some() }

    /// Attach this object to `target`. Does not update `target.attachments`
    /// — that's the caller's responsibility (usually a state-level helper
    /// that maintains both sides of the relationship).
    pub fn attach_to(&mut self, target: ObjectId) {
        self.attached_to = Some(target);
    }

    pub fn detach(&mut self) { self.attached_to = None; }

    /// Reset per-zone transient fields when this object changes zones
    /// (CR 400.7). Called by the zone-transition helper.
    pub fn reset_on_zone_change(&mut self) {
        self.counters.clear();
        self.attachments.clear();
        self.attached_to = None;
        self.damage_marked = 0;
        self.has_deathtouch_damage = false;
        self.status = PermanentStatus::default();
        // Madness marker is a single-use zone-specific flag (CR
        // 702.34a). It's set in exile when the discard-replacement
        // fires; leaving exile (via madness cast, cleanup, or any
        // other effect) drops the flag. The re-id on zone change
        // then gives the downstream zone a clean object.
        self.madness_pending = false;
        // Adventure-exile marker is similarly zone-local (CR 715):
        // set when an Adventure spell leaves the stack to exile,
        // cleared the moment the card moves anywhere (creature
        // cast, effect-driven exile, cleanup). Dropping it on re-id
        // keeps the post-exile object from carrying stale adventure
        // residue into the battlefield.
        self.adventure_exile_pending = false;
    }
}

// =============================================================================
// Characteristics
// =============================================================================

/// Computed characteristics of an object. For cards, this is populated from
/// the card registry's `base_characteristics` and then modified by the layer
/// system. For tokens, this is assembled from the `TokenDefinition` when the
/// token is created.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Characteristics {
    pub name: SmallString,
    pub mana_cost: Option<ManaCost>,
    pub colors: ColorSet,
    pub types: TypeLine,
    pub subtypes: SubtypeSet,
    pub supertypes: SupertypeSet,
    pub power: Option<PtValue>,      // None for non-creatures
    pub toughness: Option<PtValue>,  // None for non-creatures
    pub loyalty: Option<i32>,        // Some for planeswalkers
    pub abilities_text: Vec<AbilityId>,
    /// Keyword abilities printed on the card (or granted by copy/text-change
    /// effects in higher layers). Keywords granted by Layer 6 continuous
    /// effects (`GrantKeywordTarget`) are folded in during
    /// [`crate::state::GameState::compute_characteristics`] — this base
    /// field holds only the characteristics' inherent set.
    pub keywords: Vec<crate::effects::KeywordAbility>,
}

impl Characteristics {
    pub fn is_permanent(&self) -> bool { self.types.is_permanent() }
    pub fn is_creature(&self)  -> bool { self.types.is_creature() }
    pub fn is_land(&self)      -> bool { self.types.is_land() }
    pub fn is_instant(&self)   -> bool { self.types.is_instant() }
    pub fn is_sorcery(&self)   -> bool { self.types.is_sorcery() }
    pub fn is_spell(&self)     -> bool { self.types.is_spell() }

    /// Printed mana value (CR 202.3). Zero for cards with no mana cost
    /// (tokens, most lands).
    pub fn mana_value(&self) -> u32 {
        self.mana_cost.as_ref().map_or(0, |c| c.mana_value())
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mana::ManaCost;
    use crate::types::{Color, StringInterner};

    /// Build a vanilla creature's `Characteristics`, interning "Test Bear"
    /// and its subtype into `interner`. Tests that need to assert on names
    /// should use this directly with their own interner.
    fn vanilla_creature(
        interner: &mut StringInterner,
        power: i32,
        toughness: i32,
    ) -> Characteristics {
        Characteristics {
            name: interner.intern("Test Bear"),
            mana_cost: Some(ManaCost::parse("{1}{G}").unwrap()),
            colors: ColorSet::green(),
            types: TypeLine::CREATURE.into(),
            subtypes: SubtypeSet::from_names(interner, ["Bear"]),
            supertypes: SupertypeSet::default(),
            power: Some(PtValue::Fixed(power)),
            toughness: Some(PtValue::Fixed(toughness)),
            loyalty: None,
            abilities_text: vec![],
            keywords: vec![],
        }
    }

    /// Convenience: build a creature object with a fresh disposable
    /// interner. The interner is discarded afterwards — fine for tests
    /// that don't assert on names. Tests that *do* want to look up names
    /// on the resulting object should call `vanilla_creature` directly
    /// with their own interner and pass it to `GameObject::new`.
    fn make_creature(id: ObjectId, zone: Zone, power: i32, toughness: i32) -> GameObject {
        let mut interner = StringInterner::new();
        GameObject::new(
            id, /*owner=*/ 0, zone, /*card_id=*/ 1,
            vanilla_creature(&mut interner, power, toughness),
        )
    }

    // --- ObjectArena basics --------------------------------------------------

    #[test]
    fn arena_starts_empty() {
        let a = ObjectArena::new();
        assert!(a.is_empty());
        assert_eq!(a.len(), 0);
    }

    #[test]
    fn arena_insert_and_get() {
        let mut a = ObjectArena::new();
        let obj = make_creature(1, Zone::Battlefield, 2, 2);
        let id = a.insert(obj);
        assert_eq!(id, 1);
        assert_eq!(a.len(), 1);
        assert!(a.contains(1));
        assert_eq!(a.get(1).unwrap().id, 1);
        assert!(a.get(999).is_none());
    }

    #[test]
    fn arena_remove_returns_object_and_decrements_len() {
        let mut a = ObjectArena::new();
        a.insert(make_creature(7, Zone::Battlefield, 2, 2));
        let removed = a.remove(7).expect("should exist");
        assert_eq!(removed.id, 7);
        assert!(!a.contains(7));
        assert_eq!(a.len(), 0);
        assert!(a.remove(7).is_none());
    }

    #[test]
    #[should_panic(expected = "duplicate ObjectId")]
    fn arena_duplicate_insert_panics() {
        let mut a = ObjectArena::new();
        a.insert(make_creature(1, Zone::Battlefield, 2, 2));
        a.insert(make_creature(1, Zone::Hand(0), 3, 3));
    }

    #[test]
    fn arena_get_mut_allows_mutation() {
        let mut a = ObjectArena::new();
        a.insert(make_creature(1, Zone::Battlefield, 2, 2));
        a.get_mut(1).unwrap().tap();
        assert!(a.get(1).unwrap().is_tapped());
    }

    #[test]
    fn arena_iter_returns_all() {
        let mut a = ObjectArena::new();
        a.insert(make_creature(1, Zone::Battlefield, 2, 2));
        a.insert(make_creature(2, Zone::Hand(0), 3, 3));
        a.insert(make_creature(3, Zone::Graveyard(0), 1, 1));
        let ids: crate::collections::HashSet<_> = a.iter().map(|o| o.id).collect();
        assert_eq!(ids, [1, 2, 3].iter().copied().collect());
    }

    #[test]
    fn arena_zone_filters() {
        let mut a = ObjectArena::new();
        a.insert(make_creature(1, Zone::Battlefield, 2, 2));
        a.insert(make_creature(2, Zone::Battlefield, 3, 3));
        a.insert(make_creature(3, Zone::Hand(0), 1, 1));
        a.insert(make_creature(4, Zone::Graveyard(0), 0, 0));
        a.insert(make_creature(5, Zone::Graveyard(1), 0, 0));

        assert_eq!(a.count_in_zone(Zone::Battlefield), 2);
        assert_eq!(a.count_in_zone(Zone::Hand(0)), 1);

        // "any graveyard"
        let in_any_gy: Vec<_> = a.objects_in_zone_kind(ZoneKind::Graveyard)
            .map(|o| o.id).collect();
        assert_eq!(in_any_gy.len(), 2);
    }

    #[test]
    fn arena_controlled_by_filter() {
        let mut a = ObjectArena::new();
        let mut c1 = make_creature(1, Zone::Battlefield, 2, 2);
        c1.controller = 0;
        a.insert(c1);
        let mut c2 = make_creature(2, Zone::Battlefield, 3, 3);
        c2.controller = 1;
        a.insert(c2);

        let p0: Vec<_> = a.objects_controlled_by(0).map(|o| o.id).collect();
        let p1: Vec<_> = a.objects_controlled_by(1).map(|o| o.id).collect();
        assert_eq!(p0, vec![1]);
        assert_eq!(p1, vec![2]);
    }

    #[test]
    fn arena_ids_in_zone_sorted_is_deterministic() {
        let mut a = ObjectArena::new();
        // Insert out of order.
        a.insert(make_creature(3, Zone::Battlefield, 2, 2));
        a.insert(make_creature(1, Zone::Battlefield, 2, 2));
        a.insert(make_creature(2, Zone::Battlefield, 2, 2));
        assert_eq!(a.ids_in_zone_sorted(Zone::Battlefield), vec![1, 2, 3]);
    }

    #[test]
    fn arena_clone_independence() {
        // Every cloned arena should be an independent mutable copy — this is
        // the core invariant that makes tree search work.
        let mut a = ObjectArena::new();
        a.insert(make_creature(1, Zone::Battlefield, 2, 2));
        let mut b = a.clone();
        b.get_mut(1).unwrap().tap();
        assert!(b.get(1).unwrap().is_tapped());
        assert!(!a.get(1).unwrap().is_tapped());
    }

    // --- GameObject: tap/untap ----------------------------------------------

    #[test]
    fn tap_untap_returns_whether_state_changed() {
        let mut obj = make_creature(1, Zone::Battlefield, 2, 2);
        assert!(!obj.is_tapped());
        assert!(obj.tap());        // state changed
        assert!(!obj.tap());       // idempotent
        assert!(obj.is_tapped());

        assert!(obj.untap());      // state changed
        assert!(!obj.untap());     // idempotent
        assert!(!obj.is_tapped());
    }

    // --- GameObject: counters -----------------------------------------------

    #[test]
    fn add_counters_accumulates() {
        let mut obj = make_creature(1, Zone::Battlefield, 2, 2);
        obj.add_counters(CounterKind::PlusOnePlusOne, 1);
        obj.add_counters(CounterKind::PlusOnePlusOne, 2);
        assert_eq!(obj.count_counters(CounterKind::PlusOnePlusOne), 3);
        assert!(obj.has_counter(CounterKind::PlusOnePlusOne));
    }

    #[test]
    fn add_counters_zero_is_noop() {
        let mut obj = make_creature(1, Zone::Battlefield, 2, 2);
        obj.add_counters(CounterKind::Loyalty, 0);
        assert!(!obj.has_counter(CounterKind::Loyalty));
    }

    #[test]
    fn remove_counters_returns_actual_removed() {
        let mut obj = make_creature(1, Zone::Battlefield, 2, 2);
        obj.add_counters(CounterKind::PlusOnePlusOne, 3);
        assert_eq!(obj.remove_counters(CounterKind::PlusOnePlusOne, 2), 2);
        assert_eq!(obj.count_counters(CounterKind::PlusOnePlusOne), 1);

        // Removing more than exist removes all, returns actual count.
        assert_eq!(obj.remove_counters(CounterKind::PlusOnePlusOne, 99), 1);
        assert_eq!(obj.count_counters(CounterKind::PlusOnePlusOne), 0);
        assert!(!obj.has_counter(CounterKind::PlusOnePlusOne));

        // Removing from empty returns 0.
        assert_eq!(obj.remove_counters(CounterKind::PlusOnePlusOne, 1), 0);
    }

    #[test]
    fn annihilate_pt_counters_pairs_them_up() {
        // 3 +1/+1 and 5 -1/-1 → annihilate 3 pairs, leaving 0 and 2.
        let mut obj = make_creature(1, Zone::Battlefield, 2, 2);
        obj.add_counters(CounterKind::PlusOnePlusOne, 3);
        obj.add_counters(CounterKind::MinusOneMinusOne, 5);
        let annihilated = obj.annihilate_pt_counters();
        assert_eq!(annihilated, 3);
        assert_eq!(obj.count_counters(CounterKind::PlusOnePlusOne), 0);
        assert_eq!(obj.count_counters(CounterKind::MinusOneMinusOne), 2);
    }

    #[test]
    fn annihilate_pt_counters_no_op_when_one_side_zero() {
        let mut obj = make_creature(1, Zone::Battlefield, 2, 2);
        obj.add_counters(CounterKind::PlusOnePlusOne, 2);
        assert_eq!(obj.annihilate_pt_counters(), 0);
        assert_eq!(obj.count_counters(CounterKind::PlusOnePlusOne), 2);
    }

    // --- GameObject: damage -------------------------------------------------

    #[test]
    fn mark_damage_accumulates_and_saturates() {
        let mut obj = make_creature(1, Zone::Battlefield, 2, 2);
        obj.mark_damage(2);
        obj.mark_damage(3);
        assert_eq!(obj.damage_marked, 5);
        // Saturation on overflow — shouldn't panic.
        obj.mark_damage(u32::MAX);
        assert_eq!(obj.damage_marked, u32::MAX);
    }

    #[test]
    fn clear_damage_resets() {
        let mut obj = make_creature(1, Zone::Battlefield, 2, 2);
        obj.mark_damage(5);
        obj.clear_damage();
        assert_eq!(obj.damage_marked, 0);
    }

    // --- GameObject: effective P/T -----------------------------------------

    #[test]
    fn raw_pt_with_counters_base_values() {
        let obj = make_creature(1, Zone::Battlefield, 3, 4);
        assert_eq!(obj.raw_power_with_counters(None),     Some(3));
        assert_eq!(obj.raw_toughness_with_counters(None), Some(4));
    }

    #[test]
    fn raw_pt_with_counters_with_plus_counters() {
        let mut obj = make_creature(1, Zone::Battlefield, 2, 2);
        obj.add_counters(CounterKind::PlusOnePlusOne, 3);
        assert_eq!(obj.raw_power_with_counters(None),     Some(5));
        assert_eq!(obj.raw_toughness_with_counters(None), Some(5));
    }

    #[test]
    fn raw_pt_with_counters_with_minus_counters() {
        let mut obj = make_creature(1, Zone::Battlefield, 2, 2);
        obj.add_counters(CounterKind::MinusOneMinusOne, 1);
        assert_eq!(obj.raw_power_with_counters(None),     Some(1));
        assert_eq!(obj.raw_toughness_with_counters(None), Some(1));
    }

    #[test]
    fn raw_pt_with_counters_with_mixed_counters() {
        let mut obj = make_creature(1, Zone::Battlefield, 2, 2);
        obj.add_counters(CounterKind::PlusOnePlusOne, 3);
        obj.add_counters(CounterKind::MinusOneMinusOne, 1);
        // 2 + 3 - 1 = 4. Counters aren't annihilated yet; SBA would do that.
        assert_eq!(obj.raw_power_with_counters(None),     Some(4));
        assert_eq!(obj.raw_toughness_with_counters(None), Some(4));
    }

    #[test]
    fn raw_pt_with_counters_none_for_non_creature() {
        // Sorcery has no power/toughness.
        let chars = Characteristics {
            types: TypeLine::SORCERY.into(),
            ..Default::default()
        };
        let obj = GameObject::new(1, 0, Zone::Hand(0), 1, chars);
        assert_eq!(obj.raw_power_with_counters(None), None);
        assert_eq!(obj.raw_toughness_with_counters(None), None);
    }

    #[test]
    fn raw_pt_with_counters_star_requires_star_value() {
        let chars = Characteristics {
            types: TypeLine::CREATURE.into(),
            power: Some(PtValue::Star),
            toughness: Some(PtValue::StarPlus(1)),
            ..Default::default()
        };
        let obj = GameObject::new(1, 0, Zone::Battlefield, 1, chars);
        assert_eq!(obj.raw_power_with_counters(None), None);
        assert_eq!(obj.raw_power_with_counters(Some(7)), Some(7));
        assert_eq!(obj.raw_toughness_with_counters(Some(7)), Some(8));
    }

    // --- GameObject: lethal damage ------------------------------------------

    #[test]
    fn raw_lethal_damage_below_toughness_is_not_lethal() {
        let mut obj = make_creature(1, Zone::Battlefield, 3, 3);
        obj.mark_damage(2);
        assert!(!obj.has_raw_lethal_damage());
    }

    #[test]
    fn raw_lethal_damage_equal_to_toughness_is_lethal() {
        let mut obj = make_creature(1, Zone::Battlefield, 3, 3);
        obj.mark_damage(3);
        assert!(obj.has_raw_lethal_damage());
    }

    #[test]
    fn raw_lethal_damage_accounts_for_plus_counters() {
        let mut obj = make_creature(1, Zone::Battlefield, 2, 2);
        obj.add_counters(CounterKind::PlusOnePlusOne, 2); // effective toughness 4
        obj.mark_damage(3);
        assert!(!obj.has_raw_lethal_damage());
        obj.mark_damage(1); // now 4
        assert!(obj.has_raw_lethal_damage());
    }

    #[test]
    fn raw_lethal_damage_false_when_toughness_zero_or_less() {
        // CR 704.5f puts these in graveyard by toughness check, not damage.
        // has_lethal_damage should return false (the other SBA handles it).
        let mut obj = make_creature(1, Zone::Battlefield, 1, 1);
        obj.add_counters(CounterKind::MinusOneMinusOne, 1); // toughness 0
        obj.mark_damage(1);
        assert!(!obj.has_raw_lethal_damage());
    }

    // --- GameObject: attachment / zone change ------------------------------

    #[test]
    fn attach_and_detach() {
        let mut aura = make_creature(1, Zone::Battlefield, 0, 0);
        assert!(!aura.is_attached());
        aura.attach_to(42);
        assert_eq!(aura.attached_to, Some(42));
        assert!(aura.is_attached());
        aura.detach();
        assert!(!aura.is_attached());
    }

    #[test]
    fn reset_on_zone_change_clears_transient_fields() {
        let mut obj = make_creature(1, Zone::Battlefield, 2, 2);
        obj.add_counters(CounterKind::PlusOnePlusOne, 3);
        obj.mark_damage(2);
        obj.tap();
        obj.attach_to(99);
        obj.reset_on_zone_change();

        assert_eq!(obj.count_counters(CounterKind::PlusOnePlusOne), 0);
        assert_eq!(obj.damage_marked, 0);
        assert!(!obj.is_tapped());
        assert!(!obj.is_attached());
    }

    // --- Characteristics helpers --------------------------------------------

    #[test]
    fn characteristics_type_queries() {
        let mut interner = StringInterner::new();
        let c = vanilla_creature(&mut interner, 2, 2);
        assert!(c.is_creature());
        assert!(c.is_permanent());
        assert!(!c.is_spell());
        assert!(!c.is_land());

        let inst = Characteristics {
            types: TypeLine::INSTANT.into(),
            ..Default::default()
        };
        assert!(inst.is_instant());
        assert!(inst.is_spell());
        assert!(!inst.is_permanent());
    }

    #[test]
    fn characteristics_mana_value() {
        let mut interner = StringInterner::new();
        let c = vanilla_creature(&mut interner, 2, 2);
        assert_eq!(c.mana_value(), 2); // {1}{G}

        let no_cost = Characteristics::default();
        assert_eq!(no_cost.mana_value(), 0);
    }

    #[test]
    fn characteristics_subtypes_populated() {
        let mut interner = StringInterner::new();
        let c = vanilla_creature(&mut interner, 2, 2);
        assert!(c.subtypes.contains_name(&interner, "Bear"));
        assert!(!c.subtypes.contains_name(&interner, "Horror"));
    }

    #[test]
    fn characteristics_colors_populated() {
        let mut interner = StringInterner::new();
        let c = vanilla_creature(&mut interner, 2, 2);
        assert!(c.colors.contains(Color::Green));
        assert!(!c.colors.contains(Color::Red));
    }

    // --- Unused import check: silence warning for now ----------------------
    #[allow(dead_code)]
    fn _uses_null_id() { let _ = NULL_OBJECT_ID; }
}
