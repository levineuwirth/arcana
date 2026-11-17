//! Plays as candidates at the top of a search (A1.5ci, D37).
//!
//! A spell is cast only from mana already floating, and at priority only a
//! mana ability floats it ([`potential_actions`]' doc). With an empty pool
//! the legal list at priority is the untapped mana abilities, the pass, any
//! land drop and the casts that need no mana, so a search that takes that
//! list as its candidates casts a three-drop only by choosing a lone tap
//! three decisions running, each against passing; a tap barely moves a
//! rollout, and those choices came out near chance (handoff 14). The
//! candidates [`top_candidates`] gives a search instead are:
//!
//! * every legal action that is not a mana ability;
//! * every cast or activation that tapping would make legal now and that is
//!   not legal now, bundled with the mana abilities that make it legal: a
//!   *play*, one for each variant the legal list would enumerate once the
//!   mana is there (targets, modes, kicker, each payable X), its taps
//!   planned for that exact variant by the engine's auto-tap planner
//!   ([`auto_tap_for`]);
//! * a bare mana ability only inside a choice that asks for mana
//!   ([`pending_choice_asks_mana`]), where every legal action is a
//!   candidate, as before.
//!
//! A play is scored as its actions applied in order and then rolled out as
//! any candidate is. Mana abilities use no stack and the player keeps
//! priority, so nothing intervenes between the taps and the cast. A policy
//! that chooses a play returns its first action and keeps the rest
//! ([`KeptPlan`]).

use arcana_core::actions::Action;
use arcana_core::engine::{step, EngineYield};
use arcana_core::legal_actions::{
    activatable_sources, auto_tap_activate_sequence, auto_tap_for, auto_tap_sequence,
    is_mana_activation, legal_actions, pending_choice_asks_mana, playable_cards,
    potential_actions,
};
use arcana_core::objects::ObjectId;
use arcana_core::registry::CardRegistry;
use arcana_core::state::GameState;
use arcana_core::turn::{Phase, Step};
use arcana_core::types::PlayerId;

/// One candidate at the top of a search: a legal action, or a play.
///
/// `index` places it in one index space over both kinds: below
/// `legal.len()` it is the position of the action in the legal list; a
/// play is numbered `legal.len() + k`, `k` its place among the plays. A
/// host that applies an action by its index into the legal list refuses a
/// play's index as out of range, so a play is applied through its `plan`.
#[derive(Clone, Debug, PartialEq)]
pub struct Candidate {
    pub index: usize,
    /// The actions it takes, in order: the one legal action, or a play's
    /// mana abilities followed by its cast or activation.
    pub plan: Vec<Action>,
}

impl Candidate {
    /// Is this a play (taps then a cast or activation), not one legal action?
    pub fn is_play(&self, legal_len: usize) -> bool {
        self.index >= legal_len
    }
    /// Is this the lone pass?
    pub fn is_pass(&self) -> bool {
        self.plan.len() == 1 && self.plan[0].is_pass()
    }
}

/// Every legal action as a candidate, in legal order: the pool every search
/// took before A1.5ci, and the pool inside a choice that asks for mana.
pub fn every_action(legal: &[Action]) -> Vec<Candidate> {
    legal.iter().cloned().enumerate()
        .map(|(index, a)| Candidate { index, plan: vec![a] })
        .collect()
}

/// The candidates a search chooses among at `decider`'s decision (see the
/// module doc): the legal actions that are not mana abilities, in legal
/// order, then the plays. Every legal action, as before, where no mana
/// ability is on offer, inside a choice that asks for mana, and where
/// setting the mana abilities aside would leave nothing but conceding.
pub fn top_candidates(
    state: &GameState, registry: &CardRegistry, decider: PlayerId, legal: &[Action],
) -> Vec<Candidate> {
    if pending_choice_asks_mana(state, registry) {
        return every_action(legal);
    }
    let mana: Vec<bool> = legal.iter().map(|a| is_mana_activation(state, a, registry)).collect();
    if !mana.iter().any(|&m| m) {
        return every_action(legal);
    }
    let mut out: Vec<Candidate> = legal.iter().cloned().enumerate()
        .filter(|&(i, _)| !mana[i])
        .map(|(index, a)| Candidate { index, plan: vec![a] })
        .collect();
    if out.iter().all(|c| c.plan[0].is_concede()) {
        return every_action(legal);
    }
    let n = legal.len();
    out.extend(plays(state, registry, decider, legal).plays.into_iter().enumerate()
        .map(|(k, plan)| Candidate { index: n + k, plan }));
    out
}

/// What [`plays`] found: each play's actions, and the variants tapping out
/// would make legal for which the planner found no taps (reported, and
/// left out of the candidates).
#[derive(Clone, Debug, Default)]
pub struct PlaySet {
    pub plays: Vec<Vec<Action>>,
    pub unplanned: Vec<Action>,
}

/// The plays open to `decider` now: every cast or non-mana activation that
/// tapping out would make legal and that is not legal now, one per variant
/// as the legal list would enumerate it, each with the mana abilities that
/// make that variant legal, planned on a clone by [`auto_tap_for`] with
/// the cost's colors as its guide. The variants come from
/// [`potential_actions`]; where `decider` has a source with more than one
/// mana ability on offer (a dual land), whose second color that list does
/// not see, also from [`playable_cards`] and [`activatable_sources`],
/// which do. Changes nothing.
pub fn plays(
    state: &GameState, registry: &CardRegistry, decider: PlayerId, legal: &[Action],
) -> PlaySet {
    let now: Vec<Action> = legal.iter()
        .filter(|a| is_play_kind(state, registry, a))
        .map(variant_key)
        .collect();
    let mut variants: Vec<Action> = Vec::new();
    let add = |variants: &mut Vec<Action>, a: &Action| {
        if !is_play_kind(state, registry, a) { return; }
        let k = variant_key(a);
        if !now.contains(&k) && !variants.contains(&k) { variants.push(k); }
    };
    for a in potential_actions(state, decider, registry) {
        add(&mut variants, &a);
    }
    if has_flexible_source(state, registry, legal) {
        let covered = |variants: &[Action], id: ObjectId| {
            legal.iter().chain(variants.iter()).any(|a| refers_to(a, id))
        };
        let mut hand = playable_cards(state, decider, registry);
        hand.sort_unstable();
        let mut sources: Vec<ObjectId> =
            activatable_sources(state, decider, registry).into_iter().collect();
        sources.sort_unstable();
        let found: Vec<(ObjectId, bool)> = hand.into_iter().map(|id| (id, false))
            .chain(sources.into_iter().map(|id| (id, true)))
            .collect();
        for (id, ability) in found {
            if covered(&variants, id) { continue; }
            let seq = if ability {
                auto_tap_activate_sequence(state, registry, decider, id)
            } else {
                auto_tap_sequence(state, registry, decider, id)
            };
            let Some(mut taps) = seq else { continue };
            if taps.last().is_some_and(|a| refers_to(a, id)) { taps.pop(); }
            let Some((_, after)) = replay(state, registry, decider, &taps) else { continue };
            for a in after.iter().filter(|a| refers_to(a, id)) {
                add(&mut variants, a);
            }
        }
    }

    // Plan the taps once for each object, ability and X, and give every
    // variant legal after them those taps; plan the rest one by one.
    let mut groups: Vec<(VariantGroup, Vec<Action>)> = Vec::new();
    for v in variants {
        let g = group_of(&v);
        match groups.iter_mut().find(|(k, _)| *k == g) {
            Some((_, vs)) => vs.push(v),
            None => groups.push((g, vec![v])),
        }
    }
    let mut out = PlaySet::default();
    for (g, keys) in groups {
        let cost = match g.object {
            VariantObject::Card(id) =>
                state.objects.get(id).and_then(|o| o.characteristics.mana_cost.clone()),
            VariantObject::Ability(..) => None,
        };
        let shared = plan_taps(state, registry, decider, cost.as_ref(),
            |a| keys.contains(&variant_key(a)));
        for key in &keys {
            let from_shared = shared.as_ref().and_then(|(taps, after)| {
                after.iter().find(|a| variant_key(a) == *key).map(|a| (taps.clone(), a.clone()))
            });
            let planned = from_shared.or_else(|| {
                plan_taps(state, registry, decider, cost.as_ref(), |a| variant_key(a) == *key)
                    .and_then(|(taps, after)| after.into_iter()
                        .find(|a| variant_key(a) == *key).map(|a| (taps, a)))
            });
            match planned {
                Some((mut taps, play)) => { taps.push(play); out.plays.push(taps); }
                None => out.unplanned.push(key.clone()),
            }
        }
    }
    out
}

/// [`auto_tap_for`] for the actions `is_play` matches, without the play it
/// may append, replayed on a clone: the taps and the legal list after them.
/// `None` without a tap (the play would be legal now) or when the planner
/// finds none.
fn plan_taps(
    state: &GameState, registry: &CardRegistry, decider: PlayerId,
    cost: Option<&arcana_core::mana::ManaCost>, is_play: impl Fn(&Action) -> bool,
) -> Option<(Vec<Action>, Vec<Action>)> {
    let mut taps = auto_tap_for(state, registry, decider, cost, &is_play)?;
    if taps.last().is_some_and(&is_play) { taps.pop(); }
    if taps.is_empty() { return None; }
    let (_, after) = replay(state, registry, decider, &taps)?;
    Some((taps, after))
}

/// Apply `taps` in order to a clone of `state`, each only if it is in the
/// legal list of the moment and `decider` decides again after it. The
/// state and legal list after the last; `None` if any step fails that.
fn replay(
    state: &GameState, registry: &CardRegistry, decider: PlayerId, taps: &[Action],
) -> Option<(GameState, Vec<Action>)> {
    let mut sim = state.clone();
    let mut now = legal_actions(&sim, registry);
    for t in taps {
        if !now.contains(t) { return None; }
        let (s, y) = step(sim, t.clone(), registry);
        sim = s;
        match y {
            EngineYield::PendingDecision { player, legal_actions, .. } if player == decider =>
                now = legal_actions,
            _ => return None,
        }
    }
    Some((sim, now))
}

/// A cast, or an activation that is not a mana ability: what a play ends in.
fn is_play_kind(state: &GameState, registry: &CardRegistry, a: &Action) -> bool {
    match a {
        Action::CastSpell { .. } => true,
        Action::ActivateAbility { .. } => !is_mana_activation(state, a, registry),
        _ => false,
    }
}

/// The variant an action is, whatever pool pays it: the action with its
/// payment plan's pool assignments cleared. Two pools pay one variant with
/// different assignments; convoke, delve and Phyrexian choices stay in.
pub fn variant_key(a: &Action) -> Action {
    let mut k = a.clone();
    match &mut k {
        Action::CastSpell { mana_payment, .. } | Action::ActivateAbility { mana_payment, .. } =>
            mana_payment.assignments.clear(),
        _ => {}
    }
    k
}

/// Does `a` cast or play the card `id`, or activate an ability of `id`?
fn refers_to(a: &Action, id: ObjectId) -> bool {
    match a {
        Action::CastSpell { object_id, .. } | Action::PlayLand { object_id, .. } => *object_id == id,
        Action::ActivateAbility { source, .. } => *source == id,
        _ => false,
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum VariantObject {
    Card(ObjectId),
    Ability(ObjectId, usize),
}

/// Variants whose taps are planned together: one card or ability at one X.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct VariantGroup {
    object: VariantObject,
    x: Option<u32>,
}

fn group_of(a: &Action) -> VariantGroup {
    match a {
        Action::CastSpell { object_id, x_value, .. } =>
            VariantGroup { object: VariantObject::Card(*object_id), x: *x_value },
        Action::ActivateAbility { source, ability_index, .. } =>
            VariantGroup { object: VariantObject::Ability(*source, *ability_index), x: None },
        _ => unreachable!("only casts and activations are variants"),
    }
}

/// Does one source offer more than one mana ability in `legal` (a dual
/// land, or a creature with a granted mana ability)?
fn has_flexible_source(state: &GameState, registry: &CardRegistry, legal: &[Action]) -> bool {
    let mut seen: Vec<ObjectId> = Vec::new();
    for a in legal {
        if let Action::ActivateAbility { source, .. } = a {
            if is_mana_activation(state, a, registry) {
                if seen.contains(source) { return true; }
                seen.push(*source);
            }
        }
    }
    false
}

/// A play a policy has chosen and is carrying out: the actions still to
/// take, for whom, and where. Part of the policy's state; deterministic
/// under its seed, since it holds only what the policy chose.
#[derive(Clone, Debug, Default)]
pub struct KeptPlan {
    rest: Vec<Action>,
    decider: PlayerId,
    at: Option<(u32, Phase, Step)>,
}

impl KeptPlan {
    /// Keep the rest of `plan` after its first action, which the policy is
    /// about to return for `decider` at `state`.
    pub fn keep(&mut self, plan: &[Action], state: &GameState, decider: PlayerId) {
        self.rest = plan[1..].to_vec();
        self.decider = decider;
        self.at = Some((state.turn.turn_number, state.turn.phase, state.turn.step));
    }

    /// The next planned action, taken off the plan, if it is in `legal`,
    /// the same player decides, and the turn, phase and step are the ones
    /// the plan was made in. Anything else (a new game, another decider, an
    /// action that is not the planned one, a plan already done) drops the
    /// plan and returns `None`, and the policy searches again.
    pub fn next(&mut self, state: &GameState, decider: PlayerId, legal: &[Action]) -> Option<Action> {
        if self.rest.is_empty() { return None; }
        let here = Some((state.turn.turn_number, state.turn.phase, state.turn.step));
        if self.decider != decider || self.at != here || !legal.contains(&self.rest[0]) {
            self.rest.clear();
            return None;
        }
        Some(self.rest.remove(0))
    }

    /// The actions still planned.
    pub fn rest(&self) -> &[Action] {
        &self.rest
    }
}
