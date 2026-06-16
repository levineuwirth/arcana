//! Personal Decoy — `{5}{W}{U}` Planeswalker — Duck.
//!
//! Statics (GAP'd — not loyalty abilities):
//! * "Personal Decoy enters with a number of loyalty counters equal to
//!   your life total." The printed-loyalty field is a fixed `u32`; a
//!   life-total-derived ETB count isn't expressible. We place a nominal
//!   `loyalty: Some(20)` so the card is a legal planeswalker; the
//!   dynamic ETB count is the GAP.
//! * "If it would leave the battlefield, exile it instead." — leave-the-
//!   battlefield replacement; no demonstrated surface. GAP.
//! * "You can't be attacked." — combat restriction static. GAP.
//!
//! Loyalty abilities (CR 606):
//! * `+1`: You gain 1 life.
//! * `−4`: Draw a card.
//!
//! Both loyalty abilities are fully expressible.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, CounterKind, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Personal Decoy");
    let duck = reg.interner_mut().intern("Duck");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(duck);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{5}{W}{U}").expect("valid cost")),
        colors: ColorSet::white() | ColorSet::blue(),
        types: TypeLine::PLANESWALKER.into(),
        subtypes,
        // GAP: enters with loyalty equal to your life total — dynamic ETB
        // count not expressible; nominal fixed value placed instead.
        loyalty: Some(20),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "+1: You gain 1 life.".into(),
                cost: ActivationCost {
                    add_self_counter: Some((CounterKind::Loyalty, 1)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: plus_one_gain,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "−4: Draw a card.".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 4)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_four_draw,
            }),
    )
}

/// `+1`: gain 1 life.
fn plus_one_gain(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::GainLife {
        player: ctx.controller,
        amount: 1,
    }]
}

/// `−4`: draw a card.
fn minus_four_draw(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::DrawCards {
        player: ctx.controller,
        count: 1,
    }]
}
