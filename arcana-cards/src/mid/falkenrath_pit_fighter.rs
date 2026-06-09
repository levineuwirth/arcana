//! Falkenrath Pit Fighter — `{R}` 2/1 red Vampire Warrior. "{1}{R}, Discard a
//! card, Sacrifice a Vampire: Draw two cards. Activate only if an opponent lost
//! life this turn."
//!
//! Cost wired: "Discard a card" via `discard_other` (any-card filter),
//! "Sacrifice a Vampire" via `sacrifice_other` (Vampire filter), and
//! "Activate only if an opponent lost life this turn" via
//! `activation_condition` (conditions::an_opponent_lost_life_this_turn).
//! GAP: sacrifice_other enumeration always excludes the source, but the real
//! card's "Sacrifice a Vampire" may sacrifice Falkenrath Pit Fighter itself.

use arcana_core::conditions;
use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, ObjectId};
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::types::{CardId, ColorSet, PlayerId, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Falkenrath Pit Fighter");
    let vampire = reg.interner_mut().intern("Vampire");
    let warrior = reg.interner_mut().intern("Warrior");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(vampire);
    subtypes.0.insert(warrior);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{1}{R}, Discard a card, Sacrifice a Vampire: Draw two cards. Activate only if an opponent lost life this turn.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{1}{R}").unwrap(),
                    // "Discard a card" — any card from hand.
                    discard_other: Some(ObjectFilter::default()),
                    // "Sacrifice a Vampire" — chosen Vampire you control.
                    // GAP: enumeration excludes the source; the real card may
                    // sacrifice itself.
                    sacrifice_other: Some(ObjectFilter::new().with_subtype_sym(vampire)),
                    // "Activate only if an opponent lost life this turn."
                    activation_condition: Some(cond_opponent_lost_life),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: pit_fighter_draw,
            }),
    )
}

fn cond_opponent_lost_life(state: &GameState, _source: ObjectId, you: PlayerId, _reg: &CardRegistry) -> bool {
    conditions::an_opponent_lost_life_this_turn(state, you)
}

fn pit_fighter_draw(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::DrawCards { player: ctx.controller, count: 2 }]
}
