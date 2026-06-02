//! Behavioral smoke-probe — does a card's resolver actually DO anything?
//!
//! Layer-2 verify certifies a card's bones; the L3 stub check is purely
//! TEXTUAL (it greps the source for `Vec::new()` / `// GAP`). Neither
//! catches the dangerous middle case: a resolver that returns a
//! NON-EMPTY effect list which, when executed, changes nothing
//! observable. That's exactly what hid the `Effect::ForEach`
//! no-substitution bug across ~294 catalog cards — they compiled, had
//! "some effect", and silently no-op'd.
//!
//! This probe closes that blind spot. It builds a generously-populated
//! game state, drives the card's spell resolver, executes the resulting
//! effects, and reports whether anything in a broad state fingerprint
//! moved. A card whose resolver yields effects but produces NO delta is
//! a silent-no-op suspect.
//!
//! Scope: spell abilities (instants/sorceries — the cleanest path, no
//! permanent/timing setup). Triggered/activated resolvers can reuse the
//! same `Snapshot` machinery later.

use crate::effects::Effect;
use crate::objects::{Characteristics, GameObject, ObjectId};
use crate::registry::CardRegistry;
use crate::stack::StackEntry;
use crate::state::GameState;
use crate::targets::{ObjectOrPlayer, TargetChoice, TargetFilter, TargetSelection};
use crate::types::{CardId, PlayerId, PtValue, TypeLine};
use crate::zones::Zone;

/// A broad fingerprint of everything a resolving spell could plausibly
/// change. Two snapshots differing anywhere = the resolver did
/// something observable.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Snapshot {
    life: Vec<i32>,
    energy: Vec<u32>,
    poison: Vec<u32>,
    /// Mana pool size per player — without this, every ritual ("add
    /// {C}{C}{R}") looks like a no-op.
    mana: Vec<usize>,
    /// (battlefield, hand, graveyard, library) object counts per player.
    zones: Vec<(usize, usize, usize, usize)>,
    /// Top-of-library card id per player — so "search and put on TOP"
    /// (Harbingers, Recruiters, tutor-to-top) shows a delta even though
    /// the library COUNT is unchanged.
    library_tops: Vec<Option<ObjectId>>,
    exile: usize,
    total_objects: usize,
    total_counters: u32,
    /// Sum of visible_face across objects — so a transform (flip
    /// front↔back, e.g. werewolves / "transform this Saga") shows a
    /// delta even though it changes no count.
    visible_faces: u32,
    /// Control fingerprint: sum of (id+1)*(controller+1) over objects, so
    /// a control change — including a symmetric EXCHANGE (Spawnbroker)
    /// that leaves per-player counts unchanged — shows a delta.
    control_fingerprint: u64,
    total_damage: u32,
    tapped: usize,
    pending_choice: bool,
    stack_len: usize,
    /// Installed continuous effects (pump, anthems, keyword/type/colour
    /// grants) — without this, every combat-trick / anthem spell looks
    /// like a no-op because P/T and granted abilities live in layers,
    /// not on the object's printed characteristics.
    continuous_effects: usize,
    /// "Install something for later" effects: replacement effects,
    /// delayed triggers, and granted triggered abilities (Feign Death,
    /// Undying Malice, Showstopper) — none touch the board now, so
    /// without these they read as no-ops.
    replacements: usize,
    delayed_triggers: usize,
    granted_triggers: usize,
    dungeons: Vec<bool>,
}

impl Snapshot {
    pub fn capture(state: &GameState) -> Self {
        let n = state.num_players();
        let mut life = Vec::with_capacity(n as usize);
        let mut energy = Vec::with_capacity(n as usize);
        let mut poison = Vec::with_capacity(n as usize);
        let mut mana = Vec::with_capacity(n as usize);
        let mut zones = Vec::with_capacity(n as usize);
        let mut library_tops = Vec::with_capacity(n as usize);
        let mut dungeons = Vec::with_capacity(n as usize);
        for p in 0..n {
            life.push(state.player(p).life);
            energy.push(state.player(p).energy);
            poison.push(state.player(p).poison_counters);
            mana.push(state.player(p).mana_pool.total());
            dungeons.push(state.player(p).dungeon.is_some());
            library_tops.push(state.top_of_library(p));
            zones.push((
                state.objects.objects_in_zone(Zone::Battlefield)
                    .filter(|o| o.controller == p).count(),
                state.objects.count_in_zone(Zone::Hand(p)),
                state.objects.count_in_zone(Zone::Graveyard(p)),
                state.objects.count_in_zone(Zone::Library(p)),
            ));
        }
        let total_counters: u32 = state.objects.iter()
            .map(|o| {
                let kinds: Vec<_> = o.counters.keys().copied().collect();
                kinds.iter().map(|k| o.count_counters(*k)).sum::<u32>()
            })
            .sum();
        let total_damage: u32 = state.objects.iter().map(|o| o.damage_marked).sum();
        let tapped = state.objects.iter().filter(|o| o.is_tapped()).count();
        Self {
            life, energy, poison, mana, zones, library_tops,
            exile: state.objects.count_in_zone(Zone::Exile),
            total_objects: state.objects.iter().count(),
            total_counters,
            visible_faces: state.objects.iter().map(|o| o.visible_face as u32).sum(),
            control_fingerprint: state.objects.iter()
                .map(|o| (o.id as u64 + 1) * (o.controller as u64 + 1)).sum(),
            total_damage,
            tapped,
            pending_choice: state.pending_choice.is_some(),
            stack_len: state.stack_size(),
            continuous_effects: state.continuous_effects.len(),
            replacements: state.replacement_effects.len(),
            delayed_triggers: state.delayed_triggers.len(),
            granted_triggers: state.objects.iter()
                .map(|o| o.granted_triggered_abilities.len()).sum(),
            dungeons,
        }
    }
}

/// Outcome of probing one card.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ProbeResult {
    /// The resolver returned at least one `Effect`.
    pub had_effects: bool,
    /// Executing those effects changed the state fingerprint.
    pub observable_delta: bool,
}

impl ProbeResult {
    /// A silent-no-op suspect: produced effects, but nothing moved.
    pub fn is_silent_noop(&self) -> bool {
        self.had_effects && !self.observable_delta
    }
}

/// Probe a spell card. Returns `None` if the card has no spell ability
/// (not in scope for this probe).
pub fn probe_spell(reg: &CardRegistry, card_id: CardId) -> Option<ProbeResult> {
    let def = reg.get(card_id)?;
    let spell = def.spell_ability.as_ref()?;

    let mut state = populated_state(reg);
    // The spell's source object, sitting on the stack.
    let src = state.allocate_object_id();
    state.objects.insert(GameObject::new(
        src, 0, Zone::Stack, card_id, Characteristics::default()));
    state.currently_resolving = Some(src);

    // A second spell on the stack so "counter target spell" has a
    // referent (otherwise every counterspell reads as a no-op).
    let stack_spell = add_dummy_stack_spell(&mut state);

    // Build a target selection that satisfies each requirement with a
    // legal-shaped referent from the populated board.
    let dummy = first_battlefield_creature(&state, 0);
    let targets = selection_for(&state, &spell.target_requirements, dummy, stack_spell);
    let entry = StackEntry::new_spell(
        src, 0, card_id, Characteristics::default(),
        // x_value Some(3): X-cost spells compute 0 and no-op without it.
        targets, Vec::new(), Some(3));

    let before = Snapshot::capture(&state);
    let effects: Vec<Effect> = (spell.effect)(&state, &entry, reg);
    let had_effects = !effects.is_empty();
    // Flatten top-level Sequences exactly as the engine's resolution loop
    // does, so a multi-choice Sequence (e.g. Pox) parks step-by-step here
    // instead of tripping the single-pending-choice invariant.
    for eff in flatten_sequences(effects) {
        eff.execute(&mut state);
        // Real resolution PARKS when an effect posts a choice and
        // resumes after the answer; executing further effects
        // straight-line would trip the invariant. A posted choice is
        // itself an observable delta, so stop here.
        if state.pending_choice.is_some() { break; }
    }
    let after = Snapshot::capture(&state);
    Some(ProbeResult { had_effects, observable_delta: before != after })
}

/// Probe each of a card's TRIGGERED abilities: fire it in a populated
/// state (source on the battlefield, a generic ETB event) and report
/// whether its resolver moved anything. Returns one [`ProbeResult`] per
/// triggered ability (empty if the card has none). Event-reading
/// triggers that need a different event than the synthesized ETB read
/// nothing and may show as no-ops — harness-limited, allowlisted, same
/// posture as spell target shapes.
pub fn probe_triggered(reg: &CardRegistry, card_id: CardId) -> Vec<ProbeResult> {
    let Some(def) = reg.get(card_id) else { return Vec::new(); };
    if def.triggered_abilities.is_empty() { return Vec::new(); }
    let mut out = Vec::with_capacity(def.triggered_abilities.len());
    for ability in &def.triggered_abilities {
        let mut state = populated_state(reg);
        // The triggering permanent itself, on the battlefield.
        let src = state.allocate_object_id();
        let chars = Characteristics {
            types: TypeLine::CREATURE.into(),
            power: Some(PtValue::Fixed(2)),
            toughness: Some(PtValue::Fixed(2)),
            ..Default::default()
        };
        state.objects.insert(GameObject::new(src, 0, Zone::Battlefield, card_id, chars));
        state.currently_resolving = Some(src);
        // Saga fidelity: actually PLACE the lore counters on the source so
        // chapter dispatch that reads the COUNT (not just the event) fires.
        if let crate::triggers::TriggerCondition::CounterAdded { kind, chapter, .. } =
            &ability.trigger_condition
        {
            state.place_counters(
                crate::replacement::CounterTarget::Object(src),
                kind.unwrap_or(crate::types::CounterKind::Lore),
                chapter.unwrap_or(1));
        }
        let stack_spell = add_dummy_stack_spell(&mut state);
        let dummy = first_battlefield_creature(&state, 0);
        let targets = selection_for(&state, &ability.target_requirements, dummy, stack_spell);
        // Synthesize the event the trigger's CONDITION actually matches
        // (Saga lore counter, transform, combat, death, cast, …) so it
        // fires faithfully; falls back to a generic ETB for execution
        // (panic-catching) when the condition can't be synthesized.
        let synth = synth_event(&ability.trigger_condition, src, 0, stack_spell, dummy);
        let event = synth.clone().unwrap_or(
            crate::events::GameEvent::EntersBattlefield {
                object_id: src, from_zone: Zone::Stack, was_cast: true,
            });
        let pt = crate::triggers::PendingTrigger {
            source: src, trigger_id: ability.id, controller: 0,
            trigger_event: event, targets,
        };
        let before = Snapshot::capture(&state);
        let effects = (ability.effect)(&state, &pt, reg);
        let had_effects = !effects.is_empty();
        // Always EXECUTE — this is what surfaces panics (e.g. Yukora's
        // ForEach-of-Sacrifice), regardless of trigger condition.
        for eff in flatten_sequences(effects) {
            eff.execute(&mut state);
            if state.pending_choice.is_some() { break; }
        }
        let after = Snapshot::capture(&state);
        // Render a SILENT-NO-OP verdict only when we synthesized the
        // CONDITION'S real event (so the trigger fired faithfully).
        // Custom predicates / un-synthesizable conditions are executed
        // (panic-catching) but skip the verdict.
        if synth.is_some() {
            out.push(ProbeResult { had_effects, observable_delta: before != after });
        }
    }
    out
}

/// Synthesize a [`crate::events::GameEvent`] that fires `cond` on
/// `source`, so the trigger resolves faithfully in the probe. `None`
/// when the condition can't be reproduced (Custom predicates, or filters
/// no harness object satisfies) — caller skips the no-op verdict.
fn synth_event(
    cond: &crate::triggers::TriggerCondition,
    source: ObjectId,
    controller: PlayerId,
    stack_spell: ObjectId,
    dummy: Option<ObjectId>,
) -> Option<crate::events::GameEvent> {
    use crate::triggers::{TriggerCondition as TC, TriggerSelf};
    use crate::events::{GameEvent as GE, DamageTarget};
    use crate::targets::ControllerConstraint as CC;
    // ControllerConstraint → a concrete player.
    let who = |c: &CC| match c { CC::Opponent => 1 - controller, _ => controller };
    let other = dummy.unwrap_or(source);
    Some(match cond {
        TC::SelfEntersBattlefield =>
            GE::EntersBattlefield { object_id: source, from_zone: Zone::Stack, was_cast: true },
        TC::SelfDies => GE::Dies { object_id: source },
        TC::SelfAttacks => GE::CreatureAttacks {
            attacker: source, defending: crate::combat::DefendingEntity::Player(1 - controller) },
        TC::SelfAttacksUnblocked => GE::CreatureNotBlocked { attacker: source },
        TC::SelfBecomesBlocked => GE::CreatureBlocked { attacker: source, blockers: vec![other] },
        TC::SelfBlocks => GE::CreatureBlocks { blocker: source, attacker: other },
        TC::SelfBlocksOrBecomesBlocked => GE::CreatureBlocks { blocker: source, attacker: other },
        TC::SelfBecomesTapped => GE::Tapped { object_id: source },
        TC::SelfSpecializes => GE::Specialized { object_id: source },
        TC::SelfBecomesTarget { caster } => GE::BecomesTarget {
            target: source, source: stack_spell, controller: who(caster) },
        TC::SelfIsDealtDamage { combat_only } => GE::DamageDealt {
            source: other, target: DamageTarget::Object(source), amount: 1, is_combat: *combat_only },
        TC::StepBegins { step, .. } => GE::StepBegins { step: *step },
        TC::PhaseBegins { phase, .. } => GE::PhaseBegins { phase: *phase },
        TC::LifeGained { player } => GE::LifeGained { player: who(player), amount: 1 },
        TC::CardDrawn { player } => GE::DrawCard { player: who(player), object_id: other },
        TC::CardDiscarded { player } => GE::Discarded { player: who(player), object_id: other },
        TC::CreatureAttacks { .. } => GE::CreatureAttacks {
            attacker: other, defending: crate::combat::DefendingEntity::Player(1 - controller) },
        TC::Sacrificed { .. } => GE::Sacrifice { player: controller, object_id: other },
        TC::SpellCast { caster, .. } => GE::SpellCast {
            object_id: stack_spell, card_id: 0, controller: who(caster),
            targets: crate::targets::TargetSelection::new() },
        TC::DamageDealt { combat_only, .. } => GE::DamageDealt {
            source, target: DamageTarget::Object(other), amount: 1, is_combat: *combat_only },
        TC::ZoneChange { to, from, .. } => GE::ZoneChange {
            object_id: source, from: from.unwrap_or(Zone::Battlefield), to: *to,
            new_id: source, cause: crate::events::MoveCause::SpellResolution },
        // Saga chapter / counter triggers: stamp the source with the
        // condition's counter kind + chapter number.
        TC::CounterAdded { on, kind, chapter } => {
            let object_id = match on { TriggerSelf::Source => source, _ => source };
            GE::CounterAdded {
                object_id,
                kind: kind.unwrap_or(crate::types::CounterKind::PlusOnePlusOne),
                count: chapter.unwrap_or(1),
            }
        }
        // Unreproducible — skip the verdict (still executed via ETB fallback).
        TC::Custom(_) => return None,
    })
}

/// Probe each of a card's ACTIVATED abilities: invoke its effect in a
/// populated state (the cost is treated as already paid — the
/// silent-no-op concern is the EFFECT) and report whether it moved
/// anything. One [`ProbeResult`] per ability. Cost/mode-dependent
/// effects (e.g. "sacrifice a creature: deal damage equal to its power")
/// may read as no-ops since no cost was actually paid — harness-limited,
/// allowlisted, same posture as the spell/trigger probes.
pub fn probe_activated(reg: &CardRegistry, card_id: CardId) -> Vec<ProbeResult> {
    let Some(def) = reg.get(card_id) else { return Vec::new(); };
    if def.activated_abilities.is_empty() { return Vec::new(); }
    let mut out = Vec::with_capacity(def.activated_abilities.len());
    for (i, ability) in def.activated_abilities.iter().enumerate() {
        let mut state = populated_state(reg);
        let src = state.allocate_object_id();
        let chars = Characteristics {
            types: TypeLine::CREATURE.into(),
            power: Some(PtValue::Fixed(2)),
            toughness: Some(PtValue::Fixed(2)),
            ..Default::default()
        };
        state.objects.insert(GameObject::new(src, 0, Zone::Battlefield, card_id, chars));
        state.currently_resolving = Some(src);
        let stack_spell = add_dummy_stack_spell(&mut state);
        let dummy = first_battlefield_creature(&state, 0);
        let targets = selection_for(&state, &ability.target_requirements, dummy, stack_spell);
        let ctx = crate::registry::ActivationContext {
            source: src,
            controller: 0,
            ability_index: i,
            targets,
            x_value: Some(3),
            card_id,
        };
        let before = Snapshot::capture(&state);
        let effects = (ability.effect)(&state, &ctx, reg);
        let had_effects = !effects.is_empty();
        for eff in flatten_sequences(effects) {
            eff.execute(&mut state);
            if state.pending_choice.is_some() { break; }
        }
        let after = Snapshot::capture(&state);
        out.push(ProbeResult { had_effects, observable_delta: before != after });
    }
    out
}

/// A 2-player state stocked so most effects have something to act on:
/// libraries with cards (draw/mill), creatures on each battlefield
/// (targets + board-wide), cards in hand and graveyard.
fn populated_state(reg: &CardRegistry) -> GameState {
    use crate::types::ColorSet;
    let mut state = GameState::new(2, 0);
    // Distinct colours (with a colored pip each) so colour-matters
    // destroys (Cleanse/Perish) and devotion (Aspect of Hydra) find
    // referents.
    let colors = [
        (ColorSet::white(), "{W}"), (ColorSet::blue(), "{U}"),
        (ColorSet::black(), "{B}"), (ColorSet::red(), "{R}"),
        (ColorSet::green(), "{G}"),
    ];
    for p in 0..2 {
        for (cs, cost) in colors {
            make_creature(&mut state, p, cs, cost);
        }
        // An EXTRA already-tapped creature so untap-all effects show a
        // delta — but NOT the target creature (selection_for targets the
        // first creature, and a tap-spell on an already-tapped target
        // would falsely read as a no-op; tap-target is the common case).
        let tapped = make_creature(&mut state, p, ColorSet::colorless(), "{C}");
        state.objects.get_mut(tapped).map(|o| o.tap());
        // Seed a +1/+1 counter so Proliferate ("add another counter to
        // each permanent/player that has one") has something to act on.
        state.place_counters(
            crate::replacement::CounterTarget::Object(tapped),
            crate::types::CounterKind::PlusOnePlusOne, 1);
        // One of each other permanent type — satisfies "if you control
        // an artifact / enchantment / land" conditions and type-filtered
        // targets — plus the five basic land types (interned via the
        // registry) so "destroy all Mountains" / "X = Forests you
        // control" / land-subtype counts find referents.
        make_permanent(&mut state, p, TypeLine::ARTIFACT.into());
        make_permanent(&mut state, p, TypeLine::ENCHANTMENT.into());
        for basic in ["Plains", "Island", "Swamp", "Mountain", "Forest"] {
            make_basic_land(&mut state, reg, p, basic);
        }
        // Stock library + graveyard with EVERY card type so type-tutors
        // ("search for an enchantment/artifact/planeswalker card") and
        // reanimation find matches — type-less dummies no-op them.
        let kinds = [TypeLine::CREATURE, TypeLine::LAND, TypeLine::INSTANT,
            TypeLine::SORCERY, TypeLine::ENCHANTMENT, TypeLine::ARTIFACT,
            TypeLine::PLANESWALKER];
        let mut lib = Vec::new();
        for k in kinds { lib.push(make_typed_card(&mut state, p, Zone::Library(p), k.into())); }
        state.player_mut(p).library_top_to_bottom = lib;
        for _ in 0..2 { make_card(&mut state, p, Zone::Hand(p)); }
        make_typed_card(&mut state, p, Zone::Graveyard(p), TypeLine::CREATURE.into());
        make_typed_card(&mut state, p, Zone::Graveyard(p), TypeLine::LAND.into());
    }
    state
}

fn make_creature(state: &mut GameState, controller: PlayerId,
    colors: crate::types::ColorSet, cost: &str) -> ObjectId {
    let id = state.allocate_object_id();
    let chars = Characteristics {
        types: TypeLine::CREATURE.into(),
        colors,
        mana_cost: crate::mana::ManaCost::parse(cost).ok(),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };
    state.objects.insert(GameObject::new(id, controller, Zone::Battlefield, 0, chars));
    id
}

/// A basic land with its interned subtype (Plains/Island/…) so
/// land-subtype counts and "destroy all <type>" find referents. If the
/// subtype was never interned this game, falls back to a typeless land.
fn make_basic_land(state: &mut GameState, reg: &CardRegistry, controller: PlayerId, basic: &str) -> ObjectId {
    let id = state.allocate_object_id();
    let mut subtypes = crate::types::SubtypeSet::default();
    if let Some(s) = reg.interner().lookup(basic) { subtypes.0.insert(s); }
    let chars = Characteristics {
        types: TypeLine::LAND.into(), subtypes, ..Default::default()
    };
    let mut obj = GameObject::new(id, controller, Zone::Battlefield, 0, chars);
    // Enter tapped so "untap target land / Forest" (mana dorks) shows a
    // delta; creatures stay untapped for tap-target effects.
    obj.tap();
    state.objects.insert(obj);
    id
}

fn make_permanent(state: &mut GameState, controller: PlayerId, types: TypeLine) -> ObjectId {
    let id = state.allocate_object_id();
    let chars = Characteristics { types, ..Default::default() };
    state.objects.insert(GameObject::new(id, controller, Zone::Battlefield, 0, chars));
    id
}

/// Push a dummy spell onto the stack (under the opponent) so
/// "counter target spell" effects have a legal referent.
fn add_dummy_stack_spell(state: &mut GameState) -> ObjectId {
    let id = state.allocate_object_id();
    let chars = Characteristics { types: TypeLine::INSTANT.into(), ..Default::default() };
    state.objects.insert(GameObject::new(id, 1, Zone::Stack, 0, chars.clone()));
    let entry = StackEntry::new_spell(
        id, 1, 0, chars, TargetSelection::new(), Vec::new(), None);
    state.stack.push(entry);
    id
}

fn make_card(state: &mut GameState, owner: PlayerId, zone: Zone) -> ObjectId {
    let id = state.allocate_object_id();
    state.objects.insert(GameObject::new(id, owner, zone, 0, Characteristics::default()));
    id
}

fn make_typed_card(state: &mut GameState, owner: PlayerId, zone: Zone, types: TypeLine) -> ObjectId {
    let id = state.allocate_object_id();
    let chars = Characteristics { types, ..Default::default() };
    state.objects.insert(GameObject::new(id, owner, zone, 0, chars));
    id
}

/// Mirror of the engine's resolution-time flattening (see
/// `engine::flatten_sequences`): unwrap top-level `Sequence`s so a
/// multi-choice sequence parks step-by-step rather than running
/// straight-line into the single-pending-choice invariant.
fn flatten_sequences(effects: Vec<Effect>) -> Vec<Effect> {
    let mut out = Vec::with_capacity(effects.len());
    for e in effects {
        match e {
            Effect::Sequence(steps) => out.extend(flatten_sequences(steps)),
            Effect::ForEach { targets, effect } => {
                for id in targets {
                    out.extend(flatten_sequences(vec![effect.retargeted(id)]));
                }
            }
            other => out.push(other),
        }
    }
    out
}

fn first_battlefield_creature(state: &GameState, controller: PlayerId) -> Option<ObjectId> {
    state.objects.objects_in_zone(Zone::Battlefield)
        .find(|o| o.controller == controller && o.is_creature())
        .map(|o| o.id)
}

/// One `TargetChoice` per requirement, shaped to the filter: players for
/// player-ish filters, a battlefield creature otherwise. The opponent
/// (player 1) is used for player targets so "target opponent" clauses
/// see a legal referent.
fn selection_for(
    state: &GameState,
    reqs: &[crate::targets::TargetRequirement],
    dummy: Option<ObjectId>,
    stack_spell: ObjectId,
) -> TargetSelection {
    let mut sel = TargetSelection::new();
    for req in reqs {
        // FILTER-AWARE: prefer a candidate the requirement actually
        // accepts (type/controller). Without this, "untap target Forest"
        // / "destroy target artifact" got a creature and no-op'd.
        if let Some(choice) = legal_target(state, req, stack_spell) {
            sel.targets.push(choice);
            continue;
        }
        // Fallback heuristic when nothing in the harness matches.
        let choice = match (&req.filter, dummy) {
            (TargetFilter::Player, _) => TargetChoice::Player(1),
            (TargetFilter::Spell(_), _) => TargetChoice::Object(stack_spell),
            (TargetFilter::CreatureOrPlayer, Some(id))
            | (TargetFilter::AnyTarget, Some(id)) =>
                TargetChoice::ObjectOrPlayer(ObjectOrPlayer::Object(id)),
            (_, Some(id)) => TargetChoice::Object(id),
            (_, None) => TargetChoice::Player(1),
        };
        sel.targets.push(choice);
    }
    sel
}

/// First candidate (object in any non-library zone, or a player, or the
/// stack spell) that `req` legally accepts. Tapped permanents are tried
/// FIRST so "untap target …" effects show a delta; players last.
fn legal_target(
    state: &GameState,
    req: &crate::targets::TargetRequirement,
    stack_spell: ObjectId,
) -> Option<TargetChoice> {
    let mut ids: Vec<ObjectId> = state.objects.iter()
        .filter(|o| o.zone != Zone::Library(o.owner))
        .map(|o| o.id).collect();
    ids.push(stack_spell);
    // Deterministic id order. Creatures are seeded UNTAPPED (so
    // "tap target creature" — the common attack/ETB trigger — shows a
    // delta) and basic lands TAPPED (so "untap target land" mana-dorks
    // show a delta); the filter routes each to the right type, so we
    // don't bias by tap state (tapped-first broke tap-target).
    ids.sort();
    for id in ids {
        for choice in [
            TargetChoice::Object(id),
            TargetChoice::ObjectOrPlayer(ObjectOrPlayer::Object(id)),
        ] {
            if req.matches_choice(&choice, state, 0) { return Some(choice); }
        }
    }
    // Opponent (player 1) FIRST: "target player loses N life" punishers
    // self-target to a net-zero delta otherwise (Blood Artist drains
    // player 0 by 1 and gains 1 → no observable change).
    for p in [1, 0] {
        for choice in [
            TargetChoice::Player(p),
            TargetChoice::ObjectOrPlayer(ObjectOrPlayer::Player(p)),
        ] {
            if req.matches_choice(&choice, state, 0) { return Some(choice); }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::registry::{CardDefinition, SpellAbilityDef};

    fn draws_a_card(_s: &GameState, e: &StackEntry, _r: &CardRegistry) -> Vec<Effect> {
        vec![Effect::DrawCards { player: e.controller, count: 1 }]
    }
    fn noop_nonempty(_s: &GameState, _e: &StackEntry, _r: &CardRegistry) -> Vec<Effect> {
        // Non-empty, but targets nothing real — the ForEach-class bug shape.
        vec![Effect::DestroyPermanent { target: crate::objects::NULL_OBJECT_ID }]
    }
    fn honest_gap(_s: &GameState, _e: &StackEntry, _r: &CardRegistry) -> Vec<Effect> {
        Vec::new()
    }

    fn register(reg: &mut CardRegistry, name: &str, effect: crate::registry::SpellEffectFn) -> CardId {
        let n = reg.interner_mut().intern(name);
        let chars = Characteristics { name: n, types: TypeLine::INSTANT.into(), ..Default::default() };
        reg.register(CardDefinition::new(n, chars).with_spell_ability(SpellAbilityDef {
            text: name.into(),
            target_requirements: Vec::new(),
            modal: None,
            effect,
        }))
    }

    #[test]
    fn live_spell_shows_a_delta() {
        let mut reg = CardRegistry::new();
        let id = register(&mut reg, "Live", draws_a_card);
        let r = probe_spell(&reg, id).unwrap();
        assert!(r.had_effects && r.observable_delta, "drawing a card is observable");
        assert!(!r.is_silent_noop());
    }

    #[test]
    fn silent_noop_is_flagged() {
        // The whole point: non-empty effects, zero delta → caught.
        let mut reg = CardRegistry::new();
        let id = register(&mut reg, "Silent", noop_nonempty);
        let r = probe_spell(&reg, id).unwrap();
        assert!(r.had_effects, "returned an effect");
        assert!(!r.observable_delta, "but destroying NULL changes nothing");
        assert!(r.is_silent_noop(), "flagged as a silent no-op");
    }

    #[test]
    fn triggered_ability_probe_flags_silent_noop_but_not_a_live_one() {
        use crate::triggers::{TriggeredAbilityDef, TriggerCondition, TriggerFrequency};
        use crate::objects::NULL_OBJECT_ID;
        fn draw(_s: &GameState, t: &crate::triggers::PendingTrigger,
            _r: &CardRegistry) -> Vec<Effect> {
            vec![Effect::DrawCards { player: t.controller, count: 1 }]
        }
        fn noop(_s: &GameState, _t: &crate::triggers::PendingTrigger,
            _r: &CardRegistry) -> Vec<Effect> {
            vec![Effect::DestroyPermanent { target: NULL_OBJECT_ID }]
        }
        let mk = |id, effect| TriggeredAbilityDef {
            id, trigger_condition: TriggerCondition::SelfEntersBattlefield,
            intervening_if: None, effect,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        };
        let mut reg = CardRegistry::new();
        let n = reg.interner_mut().intern("TrigTest");
        let chars = Characteristics { name: n, types: TypeLine::CREATURE.into(),
            power: Some(PtValue::Fixed(1)), toughness: Some(PtValue::Fixed(1)),
            ..Default::default() };
        let id = reg.register(CardDefinition::new(n, chars)
            .with_triggered_ability(mk(1, draw))
            .with_triggered_ability(mk(2, noop)));
        let results = probe_triggered(&reg, id);
        assert_eq!(results.len(), 2, "one result per triggered ability");
        assert!(!results[0].is_silent_noop(), "draw-on-ETB is observable");
        assert!(results[1].is_silent_noop(), "destroy-NULL trigger is a silent no-op");
    }

    #[test]
    fn activated_ability_probe_flags_silent_noop_but_not_a_live_one() {
        use crate::registry::{ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone};
        use crate::objects::NULL_OBJECT_ID;
        fn draw(_s: &GameState, c: &ActivationContext, _r: &CardRegistry) -> Vec<Effect> {
            vec![Effect::DrawCards { player: c.controller, count: 1 }]
        }
        fn noop(_s: &GameState, _c: &ActivationContext, _r: &CardRegistry) -> Vec<Effect> {
            vec![Effect::DestroyPermanent { target: NULL_OBJECT_ID }]
        }
        let mk = |effect| ActivatedAbilityDef {
            text: String::new(), cost: ActivationCost::default(),
            target_requirements: Vec::new(), is_mana_ability: false,
            is_loyalty_ability: false, activation_zone: ActivationZone::Battlefield,
            is_instant_speed: true, face_gate: None, effect,
        };
        let mut reg = CardRegistry::new();
        let n = reg.interner_mut().intern("ActTest");
        let chars = Characteristics { name: n, types: TypeLine::CREATURE.into(),
            power: Some(PtValue::Fixed(1)), toughness: Some(PtValue::Fixed(1)),
            ..Default::default() };
        let id = reg.register(CardDefinition::new(n, chars)
            .with_activated_ability(mk(draw))
            .with_activated_ability(mk(noop)));
        let results = probe_activated(&reg, id);
        assert_eq!(results.len(), 2);
        assert!(!results[0].is_silent_noop(), "draw is observable");
        assert!(results[1].is_silent_noop(), "destroy-NULL is a silent no-op");
    }

    #[test]
    fn honest_gap_is_not_a_noop_suspect() {
        // Empty effects = honest GAP (L3's job), NOT a silent no-op.
        let mut reg = CardRegistry::new();
        let id = register(&mut reg, "Gap", honest_gap);
        let r = probe_spell(&reg, id).unwrap();
        assert!(!r.had_effects);
        assert!(!r.is_silent_noop(), "an honest GAP is not a silent no-op");
    }
}
