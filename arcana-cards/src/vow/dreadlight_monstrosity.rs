//! Dreadlight Monstrosity — `{4}{U}{U}` 5/5 Crab Horror with Ward {2}.
//!
//! Oracle:
//! * Ward {2}.
//! * {3}{U}{U}: This creature can't be blocked this turn. Activate
//!   only if you own a card in exile.
//!
//! The "Activate only if you own a card in exile" precondition is not
//! expressible with the available activation conditions, so it is a
//! GAP; the can't-be-blocked effect is implemented.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Dreadlight Monstrosity");
    let crab = reg.interner_mut().intern("Crab");
    let horror = reg.interner_mut().intern("Horror");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(crab);
    subtypes.0.insert(horror);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{U}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(5)),
        keywords: vec![KeywordAbility::Ward(ManaCost::parse("{2}").expect("valid cost"))],
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            // GAP: "Activate only if you own a card in exile" precondition
            // not expressible with the available activation conditions.
            .with_activated_ability(ActivatedAbilityDef {
                text: "{3}{U}{U}: This creature can't be blocked this turn.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{3}{U}{U}").unwrap(),
                    ..ActivationCost::default()
                },
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: cant_be_blocked,
            }),
    )
}

fn cant_be_blocked(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::CantBeBlocked {
        target: ctx.source,
        duration: Duration::EndOfTurn,
    }]
}
