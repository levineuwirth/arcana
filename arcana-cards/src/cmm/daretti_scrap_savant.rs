//! Daretti, Scrap Savant — `{3}{R}` Legendary Planeswalker — Daretti.
//! Starting loyalty inferred 3.
//! +2: Discard up to two cards, then draw that many cards.
//! −2: Sacrifice an artifact. If you do, return target artifact card from your
//!   graveyard to the battlefield.
//! −10: You get an emblem with "Whenever an artifact is put into your
//!   graveyard from the battlefield, return that card to the battlefield at
//!   the beginning of the next end step."
//! (Daretti, Scrap Savant can be your commander.)
//!
//! GAP: +2 "discard up to two cards, then draw THAT MANY" — the draw count is
//!   dynamically linked to the number actually discarded ("up to two"); no
//!   discard-then-draw-equal primitive in the demonstrated surface. Declared
//!   with correct +2 cost, effect GAP'd.
//! GAP: −2 requires a sacrifice-an-artifact additional action gating a
//!   controller-relative graveyard return; neither the sac-gate nor the
//!   "your graveyard" target sentinel is expressible. Declared, GAP'd.
//! GAP: −10 emblem carries a triggered recursion ability whose effect (delayed
//!   return at next end step of a specific just-died artifact) is not
//!   expressible as a self-contained EmblemDefinition trigger here. Declared,
//!   GAP'd.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{
    CardId, ColorSet, CounterKind, SubtypeSet, SupertypeSet, TypeLine,
};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Daretti, Scrap Savant");
    let daretti = reg.interner_mut().intern("Daretti");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(daretti);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::PLANESWALKER.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        loyalty: Some(3),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "+2: Discard up to two cards, then draw that many cards.".into(),
                cost: ActivationCost {
                    add_self_counter: Some((CounterKind::Loyalty, 2)),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: plus_two_loot,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "-2: Sacrifice an artifact. If you do, return target artifact card \
                       from your graveyard to the battlefield.".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 2)),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_two_recur,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "-10: You get an emblem with \"Whenever an artifact is put into \
                       your graveyard from the battlefield, return that card to the \
                       battlefield at the beginning of the next end step.\"".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 10)),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_ten_emblem,
            }),
    )
}

fn plus_two_loot(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "discard up to two, then draw that many" — dynamic draw count tied
    // to actual discards is not expressible.
    Vec::new()
}

fn minus_two_recur(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: sacrifice-an-artifact gate + controller-relative graveyard return.
    Vec::new()
}

fn minus_ten_emblem(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: emblem's delayed-return triggered ability not expressible here.
    Vec::new()
}
