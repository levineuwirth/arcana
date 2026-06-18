//! Icing Manipulator — `{1}{G}` 1/3 Human Employee.
//! Each +1/+1 counter on a creature you control is also a Food token.
//! {3}{G}, {T}: Roll two d6; for each odd result, put a +1/+1 counter on a
//! creature of your choice. Sorcery speed.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone, CardDefinition,
    CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Icing Manipulator");
    let human = reg.interner_mut().intern("Human");
    let employee = reg.interner_mut().intern("Employee");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(employee);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };

    // GAP: static "Each +1/+1 counter on a creature you control is also a Food
    // token" — a counter-is-also-a-token static, not a triggered/activated
    // ability, and not expressible.
    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "{3}{G}, {T}: Roll two six-sided dice. For each odd result, put a +1/+1 counter on a creature of your choice. Activate only as a sorcery.".into(),
            cost: ActivationCost {
                mana_cost: ManaCost::parse("{3}{G}").expect("valid cost"),
                tap: true,
                ..ActivationCost::default()
            },
            target_requirements: Vec::new(),
            is_mana_ability: false,
            is_loyalty_ability: false,
            activation_zone: ActivationZone::Battlefield,
            is_instant_speed: false,
            face_gate: None,
            effect: roll_dice,
        }),
    )
}

fn roll_dice(_state: &GameState, _ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: "Roll two six-sided dice; for each odd result, put a +1/+1 counter
    // on a creature of your choice." No dice-rolling effect primitive exists.
    Vec::new()
}
