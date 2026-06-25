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
    /// Location-weighted counter sum: Σ (id+1)·(counters on that object).
    /// Without this, MOVING a counter from one object to another
    /// (Steel Dromedary, Spike creatures' "move a +1/+1 counter") leaves
    /// `total_counters` unchanged and reads as a silent no-op. Strictly
    /// finer than the total — detects relocation and per-object shifts.
    counter_fingerprint: u64,
    /// Sum of visible_face across objects — so a transform (flip
    /// front↔back, e.g. werewolves / "transform this Saga") shows a
    /// delta even though it changes no count.
    visible_faces: u32,
    /// Control fingerprint: sum of (id+1)*(controller+1) over objects, so
    /// a control change — including a symmetric EXCHANGE (Spawnbroker)
    /// that leaves per-player counts unchanged — shows a delta.
    control_fingerprint: u64,
    /// Attachment fingerprint: sum over attached objects of
    /// (id+1)*(attached_to+2) — so Equip / Attach (which move
    /// `attached_to` without changing any zone count) show a delta.
    /// Without this every Equipment's equip activation read as a
    /// silent no-op (the class that kept Bonesplitter allowlisted).
    attachment_fingerprint: u64,
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
        let per_obj_counters = |o: &GameObject| -> u32 {
            let kinds: Vec<_> = o.counters.keys().copied().collect();
            kinds.iter().map(|k| o.count_counters(*k)).sum::<u32>()
        };
        let total_counters: u32 = state.objects.iter().map(per_obj_counters).sum();
        let counter_fingerprint: u64 = state.objects.iter()
            .map(|o| (o.id as u64 + 1) * per_obj_counters(o) as u64)
            .sum();
        let total_damage: u32 = state.objects.iter().map(|o| o.damage_marked).sum();
        let tapped = state.objects.iter().filter(|o| o.is_tapped()).count();
        Self {
            life, energy, poison, mana, zones, library_tops,
            exile: state.objects.count_in_zone(Zone::Exile),
            total_objects: state.objects.iter().count(),
            total_counters,
            counter_fingerprint,
            visible_faces: state.objects.iter().map(|o| o.visible_face as u32).sum(),
            control_fingerprint: state.objects.iter()
                .map(|o| (o.id as u64 + 1) * (o.controller as u64 + 1)).sum(),
            attachment_fingerprint: state.objects.iter()
                .filter_map(|o| o.attached_to.map(|t|
                    (o.id as u64 + 1) * (t as u64 + 2)))
                .sum(),
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
        // Place the source where the trigger fires FROM: the graveyard
        // for death triggers ("when this dies, exile/return it" needs the
        // dead card there) and for graveyard-zone abilities (Lingering
        // Phantom: "when you cast …, return THIS from your graveyard").
        let from_graveyard = matches!(ability.trigger_condition,
            crate::triggers::TriggerCondition::SelfDies)
            || (ability.trigger_zones.iter().any(|z| matches!(z, Zone::Graveyard(_)))
                && !ability.trigger_zones.iter().any(|z| matches!(z, Zone::Battlefield)));
        let src_zone = if from_graveyard { Zone::Graveyard(0) } else { Zone::Battlefield };
        let mut src_obj = GameObject::new(src, 0, src_zone, card_id, chars);
        // Seed the transform back face (as the battlefield-entry path
        // does) so "transform this" (werewolf upkeep triggers) actually
        // flips visible_face — without it Transform hits the no-back
        // fallback that only toggles a status flag, reading as a no-op.
        if let Some(back) = def.alternate_face.as_ref().and_then(|a| a.as_transform()) {
            src_obj.back_face_characteristics = Some(back.characteristics.clone());
        }
        state.objects.insert(src_obj);
        state.currently_resolving = Some(src);
        // Removable counters on the source, so "remove a counter from
        // this" triggers (Bristlebane's counter-removal, Spike creatures)
        // delta instead of no-op'ing on a counterless source.
        seed_source_counters(&mut state, src);
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
            effect_override: None,
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
        TC::SelfLeavesBattlefield => GE::LeavesBattlefield {
            object_id: source, destination: Zone::Graveyard(controller) },
        TC::SelfAttacks => GE::CreatureAttacks {
            attacker: source, defending: crate::combat::DefendingEntity::Player(1 - controller) },
        TC::SelfAttacksUnblocked => GE::CreatureNotBlocked { attacker: source },
        TC::SelfAttacksAlone => GE::AttacksDeclared {
            attackers: vec![crate::combat::AttackerDeclaration {
                attacker: source,
                defending: crate::combat::DefendingEntity::Player(1 - controller),
            }],
        },
        // Filtered alone: the omni-tribal seed is the sole attacker,
        // same posture as the other filtered forms.
        TC::AttacksAlone { .. } => GE::AttacksDeclared {
            attackers: vec![crate::combat::AttackerDeclaration {
                attacker: other,
                defending: crate::combat::DefendingEntity::Player(1 - controller),
            }],
        },
        TC::SelfBecomesBlocked => GE::CreatureBlocked { attacker: source, blockers: vec![other] },
        TC::SelfBlocks => GE::CreatureBlocks { blocker: source, attacker: other },
        TC::SelfBlocksOrBecomesBlocked => GE::CreatureBlocks { blocker: source, attacker: other },
        // Filtered forms: same event shapes with `other` as the paired
        // creature — `other` is the omni-tribal seed, so subtype
        // filters (Orc) match; color/type/power filters may not, in
        // which case the condition verdict reports no-fire (same
        // posture as other filter-bearing conditions).
        TC::SelfBecomesBlockedBy { .. } =>
            GE::CreatureBlocked { attacker: source, blockers: vec![other] },
        TC::SelfBlocksOrBecomesBlockedBy { .. } =>
            GE::CreatureBlocks { blocker: source, attacker: other },
        TC::SelfBecomesTapped => GE::Tapped { object_id: source },
        // Filtered tapped: `other` (the omni-tribal seed) is the
        // tapped object, same posture as the other filtered forms.
        TC::BecomesTapped { .. } => GE::Tapped { object_id: other },
        TC::SelfSpecializes => GE::Specialized { object_id: source },
        // The face gate reads live state (probe source's visible_face
        // is 0), so to_face: Some(1) conditions report no-fire — same
        // posture as other state-dependent filters.
        TC::SelfTransforms { .. } => GE::Transformed { object_id: source },
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
            targets: crate::targets::TargetSelection::new(), mana_spent: 0 },
        TC::SpellCastFromZone { caster, .. } => GE::SpellCast {
            object_id: stack_spell, card_id: 0, controller: who(caster),
            targets: crate::targets::TargetSelection::new(), mana_spent: 0 },
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
        // Aura host trigger: synthesize the INNER condition's event on
        // `source` so the effect still executes (panic-surfacing via the
        // fallback). The live `matches` keys on `source.attached_to`,
        // which the minimal probe doesn't seed, so the verdict honestly
        // reports no-fire — same posture as other attachment-dependent
        // checks.
        TC::AttachedCreatureDoes { condition } =>
            return synth_event(condition, source, controller, stack_spell, dummy),
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
        // Place the source where the ability is activated FROM: the
        // graveyard for Unearth/Embalm/"return this from your graveyard"
        // (Firewing Phoenix), the hand for cycling/channel — otherwise a
        // "do X to/with this card in zone Z" effect finds no source in Z
        // and reads as a silent no-op.
        let src_zone = match ability.activation_zone {
            crate::registry::ActivationZone::Graveyard => Zone::Graveyard(0),
            crate::registry::ActivationZone::Hand => Zone::Hand(0),
            crate::registry::ActivationZone::Battlefield => Zone::Battlefield,
        };
        let mut src_obj = GameObject::new(src, 0, src_zone, card_id, chars);
        // Transform back-face seeding, mirroring the triggered-probe
        // source: "{cost}: Transform this" activations must flip
        // visible_face instead of hitting the no-back fallback.
        if let Some(back) = def.alternate_face.as_ref().and_then(|a| a.as_transform()) {
            src_obj.back_face_characteristics = Some(back.characteristics.clone());
        }
        state.objects.insert(src_obj);
        state.currently_resolving = Some(src);
        // Removable counters on the source (see seed_source_counters):
        // "{cost}, Remove a +1/+1 counter: …" (Spike Feeder, Fertilid)
        // acts on a real counter instead of no-op'ing.
        seed_source_counters(&mut state, src);
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

/// Top-level variant name of an effect (e.g. `DealDamage`, `Destroy`),
/// derived from its Debug form. Used by the triage classifier to bucket
/// silent no-ops by mechanism.
fn effect_label(e: &Effect) -> String {
    let s = format!("{e:?}");
    s.split(|c: char| c == '{' || c == '(' || c == ' ' || c == '[')
        .next().unwrap_or("?").to_string()
}

/// DIAGNOSTIC (not a gate): for each ability whose probe is a silent
/// no-op, return `(surface, first_effect_label)` — surface ∈
/// {`spell`,`trig`,`act`}. Mirrors the three probes' verdict logic
/// exactly (same seeding, same synth_event skip-on-None for triggers) so
/// the buckets line up 1:1 with the gate's flag set. Lets the audit see
/// WHAT each residual no-op does, so an effect that should ALWAYS delta
/// (DealDamage/Destroy/Draw/Mill/CreateToken) standing out in the
/// buckets is a real-bug candidate, not a harness limit.
pub fn noop_mechanisms(reg: &CardRegistry, card_id: CardId) -> Vec<(&'static str, String)> {
    let mut out = Vec::new();
    let Some(def) = reg.get(card_id) else { return out; };
    // ---- spell ----
    if let Some(spell) = def.spell_ability.as_ref() {
        let mut state = populated_state(reg);
        let src = state.allocate_object_id();
        state.objects.insert(GameObject::new(
            src, 0, Zone::Stack, card_id, Characteristics::default()));
        state.currently_resolving = Some(src);
        let stack_spell = add_dummy_stack_spell(&mut state);
        let dummy = first_battlefield_creature(&state, 0);
        let targets = selection_for(&state, &spell.target_requirements, dummy, stack_spell);
        let entry = StackEntry::new_spell(
            src, 0, card_id, Characteristics::default(), targets, Vec::new(), Some(3));
        let before = Snapshot::capture(&state);
        let effects = (spell.effect)(&state, &entry, reg);
        let label = effects.first().map(effect_label).unwrap_or_else(|| "?".into());
        let had = !effects.is_empty();
        for eff in flatten_sequences(effects) {
            eff.execute(&mut state);
            if state.pending_choice.is_some() { break; }
        }
        if had && before == Snapshot::capture(&state) { out.push(("spell", label)); }
    }
    // ---- triggered (mirror probe_triggered) ----
    for ability in &def.triggered_abilities {
        let mut state = populated_state(reg);
        let src = state.allocate_object_id();
        let chars = Characteristics {
            types: TypeLine::CREATURE.into(),
            power: Some(PtValue::Fixed(2)), toughness: Some(PtValue::Fixed(2)),
            ..Default::default()
        };
        let from_graveyard = matches!(ability.trigger_condition,
            crate::triggers::TriggerCondition::SelfDies)
            || (ability.trigger_zones.iter().any(|z| matches!(z, Zone::Graveyard(_)))
                && !ability.trigger_zones.iter().any(|z| matches!(z, Zone::Battlefield)));
        let src_zone = if from_graveyard { Zone::Graveyard(0) } else { Zone::Battlefield };
        let mut src_obj = GameObject::new(src, 0, src_zone, card_id, chars);
        if let Some(back) = def.alternate_face.as_ref().and_then(|a| a.as_transform()) {
            src_obj.back_face_characteristics = Some(back.characteristics.clone());
        }
        state.objects.insert(src_obj);
        state.currently_resolving = Some(src);
        seed_source_counters(&mut state, src);
        if let crate::triggers::TriggerCondition::CounterAdded { kind, chapter, .. } =
            &ability.trigger_condition
        {
            state.place_counters(crate::replacement::CounterTarget::Object(src),
                kind.unwrap_or(crate::types::CounterKind::Lore), chapter.unwrap_or(1));
        }
        let stack_spell = add_dummy_stack_spell(&mut state);
        let dummy = first_battlefield_creature(&state, 0);
        let targets = selection_for(&state, &ability.target_requirements, dummy, stack_spell);
        let synth = synth_event(&ability.trigger_condition, src, 0, stack_spell, dummy);
        let event = synth.clone().unwrap_or(crate::events::GameEvent::EntersBattlefield {
            object_id: src, from_zone: Zone::Stack, was_cast: true });
        let pt = crate::triggers::PendingTrigger {
            effect_override: None, source: src, trigger_id: ability.id,
            controller: 0, trigger_event: event, targets,
        };
        let before = Snapshot::capture(&state);
        let effects = (ability.effect)(&state, &pt, reg);
        let label = effects.first().map(effect_label).unwrap_or_else(|| "?".into());
        let had = !effects.is_empty();
        for eff in flatten_sequences(effects) {
            eff.execute(&mut state);
            if state.pending_choice.is_some() { break; }
        }
        if synth.is_some() && had && before == Snapshot::capture(&state) {
            out.push(("trig", label));
        }
    }
    // ---- activated (mirror probe_activated) ----
    for (i, ability) in def.activated_abilities.iter().enumerate() {
        let mut state = populated_state(reg);
        let src = state.allocate_object_id();
        let chars = Characteristics {
            types: TypeLine::CREATURE.into(),
            power: Some(PtValue::Fixed(2)), toughness: Some(PtValue::Fixed(2)),
            ..Default::default()
        };
        let src_zone = match ability.activation_zone {
            crate::registry::ActivationZone::Graveyard => Zone::Graveyard(0),
            crate::registry::ActivationZone::Hand => Zone::Hand(0),
            crate::registry::ActivationZone::Battlefield => Zone::Battlefield,
        };
        let mut src_obj = GameObject::new(src, 0, src_zone, card_id, chars);
        if let Some(back) = def.alternate_face.as_ref().and_then(|a| a.as_transform()) {
            src_obj.back_face_characteristics = Some(back.characteristics.clone());
        }
        state.objects.insert(src_obj);
        state.currently_resolving = Some(src);
        seed_source_counters(&mut state, src);
        let stack_spell = add_dummy_stack_spell(&mut state);
        let dummy = first_battlefield_creature(&state, 0);
        let targets = selection_for(&state, &ability.target_requirements, dummy, stack_spell);
        let ctx = crate::registry::ActivationContext {
            source: src, controller: 0, ability_index: i, targets,
            x_value: Some(3), card_id,
        };
        let before = Snapshot::capture(&state);
        let effects = (ability.effect)(&state, &ctx, reg);
        let label = effects.first().map(effect_label).unwrap_or_else(|| "?".into());
        let had = !effects.is_empty();
        for eff in flatten_sequences(effects) {
            eff.execute(&mut state);
            if state.pending_choice.is_some() { break; }
        }
        if had && before == Snapshot::capture(&state) { out.push(("act", label)); }
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
    let tribes = tribal_subtypes(reg);
    // Battlefield ids captured per player for the combat seeding below.
    let mut omni_ids  = [crate::objects::NULL_OBJECT_ID; 2];
    let mut big_ids   = [crate::objects::NULL_OBJECT_ID; 2];
    let mut first_2x2 = [crate::objects::NULL_OBJECT_ID; 2];
    let mut dead_ids  = [crate::objects::NULL_OBJECT_ID; 2];
    for p in 0..2 {
        for (i, (cs, cost)) in colors.into_iter().enumerate() {
            let id = make_creature(&mut state, p, cs, cost);
            if i == 0 { first_2x2[p as usize] = id; }
        }
        // An "omni-tribal" creature carrying every common creature
        // subtype, so subtype COUNTS ("X = Goblins you control") and
        // tribal lords find a referent. One on the battlefield, plus a
        // copy in the library (subtype TUTORS: "search for an Elf card")
        // and graveyard (subtype recursion: "return a Zombie card").
        omni_ids[p as usize] =
            make_tribal_creature(&mut state, p, Zone::Battlefield, &tribes);
        // A big 8/8 creature so power/toughness-gated conditions ("if you
        // control a creature with power 4 or greater" — Saga chapters,
        // power-matters triggers) are satisfied; the 2/2s cover the
        // low-power / max-power side.
        // LEGENDARY so "other legendary creatures you control" sweeps
        // (Casal-class) find a referent; one per player, distinct
        // boards, so the legend-rule SBA never trips.
        let big = state.allocate_object_id();
        state.objects.insert(GameObject::new(big, p, Zone::Battlefield, 0, Characteristics {
            types: TypeLine::CREATURE.into(),
            supertypes: crate::types::SupertypeSet::new()
                .with(crate::types::SupertypeSet::LEGENDARY),
            power: Some(PtValue::Fixed(8)),
            toughness: Some(PtValue::Fixed(8)),
            ..Default::default()
        }));
        big_ids[p as usize] = big;
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
        // FIRST artifact TAPPED: selection_for picks the lowest matching
        // id, so "untap target artifact" (Voltaic Key) must find a
        // tapped one first; the second, untapped artifact keeps
        // "if you control an artifact" + sacrifice referents honest.
        let tapped_artifact = make_permanent(&mut state, p, TypeLine::ARTIFACT.into());
        state.objects.get_mut(tapped_artifact).map(|o| o.tap());
        make_permanent(&mut state, p, TypeLine::ARTIFACT.into());
        make_permanent(&mut state, p, TypeLine::ENCHANTMENT.into());
        for basic in ["Plains", "Island", "Swamp", "Mountain", "Forest"] {
            make_basic_land(&mut state, reg, p, basic);
        }
        // An UNTAPPED NONBASIC land AFTER the basics (higher id):
        // "damage per nonbasic land" (Price of Progress) finds a
        // referent, while every untap-land effect still first-picks a
        // tapped basic and deltas. ("Tap target land" first-picks a
        // tapped basic and no-ops — allowlisted residue; the untap
        // class is an order of magnitude bigger.)
        make_permanent(&mut state, p, TypeLine::LAND.into());
        // Stock library + graveyard with EVERY card type so type-tutors
        // ("search for an enchantment/artifact/planeswalker card") and
        // reanimation find matches — type-less dummies no-op them.
        let kinds = [TypeLine::CREATURE, TypeLine::LAND, TypeLine::INSTANT,
            TypeLine::SORCERY, TypeLine::ENCHANTMENT, TypeLine::ARTIFACT,
            TypeLine::PLANESWALKER];
        let mut lib = Vec::new();
        for k in kinds { lib.push(make_typed_card(&mut state, p, Zone::Library(p), k.into())); }
        lib.push(make_tribal_creature(&mut state, p, Zone::Library(p), &tribes));
        state.player_mut(p).library_top_to_bottom = lib;
        // Hand: a GREEN ARTIFACT CREATURE and a BASIC LAND (not
        // type-less dummies) so "put a [type] card from your hand
        // onto the battlefield" effects find a candidate across the
        // common filters (creature / artifact / colored creature /
        // basic land). Exactly two cards — hand-SIZE reads (Iron
        // Maiden, Wheel of Torture's 3-minus-hand) must not shift.
        {
            let id = state.allocate_object_id();
            let chars = Characteristics {
                types: crate::types::TypeLine(
                    TypeLine::CREATURE | TypeLine::ARTIFACT).into(),
                colors: crate::types::ColorSet::green(),
                power: Some(crate::types::PtValue::Fixed(2)),
                toughness: Some(crate::types::PtValue::Fixed(2)),
                ..Default::default()
            };
            state.objects.insert(GameObject::new(id, p, Zone::Hand(p), 0, chars));
        }
        {
            let id = state.allocate_object_id();
            let chars = Characteristics {
                types: TypeLine::LAND.into(),
                supertypes: crate::types::SupertypeSet::new()
                    .with(crate::types::SupertypeSet::BASIC),
                ..Default::default()
            };
            state.objects.insert(GameObject::new(id, p, Zone::Hand(p), 0, chars));
        }
        make_typed_card(&mut state, p, Zone::Graveyard(p), TypeLine::CREATURE.into());
        make_typed_card(&mut state, p, Zone::Graveyard(p), TypeLine::LAND.into());
        make_typed_card(&mut state, p, Zone::Graveyard(p), TypeLine::ARTIFACT.into());
        make_typed_card(&mut state, p, Zone::Graveyard(p), TypeLine::INSTANT.into());
        make_typed_card(&mut state, p, Zone::Graveyard(p), TypeLine::SORCERY.into());
        make_typed_card(&mut state, p, Zone::Graveyard(p), TypeLine::ENCHANTMENT.into());
        // A multi-type ARTIFACT CREATURE card so "return target artifact
        // creature card from your graveyard" (Skeleton Shard) and other
        // type-intersection graveyard returns/reanimation find a match;
        // the single-type cards above miss the AND of two types.
        make_typed_card(&mut state, p, Zone::Graveyard(p),
            crate::types::TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE).into());
        let dead = make_tribal_creature(&mut state, p, Zone::Graveyard(p), &tribes);
        dead_ids[p as usize] = dead;
    }
    // COMBAT seeding (mirrors the tribal/keyword seeding): a live
    // CombatState so combat-status filters ("each attacking creature",
    // "target blocking creature", "creature attacking you") find
    // referents instead of silently matching nothing. Harness fiction:
    // BOTH players have declared attackers in the one CombatState —
    // rules-impossible (one combat has one attacking player) but the
    // filter code only reads the vecs, and it gives every combat
    // filter a referent regardless of which side the probe card's
    // controller is on. Per player: the omni-tribal creature attacks
    // (so "attacking <subtype>" / "attacking + keyword" match) and is
    // blocked by an opposing 2/2; the 8/8 attacks unblocked (a
    // keyword-less attacker, so "attacking without flying" matches).
    // Bystanders (the other color 2/2s) stay out of combat so
    // NotAttacking keeps referents too.
    {
        use crate::combat::{AttackerInfo, BlockerInfo, CombatState};
        let mut combat = CombatState::new();
        for p in 0..2usize {
            let foe = 1 - p;
            combat.attackers.push(AttackerInfo {
                object_id: omni_ids[p],
                defending_player: foe as PlayerId,
                defending_planeswalker: None,
                blocked_by: vec![first_2x2[foe]],
                is_blocked: true,
            });
            combat.attackers.push(AttackerInfo {
                object_id: big_ids[p],
                defending_player: foe as PlayerId,
                defending_planeswalker: None,
                blocked_by: Vec::new(),
                is_blocked: false,
            });
            combat.blockers.push(BlockerInfo {
                object_id: first_2x2[foe],
                blocking: omni_ids[p],
            });
        }
        state.combat = Some(combat);
    }
    // EVENT-HISTORY seeding (companion to the combat seeding): this-turn
    // event-log entries so `*_this_turn` conditions (Boast "attacked this
    // turn", morbid "a creature died this turn", "an opponent lost life
    // this turn", "entered this turn") find referents. The probe state
    // has turn_event_log_start = 0, so everything pushed here is in the
    // live turn's slice and the LAST-turn slice stays empty —
    // deliberately NO SpellCast events, which would flip the werewolf
    // "no spells were cast last turn" transform conditions. The dead
    // referent is the seeded GRAVEYARD tribal creature (coherent: it's
    // in the graveyard and "died this turn", and carries subtypes for
    // "a <tribe> died this turn" counts).
    {
        use crate::combat::DefendingEntity;
        use crate::events::GameEvent;
        for p in 0..2usize {
            let foe = (1 - p) as PlayerId;
            state.event_log.push(GameEvent::CreatureAttacks {
                attacker: omni_ids[p],
                defending: DefendingEntity::Player(foe),
            });
            state.event_log.push(GameEvent::CreatureAttacks {
                attacker: big_ids[p],
                defending: DefendingEntity::Player(foe),
            });
            state.event_log.push(GameEvent::Dies { object_id: dead_ids[p] });
            state.event_log.push(GameEvent::LifeLost { player: p as PlayerId, amount: 2 });
            state.event_log.push(GameEvent::LifeGained { player: p as PlayerId, amount: 2 });
            state.event_log.push(GameEvent::EntersBattlefield {
                object_id: omni_ids[p],
                from_zone: Zone::Hand(p as PlayerId),
                was_cast: false,
            });
        }
    }
    state
}

/// The interned ids of the common creature subtypes that show up in
/// tribal counts/tutors/lords. Looked up read-only from the registry;
/// subtypes no card ever interned are skipped (they match nothing
/// anyway). Piling them all onto one creature lets a single seed satisfy
/// any "X you control" / "search for an X" filter.
fn tribal_subtypes(reg: &CardRegistry) -> crate::types::SubtypeSet {
    const NAMES: &[&str] = &[
        "Goblin", "Elf", "Zombie", "Human", "Soldier", "Warrior", "Wizard",
        "Cleric", "Rogue", "Beast", "Spirit", "Elemental", "Vampire",
        "Merfolk", "Dragon", "Angel", "Sliver", "Ally", "Rebel", "Knight",
        "Dwarf", "Faerie", "Kithkin", "Giant", "Saproling", "Zubera",
        "Ninja", "Snake", "Wolf", "Cat", "Bird", "Demon", "Druid", "Shaman",
        "Myr", "Construct", "Golem", "Spider", "Treefolk", "Wall", "Insect",
        "Samurai", "Minotaur", "Skeleton", "Fungus", "Horror", "Pirate", "Orc",
    ];
    let mut s = crate::types::SubtypeSet::default();
    for n in NAMES {
        if let Some(sym) = reg.interner().lookup(n) { s.0.insert(sym); }
    }
    s
}

fn make_tribal_creature(state: &mut GameState, owner: PlayerId, zone: Zone,
    tribes: &crate::types::SubtypeSet) -> ObjectId {
    let id = state.allocate_object_id();
    // Keyword seeding mirrors the tribal seeding: one creature carrying
    // the keywords that keyword-filtered sweeps/targets/tutors look for
    // ("each creature with flying", "with deathtouch, hexproof, reach,
    // or trample", …) so they find a referent. Deliberately NO
    // Hexproof/Shroud — the probe's selection prefers opponent
    // creatures, and an untargetable seed would falsely no-op every
    // targeted effect. The plain `make_creature` seeds stay keyword-
    // less so "without flying" filters keep a referent too.
    use crate::effects::KeywordAbility as KA;
    let chars = Characteristics {
        types: TypeLine::CREATURE.into(),
        subtypes: tribes.clone(),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![
            KA::Flying, KA::Shadow, KA::Horsemanship, KA::Defender,
            KA::Deathtouch, KA::Reach, KA::Trample,
        ],
        ..Default::default()
    };
    state.objects.insert(GameObject::new(id, owner, zone, 0, chars));
    id
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
        types: TypeLine::LAND.into(), subtypes,
        supertypes: crate::types::SupertypeSet::new()
            .with(crate::types::SupertypeSet::BASIC),
        ..Default::default()
    };
    let mut obj = GameObject::new(id, controller, Zone::Battlefield, 0, chars);
    // Enter tapped so "untap target land / Forest" (mana dorks) shows a
    // delta; creatures stay untapped for tap-target effects. (One land
    // per player is left UNTAPPED at the call site so "tap target
    // land" — Rishadan Port — keeps a referent too.)
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
    // An opponent's ACTIVATED ABILITY entry beneath it (source: an
    // opponent artifact) so "counter target activated ability"
    // (TargetFilter::AbilityOnStack) finds a candidate — ability
    // entries aren't stack-zone objects, so the spell above can't
    // serve. Noncreature source on purpose: "from a noncreature
    // source" filters must also match.
    let ability_src = state.allocate_object_id();
    state.objects.insert(GameObject::new(
        ability_src, 1, Zone::Battlefield, 0,
        Characteristics { types: TypeLine::ARTIFACT.into(), ..Default::default() },
    ));
    let entry_id = state.allocate_object_id();
    state.stack.push(StackEntry::new_activated_ability(
        entry_id, ability_src, 1, /*card_id=*/ 0, /*ability_id=*/ 0,
        "probe dummy ability".into(), TargetSelection::new(), Vec::new(), None,
    ));
    id
}

fn make_typed_card(state: &mut GameState, owner: PlayerId, zone: Zone, types: TypeLine) -> ObjectId {
    let id = state.allocate_object_id();
    let chars = Characteristics { types, ..Default::default() };
    state.objects.insert(GameObject::new(id, owner, zone, 0, chars));
    id
}

/// Seed a few removable counters on the probe source so "remove a +1/+1
/// / -1/-1 / charge counter from this" abilities (Triskelion, the Spike
/// creatures, Bristlebane's counter-removal trigger, Fertilid) act on a
/// real counter instead of reading as a silent no-op. A real activation
/// or trigger of those abilities can only happen with the counter already
/// present, so a delta here is genuine; abilities that don't touch the
/// source's counters are unaffected (seeding never manufactures a delta
/// on its own — it only changes the BEFORE baseline, which both snapshots
/// share).
fn seed_source_counters(state: &mut GameState, src: ObjectId) {
    use crate::types::CounterKind as CK;
    use crate::replacement::CounterTarget;
    for kind in [CK::PlusOnePlusOne, CK::MinusOneMinusOne, CK::Charge] {
        state.place_counters(CounterTarget::Object(src), kind, 2);
    }
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
    // Exclude the object currently resolving: a spell/ability can't pick
    // the very object resolving it as its target. Without this, "counter
    // target spell" / "copy target spell" chose the probe's own source
    // (lowest-id stack object, with no stack ENTRY) and Counter found
    // nothing to remove — a probe artifact that no-op'd ~90 counters.
    let resolving = state.currently_resolving;
    let mut ids: Vec<ObjectId> = state.objects.iter()
        .filter(|o| o.zone != Zone::Library(o.owner))
        .filter(|o| Some(o.id) != resolving)
        .map(|o| o.id).collect();
    ids.push(stack_spell);
    // Ability stack entries aren't GameObjects — push their entry ids
    // so AbilityOnStack requirements ("counter target activated
    // ability") find the seeded dummy ability.
    ids.extend(state.stack.iter()
        .filter(|e| !e.is_spell())
        .map(|e| e.id));
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
            if req.matches_choice(&choice, state, stack_spell, 0) { return Some(choice); }
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
            if req.matches_choice(&choice, state, stack_spell, 0) { return Some(choice); }
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
