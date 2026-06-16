//! Narset of the Ancient Way — `{1}{U}{R}{W}` legendary planeswalker,
//! starting loyalty 5. Subtype Narset; white-blue-red.
//!
//! Loyalty abilities:
//! * `+1`: You gain 2 life; add {U}, {R}, or {W} (spend only on a
//!   noncreature spell). (Gain 2 life is functional; the restricted mana
//!   add is omitted — the "spend only on a noncreature spell" restriction
//!   isn't expressible.)
//! * `−2`: Draw a card, then you may discard a card; on a nonland discard,
//!   deal damage equal to its mana value to a creature or planeswalker.
//!   (The draw is functional; the may-discard reflexive damage of variable
//!   amount is omitted.)
//! * `−6`: You get an emblem that deals 2 damage on each noncreature cast.
//!   GAP — emblem creation with a bespoke triggered ability.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, CardDefinition,
    CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, CounterKind, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Narset of the Ancient Way");
    let narset = reg.interner_mut().intern("Narset");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(narset);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{U}{R}{W}").expect("valid cost")),
        colors: ColorSet::red() | ColorSet::blue() | ColorSet::white(),
        types: TypeLine::PLANESWALKER.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        loyalty: Some(5),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "+1: You gain 2 life. Add {U}, {R}, or {W}. Spend this \
                       mana only to cast a noncreature spell."
                    .into(),
                cost: ActivationCost {
                    add_self_counter: Some((CounterKind::Loyalty, 1)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: arcana_core::registry::ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: plus_one_life,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "−2: Draw a card, then you may discard a card. When you \
                       discard a nonland card this way, Narset deals damage \
                       equal to that card's mana value to target creature or \
                       planeswalker."
                    .into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 2)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: arcana_core::registry::ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_two_draw,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "−6: You get an emblem with \"Whenever you cast a \
                       noncreature spell, this emblem deals 2 damage to any \
                       target.\""
                    .into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 6)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: arcana_core::registry::ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_six_gap,
            }),
    )
}

fn plus_one_life(_state: &GameState, ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    // Gain 2 life is functional; the restricted mana add (noncreature-only)
    // is omitted — the spend restriction is not expressible.
    vec![Effect::GainLife { player: ctx.controller, amount: 2 }]
}

fn minus_two_draw(_state: &GameState, ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    // The draw is functional; the may-discard reflexive variable-amount
    // damage is omitted.
    vec![Effect::DrawCards { player: ctx.controller, count: 1 }]
}

fn minus_six_gap(_state: &GameState, _ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: emblem creation with a bespoke noncreature-cast damage trigger.
    Vec::new()
}
